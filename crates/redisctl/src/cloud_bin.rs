// Redis Cloud only binary
// This binary only includes Cloud functionality to reduce size for Cloud-only deployments

use anyhow::Result;
use clap::Parser;
use redis_common::{Config, DeploymentType};
use tracing::info;

mod cli;
mod commands;

use cli::{Cli, Commands};
use commands::{cloud, profile};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    info!("Starting redis-cloud CLI");

    let config = Config::load()?;

    match cli.command {
        Commands::Profile { command } => {
            profile::handle_profile_command(command, &config, cli.output).await
        }
        Commands::Cloud { command } => {
            let profile = get_cloud_profile(&config, &cli.profile)?;
            cloud::handle_cloud_command(command, profile, cli.output, cli.query.as_deref()).await
        }
        Commands::Database { command } => {
            let profile = get_cloud_profile(&config, &cli.profile)?;
            cloud::handle_database_command(command, profile, cli.output, cli.query.as_deref()).await
        }
        Commands::User { command } => {
            let profile = get_cloud_profile(&config, &cli.profile)?;
            cloud::handle_user_command(command, profile, cli.output, cli.query.as_deref()).await
        }
        Commands::Account { command } => {
            let profile = get_cloud_profile(&config, &cli.profile)?;
            cloud::handle_account_command(command, profile, cli.output, cli.query.as_deref()).await
        }
        _ => {
            anyhow::bail!(
                "Command not supported in Cloud-only binary. Use full 'redisctl' for Enterprise commands."
            )
        }
    }
}

fn get_cloud_profile<'a>(
    config: &'a Config,
    profile_name: &Option<String>,
) -> Result<&'a redis_common::Profile> {
    let env_profile = std::env::var("REDISCTL_PROFILE").ok();
    let profile_name = profile_name
        .as_deref()
        .or(config.default.as_deref())
        .or(env_profile.as_deref())
        .ok_or_else(|| anyhow::anyhow!("No profile specified"))?;

    let profile = config
        .profiles
        .get(profile_name)
        .ok_or_else(|| anyhow::anyhow!("Profile '{}' not found", profile_name))?;

    if profile.deployment_type != DeploymentType::Cloud {
        anyhow::bail!("Profile '{}' is not a Cloud profile", profile_name);
    }

    Ok(profile)
}
