use anyhow::{bail, Result};
use redis_common::{Config, DeploymentType, Profile, ProfileError, RoutingError};
use tracing::{debug, info};

use crate::cli::{Cli, Commands};
use crate::commands::{cloud, enterprise, profile};

pub async fn route_command(cli: Cli, config: &Config) -> Result<()> {
    let output_format = cli.output;
    let query = cli.query.as_deref();
    
    match cli.command {
        Commands::Profile { command } => {
            profile::handle_profile_command(command, config, output_format).await
        }
        Commands::Cloud { command } => {
            let profile = get_profile_for_deployment(config, &cli.profile, DeploymentType::Cloud)?;
            cloud::handle_cloud_command(command, profile, output_format, query).await
        }
        Commands::Enterprise { command } => {
            let profile = get_profile_for_deployment(config, &cli.profile, DeploymentType::Enterprise)?;
            enterprise::handle_enterprise_command(command, profile, output_format, query).await
        }
        // Smart routing commands
        Commands::Database { command } => {
            route_database_command(config, &cli.profile, cli.deployment, command, output_format, query).await
        }
        Commands::Cluster { command } => {
            route_cluster_command(config, &cli.profile, cli.deployment, command, output_format, query).await
        }
        Commands::User { command } => {
            route_user_command(config, &cli.profile, cli.deployment, command, output_format, query).await
        }
        Commands::Account { command } => {
            route_account_command(config, &cli.profile, cli.deployment, command, output_format, query).await
        }
    }
}

async fn route_database_command(
    config: &Config,
    profile_name: &Option<String>,
    deployment_type: Option<DeploymentType>,
    command: crate::cli::DatabaseCommands,
    output_format: redis_common::OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    if let Some(dep_type) = deployment_type {
        let profile = get_profile_for_deployment(config, profile_name, dep_type)?;
        match dep_type {
            DeploymentType::Cloud => {
                cloud::handle_database_command(command, profile, output_format, query).await
            }
            DeploymentType::Enterprise => {
                enterprise::handle_database_command(command, profile, output_format, query).await
            }
        }
    } else {
        let (profile, dep_type) = get_profile_with_type(config, profile_name)?;
        match dep_type {
            DeploymentType::Cloud => {
                cloud::handle_database_command(command, profile, output_format, query).await
            }
            DeploymentType::Enterprise => {
                enterprise::handle_database_command(command, profile, output_format, query).await
            }
        }
    }
}

async fn route_cluster_command(
    config: &Config,
    profile_name: &Option<String>,
    deployment_type: Option<DeploymentType>,
    command: crate::cli::ClusterCommands,
    output_format: redis_common::OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    if let Some(dep_type) = deployment_type {
        let profile = get_profile_for_deployment(config, profile_name, dep_type)?;
        match dep_type {
            DeploymentType::Cloud => {
                bail!("Cluster commands are not available for Redis Cloud. Use 'redisctl cloud subscription' instead.")
            }
            DeploymentType::Enterprise => {
                enterprise::handle_cluster_command(command, profile, output_format, query).await
            }
        }
    } else {
        let (profile, dep_type) = get_profile_with_type(config, profile_name)?;
        match dep_type {
            DeploymentType::Cloud => {
                bail!("Cluster commands are not available for Redis Cloud. Use 'redisctl cloud subscription' instead.")
            }
            DeploymentType::Enterprise => {
                enterprise::handle_cluster_command(command, profile, output_format, query).await
            }
        }
    }
}

async fn route_user_command(
    config: &Config,
    profile_name: &Option<String>,
    deployment_type: Option<DeploymentType>,
    command: crate::cli::UserCommands,
    output_format: redis_common::OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    if let Some(dep_type) = deployment_type {
        let profile = get_profile_for_deployment(config, profile_name, dep_type)?;
        match dep_type {
            DeploymentType::Cloud => {
                cloud::handle_user_command(command, profile, output_format, query).await
            }
            DeploymentType::Enterprise => {
                enterprise::handle_user_command(command, profile, output_format, query).await
            }
        }
    } else {
        let (profile, dep_type) = get_profile_with_type(config, profile_name)?;
        match dep_type {
            DeploymentType::Cloud => {
                cloud::handle_user_command(command, profile, output_format, query).await
            }
            DeploymentType::Enterprise => {
                enterprise::handle_user_command(command, profile, output_format, query).await
            }
        }
    }
}

async fn route_account_command(
    config: &Config,
    profile_name: &Option<String>,
    deployment_type: Option<DeploymentType>,
    command: crate::cli::AccountCommands,
    output_format: redis_common::OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    if let Some(dep_type) = deployment_type {
        let profile = get_profile_for_deployment(config, profile_name, dep_type)?;
        match dep_type {
            DeploymentType::Cloud => {
                cloud::handle_account_command(command, profile, output_format, query).await
            }
            DeploymentType::Enterprise => {
                bail!("Account commands are not available for Redis Enterprise. Use 'redisctl enterprise cluster' instead.")
            }
        }
    } else {
        let (profile, dep_type) = get_profile_with_type(config, profile_name)?;
        match dep_type {
            DeploymentType::Cloud => {
                cloud::handle_account_command(command, profile, output_format, query).await
            }
            DeploymentType::Enterprise => {
                bail!("Account commands are not available for Redis Enterprise. Use 'redisctl enterprise cluster' instead.")
            }
        }
    }
}

fn get_profile_with_type<'a>(
    config: &'a Config,
    profile_name: &Option<String>,
) -> Result<(&'a Profile, DeploymentType)> {
    let env_profile = std::env::var("REDISCTL_PROFILE").ok();
    let profile_name = profile_name.as_deref()
        .or(config.default.as_deref())
        .or(env_profile.as_deref());

    if let Some(name) = profile_name {
        if let Some(profile) = config.profiles.get(name) {
            info!("Using profile '{}' with deployment type {:?}", name, profile.deployment_type);
            return Ok((profile, profile.deployment_type));
        }
    }

    // No profile specified or found
    bail!(RoutingError::NoProfileSpecified)
}

fn get_profile_for_deployment<'a>(
    config: &'a Config,
    profile_name: &Option<String>,
    expected_type: DeploymentType,
) -> Result<&'a Profile> {
    let env_profile = std::env::var("REDISCTL_PROFILE").ok();
    let profile_name = profile_name.as_deref()
        .or(config.default.as_deref())
        .or(env_profile.as_deref());

    let profile_name = profile_name.ok_or(RoutingError::NoProfileSpecified)?;
    
    let profile = config.profiles.get(profile_name)
        .ok_or_else(|| ProfileError::MissingCredentials { name: profile_name.to_string() })?;

    if profile.deployment_type != expected_type {
        bail!(ProfileError::TypeMismatch {
            name: profile_name.to_string(),
            actual_type: format!("{:?}", profile.deployment_type),
            expected_type: format!("{:?}", expected_type),
        });
    }

    debug!("Using profile '{}' for {:?}", profile_name, expected_type);
    Ok(profile)
}