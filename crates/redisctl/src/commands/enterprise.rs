use crate::config::{Profile, ProfileCredentials};
use crate::output::{OutputFormat, print_output};
use anyhow::Result;
use redis_enterprise::EnterpriseClient;
use std::io::Write;

use crate::cli::{
    AlertCommands, AuthCommands, BootstrapCommands, CcsServerCommands, ClientCertCommands,
    ClusterCommands, CmServerCommands, CrdbTaskCommands, CrdtCommands, DatabaseCommands,
    DebugInfoCommands, DiagnosticsCommands, DmcServerCommands, EndpointCommands,
    EnterpriseActionCommands, EnterpriseCommands, EnterpriseCrdbCommands, EnterpriseLogsCommands,
    EnterpriseStatsCommands, JobSchedulerCommands, JsonSchemaCommands, LdapMappingCommands,
    LicenseCommands, MigrationCommands, ModuleCommands, NodeCommands, OcspCommands,
    PdnServerCommands, ProxyCommands, RedisAclCommands, RoleCommands, ServiceCommands,
    ShardCommands, SuffixCommands, UsageReportCommands, UserCommands,
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
        EnterpriseCommands::RedisAcl { command } => {
            handle_redis_acl_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::Shard { command } => {
            handle_shard_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::Proxy { command } => {
            handle_proxy_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::Service { command } => {
            handle_service_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::CrdbTask { command } => {
            handle_crdb_task_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::DebugInfo { command } => {
            handle_debug_info_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::Diagnostics { command } => {
            handle_diagnostics_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::Endpoint { command } => {
            handle_endpoint_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::Migration { command } => {
            handle_migration_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::Ocsp { command } => {
            handle_ocsp_status_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::UsageReport { command } => {
            handle_usage_report_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::JobScheduler { command } => {
            handle_job_scheduler_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::JsonSchema { command } => {
            handle_json_schema_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::LdapMapping { command } => {
            handle_ldap_mapping_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::Suffix { command } => {
            handle_suffix_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::Auth { command } => {
            handle_auth_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::Crdt { command } => {
            handle_crdt_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::ClientCert { command } => {
            handle_client_cert_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::CmServer { command } => {
            handle_cm_server_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::CcsServer { command } => {
            handle_ccs_server_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::DmcServer { command } => {
            handle_dmc_server_command(command, profile, output_format, query).await
        }
        EnterpriseCommands::PdnServer { command } => {
            handle_pdn_server_command(command, profile, output_format, query).await
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
                memory_size: Some(memory_limit.unwrap_or(100) * 1024 * 1024), // Convert MB to bytes
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
                sharding: None,
                shards_count: None,
                shard_count: None,
                proxy_policy: None,
                rack_aware: None,
                crdt: None,
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
            let request = redis_enterprise::CreateUserRequest::builder()
                .email(email.unwrap_or_else(|| format!("{}@redis.local", name)))
                .password(password.unwrap_or_else(|| "default_password".to_string()))
                .role(roles.first().unwrap_or(&"db_viewer".to_string()).clone())
                .name(name.clone())
                .build();
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
                name: None,
                role: None,
                email_alerts: None,
                bdbs_email_alerts: None,
                role_uids: None,
                auth_method: None,
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

/// Handle Redis ACL commands
pub async fn handle_redis_acl_command(
    command: RedisAclCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;
    let handler = redis_enterprise::RedisAclHandler::new(client.clone());

    match command {
        RedisAclCommands::List => {
            let acls = handler.list().await?;
            let value = serde_json::to_value(acls)?;
            print_output(value, output_format, query)?;
        }
        RedisAclCommands::Show { uid } => {
            let acl = handler.get(uid).await?;
            let value = serde_json::to_value(acl)?;
            print_output(value, output_format, query)?;
        }
        RedisAclCommands::Create {
            name,
            acl,
            description,
        } => {
            let request = redis_enterprise::CreateRedisAclRequest {
                name: name.clone(),
                acl,
                description,
            };
            let acl_result = handler.create(request).await?;
            let value = serde_json::to_value(acl_result)?;
            print_output(value, output_format, query)?;
        }
        RedisAclCommands::Update {
            uid,
            acl,
            description: _,
        } => {
            // Use raw API for update since there's no UpdateRedisAclRequest
            let request = serde_json::json!({
                "acl": acl
            });
            let client = create_enterprise_client(profile).await?;
            let result = client
                .put_raw(&format!("/v1/redis_acls/{}", uid), request)
                .await?;
            print_output(result, output_format, query)?;
        }
        RedisAclCommands::Delete { uid, force: _ } => {
            handler.delete(uid).await?;
            println!("Redis ACL {} deleted successfully", uid);
        }
    }

    Ok(())
}

/// Handle shard commands
pub async fn handle_shard_command(
    command: ShardCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;
    let handler = redis_enterprise::ShardHandler::new(client.clone());

    match command {
        ShardCommands::List { database, node: _ } => {
            let shards = if let Some(db_uid) = database {
                // There's no list_by_bdb, use raw API and return as Value
                let client = create_enterprise_client(profile).await?;
                let result = client
                    .get_raw(&format!("/v1/bdbs/{}/shards", db_uid))
                    .await?;
                print_output(result, output_format, query)?;
                return Ok(());
            } else {
                handler.list().await?
            };
            let value = serde_json::to_value(shards)?;
            print_output(value, output_format, query)?;
        }
        ShardCommands::Show { uid } => {
            let shard = handler.get(&uid.to_string()).await?;
            let value = serde_json::to_value(shard)?;
            print_output(value, output_format, query)?;
        }
        ShardCommands::Stats { uid, interval: _ } => {
            let stats = handler.stats(&uid.to_string()).await?;
            print_output(stats, output_format, query)?;
        }
    }

    Ok(())
}

/// Handle proxy commands
pub async fn handle_proxy_command(
    command: ProxyCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;
    let handler = redis_enterprise::ProxyHandler::new(client.clone());

    match command {
        ProxyCommands::List { node: _ } => {
            let proxies = handler.list().await?;
            let value = serde_json::to_value(proxies)?;
            print_output(value, output_format, query)?;
        }
        ProxyCommands::Show { uid } => {
            let proxy = handler.get(uid).await?;
            let value = serde_json::to_value(proxy)?;
            print_output(value, output_format, query)?;
        }
        ProxyCommands::Reload { uid } => {
            handler.reload(uid).await?;
            println!("Proxy {} reloaded successfully", uid);
        }
        ProxyCommands::Stats { uid } => {
            let stats = handler.stats(uid).await?;
            print_output(stats, output_format, query)?;
        }
    }

    Ok(())
}

/// Handle service commands
pub async fn handle_service_command(
    command: ServiceCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;
    let handler = redis_enterprise::ServicesHandler::new(client.clone());

    match command {
        ServiceCommands::List { node: _ } => {
            let services = handler.list().await?;
            let value = serde_json::to_value(services)?;
            print_output(value, output_format, query)?;
        }
        ServiceCommands::Show { name, node } => {
            // Use raw API for service lookup by name
            let path = if let Some(node_uid) = node {
                format!("/v1/nodes/{}/services/{}", node_uid, name)
            } else {
                format!("/v1/services/{}", name)
            };
            let client = create_enterprise_client(profile).await?;
            let result = client.get_raw(&path).await?;
            print_output(result, output_format, query)?;
        }
        ServiceCommands::Restart { name, node } => {
            // Use raw API for service restart
            let path = if let Some(node_uid) = node {
                format!("/v1/nodes/{}/services/{}/restart", node_uid, name)
            } else {
                format!("/v1/services/{}/restart", name)
            };
            let client = create_enterprise_client(profile).await?;
            let result = client.post_raw(&path, serde_json::Value::Null).await?;
            print_output(result, output_format, query)?;
        }
    }

    Ok(())
}

/// Handle CRDT commands
pub async fn handle_crdt_command(
    command: CrdtCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;
    let handler = redis_enterprise::CrdbHandler::new(client.clone());

    match command {
        CrdtCommands::List => {
            let crdts = handler.list().await?;
            let value = serde_json::to_value(crdts)?;
            print_output(value, output_format, query)?;
        }
        CrdtCommands::Show { id } => {
            let crdt = handler.get(&id).await?;
            let value = serde_json::to_value(crdt)?;
            print_output(value, output_format, query)?;
        }
        CrdtCommands::Create { database_id } => {
            let request = redis_enterprise::CreateCrdbRequest {
                name: format!("crdb_{}", database_id),
                instances: vec![],
                encryption: None,
                data_persistence: None,
                eviction_policy: None,
                memory_size: 1024 * 1024 * 100, // 100MB default
            };
            let crdt = handler.create(request).await?;
            let value = serde_json::to_value(crdt)?;
            print_output(value, output_format, query)?;
        }
        CrdtCommands::Update { id, database_id: _ } => {
            // Use raw API for update
            let request = serde_json::json!({});
            let crdt = handler.update(&id, request).await?;
            let value = serde_json::to_value(crdt)?;
            print_output(value, output_format, query)?;
        }
        CrdtCommands::Delete { id } => {
            handler.delete(&id).await?;
            println!("CRDT {} deleted successfully", id);
        }
    }

    Ok(())
}

/// Handle authentication commands
pub async fn handle_auth_command(
    command: AuthCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;

    match command {
        AuthCommands::Test {
            profile: _,
            deployment: _,
        } => {
            // Test authentication by trying to get cluster info
            let result = client.get_raw("/v1/cluster").await?;
            println!("Authentication successful");
            print_output(result, output_format, query)?;
        }
        AuthCommands::Setup => {
            // Setup authentication (not implemented)
            anyhow::bail!("Auth setup is not yet implemented")
        }
    }

    Ok(())
}

/// Handle client certificate commands
pub async fn handle_client_cert_command(
    command: ClientCertCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;

    match command {
        ClientCertCommands::List => {
            let result = client.get_raw("/v1/client_certs").await?;
            print_output(result, output_format, query)?;
        }
        ClientCertCommands::Show { id } => {
            let result = client.get_raw(&format!("/v1/client_certs/{}", id)).await?;
            print_output(result, output_format, query)?;
        }
        ClientCertCommands::Create { name, cert } => {
            let request = serde_json::json!({
                "name": name,
                "certificate": cert
            });
            let result = client.post_raw("/v1/client_certs", request).await?;
            print_output(result, output_format, query)?;
        }
        ClientCertCommands::Delete { id } => {
            client
                .delete_raw(&format!("/v1/client_certs/{}", id))
                .await?;
            println!("Client certificate {} deleted successfully", id);
        }
    }

    Ok(())
}

/// Handle CM server commands
pub async fn handle_cm_server_command(
    command: CmServerCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;
    match command {
        CmServerCommands::List => {
            let result = client.get_raw("/v1/cm_servers").await?;
            print_output(result, output_format, query)?;
        }
        CmServerCommands::Show { id } => {
            let result = client.get_raw(&format!("/v1/cm_servers/{}", id)).await?;
            print_output(result, output_format, query)?;
        }
        CmServerCommands::Stats { id } => {
            let result = client
                .get_raw(&format!("/v1/cm_servers/{}/stats", id))
                .await?;
            print_output(result, output_format, query)?;
        }
    }

    Ok(())
}

/// Handle CCS server commands
pub async fn handle_ccs_server_command(
    command: CcsServerCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;
    match command {
        CcsServerCommands::List => {
            let result = client.get_raw("/v1/ccs_servers").await?;
            print_output(result, output_format, query)?;
        }
        CcsServerCommands::Show { id } => {
            let result = client.get_raw(&format!("/v1/ccs_servers/{}", id)).await?;
            print_output(result, output_format, query)?;
        }
        CcsServerCommands::Stats { id } => {
            let result = client
                .get_raw(&format!("/v1/ccs_servers/{}/stats", id))
                .await?;
            print_output(result, output_format, query)?;
        }
    }

    Ok(())
}

/// Handle DMC server commands
pub async fn handle_dmc_server_command(
    command: DmcServerCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;
    match command {
        DmcServerCommands::List => {
            let result = client.get_raw("/v1/dmc_servers").await?;
            print_output(result, output_format, query)?;
        }
        DmcServerCommands::Show { id } => {
            let result = client.get_raw(&format!("/v1/dmc_servers/{}", id)).await?;
            print_output(result, output_format, query)?;
        }
        DmcServerCommands::Stats { id } => {
            let result = client
                .get_raw(&format!("/v1/dmc_servers/{}/stats", id))
                .await?;
            print_output(result, output_format, query)?;
        }
    }

    Ok(())
}

/// Handle PDN server commands
pub async fn handle_pdn_server_command(
    command: PdnServerCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;
    match command {
        PdnServerCommands::List => {
            let result = client.get_raw("/v1/pdn_servers").await?;
            print_output(result, output_format, query)?;
        }
        PdnServerCommands::Show { id } => {
            let result = client.get_raw(&format!("/v1/pdn_servers/{}", id)).await?;
            print_output(result, output_format, query)?;
        }
        PdnServerCommands::Stats { id } => {
            let result = client
                .get_raw(&format!("/v1/pdn_servers/{}/stats", id))
                .await?;
            print_output(result, output_format, query)?;
        }
    }

    Ok(())
}

/// Handle LDAP mapping commands
pub async fn handle_ldap_mapping_command(
    command: LdapMappingCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;
    let handler = redis_enterprise::LdapMappingHandler::new(client.clone());

    match command {
        LdapMappingCommands::List => {
            let mappings = handler.list().await?;
            let value = serde_json::to_value(mappings)?;
            print_output(value, output_format, query)?;
        }
        LdapMappingCommands::Show { id } => {
            let mapping = handler.get(id).await?;
            let value = serde_json::to_value(mapping)?;
            print_output(value, output_format, query)?;
        }
        LdapMappingCommands::Create { dn, role } => {
            let request = redis_enterprise::CreateLdapMappingRequest {
                name: dn.clone(), // Use DN as name
                dn: dn.clone(),
                role,
                email: None,
                role_uids: None,
            };
            let mapping = handler.create(request).await?;
            let value = serde_json::to_value(mapping)?;
            print_output(value, output_format, query)?;
        }
        LdapMappingCommands::Update { id, role } => {
            // Use raw API for update since there's no UpdateLdapMappingRequest
            let request = serde_json::json!({
                "role": role
            });
            let client = create_enterprise_client(profile).await?;
            let result = client
                .put_raw(&format!("/v1/ldap_mappings/{}", id), request)
                .await?;
            print_output(result, output_format, query)?;
        }
        LdapMappingCommands::Delete { id, force: _ } => {
            handler.delete(id).await?;
            println!("LDAP mapping {} deleted successfully", id);
        }
    }

    Ok(())
}

/// Handle OCSP status commands
pub async fn handle_ocsp_status_command(
    command: OcspCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;
    let handler = redis_enterprise::OcspHandler::new(client.clone());

    match command {
        OcspCommands::Status => {
            let status = handler.get_status().await?;
            print_output(status, output_format, query)?;
        }
        OcspCommands::Test { server: _ } => {
            let result = handler.test().await?;
            let value = serde_json::to_value(result)?;
            print_output(value, output_format, query)?;
        }
        OcspCommands::Update { enabled, server } => {
            // Use raw API for update
            let request = serde_json::json!({
                "enabled": enabled,
                "server": server
            });
            let client = create_enterprise_client(profile).await?;
            let result = client.put_raw("/v1/ocsp", request).await?;
            print_output(result, output_format, query)?;
        }
    }

    Ok(())
}

/// Handle suffix commands
pub async fn handle_suffix_command(
    command: SuffixCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;
    let handler = redis_enterprise::SuffixesHandler::new(client.clone());

    match command {
        SuffixCommands::List => {
            let suffixes = handler.list().await?;
            let value = serde_json::to_value(suffixes)?;
            print_output(value, output_format, query)?;
        }
        SuffixCommands::Show { id } => {
            // SuffixesHandler.get() takes a string
            let suffix = handler.get(&id).await?;
            let value = serde_json::to_value(suffix)?;
            print_output(value, output_format, query)?;
        }
        SuffixCommands::Create { name, dns_suffix } => {
            let request = redis_enterprise::CreateSuffixRequest {
                name: name.clone(),
                dns_suffix: dns_suffix.clone(),
                use_external_addr: None,
                use_internal_addr: None,
            };
            let suffix = handler.create(request).await?;
            let value = serde_json::to_value(suffix)?;
            print_output(value, output_format, query)?;
        }
        SuffixCommands::Update { id, dns_suffix } => {
            // Use raw API for update since there's no UpdateSuffixRequest
            let request = serde_json::json!({
                "dns_suffix": dns_suffix
            });
            let client = create_enterprise_client(profile).await?;
            let result = client
                .put_raw(&format!("/v1/suffixes/{}", id), request)
                .await?;
            print_output(result, output_format, query)?;
        }
        SuffixCommands::Delete { id } => {
            handler.delete(&id).await?;
            println!("Suffix {} deleted successfully", id);
        }
    }

    Ok(())
}

/// Handle CRDB task commands
pub async fn handle_crdb_task_command(
    command: CrdbTaskCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;

    match command {
        CrdbTaskCommands::List { crdb } => {
            let path = if let Some(crdb_guid) = crdb {
                format!("/v1/crdbs/{}/tasks", crdb_guid)
            } else {
                "/v1/crdb_tasks".to_string()
            };
            let result = client.get_raw(&path).await?;
            print_output(result, output_format, query)?;
        }
        CrdbTaskCommands::Show { uid } => {
            let result = client.get_raw(&format!("/v1/crdb_tasks/{}", uid)).await?;
            print_output(result, output_format, query)?;
        }
        CrdbTaskCommands::Create { task_type, crdb } => {
            let request = serde_json::json!({
                "task_type": task_type,
                "crdb_guid": crdb
            });
            let result = client.post_raw("/v1/crdb_tasks", request).await?;
            print_output(result, output_format, query)?;
        }
    }

    Ok(())
}

/// Handle debug info commands
pub async fn handle_debug_info_command(
    command: DebugInfoCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;

    match command {
        DebugInfoCommands::Collect { from, to } => {
            let mut request = serde_json::json!({});
            if let Some(from_time) = from {
                request["from"] = serde_json::Value::String(from_time);
            }
            if let Some(to_time) = to {
                request["to"] = serde_json::Value::String(to_time);
            }
            let result = client.post_raw("/v1/debuginfo", request).await?;
            print_output(result, output_format, query)?;
        }
        DebugInfoCommands::Status { id } => {
            let result = client.get_raw(&format!("/v1/debuginfo/{}", id)).await?;
            print_output(result, output_format, query)?;
        }
        DebugInfoCommands::Download { id, output: _ } => {
            let result = client
                .get_raw(&format!("/v1/debuginfo/{}/download", id))
                .await?;
            print_output(result, output_format, query)?;
        }
    }

    Ok(())
}

/// Handle diagnostics commands
pub async fn handle_diagnostics_command(
    command: DiagnosticsCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;

    match command {
        DiagnosticsCommands::Run { diagnostic_type } => {
            let request = if let Some(diag_type) = diagnostic_type {
                serde_json::json!({
                    "type": diag_type
                })
            } else {
                serde_json::json!({})
            };
            let result = client.post_raw("/v1/diagnostics", request).await?;
            print_output(result, output_format, query)?;
        }
        DiagnosticsCommands::Status => {
            let result = client.get_raw("/v1/diagnostics/status").await?;
            print_output(result, output_format, query)?;
        }
        DiagnosticsCommands::Download { output: _ } => {
            let result = client.get_raw("/v1/diagnostics/download").await?;
            print_output(result, output_format, query)?;
        }
    }

    Ok(())
}

/// Handle endpoint commands
pub async fn handle_endpoint_command(
    command: EndpointCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;

    match command {
        EndpointCommands::List { database } => {
            let mut params = Vec::new();
            if let Some(db_uid) = database {
                params.push(format!("bdb={}", db_uid));
            }

            let path = if params.is_empty() {
                "/v1/endpoints".to_string()
            } else {
                format!("/v1/endpoints?{}", params.join("&"))
            };

            let result = client.get_raw(&path).await?;
            print_output(result, output_format, query)?;
        }
        EndpointCommands::Show { uid } => {
            let result = client.get_raw(&format!("/v1/endpoints/{}", uid)).await?;
            print_output(result, output_format, query)?;
        }
        EndpointCommands::Stats { uid } => {
            let result = client
                .get_raw(&format!("/v1/endpoints/{}/stats", uid))
                .await?;
            print_output(result, output_format, query)?;
        }
    }

    Ok(())
}

/// Handle migration commands
pub async fn handle_migration_command(
    command: MigrationCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;

    match command {
        MigrationCommands::List => {
            let result = client.get_raw("/v1/migrations").await?;
            print_output(result, output_format, query)?;
        }
        MigrationCommands::Show { uid } => {
            let result = client.get_raw(&format!("/v1/migrations/{}", uid)).await?;
            print_output(result, output_format, query)?;
        }
        MigrationCommands::Create { source, target } => {
            let request = serde_json::json!({
                "source": source,
                "target": target
            });
            let result = client.post_raw("/v1/migrations", request).await?;
            print_output(result, output_format, query)?;
        }
        MigrationCommands::Status { uid } => {
            let result = client
                .get_raw(&format!("/v1/migrations/{}/status", uid))
                .await?;
            print_output(result, output_format, query)?;
        }
    }

    Ok(())
}

/// Handle usage report commands
pub async fn handle_usage_report_command(
    command: UsageReportCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;

    match command {
        UsageReportCommands::Get { period } => {
            let path = if let Some(period_str) = period {
                format!("/v1/usage_reports?period={}", period_str)
            } else {
                "/v1/usage_reports".to_string()
            };
            let result = client.get_raw(&path).await?;
            print_output(result, output_format, query)?;
        }
        UsageReportCommands::Download { period, output: _ } => {
            let path = if let Some(p) = period {
                format!("/v1/usage_reports/download?period={}", p)
            } else {
                "/v1/usage_reports/download".to_string()
            };
            let result = client.get_raw(&path).await?;
            print_output(result, output_format, query)?;
        }
    }

    Ok(())
}

/// Handle job scheduler commands
pub async fn handle_job_scheduler_command(
    command: JobSchedulerCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;

    match command {
        JobSchedulerCommands::List => {
            let result = client.get_raw("/v1/jobs").await?;
            print_output(result, output_format, query)?;
        }
        JobSchedulerCommands::Show { id } => {
            let result = client.get_raw(&format!("/v1/jobs/{}", id)).await?;
            print_output(result, output_format, query)?;
        }
        JobSchedulerCommands::Create {
            name,
            cron,
            command: job_command,
        } => {
            let request = serde_json::json!({
                "name": name,
                "cron": cron,
                "command": job_command
            });
            let result = client.post_raw("/v1/jobs", request).await?;
            print_output(result, output_format, query)?;
        }
        JobSchedulerCommands::Delete { id, force: _ } => {
            client.delete_raw(&format!("/v1/jobs/{}", id)).await?;
            println!("Scheduled job {} deleted successfully", id);
        }
    }

    Ok(())
}

/// Handle JSON schema commands
pub async fn handle_json_schema_command(
    command: JsonSchemaCommands,
    profile: &Profile,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let client = create_enterprise_client(profile).await?;

    match command {
        JsonSchemaCommands::Get { path } => {
            let result = client.get_raw(&format!("/v1/jsonschema{}", path)).await?;
            print_output(result, output_format, query)?;
        }
    }

    Ok(())
}
