//! Connection management for Redis Cloud and Enterprise clients

use crate::config::{Config, Profile};
use crate::error::Result as CliResult;
use anyhow::Context;

/// Connection manager for creating authenticated clients
pub struct ConnectionManager {
    pub config: Config,
}

impl ConnectionManager {
    /// Create a new connection manager with the given configuration
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Get a profile by name, or the default profile if no name provided
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
    pub async fn create_cloud_client(
        &self,
        profile_name: Option<&str>,
    ) -> CliResult<redis_cloud::CloudClient> {
        // Check if all required environment variables are present
        let env_api_key = std::env::var("REDIS_CLOUD_API_KEY").ok();
        let env_api_secret = std::env::var("REDIS_CLOUD_SECRET_KEY").ok();
        let env_api_url = std::env::var("REDIS_CLOUD_API_URL").ok();

        let (final_api_key, final_api_secret, final_api_url) =
            if let (Some(key), Some(secret)) = (&env_api_key, &env_api_secret) {
                // Environment variables provide complete credentials
                let url = env_api_url.unwrap_or_else(|| "https://api.redislabs.com/v1".to_string());
                (key.clone(), secret.clone(), url)
            } else {
                // Fall back to profile credentials
                let profile = self.get_profile(profile_name)?;
                let (api_key, api_secret, api_url) = profile
                    .cloud_credentials()
                    .context("Profile is not configured for Redis Cloud")?;

                // Allow partial environment variable overrides
                let key = env_api_key.unwrap_or_else(|| api_key.to_string());
                let secret = env_api_secret.unwrap_or_else(|| api_secret.to_string());
                let url = env_api_url.unwrap_or_else(|| api_url.to_string());
                (key, secret, url)
            };

        // Create and configure the Cloud client
        let client = redis_cloud::CloudClient::builder()
            .api_key(&final_api_key)
            .api_secret(&final_api_secret)
            .base_url(&final_api_url)
            .build()
            .context("Failed to create Redis Cloud client")?;

        Ok(client)
    }

    /// Create an Enterprise client from profile credentials with environment variable override support
    pub async fn create_enterprise_client(
        &self,
        profile_name: Option<&str>,
    ) -> CliResult<redis_enterprise::EnterpriseClient> {
        // Check if all required environment variables are present
        let env_url = std::env::var("REDIS_ENTERPRISE_URL").ok();
        let env_user = std::env::var("REDIS_ENTERPRISE_USER").ok();
        let env_password = std::env::var("REDIS_ENTERPRISE_PASSWORD").ok();
        let env_insecure = std::env::var("REDIS_ENTERPRISE_INSECURE").ok();

        let (final_url, final_username, final_password, final_insecure) =
            if let (Some(url), Some(user)) = (&env_url, &env_user) {
                // Environment variables provide complete credentials
                let password = env_password.clone(); // Password can be None for interactive prompting
                let insecure = env_insecure
                    .as_ref()
                    .map(|s| s.to_lowercase() == "true" || s == "1")
                    .unwrap_or(false);
                (url.clone(), user.clone(), password, insecure)
            } else {
                // Fall back to profile credentials
                let profile = self.get_profile(profile_name)?;
                let (url, username, password, insecure) = profile
                    .enterprise_credentials()
                    .context("Profile is not configured for Redis Enterprise")?;

                // Allow partial environment variable overrides
                let final_url = env_url.unwrap_or_else(|| url.to_string());
                let final_user = env_user.unwrap_or_else(|| username.to_string());
                let final_password = env_password.or_else(|| password.map(|p| p.to_string()));
                let final_insecure = env_insecure
                    .as_ref()
                    .map(|s| s.to_lowercase() == "true" || s == "1")
                    .unwrap_or(insecure);
                (final_url, final_user, final_password, final_insecure)
            };

        // Build the Enterprise client
        let mut builder = redis_enterprise::EnterpriseClient::builder()
            .base_url(&final_url)
            .username(&final_username);

        // Add password if provided
        if let Some(ref password) = final_password {
            builder = builder.password(password);
        }

        // Set insecure flag if needed
        if final_insecure {
            builder = builder.insecure(true);
        }

        let client = builder
            .build()
            .context("Failed to create Redis Enterprise client")?;

        Ok(client)
    }
}
