//! Configuration management for redisctl
//!
//! Handles configuration loading from files, environment variables, and command-line arguments.
//! Configuration is stored in TOML format with support for multiple named profiles.

#![allow(dead_code)] // Foundation code - will be used in future PRs

use anyhow::{Context, Result};
#[cfg(target_os = "macos")]
use directories::BaseDirs;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tracing::{debug, info, trace, warn};

/// Main configuration structure
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {
    /// Name of the default profile to use when none is specified
    #[serde(default, rename = "default_profile")]
    pub default_profile: Option<String>,
    /// Map of profile name -> profile configuration
    #[serde(default)]
    pub profiles: HashMap<String, Profile>,
}

/// Individual profile configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    /// Type of deployment this profile connects to
    pub deployment_type: DeploymentType,
    /// Connection credentials (flattened into the profile)
    #[serde(flatten)]
    pub credentials: ProfileCredentials,
}

/// Supported deployment types
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum DeploymentType {
    Cloud,
    Enterprise,
}

/// Connection credentials for different deployment types
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ProfileCredentials {
    Cloud {
        api_key: String,
        api_secret: String,
        #[serde(default = "default_cloud_url")]
        api_url: String,
    },
    Enterprise {
        url: String,
        username: String,
        password: Option<String>, // Optional for interactive prompting
        #[serde(default)]
        insecure: bool,
    },
}

impl std::fmt::Display for DeploymentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeploymentType::Cloud => write!(f, "cloud"),
            DeploymentType::Enterprise => write!(f, "enterprise"),
        }
    }
}

impl Profile {
    /// Returns Cloud credentials if this is a Cloud profile
    pub fn cloud_credentials(&self) -> Option<(&str, &str, &str)> {
        match &self.credentials {
            ProfileCredentials::Cloud {
                api_key,
                api_secret,
                api_url,
            } => Some((api_key.as_str(), api_secret.as_str(), api_url.as_str())),
            _ => None,
        }
    }

    /// Returns Enterprise credentials if this is an Enterprise profile
    pub fn enterprise_credentials(&self) -> Option<(&str, &str, Option<&str>, bool)> {
        match &self.credentials {
            ProfileCredentials::Enterprise {
                url,
                username,
                password,
                insecure,
            } => Some((
                url.as_str(),
                username.as_str(),
                password.as_deref(),
                *insecure,
            )),
            _ => None,
        }
    }

    /// Check if this profile has a stored password
    pub fn has_password(&self) -> bool {
        matches!(
            self.credentials,
            ProfileCredentials::Enterprise {
                password: Some(_),
                ..
            }
        )
    }
}

impl Config {
    /// Load configuration from the standard location
    pub fn load() -> Result<Self> {
        debug!("Loading configuration");
        let config_path = Self::config_path()?;
        info!("Configuration path: {:?}", config_path);

        if !config_path.exists() {
            info!("No configuration file found, using defaults");
            return Ok(Config::default());
        }

        debug!("Reading configuration from {:?}", config_path);
        let content = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config from {:?}", config_path))?;

        trace!("Raw config content: {} bytes", content.len());

        // Expand environment variables in the config content
        debug!("Expanding environment variables in configuration");
        let expanded_content = Self::expand_env_vars(&content).with_context(|| {
            format!(
                "Failed to expand environment variables in config from {:?}",
                config_path
            )
        })?;

        if expanded_content != content {
            debug!("Environment variables were expanded in configuration");
        }

        debug!("Parsing TOML configuration");
        let config: Config = toml::from_str(&expanded_content)
            .with_context(|| format!("Failed to parse config from {:?}", config_path))?;

        info!(
            "Configuration loaded: {} profiles, default: {:?}",
            config.profiles.len(),
            config.default_profile
        );

        for (name, profile) in &config.profiles {
            debug!("Profile '{}': type={:?}", name, profile.deployment_type);
        }

        Ok(config)
    }

