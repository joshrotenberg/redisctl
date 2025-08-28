#![allow(dead_code)]

use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {
    #[serde(default)]
    pub default: Option<String>, // Name of the default profile
    #[serde(default)]
    pub profiles: HashMap<String, Profile>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    pub deployment_type: DeploymentType,
    #[serde(flatten)]
    pub credentials: ProfileCredentials,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, clap::ValueEnum)]
pub enum DeploymentType {
    Cloud,
    Enterprise,
}

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
        password: Option<String>, // Optional for prompting
        #[serde(default)]
        insecure: bool,
    },
}

fn default_cloud_url() -> String {
    "https://api.redislabs.com/v1".to_string()
}

impl ProfileCredentials {
    pub fn has_password(&self) -> bool {
        match self {
            ProfileCredentials::Enterprise { password, .. } => password.is_some(),
            _ => false,
        }
    }
}

impl Config {
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

    pub fn get_profile(&self, name: Option<&str>) -> Option<&Profile> {
        let env_profile = std::env::var("REDISCTL_PROFILE").ok();
        let profile_name = name
            .or(self.default.as_deref())
            .or(env_profile.as_deref())?;

        self.profiles.get(profile_name)
    }

    pub fn set_profile(&mut self, name: String, profile: Profile) {
        self.profiles.insert(name, profile);
    }

    pub fn remove_profile(&mut self, name: &str) -> Option<Profile> {
        self.profiles.remove(name)
    }

    pub fn list_profiles(&self) -> Vec<(&String, &Profile)> {
        let mut profiles: Vec<_> = self.profiles.iter().collect();
        profiles.sort_by_key(|(name, _)| *name);
        profiles
    }

    fn config_path() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "redis", "redisctl")
            .context("Failed to determine config directory")?;

        Ok(proj_dirs.config_dir().join("config.toml"))
    }
}
