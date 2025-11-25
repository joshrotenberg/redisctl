//! Configuration management for Redis CLI tools
//!
//! Handles configuration loading from files, environment variables, and command-line arguments.
//! Configuration is stored in TOML format with support for multiple named profiles.

#[cfg(target_os = "macos")]
use directories::BaseDirs;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::credential::CredentialStore;
use crate::error::{ConfigError, Result};

/// Main configuration structure
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {
    /// Default profile for enterprise commands
    #[serde(default, rename = "default_enterprise")]
    pub default_enterprise: Option<String>,
    /// Default profile for cloud commands
    #[serde(default, rename = "default_cloud")]
    pub default_cloud: Option<String>,
    /// Global Files.com API key for support package uploads
    /// Can be overridden per-profile. Supports keyring: prefix for secure storage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files_api_key: Option<String>,
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
    /// Files.com API key for this profile (overrides global setting)
    /// Supports keyring: prefix for secure storage.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files_api_key: Option<String>,
    /// Resilience configuration for this profile
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resilience: Option<crate::ResilienceConfig>,
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

    /// Get resolved Cloud credentials (with keyring support)
    pub fn resolve_cloud_credentials(&self) -> Result<Option<(String, String, String)>> {
        match &self.credentials {
            ProfileCredentials::Cloud {
                api_key,
                api_secret,
                api_url,
            } => {
                let store = CredentialStore::new();

                // Resolve each credential with environment variable fallback
                let resolved_key = store
                    .get_credential(api_key, Some("REDIS_CLOUD_API_KEY"))
                    .map_err(|e| {
                        ConfigError::CredentialError(format!("Failed to resolve API key: {}", e))
                    })?;
                let resolved_secret = store
                    .get_credential(api_secret, Some("REDIS_CLOUD_API_SECRET"))
                    .map_err(|e| {
                        ConfigError::CredentialError(format!("Failed to resolve API secret: {}", e))
                    })?;
                let resolved_url = store
                    .get_credential(api_url, Some("REDIS_CLOUD_API_URL"))
                    .map_err(|e| {
                        ConfigError::CredentialError(format!("Failed to resolve API URL: {}", e))
                    })?;

                Ok(Some((resolved_key, resolved_secret, resolved_url)))
            }
            _ => Ok(None),
        }
    }

    /// Get resolved Enterprise credentials (with keyring support)
    #[allow(clippy::type_complexity)]
    pub fn resolve_enterprise_credentials(
        &self,
    ) -> Result<Option<(String, String, Option<String>, bool)>> {
        match &self.credentials {
            ProfileCredentials::Enterprise {
                url,
                username,
                password,
                insecure,
            } => {
                let store = CredentialStore::new();

                // Resolve each credential with environment variable fallback
                let resolved_url = store
                    .get_credential(url, Some("REDIS_ENTERPRISE_URL"))
                    .map_err(|e| {
                        ConfigError::CredentialError(format!("Failed to resolve URL: {}", e))
                    })?;
                let resolved_username = store
                    .get_credential(username, Some("REDIS_ENTERPRISE_USER"))
                    .map_err(|e| {
                        ConfigError::CredentialError(format!("Failed to resolve username: {}", e))
                    })?;
                let resolved_password = password
                    .as_ref()
                    .map(|p| {
                        store
                            .get_credential(p, Some("REDIS_ENTERPRISE_PASSWORD"))
                            .map_err(|e| {
                                ConfigError::CredentialError(format!(
                                    "Failed to resolve password: {}",
                                    e
                                ))
                            })
                    })
                    .transpose()?;

                Ok(Some((
                    resolved_url,
                    resolved_username,
                    resolved_password,
                    *insecure,
                )))
            }
            _ => Ok(None),
        }
    }
}

