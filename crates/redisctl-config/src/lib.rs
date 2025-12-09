//! Configuration and profile management for Redis CLI tools
//!
//! This library provides a reusable configuration system for managing
//! credentials and settings for Redis Cloud and Redis Enterprise deployments.
//!
//! # Features
//!
//! - Multiple named profiles for different Redis deployments
//! - Secure credential storage using OS keyring (optional)
//! - Environment variable expansion in config files
//! - Platform-specific config file locations
//! - Support for both Redis Cloud and Redis Enterprise
//!
//! # Examples
//!
//! ## Loading Configuration
//!
//! ```no_run
//! use redisctl_config::Config;
//!
//! let config = Config::load()?;
//! # Ok::<(), redisctl_config::ConfigError>(())
//! ```
//!
//! ## Creating a Profile
//!
//! ```
//! use redisctl_config::{Config, Profile, DeploymentType, ProfileCredentials};
//!
//! let profile = Profile {
//!     deployment_type: DeploymentType::Cloud,
//!     credentials: ProfileCredentials::Cloud {
//!         api_key: "your-api-key".to_string(),
//!         api_secret: "your-secret".to_string(),
//!         api_url: "https://api.redislabs.com/v1".to_string(),
//!     },
//!     files_api_key: None,
//!     resilience: None,
//! };
//!
//! let mut config = Config::default();
//! config.set_profile("production".to_string(), profile);
//! ```

pub mod config;
pub mod credential;
pub mod error;
pub mod resilience;

// Re-export main types for convenience
pub use config::{Config, DeploymentType, Profile, ProfileCredentials};
pub use credential::{CredentialStorage, CredentialStore};
pub use error::{ConfigError, Result};
pub use resilience::ResilienceConfig;
