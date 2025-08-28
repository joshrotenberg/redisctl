use crate::config::{Config, DeploymentType, Profile, ProfileCredentials};
use crate::error::{ProfileError, RoutingError};
use anyhow::{Result, bail};
use std::borrow::Cow;
use tracing::{debug, info};

use crate::cli::{Cli, Commands};
use crate::commands::{auth, cloud, config, enterprise, profile};

pub async fn route_command(cli: Cli, config: &Config) -> Result<()> {
    let output_format = cli.output;
    let query = cli.query.as_deref();

    match cli.command {
        Commands::Profile { command } => {
            profile::handle_profile_command(command, config, output_format).await
        }
        Commands::Cloud { command } => {
            let profile = get_profile_for_deployment(config, &cli.profile, DeploymentType::Cloud)?;
            cloud::handle_cloud_command(command, &profile, output_format, query).await
        }
        Commands::Enterprise { command } => {
            let profile =
                get_profile_for_deployment(config, &cli.profile, DeploymentType::Enterprise)?;
            enterprise::handle_enterprise_command(command, &profile, output_format, query).await
        }
        // Smart routing commands
        Commands::Database { command } => {
            route_database_command(
                config,
                &cli.profile,
                cli.deployment,
                command,
                output_format,
                query,
            )
            .await
        }
        Commands::Cluster { command } => {
            route_cluster_command(
                config,
                &cli.profile,
                cli.deployment,
                command,
                output_format,
                query,
            )
            .await
        }
        Commands::User { command } => {
            route_user_command(
                config,
                &cli.profile,
                cli.deployment,
                command,
                output_format,
                query,
            )
            .await
        }
        Commands::Account { command } => {
            route_account_command(
                config,
                &cli.profile,
                cli.deployment,
                command,
                output_format,
                query,
            )
            .await
        }
        Commands::Auth { command } => {
            let output = auth::OutputFormatter {
                format: output_format,
                query: query.map(|s| s.to_string()),
            };
            auth::execute(command, config, output).await
        }
        Commands::Config { command } => {
            let output = config::OutputFormatter {
                format: output_format,
                query: query.map(|s| s.to_string()),
            };
            config::execute(command, config, output).await
        }
    }
}

async fn route_database_command(
    config: &Config,
    profile_name: &Option<String>,
    deployment_type: Option<DeploymentType>,
    command: crate::cli::DatabaseCommands,
    output_format: crate::output::OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    if let Some(dep_type) = deployment_type {
        let profile = get_profile_for_deployment(config, profile_name, dep_type)?;
        match dep_type {
            DeploymentType::Cloud => {
                cloud::handle_database_command(command, &profile, output_format, query).await
            }
            DeploymentType::Enterprise => {
                enterprise::handle_database_command(command, &profile, output_format, query).await
            }
        }
    } else {
        let (profile, dep_type) = get_profile_with_type(config, profile_name)?;
        match dep_type {
            DeploymentType::Cloud => {
                cloud::handle_database_command(command, &profile, output_format, query).await
            }
            DeploymentType::Enterprise => {
                enterprise::handle_database_command(command, &profile, output_format, query).await
            }
        }
    }
}

async fn route_cluster_command(
    config: &Config,
    profile_name: &Option<String>,
    deployment_type: Option<DeploymentType>,
    command: crate::cli::ClusterCommands,
    output_format: crate::output::OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    if let Some(dep_type) = deployment_type {
        let profile = get_profile_for_deployment(config, profile_name, dep_type)?;
        match dep_type {
            DeploymentType::Cloud => {
                bail!(
                    "Cluster commands are not available for Redis Cloud. Use 'redisctl cloud subscription' instead."
                )
            }
            DeploymentType::Enterprise => {
                enterprise::handle_cluster_command(command, &profile, output_format, query).await
            }
        }
    } else {
        let (profile, dep_type) = get_profile_with_type(config, profile_name)?;
        match dep_type {
            DeploymentType::Cloud => {
                bail!(
                    "Cluster commands are not available for Redis Cloud. Use 'redisctl cloud subscription' instead."
                )
            }
            DeploymentType::Enterprise => {
                enterprise::handle_cluster_command(command, &profile, output_format, query).await
            }
        }
    }
}

async fn route_user_command(
    config: &Config,
    profile_name: &Option<String>,
    deployment_type: Option<DeploymentType>,
    command: crate::cli::UserCommands,
    output_format: crate::output::OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    if let Some(dep_type) = deployment_type {
        let profile = get_profile_for_deployment(config, profile_name, dep_type)?;
        match dep_type {
            DeploymentType::Cloud => {
                cloud::handle_user_command(command, &profile, output_format, query).await
            }
            DeploymentType::Enterprise => {
                enterprise::handle_user_command(command, &profile, output_format, query).await
            }
        }
    } else {
        let (profile, dep_type) = get_profile_with_type(config, profile_name)?;
        match dep_type {
            DeploymentType::Cloud => {
                cloud::handle_user_command(command, &profile, output_format, query).await
            }
            DeploymentType::Enterprise => {
                enterprise::handle_user_command(command, &profile, output_format, query).await
            }
        }
    }
}

