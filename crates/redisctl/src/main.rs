use anyhow::{Context, Result};
use clap::{CommandFactory, Parser};
use clap_complete::{generate, shells};
use tracing::{debug, error, info, trace};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod cli;
mod commands;
mod config;
mod connection;
mod error;
mod output;
mod workflows;

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
        Commands::Completions { shell } => {
            debug!("Generating completions for {:?}", shell);
            generate_completions(*shell);
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

/// Generate shell completions
fn generate_completions(shell: cli::Shell) {
    let mut cmd = cli::Cli::command();
    let name = cmd.get_name().to_string();

    match shell {
        cli::Shell::Bash => generate(shells::Bash, &mut cmd, name, &mut std::io::stdout()),
        cli::Shell::Zsh => generate(shells::Zsh, &mut cmd, name, &mut std::io::stdout()),
        cli::Shell::Fish => generate(shells::Fish, &mut cmd, name, &mut std::io::stdout()),
        cli::Shell::PowerShell => {
            generate(shells::PowerShell, &mut cmd, name, &mut std::io::stdout())
        }
        cli::Shell::Elvish => generate(shells::Elvish, &mut cmd, name, &mut std::io::stdout()),
    }
}

/// Format command for human-readable logging (without sensitive data)
fn format_command(command: &Commands) -> String {
    match command {
        Commands::Version => "version".to_string(),
        Commands::Completions { shell } => format!("completions {:?}", shell),
        Commands::Profile(cmd) => {
            use cli::ProfileCommands::*;
            match cmd {
                List => "profile list".to_string(),
                Path => "profile path".to_string(),
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
        Action(action_cmd) => {
            commands::enterprise::actions::handle_action_command(
                conn_mgr,
                profile,
                action_cmd.clone(),
                output,
                query,
            )
            .await
        }
        BdbGroup(bdb_group_cmd) => {
            commands::enterprise::bdb_group::handle_bdb_group_command(
                conn_mgr,
                profile,
                bdb_group_cmd.clone(),
                output,
                query,
            )
            .await
        }
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
        CmSettings(cm_settings_cmd) => {
            commands::enterprise::cm_settings::handle_cm_settings_command(
                conn_mgr,
                profile,
                cm_settings_cmd.clone(),
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
        Diagnostics(diagnostics_cmd) => {
            commands::enterprise::diagnostics::handle_diagnostics_command(
                conn_mgr,
                profile,
                diagnostics_cmd.clone(),
                output,
                query,
            )
            .await
        }
        Endpoint(endpoint_cmd) => {
            commands::enterprise::endpoint::handle_endpoint_command(
                conn_mgr,
                profile,
                endpoint_cmd.clone(),
                output,
                query,
            )
            .await
        }
        Node(node_cmd) => {
            commands::enterprise::node::handle_node_command(
                conn_mgr, profile, node_cmd, output, query,
            )
            .await
        }
        Proxy(proxy_cmd) => {
            commands::enterprise::proxy::handle_proxy_command(
                conn_mgr,
                profile,
                proxy_cmd.clone(),
                output,
                query,
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
        CrdbTask(crdb_task_cmd) => {
            commands::enterprise::crdb_task::handle_crdb_task_command(
                conn_mgr,
                profile,
                crdb_task_cmd.clone(),
                output,
                query,
            )
            .await
        }
        JobScheduler(job_scheduler_cmd) => {
            commands::enterprise::job_scheduler::handle_job_scheduler_command(
                conn_mgr,
                profile,
                job_scheduler_cmd.clone(),
                output,
                query,
            )
            .await
        }
        Logs(logs_cmd) => {
            commands::enterprise::logs_impl::handle_logs_commands(
                conn_mgr, profile, logs_cmd, output, query,
            )
            .await
        }
        Module(module_cmd) => {
            commands::enterprise::module_impl::handle_module_commands(
                conn_mgr, profile, module_cmd, output, query,
            )
            .await
        }
        Workflow(workflow_cmd) => {
            handle_enterprise_workflow_command(conn_mgr, profile, workflow_cmd, output).await
        }
        Shard(shard_cmd) => {
            commands::enterprise::shard::handle_shard_command(
                conn_mgr,
                profile,
                shard_cmd.clone(),
                output,
                query,
            )
            .await
        }
        Stats(stats_cmd) => {
            commands::enterprise::stats::handle_stats_command(
                conn_mgr, profile, stats_cmd, output, query,
            )
            .await
        }
        UsageReport(usage_report_cmd) => {
            commands::enterprise::usage_report::handle_usage_report_command(
                conn_mgr,
                profile,
                usage_report_cmd.clone(),
                output,
                query,
            )
            .await
        }
    }
}

async fn handle_cloud_workflow_command(
    conn_mgr: &ConnectionManager,
    cli: &cli::Cli,
    workflow_cmd: &cli::CloudWorkflowCommands,
) -> Result<(), RedisCtlError> {
    use cli::CloudWorkflowCommands::*;
    use workflows::{WorkflowArgs, WorkflowContext, WorkflowRegistry};

    let output = cli.output;
    let profile = cli.profile.as_deref();

    match workflow_cmd {
        List => {
            let registry = WorkflowRegistry::new();
            let workflows = registry.list();

            // Filter to show only cloud workflows
            let cloud_workflows: Vec<_> = workflows
                .into_iter()
                .filter(|(name, _)| name.contains("subscription") || name.contains("cloud"))
                .collect();

            match output {
                cli::OutputFormat::Json | cli::OutputFormat::Yaml => {
                    let workflow_list: Vec<serde_json::Value> = cloud_workflows
                        .into_iter()
                        .map(|(name, description)| {
                            serde_json::json!({
                                "name": name,
                                "description": description
                            })
                        })
                        .collect();
                    let output_format = match output {
                        cli::OutputFormat::Json => output::OutputFormat::Json,
                        cli::OutputFormat::Yaml => output::OutputFormat::Yaml,
                        _ => output::OutputFormat::Table,
                    };
                    crate::output::print_output(
                        serde_json::json!(workflow_list),
                        output_format,
                        None,
                    )?;
                }
                _ => {
                    println!("Available Cloud Workflows:");
                    println!();
                    for (name, description) in cloud_workflows {
                        println!("  {} - {}", name, description);
                    }
                }
            }
            Ok(())
        }
        SubscriptionSetup(args) => {
            let mut workflow_args = WorkflowArgs::new();
            workflow_args.insert("args", args);

            let output_format = match output {
                cli::OutputFormat::Json => output::OutputFormat::Json,
                cli::OutputFormat::Yaml => output::OutputFormat::Yaml,
                cli::OutputFormat::Table | cli::OutputFormat::Auto => output::OutputFormat::Table,
            };

            let context = WorkflowContext {
                conn_mgr: conn_mgr.clone(),
                profile_name: profile.map(String::from),
                output_format,
                wait_timeout: args.wait_timeout as u64,
            };

            let registry = WorkflowRegistry::new();
            let workflow =
                registry
                    .get("subscription-setup")
                    .ok_or_else(|| RedisCtlError::ApiError {
                        message: "Workflow not found".to_string(),
                    })?;

            let result = workflow
                .execute(context, workflow_args)
                .await
                .map_err(|e| RedisCtlError::ApiError {
                    message: e.to_string(),
                })?;

            if !result.success {
                return Err(RedisCtlError::ApiError {
                    message: result.message,
                });
            }

            // Print result as JSON/YAML if requested
            match output {
                cli::OutputFormat::Json | cli::OutputFormat::Yaml => {
                    let result_json = serde_json::json!({
                        "success": result.success,
                        "message": result.message,
                        "outputs": result.outputs,
                    });
                    crate::output::print_output(&result_json, output_format, None)?;
                }
                _ => {
                    // Human output
                    println!("{}", result.message);
                }
            }

            Ok(())
        }
    }
}

async fn handle_enterprise_workflow_command(
    conn_mgr: &ConnectionManager,
    profile: Option<&str>,
    workflow_cmd: &cli::EnterpriseWorkflowCommands,
    output: cli::OutputFormat,
) -> Result<(), RedisCtlError> {
    use cli::EnterpriseWorkflowCommands::*;
    use workflows::{WorkflowArgs, WorkflowContext, WorkflowRegistry};

    match workflow_cmd {
        List => {
            let registry = WorkflowRegistry::new();
            let workflows = registry.list();

            match output {
                cli::OutputFormat::Json | cli::OutputFormat::Yaml => {
                    let workflow_list: Vec<serde_json::Value> = workflows
                        .into_iter()
                        .map(|(name, description)| {
                            serde_json::json!({
                                "name": name,
                                "description": description
                            })
                        })
                        .collect();
                    let output_format = match output {
                        cli::OutputFormat::Json => output::OutputFormat::Json,
                        cli::OutputFormat::Yaml => output::OutputFormat::Yaml,
                        _ => output::OutputFormat::Table,
                    };
                    crate::output::print_output(
                        serde_json::json!(workflow_list),
                        output_format,
                        None,
                    )?;
                }
                _ => {
                    println!("Available Enterprise Workflows:");
                    println!();
                    for (name, description) in workflows {
                        println!("  {} - {}", name, description);
                    }
                }
            }
            Ok(())
        }
        InitCluster {
            name,
            username,
            password,
            skip_database,
            database_name,
            database_memory_gb,
            async_ops,
        } => {
            let mut args = WorkflowArgs::new();
            args.insert("name", name);
            args.insert("username", username);
            args.insert("password", password);
            args.insert("create_database", !skip_database);
            args.insert("database_name", database_name);
            args.insert("database_memory_gb", database_memory_gb);

            let output_format = match output {
                cli::OutputFormat::Json => output::OutputFormat::Json,
                cli::OutputFormat::Yaml => output::OutputFormat::Yaml,
                cli::OutputFormat::Table | cli::OutputFormat::Auto => output::OutputFormat::Table,
            };

            let context = WorkflowContext {
                conn_mgr: conn_mgr.clone(),
                profile_name: profile.map(String::from),
                output_format,
                wait_timeout: if async_ops.wait {
                    async_ops.wait_timeout
                } else {
                    0
                },
            };

            let registry = WorkflowRegistry::new();
            let workflow = registry
                .get("init-cluster")
                .ok_or_else(|| RedisCtlError::ApiError {
                    message: "Workflow not found".to_string(),
                })?;

            let result =
                workflow
                    .execute(context, args)
                    .await
                    .map_err(|e| RedisCtlError::ApiError {
                        message: e.to_string(),
                    })?;

            if !result.success {
                return Err(RedisCtlError::ApiError {
                    message: result.message,
                });
            }

            // Print result as JSON/YAML if requested
            match output {
                cli::OutputFormat::Json | cli::OutputFormat::Yaml => {
                    let result_json = serde_json::json!({
                        "success": result.success,
                        "message": result.message,
                        "outputs": result.outputs,
                    });
                    crate::output::print_output(&result_json, output_format, None)?;
                }
                _ => {
                    // Human output was already printed by the workflow
                }
            }

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
            debug!("Listing all configured profiles");
            let profiles = conn_mgr.config.list_profiles();
            trace!("Found {} profiles", profiles.len());

            // Show config file path at the top
            if let Ok(config_path) = config::Config::config_path() {
                println!("Configuration file: {}", config_path.display());
                println!();
            }

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

        Path => {
            let config_path = config::Config::config_path()?;
            println!("{}", config_path.display());
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

        Set {
            name,
            deployment,
            api_key,
            api_secret,
            api_url,
            url,
            username,
            password,
            insecure,
        } => {
            debug!("Setting profile: {}", name);

            // Check if profile already exists
            if conn_mgr.config.profiles.contains_key(name) {
                // Ask for confirmation before overwriting
                println!("Profile '{}' already exists.", name);
                print!("Do you want to overwrite it? (y/N): ");
                use std::io::{self, Write};
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim().to_lowercase();

                if input != "y" && input != "yes" {
                    println!("Profile update cancelled.");
                    return Ok(());
                }
            }

            // Create the profile based on deployment type
            let profile = match deployment {
                config::DeploymentType::Cloud => {
                    let api_key = api_key
                        .clone()
                        .ok_or_else(|| anyhow::anyhow!("API key is required for Cloud profiles"))?;
                    let api_secret = api_secret.clone().ok_or_else(|| {
                        anyhow::anyhow!("API secret is required for Cloud profiles")
                    })?;

                    config::Profile {
                        deployment_type: config::DeploymentType::Cloud,
                        credentials: config::ProfileCredentials::Cloud {
                            api_key: api_key.clone(),
                            api_secret: api_secret.clone(),
                            api_url: api_url.clone(),
                        },
                    }
                }
                config::DeploymentType::Enterprise => {
                    let url = url.clone().ok_or_else(|| {
                        anyhow::anyhow!("URL is required for Enterprise profiles")
                    })?;
                    let username = username.clone().ok_or_else(|| {
                        anyhow::anyhow!("Username is required for Enterprise profiles")
                    })?;

                    // Prompt for password if not provided
                    let password = match password {
                        Some(p) => Some(p.clone()),
                        None => {
                            let pass = rpassword::prompt_password("Enter password: ")
                                .context("Failed to read password")?;
                            Some(pass)
                        }
                    };

                    config::Profile {
                        deployment_type: config::DeploymentType::Enterprise,
                        credentials: config::ProfileCredentials::Enterprise {
                            url: url.clone(),
                            username: username.clone(),
                            password,
                            insecure: *insecure,
                        },
                    }
                }
            };

            // Update the configuration
            let mut config = conn_mgr.config.clone();
            config.profiles.insert(name.clone(), profile);

            // Save the configuration
            config.save().context("Failed to save configuration")?;

            if let Ok(config_path) = config::Config::config_path() {
                println!("Profile '{}' saved successfully to:", name);
                println!("  {}", config_path.display());
            } else {
                println!("Profile '{}' saved successfully.", name);
            }

            // Ask if this should be the default profile
            if config.default_profile.is_none() || config.profiles.len() == 1 {
                print!("Set '{}' as the default profile? (Y/n): ", name);
                use std::io::{self, Write};
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim().to_lowercase();

                if input.is_empty() || input == "y" || input == "yes" {
                    config.default_profile = Some(name.clone());
                    config.save().context("Failed to save default profile")?;
                    println!("Profile '{}' set as default.", name);
                }
            }

            Ok(())
        }
        Remove { name } => {
            debug!("Removing profile: {}", name);

            // Check if profile exists
            if !conn_mgr.config.profiles.contains_key(name) {
                return Err(RedisCtlError::ProfileNotFound { name: name.clone() });
            }

            // Check if it's the default profile
            let is_default = conn_mgr.config.default_profile.as_ref() == Some(name);
            if is_default {
                println!("Warning: '{}' is the default profile.", name);
            }

            // Ask for confirmation
            print!(
                "Are you sure you want to remove profile '{}'? (y/N): ",
                name
            );
            use std::io::{self, Write};
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim().to_lowercase();

            if input != "y" && input != "yes" {
                println!("Profile removal cancelled.");
                return Ok(());
            }

            // Remove the profile
            let mut config = conn_mgr.config.clone();
            config.profiles.remove(name);

            // Clear default if this was the default profile
            if is_default {
                config.default_profile = None;
                println!("Default profile cleared.");
            }

            // Save the configuration
            config.save().context("Failed to save configuration")?;

            println!("Profile '{}' removed successfully.", name);
            Ok(())
        }
        Default { name } => {
            debug!("Setting default profile: {}", name);

            // Check if profile exists
            if !conn_mgr.config.profiles.contains_key(name) {
                return Err(RedisCtlError::ProfileNotFound { name: name.clone() });
            }

            // Update the configuration
            let mut config = conn_mgr.config.clone();
            config.default_profile = Some(name.clone());

            // Save the configuration
            config.save().context("Failed to save configuration")?;

            println!("Default profile set to '{}'.", name);
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
        FixedDatabase(fixed_db_cmd) => {
            commands::cloud::fixed_database::handle_fixed_database_command(
                conn_mgr,
                cli.profile.as_deref(),
                fixed_db_cmd,
                cli.output,
                cli.query.as_deref(),
            )
            .await
        }
        FixedSubscription(fixed_sub_cmd) => {
            commands::cloud::fixed_subscription::handle_fixed_subscription_command(
                conn_mgr,
                cli.profile.as_deref(),
                fixed_sub_cmd,
                cli.output,
                cli.query.as_deref(),
            )
            .await
        }
        Workflow(workflow_cmd) => handle_cloud_workflow_command(conn_mgr, cli, workflow_cmd).await,
    }
}
