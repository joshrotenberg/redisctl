use crate::config::{Config, DeploymentType, Profile, ProfileCredentials};
use crate::output::{OutputFormat, print_output};
use anyhow::Result;

use crate::cli::ProfileCommands;

pub async fn handle_profile_command(
    command: ProfileCommands,
    config: &Config,
    output_format: OutputFormat,
) -> Result<()> {
    let mut config = config.clone();

    match command {
        ProfileCommands::List => {
            let profiles = config.list_profiles();
            let profile_list: Vec<_> = profiles
                .into_iter()
                .map(|(name, profile)| {
                    serde_json::json!({
                        "name": name,
                        "deployment_type": profile.deployment_type,
                        "is_default": config.default_profile.as_ref() == Some(name)
                    })
                })
                .collect();
            print_output(profile_list, output_format, None)?;
        }
        ProfileCommands::Show { name } => {
            let env_profile = std::env::var("REDISCTL_PROFILE").ok();
            let profile_name = name
                .as_deref()
                .or(config.default_profile.as_deref())
                .or(env_profile.as_deref())
                .ok_or_else(|| {
                    anyhow::anyhow!("No profile specified and no default profile set")
                })?;

            let profile = config
                .profiles
                .get(profile_name)
                .ok_or_else(|| anyhow::anyhow!("Profile '{}' not found", profile_name))?;

            let profile_info = serde_json::json!({
                "name": profile_name,
                "deployment_type": profile.deployment_type,
                "credentials": match &profile.credentials {
                    ProfileCredentials::Cloud { api_url, .. } => {
                        serde_json::json!({
                            "type": "cloud",
                            "api_url": api_url,
                            "has_credentials": true
                        })
                    }
                    ProfileCredentials::Enterprise { url, username, insecure, .. } => {
                        serde_json::json!({
                            "type": "enterprise",
                            "url": url,
                            "username": username,
                            "insecure": insecure,
                            "has_password": profile.credentials.has_password()
                        })
                    }
                },
                "is_default": config.default_profile.as_deref() == Some(profile_name)
            });

            print_output(profile_info, output_format, None)?;
        }
        ProfileCommands::Set {
            name,
            deployment_type,
            url,
            username,
            password,
            api_key,
            api_secret,
            insecure,
        } => {
            let credentials = match deployment_type {
                DeploymentType::Cloud => {
                    let api_key = api_key.or_else(|| std::env::var("REDIS_CLOUD_API_KEY").ok())
                        .ok_or_else(|| anyhow::anyhow!("API key required for Cloud profile. Use --api-key or set REDIS_CLOUD_API_KEY"))?;
                    let api_secret = api_secret.or_else(|| std::env::var("REDIS_CLOUD_API_SECRET").ok())
                        .ok_or_else(|| anyhow::anyhow!("API secret required for Cloud profile. Use --api-secret or set REDIS_CLOUD_API_SECRET"))?;
                    let api_url = url.unwrap_or_else(|| "https://api.redislabs.com/v1".to_string());

                    ProfileCredentials::Cloud {
                        api_key,
                        api_secret,
                        api_url,
                    }
                }
                DeploymentType::Enterprise => {
                    let url = url.or_else(|| std::env::var("REDIS_ENTERPRISE_URL").ok())
                        .ok_or_else(|| anyhow::anyhow!("URL required for Enterprise profile. Use --url or set REDIS_ENTERPRISE_URL"))?;
                    let username = username.or_else(|| std::env::var("REDIS_ENTERPRISE_USER").ok())
                        .ok_or_else(|| anyhow::anyhow!("Username required for Enterprise profile. Use --username or set REDIS_ENTERPRISE_USER"))?;
                    let password =
                        password.or_else(|| std::env::var("REDIS_ENTERPRISE_PASSWORD").ok());

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

            config.set_profile(name.clone(), profile);
            config.save()?;

            println!("Profile '{}' created successfully", name);
        }
        ProfileCommands::Remove { name } => {
            if config.remove_profile(&name).is_some() {
                // If we're removing the default profile, clear the default
                if config.default_profile.as_ref() == Some(&name) {
                    config.default_profile = None;
                }
                config.save()?;
                println!("Profile '{}' removed successfully", name);
            } else {
                anyhow::bail!("Profile '{}' not found", name);
            }
        }
        ProfileCommands::Default { name } => {
            if config.profiles.contains_key(&name) {
                config.default_profile = Some(name.clone());
                config.save()?;
                println!("Default profile set to '{}'", name);
            } else {
                anyhow::bail!("Profile '{}' not found", name);
            }
        }
    }

    Ok(())
}
