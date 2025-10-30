//! Connection management for Redis Cloud and Enterprise clients

use crate::error::Result as CliResult;
use anyhow::Context;
use redisctl_config::{Config, DeploymentType};
use tracing::{debug, info, trace};

/// Connection manager for creating authenticated clients
#[allow(dead_code)] // Used by binary target
#[derive(Clone)]
pub struct ConnectionManager {
    pub config: Config,
    pub config_path: Option<std::path::PathBuf>,
}

impl ConnectionManager {
    /// Create a new connection manager with the given configuration
    #[allow(dead_code)] // Used by binary target
    pub fn new(config: Config) -> Self {
        Self {
            config,
            config_path: None,
        }
    }

    /// Create a new connection manager with a custom config path
    #[allow(dead_code)] // Used by binary target
    pub fn with_config_path(config: Config, config_path: Option<std::path::PathBuf>) -> Self {
        Self {
            config,
            config_path,
        }
    }

    /// Save the configuration to the appropriate location
    #[allow(dead_code)] // Used by binary target
    pub fn save_config(&self) -> CliResult<()> {
        if let Some(ref path) = self.config_path {
            self.config
                .save_to_path(path)
                .context("Failed to save configuration")?;
        } else {
            self.config.save().context("Failed to save configuration")?;
        }
        Ok(())
    }

    /// Create a Cloud client from profile credentials with environment variable override support
    ///
    /// When --config-file is explicitly specified, environment variables are ignored to provide
    /// true configuration isolation. This allows testing with isolated configs and follows the
    /// principle of "explicit wins" (CLI args > env vars > defaults).
    #[allow(dead_code)] // Used by binary target
    pub async fn create_cloud_client(
        &self,
        profile_name: Option<&str>,
    ) -> CliResult<redis_cloud::CloudClient> {
        debug!("Creating Redis Cloud client");
        trace!("Profile name: {:?}", profile_name);

        // When --config-file is explicitly specified, ignore environment variables
        // This provides true configuration isolation for testing and follows CLI best practices
        let use_env_vars = self.config_path.is_none();

        debug!(
            "Config path: {:?}, use_env_vars: {}",
            self.config_path, use_env_vars
        );

        if !use_env_vars {
            info!("--config-file specified explicitly, ignoring environment variables");
        }

        // Check if all required environment variables are present (only if we're using them)
        let env_api_key = if use_env_vars {
            std::env::var("REDIS_CLOUD_API_KEY").ok()
        } else {
            None
        };
        let env_api_secret = if use_env_vars {
            std::env::var("REDIS_CLOUD_SECRET_KEY").ok()
        } else {
            None
        };
        let env_api_url = if use_env_vars {
            std::env::var("REDIS_CLOUD_API_URL").ok()
        } else {
            None
        };

        if env_api_key.is_some() {
            debug!("Found REDIS_CLOUD_API_KEY environment variable");
        }
        if env_api_secret.is_some() {
            debug!("Found REDIS_CLOUD_SECRET_KEY environment variable");
        }
        if env_api_url.is_some() {
            debug!("Found REDIS_CLOUD_API_URL environment variable");
        }

        let (final_api_key, final_api_secret, final_api_url) =
            if let (Some(key), Some(secret)) = (&env_api_key, &env_api_secret) {
                // Environment variables provide complete credentials
                info!("Using Redis Cloud credentials from environment variables");
                let url = env_api_url.unwrap_or_else(|| "https://api.redislabs.com/v1".to_string());
                (key.clone(), secret.clone(), url)
            } else {
                // Resolve the profile using type-specific logic
                let resolved_profile_name = self.config.resolve_cloud_profile(profile_name)?;
                info!("Using Redis Cloud profile: {}", resolved_profile_name);

                let profile = self
                    .config
                    .profiles
                    .get(&resolved_profile_name)
                    .with_context(|| format!("Profile '{}' not found", resolved_profile_name))?;

                // Verify it's a cloud profile
                if profile.deployment_type != DeploymentType::Cloud {
                    return Err(crate::error::RedisCtlError::ProfileTypeMismatch {
                        name: resolved_profile_name.to_string(),
                        actual_type: match profile.deployment_type {
                            DeploymentType::Cloud => "cloud",
                            DeploymentType::Enterprise => "enterprise",
                        }
                        .to_string(),
                        expected_type: "cloud".to_string(),
                    });
                }

                // Use the new resolve method which handles keyring lookup
                let (api_key, api_secret, api_url) = profile
                    .resolve_cloud_credentials()
                    .context("Failed to resolve Cloud credentials")?
                    .context("Profile is not configured for Redis Cloud")?;

                // Check for partial overrides before consuming the Options
                let has_overrides =
                    env_api_key.is_some() || env_api_secret.is_some() || env_api_url.is_some();

                // Allow partial environment variable overrides
                let key = env_api_key.unwrap_or(api_key);
                let secret = env_api_secret.unwrap_or(api_secret);
                let url = env_api_url.unwrap_or(api_url);

                if has_overrides {
                    debug!("Applied partial environment variable overrides");
                }

                (key, secret, url)
            };

        info!("Connecting to Redis Cloud API: {}", final_api_url);
        trace!(
            "API key: {}...",
            &final_api_key[..final_api_key.len().min(8)]
        );

        // Create and configure the Cloud client
        let client = redis_cloud::CloudClient::builder()
            .api_key(&final_api_key)
            .api_secret(&final_api_secret)
            .base_url(&final_api_url)
            .build()
            .context("Failed to create Redis Cloud client")?;

        debug!("Redis Cloud client created successfully");
        Ok(client)
    }

