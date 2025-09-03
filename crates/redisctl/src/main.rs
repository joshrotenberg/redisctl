use anyhow::Result;
use clap::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod cli;
mod config;
mod connection;
mod error;
mod output;

use cli::{Cli, Commands};
use config::Config;
use connection::ConnectionManager;
use error::RedisCtlError;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize tracing based on verbosity level
    init_tracing(cli.verbose);

    // Load configuration
    let config = Config::load()?;
    let conn_mgr = ConnectionManager::new(config);

    // Execute command
    if let Err(e) = execute_command(&cli, &conn_mgr).await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    Ok(())
}

fn init_tracing(verbose: u8) {
    let filter = match verbose {
        0 => "redisctl=warn",
        1 => "redisctl=info",
        2 => "redisctl=debug",
        _ => "redisctl=trace",
    };

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(filter))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

async fn execute_command(cli: &Cli, conn_mgr: &ConnectionManager) -> Result<(), RedisCtlError> {
    match &cli.command {
        Commands::Version => {
            println!("redisctl {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }

        Commands::Profile(profile_cmd) => execute_profile_command(profile_cmd, conn_mgr).await,

        Commands::Api {
            deployment,
            method,
            path,
            data,
        } => execute_api_command(cli, conn_mgr, deployment, method, path, data.as_deref()).await,

        Commands::Cloud(_) => {
            println!("Cloud commands are not yet implemented in this version");
            Ok(())
        }

        Commands::Enterprise(_) => {
            println!("Enterprise commands are not yet implemented in this version");
            Ok(())
        }

        Commands::Database(_) => {
            println!("Smart database commands are not yet implemented in this version");
            Ok(())
        }
    }
}

async fn execute_profile_command(
    profile_cmd: &cli::ProfileCommands,
    conn_mgr: &ConnectionManager,
) -> Result<(), RedisCtlError> {
    use cli::ProfileCommands::*;

    match profile_cmd {
        List => {
            let profiles = conn_mgr.config.list_profiles();
            if profiles.is_empty() {
                println!("No profiles configured.");
                println!("Use 'redisctl profile set' to create a profile.");
                return Ok(());
            }

            println!("{:<15} {:<12} {}", "NAME", "TYPE", "DETAILS");
            println!("{:-<15} {:-<12} {:-<30}", "", "", "");

            for (name, profile) in profiles {
                let mut details = String::new();
                match profile.deployment_type {
                    config::DeploymentType::Cloud => {
                        if let Some((_, _, url)) = profile.cloud_credentials() {
                            details = format!("URL: {}", url);
                        }
                    }
                    config::DeploymentType::Enterprise => {
                        if let Some((url, username, _, insecure)) = profile.enterprise_credentials()
                        {
                            details = format!(
                                "URL: {}, User: {}{}",
                                url,
                                username,
                                if insecure { " (insecure)" } else { "" }
                            );
                        }
                    }
                }

                let is_default = conn_mgr.config.default_profile.as_deref() == Some(name);
                let name_display = if is_default {
                    format!("{}*", name)
                } else {
                    name.to_string()
                };

                println!(
                    "{:<15} {:<12} {}",
                    name_display, profile.deployment_type, details
                );
            }

            Ok(())
        }

        Show { name } => match conn_mgr.config.profiles.get(name) {
            Some(profile) => {
                println!("Profile: {}", name);
                println!("Type: {}", profile.deployment_type);

                match profile.deployment_type {
                    config::DeploymentType::Cloud => {
                        if let Some((api_key, _, api_url)) = profile.cloud_credentials() {
                            println!(
                                "API Key: {}...",
                                &api_key[..std::cmp::min(8, api_key.len())]
                            );
                            println!("API URL: {}", api_url);
                        }
                    }
                    config::DeploymentType::Enterprise => {
                        if let Some((url, username, has_password, insecure)) =
                            profile.enterprise_credentials()
                        {
                            println!("URL: {}", url);
                            println!("Username: {}", username);
                            println!(
                                "Password: {}",
                                if has_password.is_some() {
                                    "configured"
                                } else {
                                    "not set"
                                }
                            );
                            println!("Insecure: {}", insecure);
                        }
                    }
                }

                let is_default = conn_mgr.config.default_profile.as_deref() == Some(name);
                if is_default {
                    println!("Default: yes");
                }

                Ok(())
            }
            None => Err(RedisCtlError::ProfileNotFound { name: name.clone() }),
        },

        _ => {
            println!("Profile management commands (set, remove, default) are not yet implemented");
            Ok(())
        }
    }
}

async fn execute_api_command(
    _cli: &Cli,
    _conn_mgr: &ConnectionManager,
    deployment: &config::DeploymentType,
    method: &cli::HttpMethod,
    path: &str,
    data: Option<&str>,
) -> Result<(), RedisCtlError> {
    println!("Raw API access will be implemented in the next PR");
    println!(
        "Command: {} {} {} with data: {:?}",
        deployment,
        method_to_string(method),
        path,
        data
    );
    Ok(())
}

fn method_to_string(method: &cli::HttpMethod) -> &'static str {
    match method {
        cli::HttpMethod::Get => "GET",
        cli::HttpMethod::Post => "POST",
        cli::HttpMethod::Put => "PUT",
        cli::HttpMethod::Patch => "PATCH",
        cli::HttpMethod::Delete => "DELETE",
    }
}
