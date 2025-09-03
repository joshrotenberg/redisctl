//! Configuration management commands

#![allow(dead_code)]

use anyhow::Result;
use serde_json::json;
use std::collections::HashMap;

use crate::cli::ConfigCommands;
use crate::config::{Config, ProfileCredentials};
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

/// Execute config commands
pub async fn execute(cmd: ConfigCommands, config: &Config, output: OutputFormatter) -> Result<()> {
    match cmd {
        ConfigCommands::Show { show_secrets } => show_config(config, show_secrets, output),
        ConfigCommands::Path => show_path(output),
        ConfigCommands::Validate { profile } => validate_config(config, profile, output).await,
    }
}

/// Show current configuration
fn show_config(config: &Config, show_secrets: bool, output: OutputFormatter) -> Result<()> {
    // Find active profile name
    let env_profile = std::env::var("REDISCTL_PROFILE").ok();
    let active_profile_name = env_profile.as_deref().or(config.default_profile.as_deref());

    let mut result = json!({
        "config_path": Config::config_path()?.to_string_lossy(),
        "default_profile": config.default_profile,
        "active_profile": active_profile_name,
        "profiles": {}
    });

    // Add profile details
    let profiles = result["profiles"].as_object_mut().unwrap();
    for (name, profile) in &config.profiles {
        let mut profile_info = json!({
            "name": name,
            "deployment_type": profile.deployment_type.to_string(),
        });

        match &profile.credentials {
            ProfileCredentials::Cloud {
                api_key,
                api_secret,
                api_url,
            } => {
                profile_info["cloud_api_url"] = json!(api_url);
                profile_info["cloud_api_key"] = if show_secrets {
                    json!(api_key)
                } else {
                    json!(if api_key.len() > 8 {
                        format!("{}...{}", &api_key[..4], &api_key[api_key.len() - 4..])
                    } else {
                        "***".to_string()
                    })
                };
                profile_info["cloud_api_secret"] = if show_secrets {
                    json!(api_secret)
                } else {
                    json!("***")
                };
            }
            ProfileCredentials::Enterprise {
                url,
                username,
                password,
                insecure,
            } => {
                profile_info["enterprise_url"] = json!(url);
                profile_info["enterprise_username"] = json!(username);
                profile_info["enterprise_password"] = if show_secrets {
                    json!(password)
                } else {
                    json!(password.as_ref().map(|_| "***"))
                };
                profile_info["enterprise_insecure"] = json!(insecure);
            }
        }

        profiles.insert(name.clone(), profile_info);
    }

    // Add environment variable status
    let mut env_vars = HashMap::new();

    // Check profile selection env var
    if let Ok(profile) = std::env::var("REDISCTL_PROFILE") {
        env_vars.insert("REDISCTL_PROFILE", profile);
    }

    // Check Cloud env vars
    if let Ok(key) = std::env::var("REDIS_CLOUD_API_KEY") {
        env_vars.insert(
            "REDIS_CLOUD_API_KEY",
            if show_secrets {
                key
            } else if key.len() > 8 {
                format!("{}...{}", &key[..4], &key[key.len() - 4..])
            } else {
                "***".to_string()
            },
        );
    }
    if std::env::var("REDIS_CLOUD_API_SECRET").is_ok() {
        env_vars.insert(
            "REDIS_CLOUD_API_SECRET",
            if show_secrets {
                std::env::var("REDIS_CLOUD_API_SECRET").unwrap()
            } else {
                "***".to_string()
            },
        );
    }

    // Check Enterprise env vars
    if let Ok(url) = std::env::var("REDIS_ENTERPRISE_URL") {
        env_vars.insert("REDIS_ENTERPRISE_URL", url);
    }
    if let Ok(user) = std::env::var("REDIS_ENTERPRISE_USER") {
        env_vars.insert("REDIS_ENTERPRISE_USER", user);
    }
    if std::env::var("REDIS_ENTERPRISE_PASSWORD").is_ok() {
        env_vars.insert(
            "REDIS_ENTERPRISE_PASSWORD",
            if show_secrets {
                std::env::var("REDIS_ENTERPRISE_PASSWORD").unwrap()
            } else {
                "***".to_string()
            },
        );
    }
    if let Ok(insecure) = std::env::var("REDIS_ENTERPRISE_INSECURE") {
        env_vars.insert("REDIS_ENTERPRISE_INSECURE", insecure);
    }

    if !env_vars.is_empty() {
        result["environment_variables"] = json!(env_vars);
    }

    // Add configuration priority explanation
    result["priority_order"] = json!([
        "1. Command-line flags (--profile)",
        "2. Environment variables (REDISCTL_PROFILE, REDIS_*)",
        "3. Profile configuration",
        "4. Default profile"
    ]);

    output.format(&result)?;
    Ok(())
}