    /// Save configuration to the standard location
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        // Create parent directories if they don't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory {:?}", parent))?;
        }

        let content = toml::to_string_pretty(self).context("Failed to serialize config")?;

        fs::write(&config_path, content)
            .with_context(|| format!("Failed to write config to {:?}", config_path))?;

        Ok(())
    }

    /// Get a profile by name, considering environment variables and defaults
    pub fn get_profile(&self, name: Option<&str>) -> Option<&Profile> {
        debug!("Resolving profile: explicit={:?}", name);

        let env_profile = std::env::var("REDISCTL_PROFILE").ok();
        if let Some(ref env_name) = env_profile {
            debug!("Found REDISCTL_PROFILE environment variable: {}", env_name);
        }

        let profile_name = name
            .or(env_profile.as_deref())
            .or(self.default_profile.as_deref())?;

        info!(
            "Selected profile: {} (source: {})",
            profile_name,
            if name.is_some() {
                "explicit"
            } else if env_profile.is_some() {
                "environment"
            } else {
                "default"
            }
        );

        let profile = self.profiles.get(profile_name);
        if profile.is_none() {
            warn!("Profile '{}' not found in configuration", profile_name);
        }
        profile
    }

    /// Get the active profile, returning an error if none is configured
    pub fn get_active_profile(&self) -> Result<&Profile> {
        debug!("Getting active profile");

        let env_profile = std::env::var("REDISCTL_PROFILE").ok();
        if let Some(ref env_name) = env_profile {
            debug!("REDISCTL_PROFILE environment variable: {}", env_name);
        }

        let profile_name = env_profile
            .as_deref()
            .or(self.default_profile.as_deref())
            .ok_or_else(|| {
                warn!("No profile configured - no environment variable or default profile");
                anyhow::anyhow!(
                    "No profile configured. Use 'redisctl profile' commands to configure."
                )
            })?;

        info!("Active profile: {}", profile_name);

        self.profiles.get(profile_name).ok_or_else(|| {
            warn!("Profile '{}' not found in configuration", profile_name);
            anyhow::anyhow!("Profile '{}' not found", profile_name)
        })
    }

    /// Set or update a profile
    pub fn set_profile(&mut self, name: String, profile: Profile) {
        self.profiles.insert(name, profile);
    }

    /// Remove a profile by name
    pub fn remove_profile(&mut self, name: &str) -> Option<Profile> {
        // Don't allow removing the default profile
        if self.default_profile.as_deref() == Some(name) {
            self.default_profile = None;
        }
        self.profiles.remove(name)
    }

    /// Set the default profile
    pub fn set_default_profile(&mut self, name: String) -> Result<()> {
        if !self.profiles.contains_key(&name) {
            anyhow::bail!("Profile '{}' does not exist", name);
        }
        self.default_profile = Some(name);
        Ok(())
    }

    /// List all profiles sorted by name
    pub fn list_profiles(&self) -> Vec<(&String, &Profile)> {
        let mut profiles: Vec<_> = self.profiles.iter().collect();
        profiles.sort_by_key(|(name, _)| *name);
        profiles
    }

    /// Get the path to the configuration file
    ///
    /// On macOS, this supports both the standard macOS path and Linux-style ~/.config path:
    /// 1. Check ~/.config/redisctl/config.toml (Linux-style, preferred for consistency)
    /// 2. Fall back to ~/Library/Application Support/com.redis.redisctl/config.toml (macOS standard)
    ///
    /// On Linux: ~/.config/redisctl/config.toml
    /// On Windows: %APPDATA%\redis\redisctl\config.toml
    pub fn config_path() -> Result<PathBuf> {
        trace!("Determining configuration file path");

        // On macOS, check for Linux-style path first for cross-platform consistency
        #[cfg(target_os = "macos")]
        {
            if let Some(base_dirs) = BaseDirs::new() {
                let home_dir = base_dirs.home_dir();
                let linux_style_path = home_dir
                    .join(".config")
                    .join("redisctl")
                    .join("config.toml");

                trace!("Checking Linux-style path on macOS: {:?}", linux_style_path);

                // If Linux-style config exists, use it
                if linux_style_path.exists() {
                    debug!(
                        "Using existing Linux-style config path on macOS: {:?}",
                        linux_style_path
                    );
                    return Ok(linux_style_path);
                }

                // Also check if the config directory exists (user might have created it)
                if linux_style_path.parent().is_some_and(|p| p.exists()) {
                    debug!(
                        "Using Linux-style config directory on macOS: {:?}",
                        linux_style_path
                    );
                    return Ok(linux_style_path);
                }
            }
        }

        // Use platform-specific standard path
        trace!("Using platform-specific configuration path");
        let proj_dirs = ProjectDirs::from("com", "redis", "redisctl")
            .context("Failed to determine config directory")?;

        Ok(proj_dirs.config_dir().join("config.toml"))
    }

    /// Expand environment variables in configuration content
    ///
    /// Supports ${VAR} and ${VAR:-default} syntax for environment variable expansion.
    /// This allows configs to reference environment variables while maintaining
    /// static fallback values.
    ///
    /// Example:
    /// ```toml
    /// api_key = "${REDIS_CLOUD_API_KEY}"
    /// api_url = "${REDIS_CLOUD_API_URL:-https://api.redislabs.com/v1}"
    /// ```
    fn expand_env_vars(content: &str) -> Result<String> {
        match shellexpand::env(content) {
            Ok(expanded) => Ok(expanded.to_string()),
            Err(e) => Err(anyhow::anyhow!(
                "Environment variable expansion failed: {}",
                e
            )),
        }
    }
}

