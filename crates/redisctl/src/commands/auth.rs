//! Authentication testing and management commands

#![allow(dead_code)]

use anyhow::{Context, Result};
use colored::Colorize;
use redis_cloud::CloudClient;
use redis_enterprise::EnterpriseClient;
use serde_json::json;

use crate::cli::AuthCommands;
use crate::config::{Config, DeploymentType, Profile, ProfileCredentials};
/// Simple wrapper for output formatting
pub struct OutputFormatter {
    pub format: crate::output::OutputFormat,
    pub query: Option<String>,
}

impl OutputFormatter {
    pub fn format<T: serde::Serialize>(&self, data: T) -> anyhow::Result<()> {
        crate::output::print_output(data, self.format, self.query.as_deref())
    }
}

/// Execute auth commands
pub async fn execute(cmd: AuthCommands, config: &Config, output: OutputFormatter) -> Result<()> {
    match cmd {
        AuthCommands::Test {
            profile,
            deployment,
        } => test_auth(profile, deployment, config, output).await,
        AuthCommands::Setup => setup_wizard(config).await,
    }
}

/// Test authentication for a profile
async fn test_auth(
    profile_name: Option<String>,
    deployment_override: Option<DeploymentType>,
    config: &Config,
    output: OutputFormatter,
) -> Result<()> {
    // Get the profile to test, or use environment variables if no profile is configured
    let (profile_name, profile_opt, deployment_type) = if let Some(name) = profile_name {
        let p = config
            .profiles
            .get(&name)
            .ok_or_else(|| anyhow::anyhow!("Profile '{}' not found", name))?;
        let dt = deployment_override.unwrap_or(p.deployment_type);
        (name, Some(p), dt)
    } else {
        // Find active profile with its name
        let env_profile = std::env::var("REDISCTL_PROFILE").ok();
        if let Some(name) = env_profile.as_deref().or(config.default_profile.as_deref()) {
            let p = config
                .profiles
                .get(name)
                .ok_or_else(|| anyhow::anyhow!("Profile '{}' not found", name))?;
            let dt = deployment_override.unwrap_or(p.deployment_type);
            (name.to_string(), Some(p), dt)
        } else {
            // No profile configured, try to determine deployment type from environment or flag
            let deployment_type = if let Some(dt) = deployment_override {
                dt
            } else {
                // Try to detect deployment type from environment variables
                if std::env::var("REDIS_CLOUD_API_KEY").is_ok() {
                    DeploymentType::Cloud
                } else if std::env::var("REDIS_ENTERPRISE_URL").is_ok() {
                    DeploymentType::Enterprise
                } else {
                    anyhow::bail!(
                        "No profile configured and no deployment type specified. Use --deployment flag or run 'redisctl auth setup' to create a profile."
                    )
                }
            };
            ("environment".to_string(), None, deployment_type)
        }
    };

    // Build result object to track all tests
    let mut result = json!({
        "profile": profile_name,
        "deployment_type": deployment_type.to_string(),
        "tests": {}
    });

    match deployment_type {
        DeploymentType::Cloud => test_cloud_auth(profile_opt, &mut result).await?,
        DeploymentType::Enterprise => test_enterprise_auth(profile_opt, &mut result).await?,
    }

    output.format(&result)?;
    Ok(())
}

/// Test Redis Cloud authentication
async fn test_cloud_auth(profile: Option<&Profile>, result: &mut serde_json::Value) -> Result<()> {
    let tests = result["tests"].as_object_mut().unwrap();

    // Check for API credentials from profile or environment
    let (api_key, api_secret, _api_url) =
        if let Some(p) = profile.and_then(|p| p.cloud_credentials()) {
            tests.insert("credentials_present".to_string(), json!(true));
            tests.insert("source".to_string(), json!("profile"));
            (p.0.to_string(), p.1.to_string(), p.2.to_string())
        } else {
            // Try environment variables
            match (
                std::env::var("REDIS_CLOUD_API_KEY"),
                std::env::var("REDIS_CLOUD_API_SECRET"),
            ) {
                (Ok(key), Ok(secret)) => {
                    tests.insert("credentials_present".to_string(), json!(true));
                    tests.insert("source".to_string(), json!("environment"));
                    let url = std::env::var("REDIS_CLOUD_API_URL")
                        .unwrap_or_else(|_| "https://api.redislabs.com/v1".to_string());
                    (key, secret, url)
                }
                (Err(_), _) => {
                    tests.insert("credentials_present".to_string(), json!(false));
                    tests.insert("error".to_string(), json!("Missing API key"));
                    return Ok(());
                }
                (_, Err(_)) => {
                    tests.insert("credentials_present".to_string(), json!(false));
                    tests.insert("error".to_string(), json!("Missing API secret key"));
                    return Ok(());
                }
            }
        };

    // Test API connectivity
    let client = CloudClient::builder()
        .api_key(api_key)
        .api_secret(api_secret)
        .build()
        .context("Failed to create Cloud client")?;

    // Use /subscriptions endpoint to test authentication
    // This is a documented endpoint that should exist for all Cloud accounts
    match client.get_raw("/subscriptions").await {
        Ok(subscriptions_data) => {
            tests.insert("api_connectivity".to_string(), json!(true));
            tests.insert("authentication".to_string(), json!(true));

            // Extract subscription count if available
            if let Some(subscriptions) = subscriptions_data.get("subscriptions")
                && let Some(arr) = subscriptions.as_array()
            {
                tests.insert("subscription_count".to_string(), json!(arr.len()));

                // If there are subscriptions, show the first one's name
                if let Some(first) = arr.first()
                    && let Some(name) = first.get("name")
                {
                    tests.insert("first_subscription".to_string(), name.clone());
                }
            }

            tests.insert("status".to_string(), json!("✅ Authentication successful"));
        }
        Err(e) => {
            tests.insert("api_connectivity".to_string(), json!(false));
            tests.insert("authentication".to_string(), json!(false));
            tests.insert("error".to_string(), json!(e.to_string()));
            tests.insert("status".to_string(), json!("❌ Authentication failed"));
        }
    }

    Ok(())
}