    /// Create an Enterprise client from profile credentials with environment variable override support
    ///
    /// When --config-file is explicitly specified, environment variables are ignored to provide
    /// true configuration isolation. This allows testing with isolated configs and follows the
    /// principle of "explicit wins" (CLI args > env vars > defaults).
    #[allow(dead_code)] // Used by binary target
    pub async fn create_enterprise_client(
        &self,
        profile_name: Option<&str>,
    ) -> CliResult<redis_enterprise::EnterpriseClient> {
        debug!("Creating Redis Enterprise client");
        trace!("Profile name: {:?}", profile_name);

        // When --config-file is explicitly specified, ignore environment variables
        // This provides true configuration isolation for testing and follows CLI best practices
        let use_env_vars = self.config_path.is_none();

        debug!(
            "Config path: {:?}, use_env_vars: {}",
            self.config_path, use_env_vars
        );

        if !use_env_vars {
            info!("--config-file specified explicitly, ignoring environment variables");
        }

        // Check if all required environment variables are present (only if we're using them)
        let env_url = if use_env_vars {
            std::env::var("REDIS_ENTERPRISE_URL").ok()
        } else {
            None
        };
        let env_user = if use_env_vars {
            std::env::var("REDIS_ENTERPRISE_USER").ok()
        } else {
            None
        };
        let env_password = if use_env_vars {
            std::env::var("REDIS_ENTERPRISE_PASSWORD").ok()
        } else {
            None
        };
        let env_insecure = if use_env_vars {
            std::env::var("REDIS_ENTERPRISE_INSECURE").ok()
        } else {
            None
        };

        if env_url.is_some() {
            debug!("Found REDIS_ENTERPRISE_URL environment variable");
        }
        if env_user.is_some() {
            debug!("Found REDIS_ENTERPRISE_USER environment variable");
        }
        if env_password.is_some() {
            debug!("Found REDIS_ENTERPRISE_PASSWORD environment variable");
        }
        if env_insecure.is_some() {
            debug!("Found REDIS_ENTERPRISE_INSECURE environment variable");
        }

        let (final_url, final_username, final_password, final_insecure) =
            if let (Some(url), Some(user)) = (&env_url, &env_user) {
                // Environment variables provide complete credentials
                info!("Using Redis Enterprise credentials from environment variables");
                let password = env_password.clone(); // Password can be None for interactive prompting
                let insecure = env_insecure
                    .as_ref()
                    .map(|s| s.to_lowercase() == "true" || s == "1")
                    .unwrap_or(false);
                (url.clone(), user.clone(), password, insecure)
            } else {
                // Resolve the profile using type-specific logic
                let resolved_profile_name = self.config.resolve_enterprise_profile(profile_name)?;
                info!("Using Redis Enterprise profile: {}", resolved_profile_name);

                let profile = self
                    .config
                    .profiles
                    .get(&resolved_profile_name)
                    .with_context(|| format!("Profile '{}' not found", resolved_profile_name))?;

                // Verify it's an enterprise profile
                if profile.deployment_type != DeploymentType::Enterprise {
                    return Err(crate::error::RedisCtlError::ProfileTypeMismatch {
                        name: resolved_profile_name.to_string(),
                        actual_type: match profile.deployment_type {
                            DeploymentType::Cloud => "cloud",
                            DeploymentType::Enterprise => "enterprise",
                        }
                        .to_string(),
                        expected_type: "enterprise".to_string(),
                    });
                }

                // Use the new resolve method which handles keyring lookup
                let (url, username, password, insecure) = profile
                    .resolve_enterprise_credentials()
                    .context("Failed to resolve Enterprise credentials")?
                    .context("Profile is not configured for Redis Enterprise")?;

                // Check for partial overrides before consuming the Options
                let has_overrides = env_url.is_some()
                    || env_user.is_some()
                    || env_password.is_some()
                    || env_insecure.is_some();

                // Allow partial environment variable overrides
                let final_url = env_url.unwrap_or(url);
                let final_user = env_user.unwrap_or(username);
                let final_password = env_password.or(password);
                let final_insecure = env_insecure
                    .as_ref()
                    .map(|s| s.to_lowercase() == "true" || s == "1")
                    .unwrap_or(insecure);

                if has_overrides {
                    debug!("Applied partial environment variable overrides");
                }

                (final_url, final_user, final_password, final_insecure)
            };

        info!("Connecting to Redis Enterprise: {}", final_url);
        debug!("Username: {}", final_username);
        debug!(
            "Password: {}",
            if final_password.is_some() {
                "configured"
            } else {
                "not set"
            }
        );
        debug!("Insecure mode: {}", final_insecure);

        // Build the Enterprise client
        let mut builder = redis_enterprise::EnterpriseClient::builder()
            .base_url(&final_url)
            .username(&final_username);

        // Add password if provided
        if let Some(ref password) = final_password {
            builder = builder.password(password);
            trace!("Password added to client builder");
        }

        // Set insecure flag if needed
        if final_insecure {
            builder = builder.insecure(true);
            debug!("SSL certificate verification disabled");
        }

        let client = builder
            .build()
            .context("Failed to create Redis Enterprise client")?;

        debug!("Redis Enterprise client created successfully");
        Ok(client)
    }
}