async fn route_account_command(
    config: &Config,
    profile_name: &Option<String>,
    deployment_type: Option<DeploymentType>,
    command: crate::cli::AccountCommands,
    output_format: crate::output::OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    if let Some(dep_type) = deployment_type {
        let profile = get_profile_for_deployment(config, profile_name, dep_type)?;
        match dep_type {
            DeploymentType::Cloud => {
                cloud::handle_account_command(command, &profile, output_format, query).await
            }
            DeploymentType::Enterprise => {
                bail!(
                    "Account commands are not available for Redis Enterprise. Use 'redisctl enterprise cluster' instead."
                )
            }
        }
    } else {
        let (profile, dep_type) = get_profile_with_type(config, profile_name)?;
        match dep_type {
            DeploymentType::Cloud => {
                cloud::handle_account_command(command, &profile, output_format, query).await
            }
            DeploymentType::Enterprise => {
                bail!(
                    "Account commands are not available for Redis Enterprise. Use 'redisctl enterprise cluster' instead."
                )
            }
        }
    }
}

fn get_profile_with_type<'a>(
    config: &'a Config,
    profile_name: &Option<String>,
) -> Result<(Cow<'a, Profile>, DeploymentType)> {
    let env_profile = std::env::var("REDISCTL_PROFILE").ok();
    let profile_name = profile_name
        .as_deref()
        .or(config.default_profile.as_deref())
        .or(env_profile.as_deref());

    if let Some(name) = profile_name
        && let Some(profile) = config.profiles.get(name)
    {
        info!(
            "Using profile '{}' with deployment type {:?}",
            name, profile.deployment_type
        );
        return Ok((Cow::Borrowed(profile), profile.deployment_type));
    }

    // Check for environment variables to auto-detect deployment type
    if let Some(profile) = profile_from_env_auto_detect()? {
        info!(
            "Using environment variables with deployment type {:?}",
            profile.deployment_type
        );
        let dep_type = profile.deployment_type;
        return Ok((Cow::Owned(profile), dep_type));
    }

    // No profile specified or found
    bail!(RoutingError::NoProfileSpecified)
}

fn get_profile_for_deployment<'a>(
    config: &'a Config,
    profile_name: &Option<String>,
    expected_type: DeploymentType,
) -> Result<Cow<'a, Profile>> {
    let env_profile = std::env::var("REDISCTL_PROFILE").ok();
    let profile_name = profile_name
        .as_deref()
        .or(config.default_profile.as_deref())
        .or(env_profile.as_deref());

    // Try to get profile from config first
    if let Some(name) = profile_name
        && let Some(profile) = config.profiles.get(name)
    {
        if profile.deployment_type != expected_type {
            bail!(ProfileError::TypeMismatch {
                name: name.to_string(),
                actual_type: format!("{:?}", profile.deployment_type),
                expected_type: format!("{:?}", expected_type),
            });
        }
        debug!("Using profile '{}' for {:?}", name, expected_type);
        return Ok(Cow::Borrowed(profile));
    }

    // Try to create profile from environment variables
    if let Some(profile) = profile_from_env(expected_type)? {
        debug!("Using environment variables for {:?}", expected_type);
        return Ok(Cow::Owned(profile));
    }

    // No profile found
    if let Some(name) = profile_name {
        bail!(ProfileError::MissingCredentials {
            name: name.to_string(),
        })
    } else {
        bail!(RoutingError::NoProfileSpecified)
    }
}

/// Create a profile from environment variables for a specific deployment type
fn profile_from_env(deployment_type: DeploymentType) -> Result<Option<Profile>> {
    match deployment_type {
        DeploymentType::Enterprise => {
            // Check for Enterprise environment variables
            let url = std::env::var("REDIS_ENTERPRISE_URL").ok();
            let username = std::env::var("REDIS_ENTERPRISE_USER").ok();
            let password = std::env::var("REDIS_ENTERPRISE_PASSWORD").ok();
            let insecure = std::env::var("REDIS_ENTERPRISE_INSECURE")
                .ok()
                .and_then(|v| v.parse::<bool>().ok())
                .unwrap_or(false);

            if let (Some(url), Some(username)) = (url, username) {
                return Ok(Some(Profile {
                    deployment_type: DeploymentType::Enterprise,
                    credentials: ProfileCredentials::Enterprise {
                        url,
                        username,
                        password,
                        insecure,
                    },
                }));
            }
        }
        DeploymentType::Cloud => {
            // Check for Cloud environment variables
            let api_key = std::env::var("REDIS_CLOUD_API_KEY").ok();
            let api_secret = std::env::var("REDIS_CLOUD_API_SECRET").ok();
            let api_url = std::env::var("REDIS_CLOUD_API_URL")
                .ok()
                .unwrap_or_else(|| "https://api.redislabs.com/v1".to_string());

            if let (Some(api_key), Some(api_secret)) = (api_key, api_secret) {
                return Ok(Some(Profile {
                    deployment_type: DeploymentType::Cloud,
                    credentials: ProfileCredentials::Cloud {
                        api_key,
                        api_secret,
                        api_url,
                    },
                }));
            }
        }
    }

    Ok(None)
}

/// Try to auto-detect deployment type from environment variables
fn profile_from_env_auto_detect() -> Result<Option<Profile>> {
    // Try Enterprise first
    if let Some(profile) = profile_from_env(DeploymentType::Enterprise)? {
        return Ok(Some(profile));
    }

    // Try Cloud
    if let Some(profile) = profile_from_env(DeploymentType::Cloud)? {
        return Ok(Some(profile));
    }

    Ok(None)
}
