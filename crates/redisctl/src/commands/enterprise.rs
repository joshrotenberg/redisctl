use crate::config::{Profile, ProfileCredentials};
use crate::output::{OutputFormat, print_output};
use anyhow::Result;
use redis_enterprise::EnterpriseClient;
use std::io::Write;

use crate::cli::{
    AlertCommands, BootstrapCommands, ClusterCommands, DatabaseCommands, EnterpriseActionCommands,
    EnterpriseCommands, EnterpriseCrdbCommands, EnterpriseLogsCommands, EnterpriseStatsCommands,
    LicenseCommands, ModuleCommands, NodeCommands, RoleCommands, UserCommands,
};
use crate::commands::api::handle_enterprise_api;

#[allow(dead_code)]
pub async fn handle_enterprise_command(
    command: EnterpriseCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    match command {
        EnterpriseCommands::Api { command } => {
            let client = create_enterprise_client(profile).await?;
            handle_enterprise_api(&client, command, output_format, query).await
        }
        EnterpriseCommands::Database { command } => {
            handle_database_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::Cluster { command } => {
            handle_cluster_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::Node { command } => {
            handle_node_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::User { command } => {
            handle_user_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::Bootstrap { command } => {
            handle_bootstrap_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::Module { command } => {
            handle_module_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::Role { command } => {
            handle_role_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::License { command } => {
            handle_license_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::Alert { command } => {
            handle_alert_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::Crdb { command } => {
            handle_crdb_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::Actions { command } => {
            handle_action_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::Stats { command } => {
            handle_stats_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::Logs { command } => {
            handle_logs_command(command, profile, output_format, query).await
        }
    }
}

pub async fn handle_database_command(
    command: DatabaseCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;

    match command {
        DatabaseCommands::List => {
            let handler = redis_enterprise::BdbHandler::new(client.clone());
            let databases = handler.list().await?;
            let value = serde_json::to_value(databases)?;
            print_output(value, output_format, query)?;
        }
        DatabaseCommands::Show { id } => {
            let handler = redis_enterprise::BdbHandler::new(client.clone());
            let database = handler.info(id.parse()?).await?;
            let value = serde_json::to_value(database)?;
            print_output(value, output_format, query)?;
        }
        DatabaseCommands::Create {
            name,
            memory_limit,
            modules,
        } => {
            let handler = redis_enterprise::BdbHandler::new(client.clone());
            let request = redis_enterprise::CreateDatabaseRequest {
                name: name.clone(),
                memory_size: memory_limit.unwrap_or(100) * 1024 * 1024, // Convert MB to bytes
                module_list: if modules.is_empty() {
                    None
                } else {
                    Some(
                        modules
                            .into_iter()
                            .map(|name| redis_enterprise::ModuleConfig {
                                module_name: name,
                                module_args: None,
                            })
                            .collect(),
                    )
                },
                port: None,
                replication: None,
                persistence: None,
                eviction_policy: None,
                shards_count: None,
                authentication_redis_pass: None,
            };
            let database = handler.create(request).await?;
            let value = serde_json::to_value(database)?;
            print_output(value, output_format, query)?;
        }
        DatabaseCommands::Update {
            id,
            name,
            memory_limit,
        } => {
            let handler = redis_enterprise::BdbHandler::new(client.clone());
            let mut update_data = serde_json::Map::new();

            if let Some(name) = name {
                update_data.insert("name".to_string(), serde_json::Value::String(name));
            }
            if let Some(memory_limit) = memory_limit {
                update_data.insert(
                    "memory_size".to_string(),
                    serde_json::Value::Number(serde_json::Number::from(memory_limit * 1024 * 1024)),
                );
            }

            let database = handler
                .update(id.parse()?, serde_json::Value::Object(update_data))
                .await?;
            let value = serde_json::to_value(database)?;
            print_output(value, output_format, query)?;
        }
        DatabaseCommands::Delete { id, force: _ } => {
            let handler = redis_enterprise::BdbHandler::new(client.clone());
            handler.delete(id.parse()?).await?;
            println!("Database {} deleted successfully", id);
        }
        DatabaseCommands::Backup { id } => {
            let handler = redis_enterprise::BdbHandler::new(client.clone());
            let backup = handler.backup(id.parse()?).await?;
            print_output(backup, output_format, query)?;
        }
        DatabaseCommands::Import { id, url } => {
            let handler = redis_enterprise::BdbHandler::new(client.clone());
            let import_result = handler.import(id.parse()?, &url, false).await?;
            print_output(import_result, output_format, query)?;
        }
        DatabaseCommands::Export { id, format } => {
            let _handler = redis_enterprise::BdbHandler::new(client.clone());
            // Note: export method takes a location URL, not format
            // Using raw API for format-based export
            let export_data = serde_json::json!({
                "format": format
            });
            let export_result = client
                .post_raw(&format!("/v1/bdbs/{}/export", id), export_data)
                .await?;
            print_output(export_result, output_format, query)?;
        }
    }

    Ok(())
}

pub async fn handle_cluster_command(
    command: ClusterCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;

    match command {
        ClusterCommands::Info => {
            // Use typed API to get cluster info
            let handler = redis_enterprise::ClusterHandler::new(client.clone());
            let info = handler.info().await?;
            let value = serde_json::to_value(info)?;
            print_output(value, output_format, query)?;
        }
        ClusterCommands::Nodes => {
            let handler = redis_enterprise::NodeHandler::new(client.clone());
            let nodes = handler.list().await?;
            let value = serde_json::to_value(nodes)?;
            print_output(value, output_format, query)?;
        }
        ClusterCommands::Settings => {
            let handler = redis_enterprise::CmSettingsHandler::new(client.clone());
            let settings = handler.get().await?;
            let value = serde_json::to_value(settings)?;
            print_output(value, output_format, query)?;
        }
        ClusterCommands::Update { name, value } => {
            let handler = redis_enterprise::CmSettingsHandler::new(client.clone());
            // Get current settings first
            let settings = handler.get().await?;

            // Update the specific field - this is simplified, in practice you'd want to handle specific field updates
            // For now, we'll just print that this operation needs more specific implementation
            println!(
                "Update operation for {} = {} requires specific field mapping",
                name, value
            );
            println!(
                "Current settings: {}",
                serde_json::to_string_pretty(&settings)?
            );
        }
    }

    Ok(())
}

pub async fn handle_node_command(
    command: NodeCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;

    match command {
        NodeCommands::List => {
            let handler = redis_enterprise::NodeHandler::new(client.clone());
            let nodes = handler.list().await?;
            let value = serde_json::to_value(nodes)?;
            print_output(value, output_format, query)?;
        }
        NodeCommands::Show { id } => {
            let handler = redis_enterprise::NodeHandler::new(client.clone());
            let node = handler.get(id.parse()?).await?;
            let value = serde_json::to_value(node)?;
            print_output(value, output_format, query)?;
        }
        NodeCommands::Stats { id } => {
            let handler = redis_enterprise::NodeHandler::new(client.clone());
            let stats = handler.stats(id.parse()?).await?;
            let value = serde_json::to_value(stats)?;
            print_output(value, output_format, query)?;
        }
        NodeCommands::Update { id, external_addr } => {
            let handler = redis_enterprise::NodeHandler::new(client.clone());
            let mut update_data = serde_json::Map::new();

            if let Some(external_addr) = external_addr {
                update_data.insert(
                    "external_addr".to_string(),
                    serde_json::Value::Array(vec![serde_json::Value::String(external_addr)]),
                );
            }

            let node = handler
                .update(id.parse()?, serde_json::Value::Object(update_data))
                .await?;
            let value = serde_json::to_value(node)?;
            print_output(value, output_format, query)?;
        }
        NodeCommands::Add {
            addr,
            username,
            password,
            external_addr,
        } => {
            // Note: NodeHandler doesn't have an add/create method, using raw API
            let mut add_data = serde_json::json!({
                "addr": addr,
                "username": username,
                "password": password
            });

            if let Some(external_addr) = external_addr {
                add_data["external_addr"] =
                    serde_json::Value::Array(vec![serde_json::Value::String(external_addr)]);
            }

            let node = client.post_raw("/v1/nodes", add_data).await?;
            print_output(node, output_format, query)?;
        }
        NodeCommands::Remove { id, force } => {
            if !force {
                println!(
                    "Are you sure you want to remove node {}? Use --force to skip confirmation.",
                    id
                );
                return Ok(());
            }
            let handler = redis_enterprise::NodeHandler::new(client.clone());
            handler.remove(id.parse()?).await?;
            println!("Node {} removed successfully", id);
        }
    }

    Ok(())
}

pub async fn handle_user_command(
    command: UserCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;

    match command {
        UserCommands::List => {
            let handler = redis_enterprise::UserHandler::new(client.clone());
            let users = handler.list().await?;
            let value = serde_json::to_value(users)?;
            print_output(value, output_format, query)?;
        }
        UserCommands::Show { id } => {
            let handler = redis_enterprise::UserHandler::new(client.clone());
            let user = handler.get(id.parse()?).await?;
            let value = serde_json::to_value(user)?;
            print_output(value, output_format, query)?;
        }
        UserCommands::Create {
            name,
            email,
            password,
            roles,
        } => {
            let handler = redis_enterprise::UserHandler::new(client.clone());
            let request = redis_enterprise::CreateUserRequest {
                username: name.clone(),
                password: password.unwrap_or_else(|| "default_password".to_string()),
                role: roles.first().unwrap_or(&"db_viewer".to_string()).clone(),
                email,
                email_alerts: None,
            };
            let user = handler.create(request).await?;
            let value = serde_json::to_value(user)?;
            print_output(value, output_format, query)?;
        }
        UserCommands::Update {
            id,
            email,
            password,
        } => {
            let handler = redis_enterprise::UserHandler::new(client.clone());
            let request = redis_enterprise::UpdateUserRequest {
                email,
                password,
                role: None,
                email_alerts: None,
            };
            let user = handler.update(id.parse()?, request).await?;
            let value = serde_json::to_value(user)?;
            print_output(value, output_format, query)?;
        }
        UserCommands::Delete { id, force: _ } => {
            let handler = redis_enterprise::UserHandler::new(client.clone());
            handler.delete(id.parse()?).await?;
            println!("User {} deleted successfully", id);
        }
    }

    Ok(())
}

pub async fn handle_bootstrap_command(
    command: BootstrapCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;

    match command {
        BootstrapCommands::Create {
            name,
            username,
            password,
            license_file,
            rack_aware,
            dns_suffixes,
        } => {
            // Read license file if provided
            let license_content = if let Some(path) = license_file {
                Some(std::fs::read_to_string(path)?)
            } else {
                None
            };

            // Create proper bootstrap structure
            use redis_enterprise::{BootstrapConfig, ClusterBootstrap, CredentialsBootstrap};

            let config = BootstrapConfig {
                action: "create_cluster".to_string(),
                cluster: Some(ClusterBootstrap {
                    name,
                    dns_suffixes,
                    rack_aware: Some(rack_aware),
                }),
                node: None, // Will use default paths
                credentials: Some(CredentialsBootstrap { username, password }),
                extra: if let Some(license) = license_content {
                    serde_json::json!({ "license_file": license })
                } else {
                    serde_json::json!({})
                },
            };

            let result = client.post_bootstrap("/v1/bootstrap", &config).await?;
            print_output(result, output_format, query)?;
        }
        BootstrapCommands::Status => {
            let result = client.get_raw("/v1/bootstrap").await?;
            print_output(result, output_format, query)?;
        }
    }

    Ok(())
}

pub async fn handle_module_command(
    command: ModuleCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;

    match command {
        ModuleCommands::List => {
            let modules = client.get_raw("/v1/modules").await?;
            print_output(modules, output_format, query)?;
        }
        ModuleCommands::Show { id } => {
            let module = client.get_raw(&format!("/v1/modules/{}", id)).await?;
            print_output(module, output_format, query)?;
        }
        ModuleCommands::Upload { path } => {
            // For now, provide a meaningful error about file upload limitations
            anyhow::bail!(
                "Module upload requires multipart file upload functionality. Please use the Redis Enterprise web UI or direct API calls for module uploads. Module path would be: {}",
                path
            );
        }
    }

    Ok(())
}

pub async fn handle_role_command(
    command: RoleCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;

    match command {
        RoleCommands::List => {
            let handler = redis_enterprise::RolesHandler::new(client.clone());
            let roles = handler.list().await?;
            let value = serde_json::to_value(roles)?;
            print_output(value, output_format, query)?;
        }
        RoleCommands::Show { id } => {
            let handler = redis_enterprise::RolesHandler::new(client.clone());
            let role = handler.get(id.parse()?).await?;
            let value = serde_json::to_value(role)?;
            print_output(value, output_format, query)?;
        }
        RoleCommands::Create { name, permissions } => {
            let handler = redis_enterprise::RolesHandler::new(client.clone());
            let request = redis_enterprise::CreateRoleRequest {
                name: name.clone(),
                management: if permissions.contains(&"management".to_string()) {
                    Some("all".to_string())
                } else {
                    None
                },
                data_access: Some(permissions.join(" ")),
                bdb_roles: None,
                cluster_roles: None,
            };
            let role = handler.create(request).await?;
            let value = serde_json::to_value(role)?;
            print_output(value, output_format, query)?;
        }
        RoleCommands::Update { id, permissions } => {
            let handler = redis_enterprise::RolesHandler::new(client.clone());
            // For updates, we should ideally get the current role and update it
            // For now, creating a minimal update request
            let request = redis_enterprise::CreateRoleRequest {
                name: format!("role_{}", id), // Placeholder name
                management: None,
                data_access: Some(permissions.join(" ")),
                bdb_roles: None,
                cluster_roles: None,
            };
            let role = handler.update(id.parse()?, request).await?;
            let value = serde_json::to_value(role)?;
            print_output(value, output_format, query)?;
        }
        RoleCommands::Delete { id, force: _ } => {
            let handler = redis_enterprise::RolesHandler::new(client.clone());
            handler.delete(id.parse()?).await?;
            println!("Role {} deleted successfully", id);
        }
    }

    Ok(())
}

pub async fn handle_license_command(
    command: LicenseCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;

    match command {
        LicenseCommands::Info => {
            let handler = redis_enterprise::LicenseHandler::new(client.clone());
            let license = handler.get().await?;
            let value = serde_json::to_value(license)?;
            print_output(value, output_format, query)?;
        }
        LicenseCommands::Update { key } => {
            let handler = redis_enterprise::LicenseHandler::new(client.clone());
            let request = redis_enterprise::LicenseUpdateRequest { license: key };
            let result = handler.update(request).await?;
            let value = serde_json::to_value(result)?;
            print_output(value, output_format, query)?;
        }
    }

    Ok(())
}

pub async fn create_enterprise_client(profile: &Profile) -> Result<EnterpriseClient> {
    if let ProfileCredentials::Enterprise {
        url,
        username,
        password,
        insecure,
    } = &profile.credentials
    {
        let password = if let Some(pwd) = password {
            pwd.clone()
        } else {
            // Prompt for password if not stored
            {
                print!("Password: ");
                std::io::stdout().flush()?;
                rpassword::read_password()?
            }
        };

        EnterpriseClient::builder()
            .base_url(url.clone())
            .username(username.clone())
            .password(password)
            .timeout(std::time::Duration::from_secs(30))
            .insecure(*insecure)
            .build()
            .map_err(Into::into)
    } else {
        anyhow::bail!("Invalid profile type for Enterprise commands")
    }
}

pub async fn handle_alert_command(
    command: AlertCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;

    match command {
        AlertCommands::List => {
            let handler = redis_enterprise::AlertHandler::new(client.clone());
            let result = handler.list().await?;
            let value = serde_json::to_value(result)?;
            print_output(value, output_format, query)?;
        }
        AlertCommands::Show { uid } => {
            let handler = redis_enterprise::AlertHandler::new(client.clone());
            let result = handler.get(&uid).await?;
            let value = serde_json::to_value(result)?;
            print_output(value, output_format, query)?;
        }
        AlertCommands::Database { uid } => {
            let handler = redis_enterprise::AlertHandler::new(client.clone());
            let result = handler.list_by_database(uid).await?;
            let value = serde_json::to_value(result)?;
            print_output(value, output_format, query)?;
        }
        AlertCommands::Node { uid } => {
            let handler = redis_enterprise::AlertHandler::new(client.clone());
            let result = handler.list_by_node(uid).await?;
            let value = serde_json::to_value(result)?;
            print_output(value, output_format, query)?;
        }
        AlertCommands::Cluster => {
            let handler = redis_enterprise::AlertHandler::new(client.clone());
            let result = handler.list_cluster_alerts().await?;
            let value = serde_json::to_value(result)?;
            print_output(value, output_format, query)?;
        }
        AlertCommands::Clear { uid } => {
            client.delete_raw(&format!("/v1/alerts/{}", uid)).await?;
            print_output(
                serde_json::json!({"message": "Alert cleared successfully"}),
                output_format,
                query,
            )?;
        }
        AlertCommands::ClearAll => {
            client.delete_raw("/v1/alerts").await?;
            print_output(
                serde_json::json!({"message": "All alerts cleared successfully"}),
                output_format,
                query,
            )?;
        }
        AlertCommands::Settings { name } => {
            let result = client
                .get_raw(&format!("/v1/cluster/alert_settings/{}", name))
                .await?;
            print_output(result, output_format, query)?;
        }
        AlertCommands::UpdateSettings {
            name,
            enabled,
            emails,
            webhook_url,
        } => {
            let settings = redis_enterprise::AlertSettings {
                enabled: enabled.unwrap_or(true),
                threshold: None,
                email_recipients: emails
                    .map(|e| e.split(',').map(|s| s.trim().to_string()).collect()),
                webhook_url,
            };

            let handler = redis_enterprise::AlertHandler::new(client.clone());
            let result = handler.update_settings(&name, settings).await?;
            let value = serde_json::to_value(result)?;
            print_output(value, output_format, query)?;
        }
    }

    Ok(())
}

pub async fn handle_crdb_command(
    command: EnterpriseCrdbCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;

    match command {
        EnterpriseCrdbCommands::List => {
            let handler = redis_enterprise::CrdbHandler::new(client.clone());
            let result = handler.list().await?;
            let value = serde_json::to_value(result)?;
            print_output(value, output_format, query)?;
        }
        EnterpriseCrdbCommands::Show { guid } => {
            let handler = redis_enterprise::CrdbHandler::new(client.clone());
            let result = handler.get(&guid).await?;
            let value = serde_json::to_value(result)?;
            print_output(value, output_format, query)?;
        }
        EnterpriseCrdbCommands::Create {
            name,
            memory_size,
            instances,
            encryption,
            persistence,
            eviction_policy,
        } => {
            use redis_enterprise::{CreateCrdbInstance, CreateCrdbRequest};

            // Parse instances from "cluster_url:username:password" format
            let mut crdb_instances = Vec::new();
            for instance_str in instances {
                let parts: Vec<&str> = instance_str.splitn(3, ':').collect();
                if parts.len() != 3 {
                    anyhow::bail!(
                        "Invalid instance format. Expected: cluster_url:username:password"
                    );
                }

                crdb_instances.push(CreateCrdbInstance {
                    cluster: parts[0].to_string(),
                    cluster_url: Some(parts[0].to_string()),
                    username: Some(parts[1].to_string()),
                    password: Some(parts[2].to_string()),
                });
            }

            let request = CreateCrdbRequest {
                name,
                memory_size,
                instances: crdb_instances,
                encryption: Some(encryption),
                data_persistence: persistence,
                eviction_policy,
            };

            let handler = redis_enterprise::CrdbHandler::new(client.clone());
            let result = handler.create(request).await?;
            let value = serde_json::to_value(result)?;
            print_output(value, output_format, query)?;
        }
        EnterpriseCrdbCommands::Update {
            guid,
            memory_size,
            persistence,
            eviction_policy,
        } => {
            let mut updates = serde_json::json!({});

            if let Some(memory_size) = memory_size {
                updates["memory_size"] = memory_size.into();
            }

            if let Some(persistence) = persistence {
                updates["data_persistence"] = persistence.into();
            }

            if let Some(eviction_policy) = eviction_policy {
                updates["eviction_policy"] = eviction_policy.into();
            }

            let handler = redis_enterprise::CrdbHandler::new(client.clone());
            let result = handler.update(&guid, updates).await?;
            let value = serde_json::to_value(result)?;
            print_output(value, output_format, query)?;
        }
        EnterpriseCrdbCommands::Delete { guid, force } => {
            if !force {
                print!("Are you sure you want to delete CRDB '{}'? (y/N): ", guid);
                std::io::stdout().flush()?;
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;

                if !input.trim().to_lowercase().starts_with('y') {
                    println!("Operation cancelled.");
                    return Ok(());
                }
            }

            let handler = redis_enterprise::CrdbHandler::new(client.clone());
            handler.delete(&guid).await?;
            print_output(
                serde_json::json!({"message": "CRDB deleted successfully"}),
                output_format,
                query,
            )?;
        }
        EnterpriseCrdbCommands::Tasks { guid } => {
            let handler = redis_enterprise::CrdbTasksHandler::new(client.clone());
            let result = handler.list_by_crdb(&guid).await?;
            let value = serde_json::to_value(result)?;
            print_output(value, output_format, query)?;
        }
    }

    Ok(())
}

/// Handle action commands
pub async fn handle_action_command(
    command: EnterpriseActionCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;

    match command {
        EnterpriseActionCommands::List => {
            let handler = redis_enterprise::ActionHandler::new(client.clone());
            let result = handler.list().await?;
            let value = serde_json::to_value(result)?;
            print_output(value, output_format, query)?;
        }
        EnterpriseActionCommands::Show { uid } => {
            let handler = redis_enterprise::ActionHandler::new(client.clone());
            let result = handler.get(&uid).await?;
            let value = serde_json::to_value(result)?;
            print_output(value, output_format, query)?;
        }
        EnterpriseActionCommands::Cancel { uid } => {
            let handler = redis_enterprise::ActionHandler::new(client.clone());
            handler.cancel(&uid).await?;
            print_output(
                serde_json::json!({"message": "Action cancelled successfully"}),
                output_format,
                query,
            )?;
        }
    }

    Ok(())
}

/// Handle stats commands
pub async fn handle_stats_command(
    command: EnterpriseStatsCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;

    match command {
        EnterpriseStatsCommands::Cluster { interval } => {
            let handler = redis_enterprise::StatsHandler::new(client.clone());
            let query_params = redis_enterprise::StatsQuery {
                interval: interval.clone(),
                stime: None,
                etime: None,
                metrics: None,
            };
            let result = handler.cluster(Some(query_params)).await?;
            let value = serde_json::to_value(result)?;
            print_output(value, output_format, query)?;
        }
        EnterpriseStatsCommands::Node { uid, interval } => {
            let handler = redis_enterprise::StatsHandler::new(client.clone());
            let query_params = redis_enterprise::StatsQuery {
                interval: interval.clone(),
                stime: None,
                etime: None,
                metrics: None,
            };
            let result = handler.node(uid, Some(query_params)).await?;
            let value = serde_json::to_value(result)?;
            print_output(value, output_format, query)?;
        }
        EnterpriseStatsCommands::Database { uid, interval } => {
            let handler = redis_enterprise::StatsHandler::new(client.clone());
            let query_params = redis_enterprise::StatsQuery {
                interval: interval.clone(),
                stime: None,
                etime: None,
                metrics: None,
            };
            let result = handler.database(uid, Some(query_params)).await?;
            let value = serde_json::to_value(result)?;
            print_output(value, output_format, query)?;
        }
        EnterpriseStatsCommands::Shard { uid, interval } => {
            let handler = redis_enterprise::ShardHandler::new(client.clone());
            let result = if let Some(_interval) = interval {
                // Note: Shard stats don't have typed methods with intervals
                client
                    .get_raw(&format!("/v1/shards/{}/stats?interval={}", uid, _interval))
                    .await?
            } else {
                let stats = handler.stats(&uid).await?;
                serde_json::to_value(stats)?
            };
            print_output(result, output_format, query)?;
        }
    }

    Ok(())
}

/// Handle logs commands
pub async fn handle_logs_command(
    command: EnterpriseLogsCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;

    match command {
        EnterpriseLogsCommands::List {
            severity,
            module,
            limit,
        } => {
            let mut params = Vec::new();
            if let Some(severity) = severity {
                params.push(format!("severity={}", severity));
            }
            if let Some(module) = module {
                params.push(format!("module={}", module));
            }
            if let Some(limit) = limit {
                params.push(format!("limit={}", limit));
            }

            let endpoint = if params.is_empty() {
                "/v1/logs".to_string()
            } else {
                format!("/v1/logs?{}", params.join("&"))
            };

            let result = client.get_raw(&endpoint).await?;
            print_output(result, output_format, query)?;
        }
        EnterpriseLogsCommands::Show { id } => {
            let result = client.get_raw(&format!("/v1/logs/{}", id)).await?;
            print_output(result, output_format, query)?;
        }
    }

    Ok(())
}
