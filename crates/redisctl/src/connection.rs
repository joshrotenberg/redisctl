//! Connection management for Redis Cloud and Enterprise clients

use crate::config::{Config, Profile};
use crate::error::Result as CliResult;
use anyhow::Context;
use tracing::{debug, info, trace};

/// Connection manager for creating authenticated clients
#[allow(dead_code)] // Used by binary target
pub struct ConnectionManager {
    pub config: Config,
}

impl ConnectionManager {
    /// Create a new connection manager with the given configuration
    #[allow(dead_code)] // Used by binary target
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Get a profile by name, or the default profile if no name provided
    #[allow(dead_code)] // Used by binary target
    pub fn get_profile(&self, profile_name: Option<&str>) -> CliResult<&Profile> {
        let name = match profile_name {
            Some(name) => name,
            None => self.config.default_profile.as_ref().context(
                "No profile specified and no default profile set. Use 'redisctl profile set' to create one."
            )?,
        };

        Ok(self.config.profiles.get(name).with_context(|| {
            format!(
                "Profile '{}' not found. Use 'redisctl profile list' to see available profiles.",
                name
            )
        })?)
    }

    /// Create a Cloud client from profile credentials with environment variable override support
    #[allow(dead_code)] // Used by binary target
    pub async fn create_cloud_client(
        &self,
        profile_name: Option<&str>,
    ) -> CliResult<redis_cloud::CloudClient> {
        debug!("Creating Redis Cloud client");
        trace!("Profile name: {:?}", profile_name);

        // Check if all required environment variables are present
        let env_api_key = std::env::var("REDIS_CLOUD_API_KEY").ok();
        let env_api_secret = std::env::var("REDIS_CLOUD_SECRET_KEY").ok();
        let env_api_url = std::env::var("REDIS_CLOUD_API_URL").ok();

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
                // Fall back to profile credentials
                info!("Using Redis Cloud credentials from profile");
                let profile = self.get_profile(profile_name)?;
                let (api_key, api_secret, api_url) = profile
                    .cloud_credentials()
                    .context("Profile is not configured for Redis Cloud")?;

                // Check for partial overrides before consuming the Options
                let has_overrides =
                    env_api_key.is_some() || env_api_secret.is_some() || env_api_url.is_some();

                // Allow partial environment variable overrides
                let key = env_api_key.unwrap_or_else(|| api_key.to_string());
                let secret = env_api_secret.unwrap_or_else(|| api_secret.to_string());
                let url = env_api_url.unwrap_or_else(|| api_url.to_string());

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
    #[allow(dead_code)] // Used by binary target
    pub async fn create_enterprise_client(
        &self,
        profile_name: Option<&str>,
    ) -> CliResult<redis_enterprise::EnterpriseClient> {
        debug!("Creating Redis Enterprise client");
        trace!("Profile name: {:?}", profile_name);

        // Check if all required environment variables are present
        let env_url = std::env::var("REDIS_ENTERPRISE_URL").ok();
        let env_user = std::env::var("REDIS_ENTERPRISE_USER").ok();
        let env_password = std::env::var("REDIS_ENTERPRISE_PASSWORD").ok();
        let env_insecure = std::env::var("REDIS_ENTERPRISE_INSECURE").ok();

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
                // Fall back to profile credentials
                info!("Using Redis Enterprise credentials from profile");
                let profile = self.get_profile(profile_name)?;
                let (url, username, password, insecure) = profile
                    .enterprise_credentials()
                    .context("Profile is not configured for Redis Enterprise")?;

                // Check for partial overrides before consuming the Options
                let has_overrides = env_url.is_some()
                    || env_user.is_some()
                    || env_password.is_some()
                    || env_insecure.is_some();

                // Allow partial environment variable overrides
                let final_url = env_url.unwrap_or_else(|| url.to_string());
                let final_user = env_user.unwrap_or_else(|| username.to_string());
                let final_password = env_password.or_else(|| password.map(|p| p.to_string()));
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
