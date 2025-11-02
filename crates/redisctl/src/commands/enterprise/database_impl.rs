//! Enterprise database command implementations

#![allow(dead_code)]

use crate::cli::OutputFormat;
use crate::connection::ConnectionManager;
use crate::error::{RedisCtlError, Result as CliResult};
use serde_json::Value;

use super::utils::*;

/// List all databases
pub async fn list_databases(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let response = client
        .get_raw("/v1/bdbs")
        .await
        .map_err(RedisCtlError::from)?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Get database details
pub async fn get_database(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let response = client
        .get_raw(&format!("/v1/bdbs/{}", id))
        .await
        .map_err(RedisCtlError::from)?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Create a new database
pub async fn create_database(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    data: &str,
    dry_run: bool,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let json_data = read_json_data(data)?;

    let path = if dry_run {
        "/v1/bdbs/dry-run"
    } else {
        "/v1/bdbs"
    };

    let response = client
        .post_raw(path, json_data)
        .await
        .map_err(RedisCtlError::from)?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Update database configuration
pub async fn update_database(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let json_data = read_json_data(data)?;

    let response = client
        .put_raw(&format!("/v1/bdbs/{}", id), json_data)
        .await
        .map_err(RedisCtlError::from)?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Delete a database
pub async fn delete_database(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    force: bool,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    if !force && !confirm_action(&format!("Delete database {}?", id))? {
        println!("Operation cancelled");
        return Ok(());
    }

    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let response = client
        .delete_raw(&format!("/v1/bdbs/{}", id))
        .await
        .map_err(RedisCtlError::from)?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Export database
pub async fn export_database(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let json_data = read_json_data(data)?;

    let response = client
        .post_raw(&format!("/v1/bdbs/{}/export", id), json_data)
        .await
        .map_err(RedisCtlError::from)?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Import to database
pub async fn import_database(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let json_data = read_json_data(data)?;

    let response = client
        .post_raw(&format!("/v1/bdbs/{}/import", id), json_data)
        .await
        .map_err(RedisCtlError::from)?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Trigger database backup
pub async fn backup_database(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let response = client
        .post_raw(&format!("/v1/bdbs/{}/backup", id), Value::Null)
        .await
        .map_err(RedisCtlError::from)?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Restore database
pub async fn restore_database(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let json_data = read_json_data(data)?;

    let response = client
        .post_raw(&format!("/v1/bdbs/{}/restore", id), json_data)
        .await
        .map_err(RedisCtlError::from)?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Flush database data
pub async fn flush_database(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    force: bool,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    if !force
        && !confirm_action(&format!(
            "Flush all data from database {}? This will delete all data!",
            id
        ))?
    {
        println!("Operation cancelled");
        return Ok(());
    }

    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let response = client
        .put_raw(&format!("/v1/bdbs/{}/flush", id), Value::Null)
        .await
        .map_err(RedisCtlError::from)?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Get database shards
pub async fn get_database_shards(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let response = client
        .get_raw(&format!("/v1/bdbs/{}/shards", id))
        .await
        .map_err(RedisCtlError::from)?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Update database shards
pub async fn update_database_shards(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let json_data = read_json_data(data)?;

    let response = client
        .put_raw(&format!("/v1/bdbs/{}/shards", id), json_data)
        .await
        .map_err(RedisCtlError::from)?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Get database modules
pub async fn get_database_modules(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let response = client
        .get_raw(&format!("/v1/bdbs/{}/modules", id))
        .await
        .map_err(RedisCtlError::from)?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Update database modules
pub async fn update_database_modules(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let json_data = read_json_data(data)?;

    let response = client
        .put_raw(&format!("/v1/bdbs/{}/modules", id), json_data)
        .await
        .map_err(RedisCtlError::from)?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Get database ACL
pub async fn get_database_acl(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let response = client
        .get_raw(&format!("/v1/bdbs/{}/acl", id))
        .await
        .map_err(RedisCtlError::from)?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Update database ACL
pub async fn update_database_acl(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let json_data = read_json_data(data)?;

    let response = client
        .put_raw(&format!("/v1/bdbs/{}/acl", id), json_data)
        .await
        .map_err(RedisCtlError::from)?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Get database statistics
pub async fn get_database_stats(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let response = client
        .get_raw(&format!("/v1/bdbs/{}/stats", id))
        .await
        .map_err(RedisCtlError::from)?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Get database metrics
pub async fn get_database_metrics(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    interval: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let mut path = format!("/v1/bdbs/{}/metrics", id);
    if let Some(interval) = interval {
        path.push_str(&format!("?interval={}", interval));
    }

    let response = client.get_raw(&path).await.map_err(RedisCtlError::from)?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Get database slowlog
pub async fn get_database_slowlog(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    limit: Option<u32>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let mut path = format!("/v1/bdbs/{}/slowlog", id);
    if let Some(limit) = limit {
        path.push_str(&format!("?limit={}", limit));
    }

    let response = client.get_raw(&path).await.map_err(RedisCtlError::from)?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Get connected clients
pub async fn get_database_clients(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let response = client
        .get_raw(&format!("/v1/bdbs/{}/clients", id))
        .await
        .map_err(RedisCtlError::from)?;

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Upgrade database Redis version
#[allow(clippy::too_many_arguments)]
pub async fn upgrade_database(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    version: Option<&str>,
    preserve_roles: bool,
    force_restart: bool,
    may_discard_data: bool,
    force_discard: bool,
    keep_crdt_protocol_version: bool,
    parallel_shards_upgrade: Option<u32>,
    force: bool,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    use redis_enterprise::bdb::{DatabaseHandler, DatabaseInfo, DatabaseUpgradeRequest};

    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    // Get current database info
    let db_handler = DatabaseHandler::new(client);
    let db: DatabaseInfo = db_handler.get(id).await?;
    let current_version = db.redis_version.as_deref().unwrap_or("unknown");

    // Determine target version
    let target_version = if let Some(v) = version {
        v.to_string()
    } else {
        // Get latest version from cluster - for now just use current
        // TODO: Get from cluster info when we add that endpoint
        current_version.to_string()
    };

    // Safety checks unless --force
    if !force {
        // Check if database is active
        if db.status.as_deref() != Some("active") {
            return Err(RedisCtlError::InvalidInput {
                message: format!(
                    "Database is not active (status: {}). Use --force to upgrade anyway.",
                    db.status.as_deref().unwrap_or("unknown")
                ),
            });
        }

        // Warn about persistence (check if persistence is disabled/none)
        let has_persistence = db
            .persistence
            .as_deref()
            .map(|p| p != "disabled")
            .unwrap_or(false);
        if !has_persistence && !may_discard_data {
            eprintln!("Warning: Database has no persistence enabled.");
            eprintln!("If upgrade fails, data may be lost.");
            eprintln!("Use --may-discard-data to proceed.");
            return Err(RedisCtlError::InvalidInput {
                message: "Upgrade cancelled for safety".to_string(),
            });
        }

        // Warn about replication (check if replication is enabled)
        let has_replication = db.replication.unwrap_or(false);
        if !has_replication {
            eprintln!("Warning: Database has no replication enabled.");
            eprintln!("Upgrade will cause downtime.");
            eprintln!("Use --force to proceed.");
            return Err(RedisCtlError::InvalidInput {
                message: "Upgrade cancelled for safety".to_string(),
            });
        }
    }

    // Display upgrade info
    if matches!(output_format, OutputFormat::Table | OutputFormat::Auto) {
        println!("Upgrading database '{}' (db:{})...", db.name, id);
        println!("  Current version: {}", current_version);
        println!("  Target version: {}", target_version);
    }

    // Build upgrade request
    let request = DatabaseUpgradeRequest {
        redis_version: Some(target_version.clone()),
        preserve_roles: Some(preserve_roles),
        force_restart: Some(force_restart),
        may_discard_data: Some(may_discard_data),
        force_discard: Some(force_discard),
        keep_crdt_protocol_version: Some(keep_crdt_protocol_version),
        parallel_shards_upgrade,
        modules: None,
    };

    // Call upgrade API
    let response = db_handler.upgrade_redis_version(id, request).await?;

    // Handle output
    match output_format {
        OutputFormat::Json => {
            let output = serde_json::json!({
                "database_id": id,
                "database_name": db.name,
                "old_version": current_version,
                "new_version": target_version,
                "action_uid": response.action_uid,
                "status": "upgrade_initiated"
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        OutputFormat::Table | OutputFormat::Auto => {
            println!("Upgrade initiated (action_uid: {})", response.action_uid);
            println!(
                "Use 'redisctl enterprise database get {}' to check status",
                id
            );
        }
        _ => {
            let data = serde_json::to_value(&response)?;
            let filtered = handle_output(data, output_format, query)?;
            print_formatted_output(filtered, output_format)?;
        }
    }

    Ok(())
}