/// Show configuration file path
fn show_path(output: OutputFormatter) -> Result<()> {
    let path = Config::config_path()?;
    let exists = path.exists();

    let result = json!({
        "path": path.to_string_lossy(),
        "exists": exists,
        "directory": path.parent().map(|p| p.to_string_lossy()),
    });

    output.format(&result)?;
    Ok(())
}

/// Validate configuration
async fn validate_config(
    config: &Config,
    profile_name: Option<String>,
    output: OutputFormatter,
) -> Result<()> {
    let mut results = Vec::new();

    if let Some(name) = profile_name {
        // Validate specific profile
        let profile = config
            .profiles
            .get(&name)
            .ok_or_else(|| anyhow::anyhow!("Profile '{}' not found", name))?;

        let validation = validate_profile(&name, profile);
        results.push(validation);
    } else {
        // Validate all profiles
        for (name, profile) in &config.profiles {
            let validation = validate_profile(name, profile);
            results.push(validation);
        }
    }

    let all_valid = results.iter().all(|r| r["valid"] == json!(true));

    let result = json!({
        "all_valid": all_valid,
        "profiles": results,
        "config_file": Config::config_path()?.to_string_lossy(),
        "config_file_exists": Config::config_path()?.exists(),
    });

    output.format(&result)?;

    if !all_valid {
        std::process::exit(1);
    }

    Ok(())
}

/// Validate a single profile
fn validate_profile(name: &str, profile: &crate::config::Profile) -> serde_json::Value {
    let mut issues = Vec::new();
    let mut warnings = Vec::new();

    match &profile.credentials {
        ProfileCredentials::Cloud {
            api_key,
            api_secret,
            api_url,
        } => {
            // Check for API key
            if api_key.is_empty() && std::env::var("REDIS_CLOUD_API_KEY").is_err() {
                issues.push("Missing API key (set in profile or REDIS_CLOUD_API_KEY env var)");
            }

            // Check for API secret
            if api_secret.is_empty() && std::env::var("REDIS_CLOUD_API_SECRET").is_err() {
                issues.push(
                    "Missing API secret key (set in profile or REDIS_CLOUD_API_SECRET env var)",
                );
            }

            // Check API URL format
            if !api_url.starts_with("http://") && !api_url.starts_with("https://") {
                issues.push("Invalid API URL format (must start with http:// or https://)");
            }
        }
        ProfileCredentials::Enterprise {
            url,
            username,
            password,
            insecure,
        } => {
            // Check for URL
            if url.is_empty() && std::env::var("REDIS_ENTERPRISE_URL").is_err() {
                issues.push("Missing cluster URL (set in profile or REDIS_ENTERPRISE_URL env var)");
            } else if !url.starts_with("http://") && !url.starts_with("https://") {
                issues.push("Invalid cluster URL format (must start with http:// or https://)");
            }

            // Check for username
            if username.is_empty() && std::env::var("REDIS_ENTERPRISE_USER").is_err() {
                issues.push("Missing username (set in profile or REDIS_ENTERPRISE_USER env var)");
            }

            // Check for password
            if password.is_none() && std::env::var("REDIS_ENTERPRISE_PASSWORD").is_err() {
                issues
                    .push("Missing password (set in profile or REDIS_ENTERPRISE_PASSWORD env var)");
            }

            // Warn about insecure mode
            if *insecure || std::env::var("REDIS_ENTERPRISE_INSECURE").unwrap_or_default() == "true"
            {
                warnings.push("SSL certificate verification is disabled");
            }
        }
    }

    json!({
        "profile": name,
        "deployment_type": profile.deployment_type.to_string(),
        "valid": issues.is_empty(),
        "issues": issues,
        "warnings": warnings,
    })
}