/// Test Redis Enterprise authentication
async fn test_enterprise_auth(
    profile: Option<&Profile>,
    result: &mut serde_json::Value,
) -> Result<()> {
    let tests = result["tests"].as_object_mut().unwrap();

    // Check for credentials from profile or environment
    let (url, username, password, insecure) =
        if let Some(p) = profile.and_then(|p| p.enterprise_credentials()) {
            tests.insert("credentials_present".to_string(), json!(true));
            tests.insert("source".to_string(), json!("profile"));
            (
                p.0.to_string(),
                p.1.to_string(),
                p.2.map(|p| p.to_string()),
                p.3,
            )
        } else {
            // Try environment variables
            let url = std::env::var("REDIS_ENTERPRISE_URL")
                .map_err(|_| anyhow::anyhow!("Missing Enterprise URL"))?;
            let username = std::env::var("REDIS_ENTERPRISE_USER")
                .map_err(|_| anyhow::anyhow!("Missing Enterprise username"))?;
            let password = std::env::var("REDIS_ENTERPRISE_PASSWORD").ok();
            let insecure = std::env::var("REDIS_ENTERPRISE_INSECURE").unwrap_or_default() == "true";

            tests.insert("credentials_present".to_string(), json!(true));
            tests.insert("source".to_string(), json!("environment"));
            (url, username, password, insecure)
        };

    let password = password.ok_or_else(|| anyhow::anyhow!("Missing Enterprise password"))?;

    tests.insert("url".to_string(), json!(url));

    // Test API connectivity
    let mut builder = EnterpriseClient::builder()
        .base_url(&url)
        .username(&username)
        .password(&password);

    if insecure {
        builder = builder.insecure(true);
        tests.insert("ssl_verification".to_string(), json!("disabled"));
    } else {
        tests.insert("ssl_verification".to_string(), json!("enabled"));
    }

    let client = builder
        .build()
        .context("Failed to create Enterprise client")?;

    // Test with cluster info endpoint
    match client.get_raw("/v1/cluster").await {
        Ok(cluster_info) => {
            tests.insert("api_connectivity".to_string(), json!(true));
            tests.insert("authentication".to_string(), json!(true));

            // Extract cluster details if available
            if let Some(obj) = cluster_info.as_object() {
                if let Some(name) = obj.get("name") {
                    tests.insert("cluster_name".to_string(), name.clone());
                }
                if let Some(version) = obj.get("software_version") {
                    tests.insert("cluster_version".to_string(), version.clone());
                }
            }

            tests.insert("status".to_string(), json!("✅ Authentication successful"));
        }
        Err(e) => {
            tests.insert("api_connectivity".to_string(), json!(false));
            tests.insert("authentication".to_string(), json!(false));
            tests.insert("error".to_string(), json!(e.to_string()));
            tests.insert("status".to_string(), json!("❌ Authentication failed"));

            // Provide helpful error messages
            let error_msg = e.to_string();
            if error_msg.contains("certificate") || error_msg.contains("SSL") {
                tests.insert("suggestion".to_string(), json!(
                    "Try setting REDIS_ENTERPRISE_INSECURE=true or add insecure: true to your profile"
                ));
            } else if error_msg.contains("401") || error_msg.contains("Unauthorized") {
                tests.insert(
                    "suggestion".to_string(),
                    json!("Check your username and password"),
                );
            } else if error_msg.contains("connection") || error_msg.contains("refused") {
                tests.insert(
                    "suggestion".to_string(),
                    json!("Check the URL and ensure the cluster is accessible"),
                );
            }
        }
    }

    Ok(())
}

