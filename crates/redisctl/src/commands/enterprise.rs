use anyhow::Result;
use redis_common::{OutputFormat, Profile, ProfileCredentials, print_output};
use redis_enterprise::{EnterpriseClient, EnterpriseConfig};
use std::io::Write;

use crate::cli::{
    BootstrapCommands, ClusterCommands, DatabaseCommands, EnterpriseCommands, LicenseCommands,
    ModuleCommands, NodeCommands, RoleCommands, UserCommands,
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
            let databases = client.get_raw("/v1/bdbs").await?;
            print_output(databases, output_format, query)?;
        }
        DatabaseCommands::Show { id } => {
            let database = client.get_raw(&format!("/v1/bdbs/{}", id)).await?;
            print_output(database, output_format, query)?;
        }
        DatabaseCommands::Create {
            name,
            memory_limit,
            modules,
        } => {
            let mut create_data = serde_json::json!({
                "name": name,
                "type": "redis",
                "memory_size": memory_limit.unwrap_or(100) * 1024 * 1024, // Convert MB to bytes
            });

            if !modules.is_empty() {
                create_data["module_list"] = serde_json::Value::Array(
                    modules.into_iter().map(serde_json::Value::String).collect(),
                );
            }

            let database = client.post_raw("/v1/bdbs", create_data).await?;
            print_output(database, output_format, query)?;
        }
        DatabaseCommands::Update {
            id,
            name,
            memory_limit,
        } => {
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

            let database = client
                .put_raw(
                    &format!("/v1/bdbs/{}", id),
                    serde_json::Value::Object(update_data),
                )
                .await?;
            print_output(database, output_format, query)?;
        }
        DatabaseCommands::Delete { id, force: _ } => {
            client.delete_raw(&format!("/v1/bdbs/{}", id)).await?;
            println!("Database {} deleted successfully", id);
        }
        DatabaseCommands::Backup { id } => {
            let backup = client
                .post_raw(&format!("/v1/bdbs/{}/backup", id), serde_json::json!({}))
                .await?;
            print_output(backup, output_format, query)?;
        }
        DatabaseCommands::Import { id, url } => {
            let import_data = serde_json::json!({
                "source_file": url
            });
            let import_result = client
                .post_raw(&format!("/v1/bdbs/{}/import", id), import_data)
                .await?;
            print_output(import_result, output_format, query)?;
        }
        DatabaseCommands::Export { id, format } => {
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
            let info = client.get_raw("/v1/cluster").await?;
            print_output(info, output_format, query)?;
        }
        ClusterCommands::Nodes => {
            let nodes = client.get_raw("/v1/nodes").await?;
            print_output(nodes, output_format, query)?;
        }
        ClusterCommands::Settings => {
            let settings = client.get_raw("/v1/cluster/settings").await?;
            print_output(settings, output_format, query)?;
        }
        ClusterCommands::Update { name, value } => {
            let update_data = serde_json::json!({ name: value });
            let result = client.put_raw("/v1/cluster/settings", update_data).await?;
            print_output(result, output_format, query)?;
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
            let nodes = client.get_raw("/v1/nodes").await?;
            print_output(nodes, output_format, query)?;
        }
        NodeCommands::Show { id } => {
            let node = client.get_raw(&format!("/v1/nodes/{}", id)).await?;
            print_output(node, output_format, query)?;
        }
        NodeCommands::Update { id, external_addr } => {
            let mut update_data = serde_json::Map::new();

            if let Some(external_addr) = external_addr {
                update_data.insert(
                    "external_addr".to_string(),
                    serde_json::Value::Array(vec![serde_json::Value::String(external_addr)]),
                );
            }

            let node = client
                .put_raw(
                    &format!("/v1/nodes/{}", id),
                    serde_json::Value::Object(update_data),
                )
                .await?;
            print_output(node, output_format, query)?;
        }
        NodeCommands::Add {
            addr,
            username,
            password,
            external_addr,
        } => {
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
            client.delete_raw(&format!("/v1/nodes/{}", id)).await?;
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
            let users = client.get_raw("/v1/users").await?;
            print_output(users, output_format, query)?;
        }
        UserCommands::Show { id } => {
            let user = client.get_raw(&format!("/v1/users/{}", id)).await?;
            print_output(user, output_format, query)?;
        }
        UserCommands::Create {
            name,
            email,
            password,
            roles,
        } => {
            let create_data = serde_json::json!({
                "name": name,
                "email": email,
                "password": password,
                "role": roles.first().unwrap_or(&"db_viewer".to_string()).clone()
            });

            let user = client.post_raw("/v1/users", create_data).await?;
            print_output(user, output_format, query)?;
        }
        UserCommands::Update {
            id,
            email,
            password,
        } => {
            let mut update_data = serde_json::Map::new();

            if let Some(email) = email {
                update_data.insert("email".to_string(), serde_json::Value::String(email));
            }
            if let Some(password) = password {
                update_data.insert("password".to_string(), serde_json::Value::String(password));
            }

            let user = client
                .put_raw(
                    &format!("/v1/users/{}", id),
                    serde_json::Value::Object(update_data),
                )
                .await?;
            print_output(user, output_format, query)?;
        }
        UserCommands::Delete { id, force: _ } => {
            client.delete_raw(&format!("/v1/users/{}", id)).await?;
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
            license,
            email,
            password,
        } => {
            let bootstrap_data = serde_json::json!({
                "license": license,
                "username": email,
                "password": password
            });

            let result = client
                .post_bootstrap("/v1/bootstrap", &bootstrap_data)
                .await?;
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
            let roles = client.get_raw("/v1/roles").await?;
            print_output(roles, output_format, query)?;
        }
        RoleCommands::Show { id } => {
            let role = client.get_raw(&format!("/v1/roles/{}", id)).await?;
            print_output(role, output_format, query)?;
        }
        RoleCommands::Create { name, permissions } => {
            let create_data = serde_json::json!({
                "name": name,
                "management": permissions.contains(&"management".to_string()),
                "redis_acl_rule": permissions.join(" ")
            });

            let role = client.post_raw("/v1/roles", create_data).await?;
            print_output(role, output_format, query)?;
        }
        RoleCommands::Update { id, permissions } => {
            let update_data = serde_json::json!({
                "redis_acl_rule": permissions.join(" ")
            });

            let role = client
                .put_raw(&format!("/v1/roles/{}", id), update_data)
                .await?;
            print_output(role, output_format, query)?;
        }
        RoleCommands::Delete { id, force: _ } => {
            client.delete_raw(&format!("/v1/roles/{}", id)).await?;
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
            let license = client.get_raw("/v1/license").await?;
            print_output(license, output_format, query)?;
        }
        LicenseCommands::Update { key } => {
            let update_data = serde_json::json!({
                "license": key
            });

            let result = client.put_raw("/v1/license", update_data).await?;
            print_output(result, output_format, query)?;
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

        let config = EnterpriseConfig {
            base_url: url.clone(),
            username: username.clone(),
            password,
            timeout: std::time::Duration::from_secs(30),
            insecure: *insecure,
        };

        EnterpriseClient::new(config).map_err(Into::into)
    } else {
        anyhow::bail!("Invalid profile type for Enterprise commands")
    }
}
