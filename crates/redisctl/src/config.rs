//! Configuration management for redisctl
//!
//! Handles configuration loading from files, environment variables, and command-line arguments.
//! Configuration is stored in TOML format with support for multiple named profiles.

#![allow(dead_code)] // Foundation code - will be used in future PRs

use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

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
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            return Ok(Config::default());
        }

        let content = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config from {:?}", config_path))?;

        toml::from_str(&content)
            .with_context(|| format!("Failed to parse config from {:?}", config_path))
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
        let env_profile = std::env::var("REDISCTL_PROFILE").ok();
        let profile_name = name
            .or(env_profile.as_deref())
            .or(self.default_profile.as_deref())?;

        self.profiles.get(profile_name)
    }

    /// Get the active profile, returning an error if none is configured
    pub fn get_active_profile(&self) -> Result<&Profile> {
        let env_profile = std::env::var("REDISCTL_PROFILE").ok();
        let profile_name = env_profile
            .as_deref()
            .or(self.default_profile.as_deref())
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "No profile configured. Use 'redisctl profile' commands to configure."
                )
            })?;

        self.profiles
            .get(profile_name)
            .ok_or_else(|| anyhow::anyhow!("Profile '{}' not found", profile_name))
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
    pub fn config_path() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "redis", "redisctl")
            .context("Failed to determine config directory")?;

        Ok(proj_dirs.config_dir().join("config.toml"))
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
}