fn default_cloud_url() -> String {
    "https://api.redislabs.com/v1".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_serialization() {
        let mut config = Config::default();

        let cloud_profile = Profile {
            deployment_type: DeploymentType::Cloud,
            credentials: ProfileCredentials::Cloud {
                api_key: "test-key".to_string(),
                api_secret: "test-secret".to_string(),
                api_url: "https://api.redislabs.com/v1".to_string(),
            },
        };

        config.set_profile("test".to_string(), cloud_profile);
        config.default_profile = Some("test".to_string());

        let serialized = toml::to_string(&config).unwrap();
        let deserialized: Config = toml::from_str(&serialized).unwrap();

        assert_eq!(config.default_profile, deserialized.default_profile);
        assert_eq!(config.profiles.len(), deserialized.profiles.len());
    }

    #[test]
    fn test_profile_credential_access() {
        let cloud_profile = Profile {
            deployment_type: DeploymentType::Cloud,
            credentials: ProfileCredentials::Cloud {
                api_key: "key".to_string(),
                api_secret: "secret".to_string(),
                api_url: "url".to_string(),
            },
        };

        let (key, secret, url) = cloud_profile.cloud_credentials().unwrap();
        assert_eq!(key, "key");
        assert_eq!(secret, "secret");
        assert_eq!(url, "url");
        assert!(cloud_profile.enterprise_credentials().is_none());
    }

    #[test]
    #[serial_test::serial]
    fn test_env_var_expansion() {
        // Test basic environment variable expansion
        unsafe {
            std::env::set_var("TEST_API_KEY", "test-key-value");
            std::env::set_var("TEST_API_SECRET", "test-secret-value");
        }

        let content = r#"
[profiles.test]
deployment_type = "cloud"
api_key = "${TEST_API_KEY}"
api_secret = "${TEST_API_SECRET}"
"#;

        let expanded = Config::expand_env_vars(content).unwrap();
        assert!(expanded.contains("test-key-value"));
        assert!(expanded.contains("test-secret-value"));

        // Clean up
        unsafe {
            std::env::remove_var("TEST_API_KEY");
            std::env::remove_var("TEST_API_SECRET");
        }
    }

    #[test]
    #[serial_test::serial]
    fn test_env_var_expansion_with_defaults() {
        // Test environment variable expansion with defaults
        unsafe {
            std::env::remove_var("NONEXISTENT_VAR"); // Ensure it doesn't exist
        }

        let content = r#"
[profiles.test]
deployment_type = "cloud"
api_key = "${NONEXISTENT_VAR:-default-key}"
api_url = "${NONEXISTENT_URL:-https://api.redislabs.com/v1}"
"#;

        let expanded = Config::expand_env_vars(content).unwrap();
        assert!(expanded.contains("default-key"));
        assert!(expanded.contains("https://api.redislabs.com/v1"));
    }

    #[test]
    #[serial_test::serial]
    fn test_env_var_expansion_mixed() {
        // Test mixed static and dynamic values
        unsafe {
            std::env::set_var("TEST_DYNAMIC_KEY", "dynamic-value");
        }

        let content = r#"
[profiles.test]
deployment_type = "cloud"
api_key = "${TEST_DYNAMIC_KEY}"
api_secret = "static-secret"
api_url = "${MISSING_VAR:-https://api.redislabs.com/v1}"
"#;

        let expanded = Config::expand_env_vars(content).unwrap();
        assert!(expanded.contains("dynamic-value"));
        assert!(expanded.contains("static-secret"));
        assert!(expanded.contains("https://api.redislabs.com/v1"));

        // Clean up
        unsafe {
            std::env::remove_var("TEST_DYNAMIC_KEY");
        }
    }

    #[test]
    #[serial_test::serial]
    fn test_full_config_with_env_expansion() {
        // Test complete config parsing with environment variables
        unsafe {
            std::env::set_var("REDIS_TEST_KEY", "expanded-key");
            std::env::set_var("REDIS_TEST_SECRET", "expanded-secret");
        }

        let config_content = r#"
default_profile = "test"

[profiles.test]
deployment_type = "cloud"
api_key = "${REDIS_TEST_KEY}"
api_secret = "${REDIS_TEST_SECRET}"
api_url = "${REDIS_TEST_URL:-https://api.redislabs.com/v1}"
"#;

        let expanded = Config::expand_env_vars(config_content).unwrap();
        let config: Config = toml::from_str(&expanded).unwrap();

        assert_eq!(config.default_profile, Some("test".to_string()));

        let profile = config.profiles.get("test").unwrap();
        let (key, secret, url) = profile.cloud_credentials().unwrap();
        assert_eq!(key, "expanded-key");
        assert_eq!(secret, "expanded-secret");
        assert_eq!(url, "https://api.redislabs.com/v1");

        // Clean up
        unsafe {
            std::env::remove_var("REDIS_TEST_KEY");
            std::env::remove_var("REDIS_TEST_SECRET");
        }
    }
}