/// Interactive setup wizard for creating profiles
async fn setup_wizard(config: &Config) -> Result<()> {
    use dialoguer::{Confirm, Input, Password, Select};

    println!("\n{}", "Welcome to redisctl setup wizard!".bold().green());
    println!("This wizard will help you configure authentication for Redis Cloud or Enterprise.\n");

    // Select deployment type
    let deployment_types = vec!["Redis Cloud", "Redis Enterprise"];
    let deployment_idx = Select::new()
        .with_prompt("Which Redis deployment are you using?")
        .items(&deployment_types)
        .default(0)
        .interact()?;

    let deployment_type = match deployment_idx {
        0 => DeploymentType::Cloud,
        1 => DeploymentType::Enterprise,
        _ => unreachable!(),
    };

    // Get profile name
    let profile_name: String = Input::new()
        .with_prompt("Profile name")
        .default(
            if deployment_type == DeploymentType::Cloud {
                "cloud"
            } else {
                "enterprise"
            }
            .to_string(),
        )
        .interact_text()?;

    let credentials = match deployment_type {
        DeploymentType::Cloud => {
            println!("\n{}", "Redis Cloud Configuration".bold());
            println!(
                "You'll need your API credentials from: https://app.redislabs.com/#/settings/cloud-api-keys\n"
            );

            let api_key = Input::new().with_prompt("API Key").interact_text()?;

            let api_secret = Password::new().with_prompt("API Secret Key").interact()?;

            // Optionally set API URL
            let api_url = if Confirm::new()
                .with_prompt("Use custom API URL? (default: https://api.redislabs.com/v1)")
                .default(false)
                .interact()?
            {
                Input::new()
                    .with_prompt("API URL")
                    .default("https://api.redislabs.com/v1".to_string())
                    .interact_text()?
            } else {
                "https://api.redislabs.com/v1".to_string()
            };

            ProfileCredentials::Cloud {
                api_key,
                api_secret,
                api_url,
            }
        }
        DeploymentType::Enterprise => {
            println!("\n{}", "Redis Enterprise Configuration".bold());

            let url = Input::new()
                .with_prompt("Cluster URL (e.g., https://cluster.example.com:9443)")
                .interact_text()?;

            let username = Input::new()
                .with_prompt("Username")
                .default("admin@example.com".to_string())
                .interact_text()?;

            let password = Some(Password::new().with_prompt("Password").interact()?);

            // Ask about SSL verification
            let insecure = Confirm::new()
                .with_prompt("Skip SSL certificate verification? (for self-signed certificates)")
                .default(false)
                .interact()?;

            ProfileCredentials::Enterprise {
                url,
                username,
                password,
                insecure,
            }
        }
    };

    let profile = Profile {
        deployment_type,
        credentials,
    };

    // Test the configuration
    println!("\n{}", "Testing authentication...".yellow());

    let mut test_result = json!({
        "profile": profile_name.clone(),
        "deployment_type": deployment_type.to_string(),
        "tests": {}
    });

    let success = match deployment_type {
        DeploymentType::Cloud => {
            test_cloud_auth(Some(&profile), &mut test_result)
                .await
                .is_ok()
                && test_result["tests"]["authentication"] == json!(true)
        }
        DeploymentType::Enterprise => {
            test_enterprise_auth(Some(&profile), &mut test_result)
                .await
                .is_ok()
                && test_result["tests"]["authentication"] == json!(true)
        }
    };

    if success {
        println!("{}", "✅ Authentication successful!".green().bold());

        // Save the profile
        let mut config = config.clone();
        config.profiles.insert(profile_name.clone(), profile);

        // Set as default if it's the first profile or user wants it
        if config.profiles.len() == 1
            || (config.default_profile.is_none()
                && Confirm::new()
                    .with_prompt("Set as default profile?")
                    .default(true)
                    .interact()?)
        {
            config.default_profile = Some(profile_name.clone());
        }

        config.save().context("Failed to save configuration")?;

        println!(
            "\n{}",
            format!("Profile '{}' saved successfully!", profile_name)
                .green()
                .bold()
        );
        println!("\nYou can now use redisctl commands like:");
        println!(
            "  redisctl {} database list",
            if deployment_type == DeploymentType::Cloud {
                "cloud"
            } else {
                "enterprise"
            }
        );
        println!("  redisctl cluster info");
        println!("\nTo test authentication anytime: redisctl auth test");
    } else {
        println!("{}", "❌ Authentication failed!".red().bold());
        if let Some(error) = test_result["tests"]["error"].as_str() {
            println!("Error: {}", error);
        }
        if let Some(suggestion) = test_result["tests"]["suggestion"].as_str() {
            println!("Suggestion: {}", suggestion);
        }
        println!("\nPlease check your credentials and try again.");
    }

    Ok(())
}