impl Config {
    /// Get the first profile of the specified deployment type (sorted alphabetically by name)
    pub fn find_first_profile_of_type(&self, deployment_type: DeploymentType) -> Option<&str> {
        let mut profiles: Vec<_> = self
            .profiles
            .iter()
            .filter(|(_, p)| p.deployment_type == deployment_type)
            .map(|(name, _)| name.as_str())
            .collect();
        profiles.sort();
        profiles.first().copied()
    }

    /// Get all profiles of the specified deployment type
    pub fn get_profiles_of_type(&self, deployment_type: DeploymentType) -> Vec<&str> {
        let mut profiles: Vec<_> = self
            .profiles
            .iter()
            .filter(|(_, p)| p.deployment_type == deployment_type)
            .map(|(name, _)| name.as_str())
            .collect();
        profiles.sort();
        profiles
    }

    /// Resolve the profile to use for enterprise commands
    pub fn resolve_enterprise_profile(&self, explicit_profile: Option<&str>) -> Result<String> {
        if let Some(profile_name) = explicit_profile {
            // Explicitly specified profile
            return Ok(profile_name.to_string());
        }

        if let Some(ref default) = self.default_enterprise {
            // Type-specific default
            return Ok(default.clone());
        }

        if let Some(profile_name) = self.find_first_profile_of_type(DeploymentType::Enterprise) {
            // First enterprise profile
            return Ok(profile_name.to_string());
        }

        // No enterprise profiles available
        let cloud_profiles = self.get_profiles_of_type(DeploymentType::Cloud);
        if !cloud_profiles.is_empty() {
            Err(ConfigError::NoProfilesOfType {
                deployment_type: "enterprise".to_string(),
                suggestion: format!(
                    "Available cloud profiles: {}. Use 'redisctl profile set' to create an enterprise profile.",
                    cloud_profiles.join(", ")
                ),
            })
        } else {
            Err(ConfigError::NoProfilesOfType {
                deployment_type: "enterprise".to_string(),
                suggestion: "Use 'redisctl profile set' to create a profile.".to_string(),
            })
        }
    }

    /// Resolve the profile to use for cloud commands
    pub fn resolve_cloud_profile(&self, explicit_profile: Option<&str>) -> Result<String> {
        if let Some(profile_name) = explicit_profile {
            // Explicitly specified profile
            return Ok(profile_name.to_string());
        }

        if let Some(ref default) = self.default_cloud {
            // Type-specific default
            return Ok(default.clone());
        }

        if let Some(profile_name) = self.find_first_profile_of_type(DeploymentType::Cloud) {
            // First cloud profile
            return Ok(profile_name.to_string());
        }

        // No cloud profiles available
        let enterprise_profiles = self.get_profiles_of_type(DeploymentType::Enterprise);
        if !enterprise_profiles.is_empty() {
            Err(ConfigError::NoProfilesOfType {
                deployment_type: "cloud".to_string(),
                suggestion: format!(
                    "Available enterprise profiles: {}. Use 'redisctl profile set' to create a cloud profile.",
                    enterprise_profiles.join(", ")
                ),
            })
        } else {
            Err(ConfigError::NoProfilesOfType {
                deployment_type: "cloud".to_string(),
                suggestion: "Use 'redisctl profile set' to create a profile.".to_string(),
            })
        }
    }

