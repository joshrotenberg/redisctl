use anyhow::Result;
use clap::Parser;
use tracing::{debug, error, info, trace};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod cli;
mod commands;
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
    // Check for RUST_LOG env var first, then fall back to verbosity flag
    let filter = if std::env::var("RUST_LOG").is_ok() {
        tracing_subscriber::EnvFilter::from_default_env()
    } else {
        let level = match verbose {
            0 => "redisctl=warn,redis_cloud=warn,redis_enterprise=warn",
            1 => "redisctl=info,redis_cloud=info,redis_enterprise=info",
            2 => "redisctl=debug,redis_cloud=debug,redis_enterprise=debug",
            _ => "redisctl=trace,redis_cloud=trace,redis_enterprise=trace",
        };
        tracing_subscriber::EnvFilter::new(level)
    };

    tracing_subscriber::registry()
        .with(filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_thread_ids(false)
                .with_thread_names(false)
                .compact(),
        )
        .init();

    debug!("Tracing initialized with verbosity level: {}", verbose);
}

async fn execute_command(cli: &Cli, conn_mgr: &ConnectionManager) -> Result<(), RedisCtlError> {
    // Log command execution with sanitized parameters
    trace!("Executing command: {:?}", cli.command);
    info!("Command: {}", format_command(&cli.command));

    let start = std::time::Instant::now();
    let result = match &cli.command {
        Commands::Version => {
            debug!("Showing version information");
            println!("redisctl {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }

        Commands::Profile(profile_cmd) => {
            debug!("Executing profile command");
            execute_profile_command(profile_cmd, conn_mgr).await
        }

        Commands::Api {
            deployment,
            method,
            path,
            data,
        } => {
            info!(
                "API call: {} {} {} (deployment: {:?})",
                method,
                path,
                if data.is_some() {
                    "with data"
                } else {
                    "no data"
                },
                deployment
            );
            execute_api_command(cli, conn_mgr, deployment, method, path, data.as_deref()).await
        }

        Commands::Cloud(cloud_cmd) => execute_cloud_command(cli, conn_mgr, cloud_cmd).await,

        Commands::Enterprise(enterprise_cmd) => {
            execute_enterprise_command(
                enterprise_cmd,
                conn_mgr,
                cli.profile.as_deref(),
                cli.output,
                cli.query.as_deref(),
            )
            .await
        }
    };

    let duration = start.elapsed();
    match &result {
        Ok(_) => info!("Command completed successfully in {:?}", duration),
        Err(e) => error!("Command failed after {:?}: {}", duration, e),
    }

    result
}

/// Format command for human-readable logging (without sensitive data)
fn format_command(command: &Commands) -> String {
    match command {
        Commands::Version => "version".to_string(),
        Commands::Profile(cmd) => {
            use cli::ProfileCommands::*;
            match cmd {
                List => "profile list".to_string(),
                Show { name } => format!("profile show {}", name),
                Set { name, .. } => format!("profile set {} [credentials redacted]", name),
                Remove { name } => format!("profile remove {}", name),
                Default { name } => format!("profile default {}", name),
            }
        }
        Commands::Api {
            deployment,
            method,
            path,
            ..
        } => {
            format!("api {:?} {} {}", deployment, method, path)
        }
        Commands::Cloud(cmd) => format!("cloud {:?}", cmd),
        Commands::Enterprise(cmd) => format!("enterprise {:?}", cmd),
    }
}

async fn execute_enterprise_command(
    enterprise_cmd: &cli::EnterpriseCommands,
    conn_mgr: &ConnectionManager,
    profile: Option<&str>,
    output: cli::OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    use cli::EnterpriseCommands::*;

    match enterprise_cmd {
        Cluster(cluster_cmd) => {
            commands::enterprise::cluster::handle_cluster_command(
                conn_mgr,
                profile,
                cluster_cmd,
                output,
                query,
            )
            .await
        }
        Database(db_cmd) => {
            commands::enterprise::database::handle_database_command(
                conn_mgr, profile, db_cmd, output, query,
            )
            .await
        }
        Node(node_cmd) => {
            commands::enterprise::node::handle_node_command(
                conn_mgr, profile, node_cmd, output, query,
            )
            .await
        }
        User(user_cmd) => {
            commands::enterprise::rbac::handle_user_command(
                conn_mgr, profile, user_cmd, output, query,
            )
            .await
        }
        Role(role_cmd) => {
            commands::enterprise::rbac::handle_role_command(
                conn_mgr, profile, role_cmd, output, query,
            )
            .await
        }
        Acl(acl_cmd) => {
            commands::enterprise::rbac::handle_acl_command(
                conn_mgr, profile, acl_cmd, output, query,
            )
            .await
        }
        Ldap(ldap_cmd) => {
            commands::enterprise::rbac::handle_ldap_command(
                conn_mgr, profile, ldap_cmd, output, query,
            )
            .await
        }
        Auth(auth_cmd) => {
            commands::enterprise::rbac::handle_auth_command(
                conn_mgr, profile, auth_cmd, output, query,
            )
            .await
        }
        Crdb(crdb_cmd) => {
            commands::enterprise::crdb::handle_crdb_command(
                conn_mgr, profile, crdb_cmd, output, query,
            )
            .await
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
            debug!("Listing all configured profiles");
            let profiles = conn_mgr.config.list_profiles();
            trace!("Found {} profiles", profiles.len());

            if profiles.is_empty() {
                info!("No profiles configured");
                println!("No profiles configured.");
                println!("Use 'redisctl profile set' to create a profile.");
                return Ok(());
            }

            println!("{:<15} {:<12} DETAILS", "NAME", "TYPE");
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
    cli: &Cli,
    conn_mgr: &ConnectionManager,
    deployment: &config::DeploymentType,
    method: &cli::HttpMethod,
    path: &str,
    data: Option<&str>,
) -> Result<(), RedisCtlError> {
    commands::api::handle_api_command(commands::api::ApiCommandParams {
        config: conn_mgr.config.clone(),
        profile_name: cli.profile.clone(),
        deployment: *deployment,
        method: method.clone(),
        path: path.to_string(),
        data: data.map(|s| s.to_string()),
        query: cli.query.clone(),
        output_format: cli.output,
    })
    .await
}

async fn execute_cloud_command(
    cli: &Cli,
    conn_mgr: &ConnectionManager,
    cloud_cmd: &cli::CloudCommands,
) -> Result<(), RedisCtlError> {
    use cli::CloudCommands::*;

    match cloud_cmd {
        Account(account_cmd) => {
            commands::cloud::handle_account_command(
                conn_mgr,
                cli.profile.as_deref(),
                account_cmd,
                cli.output,
                cli.query.as_deref(),
            )
            .await
        }

        Subscription(sub_cmd) => {
            commands::cloud::handle_subscription_command(
                conn_mgr,
                cli.profile.as_deref(),
                sub_cmd,
                cli.output,
                cli.query.as_deref(),
            )
            .await
        }

        Database(db_cmd) => {
            commands::cloud::handle_database_command(
                conn_mgr,
                cli.profile.as_deref(),
                db_cmd,
                cli.output,
                cli.query.as_deref(),
            )
            .await
        }

        User(user_cmd) => {
            commands::cloud::handle_user_command(
                conn_mgr,
                cli.profile.as_deref(),
                user_cmd,
                cli.output,
                cli.query.as_deref(),
            )
            .await
        }
        Acl(acl_cmd) => {
            commands::cloud::acl::handle_acl_command(
                conn_mgr,
                cli.profile.as_deref(),
                acl_cmd,
                cli.output,
                cli.query.as_deref(),
            )
            .await
        }
        ProviderAccount(provider_account_cmd) => {
            commands::cloud::cloud_account::handle_cloud_account_command(
                conn_mgr,
                cli.profile.as_deref(),
                provider_account_cmd,
                cli.output,
                cli.query.as_deref(),
            )
            .await
        }
        Task(task_cmd) => {
            commands::cloud::task::handle_task_command(
                conn_mgr,
                cli.profile.as_deref(),
                task_cmd,
                cli.output,
                cli.query.as_deref(),
            )
            .await
        }
        Connectivity(connectivity_cmd) => {
            commands::cloud::connectivity::handle_connectivity_command(
                conn_mgr,
                cli.profile.as_deref(),
                connectivity_cmd,
                cli.output,
                cli.query.as_deref(),
            )
            .await
        }
    }
}