    /// Load configuration from the standard location
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        Self::load_from_path(&config_path)
    }

    /// Load configuration from a specific path
    pub fn load_from_path(config_path: &Path) -> Result<Self> {
        if !config_path.exists() {
            return Ok(Config::default());
        }

        let content = fs::read_to_string(config_path).map_err(|e| ConfigError::LoadError {
            path: config_path.display().to_string(),
            source: e,
        })?;

        // Expand environment variables in the config content
        let expanded_content = Self::expand_env_vars(&content);

        let config: Config = toml::from_str(&expanded_content)?;

        Ok(config)
    }

    /// Save configuration to the standard location
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        self.save_to_path(&config_path)
    }

    /// Save configuration to a specific path
    pub fn save_to_path(&self, config_path: &Path) -> Result<()> {
        // Create parent directories if they don't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).map_err(|e| ConfigError::SaveError {
                path: parent.display().to_string(),
                source: e,
            })?;
        }

        let content = toml::to_string_pretty(self)?;

        fs::write(config_path, content).map_err(|e| ConfigError::SaveError {
            path: config_path.display().to_string(),
            source: e,
        })?;

        Ok(())
    }

    /// Set or update a profile
    pub fn set_profile(&mut self, name: String, profile: Profile) {
        self.profiles.insert(name, profile);
    }

    /// Remove a profile by name
    pub fn remove_profile(&mut self, name: &str) -> Option<Profile> {
        // Clear type-specific defaults if this profile was set as default
        if self.default_enterprise.as_deref() == Some(name) {
            self.default_enterprise = None;
        }
        if self.default_cloud.as_deref() == Some(name) {
            self.default_cloud = None;
        }
        self.profiles.remove(name)
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
        // On macOS, check for Linux-style path first for cross-platform consistency
        #[cfg(target_os = "macos")]
        {
            if let Some(base_dirs) = BaseDirs::new() {
                let home_dir = base_dirs.home_dir();
                let linux_style_path = home_dir
                    .join(".config")
                    .join("redisctl")
                    .join("config.toml");

                // If Linux-style config exists, use it
                if linux_style_path.exists() {
                    return Ok(linux_style_path);
                }

                // Also check if the config directory exists (user might have created it)
                if linux_style_path
                    .parent()
                    .map(|p| p.exists())
                    .unwrap_or(false)
                {
                    return Ok(linux_style_path);
                }
            }
        }

        // Use platform-specific standard path
        let proj_dirs =
            ProjectDirs::from("com", "redis", "redisctl").ok_or(ConfigError::ConfigDirError)?;

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
    fn expand_env_vars(content: &str) -> String {
        // Use shellexpand::env_with_context_no_errors which returns unexpanded vars as-is
        // This prevents errors when env vars for unused profiles aren't set
        let expanded =
            shellexpand::env_with_context_no_errors(content, |var| std::env::var(var).ok());
        expanded.to_string()
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
            files_api_key: None,
            resilience: None,
        };

        config.set_profile("test".to_string(), cloud_profile);
        config.default_cloud = Some("test".to_string());

        let serialized = toml::to_string(&config).unwrap();
        let deserialized: Config = toml::from_str(&serialized).unwrap();

        assert_eq!(config.default_cloud, deserialized.default_cloud);
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
            files_api_key: None,
            resilience: None,
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

        let expanded = Config::expand_env_vars(content);
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

        let expanded = Config::expand_env_vars(content);
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

        let expanded = Config::expand_env_vars(content);
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
default_cloud = "test"

[profiles.test]
deployment_type = "cloud"
api_key = "${REDIS_TEST_KEY}"
api_secret = "${REDIS_TEST_SECRET}"
api_url = "${REDIS_TEST_URL:-https://api.redislabs.com/v1}"
"#;

        let expanded = Config::expand_env_vars(config_content);
        let config: Config = toml::from_str(&expanded).unwrap();

        assert_eq!(config.default_cloud, Some("test".to_string()));

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

    #[test]
    fn test_enterprise_profile_resolution() {
        let mut config = Config::default();

        // Add an enterprise profile
        let enterprise_profile = Profile {
            deployment_type: DeploymentType::Enterprise,
            credentials: ProfileCredentials::Enterprise {
                url: "https://localhost:9443".to_string(),
                username: "admin".to_string(),
                password: Some("password".to_string()),
                insecure: false,
            },
            files_api_key: None,
            resilience: None,
        };
        config.set_profile("ent1".to_string(), enterprise_profile);

        // Test explicit profile
        assert_eq!(
            config.resolve_enterprise_profile(Some("ent1")).unwrap(),
            "ent1"
        );

        // Test first enterprise profile (no default set)
        assert_eq!(config.resolve_enterprise_profile(None).unwrap(), "ent1");

        // Set default enterprise
        config.default_enterprise = Some("ent1".to_string());
        assert_eq!(config.resolve_enterprise_profile(None).unwrap(), "ent1");
    }

    #[test]
    fn test_cloud_profile_resolution() {
        let mut config = Config::default();

        // Add a cloud profile
        let cloud_profile = Profile {
            deployment_type: DeploymentType::Cloud,
            credentials: ProfileCredentials::Cloud {
                api_key: "key".to_string(),
                api_secret: "secret".to_string(),
                api_url: "https://api.redislabs.com/v1".to_string(),
            },
            files_api_key: None,
            resilience: None,
        };
        config.set_profile("cloud1".to_string(), cloud_profile);

        // Test explicit profile
        assert_eq!(
            config.resolve_cloud_profile(Some("cloud1")).unwrap(),
            "cloud1"
        );

        // Test first cloud profile (no default set)
        assert_eq!(config.resolve_cloud_profile(None).unwrap(), "cloud1");

        // Set default cloud
        config.default_cloud = Some("cloud1".to_string());
        assert_eq!(config.resolve_cloud_profile(None).unwrap(), "cloud1");
    }

    #[test]
    fn test_mixed_profile_resolution() {
        let mut config = Config::default();

        // Add a cloud profile
        let cloud_profile = Profile {
            deployment_type: DeploymentType::Cloud,
            credentials: ProfileCredentials::Cloud {
                api_key: "key".to_string(),
                api_secret: "secret".to_string(),
                api_url: "https://api.redislabs.com/v1".to_string(),
            },
            files_api_key: None,
            resilience: None,
        };
        config.set_profile("cloud1".to_string(), cloud_profile.clone());
        config.set_profile("cloud2".to_string(), cloud_profile);

        // Add enterprise profiles
        let enterprise_profile = Profile {
            deployment_type: DeploymentType::Enterprise,
            credentials: ProfileCredentials::Enterprise {
                url: "https://localhost:9443".to_string(),
                username: "admin".to_string(),
                password: Some("password".to_string()),
                insecure: false,
            },
            files_api_key: None,
            resilience: None,
        };
        config.set_profile("ent1".to_string(), enterprise_profile.clone());
        config.set_profile("ent2".to_string(), enterprise_profile);

        // Without defaults, should use first of each type
        assert_eq!(config.resolve_cloud_profile(None).unwrap(), "cloud1");
        assert_eq!(config.resolve_enterprise_profile(None).unwrap(), "ent1");

        // Set type-specific defaults
        config.default_cloud = Some("cloud2".to_string());
        config.default_enterprise = Some("ent2".to_string());

        // Should now use the type-specific defaults
        assert_eq!(config.resolve_cloud_profile(None).unwrap(), "cloud2");
        assert_eq!(config.resolve_enterprise_profile(None).unwrap(), "ent2");
    }

    #[test]
    fn test_no_profile_errors() {
        let config = Config::default();

        // No profiles at all
        assert!(config.resolve_enterprise_profile(None).is_err());
        assert!(config.resolve_cloud_profile(None).is_err());
    }

    #[test]
    fn test_wrong_profile_type_help() {
        let mut config = Config::default();

        // Only add cloud profiles
        let cloud_profile = Profile {
            deployment_type: DeploymentType::Cloud,
            credentials: ProfileCredentials::Cloud {
                api_key: "key".to_string(),
                api_secret: "secret".to_string(),
                api_url: "https://api.redislabs.com/v1".to_string(),
            },
            files_api_key: None,
            resilience: None,
        };
        config.set_profile("cloud1".to_string(), cloud_profile);

        // Try to resolve enterprise profile - should get helpful error
        let err = config.resolve_enterprise_profile(None).unwrap_err();
        assert!(err.to_string().contains("No enterprise profiles"));
        assert!(err.to_string().contains("Available cloud profiles: cloud1"));
    }
}
