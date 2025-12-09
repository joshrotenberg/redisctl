//! Implementation of additional database commands

use super::async_utils::{AsyncOperationArgs, handle_async_response};
use super::utils::*;
use crate::cli::OutputFormat;
use crate::connection::ConnectionManager;
use crate::error::{RedisCtlError, Result as CliResult};
use crate::output::print_output;
use anyhow::Context;
use serde_json::{Value, json};
use tabled::{Table, Tabled, settings::Style};

/// Helper to print non-table output
fn print_json_or_yaml(data: Value, output_format: OutputFormat) -> CliResult<()> {
    match output_format {
        OutputFormat::Json => print_output(data, crate::output::OutputFormat::Json, None)?,
        OutputFormat::Yaml => print_output(data, crate::output::OutputFormat::Yaml, None)?,
        _ => print_output(data, crate::output::OutputFormat::Json, None)?,
    }
    Ok(())
}

/// Parse database ID into subscription and database IDs
fn parse_database_id(id: &str) -> CliResult<(u32, u32)> {
    let parts: Vec<&str> = id.split(':').collect();
    if parts.len() != 2 {
        return Err(RedisCtlError::InvalidInput {
            message: format!(
                "Invalid database ID format: {}. Expected format: subscription_id:database_id",
                id
            ),
        });
    }

    let subscription_id = parts[0]
        .parse::<u32>()
        .map_err(|_| RedisCtlError::InvalidInput {
            message: format!("Invalid subscription ID: {}", parts[0]),
        })?;

    let database_id = parts[1]
        .parse::<u32>()
        .map_err(|_| RedisCtlError::InvalidInput {
            message: format!("Invalid database ID: {}", parts[1]),
        })?;

    Ok((subscription_id, database_id))
}

/// Read JSON data from string or file
fn read_json_data(data: &str) -> CliResult<Value> {
    let json_str = if let Some(file_path) = data.strip_prefix('@') {
        // Read from file
        std::fs::read_to_string(file_path).map_err(|e| RedisCtlError::InvalidInput {
            message: format!("Failed to read file {}: {}", file_path, e),
        })?
    } else {
        // Use as-is
        data.to_string()
    };

    serde_json::from_str(&json_str).map_err(|e| RedisCtlError::InvalidInput {
        message: format!("Invalid JSON: {}", e),
    })
}

/// Create a new database with first-class parameters
#[allow(clippy::too_many_arguments)]
pub async fn create_database(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    subscription_id: u32,
    name: Option<&str>,
    memory: Option<f64>,
    dataset_size: Option<f64>,
    protocol: &str,
    replication: bool,
    data_persistence: Option<&str>,
    eviction_policy: &str,
    redis_version: Option<&str>,
    oss_cluster: bool,
    port: Option<i32>,
    data: Option<&str>,
    async_ops: &AsyncOperationArgs,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_cloud_client(profile_name).await?;

    // Start with JSON from --data if provided, otherwise empty object
    let mut request = if let Some(data_str) = data {
        read_json_data(data_str)?
    } else {
        json!({})
    };

    // Ensure request is an object
    if !request.is_object() {
        return Err(RedisCtlError::InvalidInput {
            message: "Database configuration must be a JSON object".to_string(),
        });
    }

    let request_obj = request.as_object_mut().unwrap();

    // CLI parameters override JSON values
    // Required parameters (when not using pure --data mode)
    if let Some(name_val) = name {
        request_obj.insert("name".to_string(), json!(name_val));
    } else if data.is_none() {
        return Err(RedisCtlError::InvalidInput {
            message: "--name is required (unless using --data with complete configuration)"
                .to_string(),
        });
    }

    // Memory configuration (must have either --memory, --dataset-size, or in --data)
    if let Some(mem) = memory {
        request_obj.insert("memoryLimitInGb".to_string(), json!(mem));
    } else if let Some(dataset) = dataset_size {
        request_obj.insert("datasetSizeInGb".to_string(), json!(dataset));
    } else if data.is_none() {
        return Err(RedisCtlError::InvalidInput {
            message: "Either --memory or --dataset-size is required (unless using --data with complete configuration)".to_string(),
        });
    }

    // Protocol (only set if non-default or not already in data)
    if protocol != "redis" || !request_obj.contains_key("protocol") {
        request_obj.insert("protocol".to_string(), json!(protocol));
    }

    // Replication (only set if true or not already in data)
    if replication || !request_obj.contains_key("replication") {
        request_obj.insert("replication".to_string(), json!(replication));
    }

    // Optional parameters - only set if provided
    if let Some(persistence) = data_persistence {
        request_obj.insert("dataPersistence".to_string(), json!(persistence));
    }

    // Eviction policy (only set if non-default or not already in data)
    if eviction_policy != "volatile-lru" || !request_obj.contains_key("dataEvictionPolicy") {
        request_obj.insert("dataEvictionPolicy".to_string(), json!(eviction_policy));
    }

    if let Some(version) = redis_version {
        request_obj.insert("redisVersion".to_string(), json!(version));
    }

    if oss_cluster {
        request_obj.insert("supportOSSClusterAPI".to_string(), json!(true));
    }

    if let Some(port_val) = port {
        request_obj.insert("port".to_string(), json!(port_val));
    }

    let response = client
        .post_raw(
            &format!("/subscriptions/{}/databases", subscription_id),
            request,
        )
        .await
        .context("Failed to create database")?;

    handle_async_response(
        conn_mgr,
        profile_name,
        response,
        async_ops,
        output_format,
        query,
        "Database created successfully",
    )
    .await
}

/// Update database configuration
pub async fn update_database(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: &str,
    data: &str,
    async_ops: &AsyncOperationArgs,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let (subscription_id, database_id) = parse_database_id(id)?;
    let client = conn_mgr.create_cloud_client(profile_name).await?;
    let request = read_json_data(data)?;

    let response = client
        .put_raw(
            &format!(
                "/subscriptions/{}/databases/{}",
                subscription_id, database_id
            ),
            request,
        )
        .await
        .context("Failed to update database")?;

    handle_async_response(
        conn_mgr,
        profile_name,
        response,
        async_ops,
        output_format,
        query,
        "Database updated successfully",
    )
    .await
}

/// Delete a database
pub async fn delete_database(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: &str,
    force: bool,
    async_ops: &AsyncOperationArgs,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let (subscription_id, database_id) = parse_database_id(id)?;

    // Confirmation prompt unless --force is used
    if !force {
        use dialoguer::Confirm;
        let confirm = Confirm::new()
            .with_prompt(format!("Are you sure you want to delete database {}?", id))
            .default(false)
            .interact()
            .map_err(|e| RedisCtlError::InvalidInput {
                message: format!("Failed to read confirmation: {}", e),
            })?;

        if !confirm {
            println!("Database deletion cancelled");
            return Ok(());
        }
    }

    let client = conn_mgr.create_cloud_client(profile_name).await?;

    let response = client
        .delete_raw(&format!(
            "/subscriptions/{}/databases/{}",
            subscription_id, database_id
        ))
        .await
        .context("Failed to delete database")?;

    handle_async_response(
        conn_mgr,
        profile_name,
        response,
        async_ops,
        output_format,
        query,
        "Database deletion initiated",
    )
    .await
}

/// Get database backup status
pub async fn get_backup_status(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let (subscription_id, database_id) = parse_database_id(id)?;
    let client = conn_mgr.create_cloud_client(profile_name).await?;

    let response = client
        .get_raw(&format!(
            "/subscriptions/{}/databases/{}/backup-status",
            subscription_id, database_id
        ))
        .await
        .context("Failed to get backup status")?;

    let result = if let Some(q) = query {
        apply_jmespath(&response, q)?
    } else {
        response
    };

    match output_format {
        OutputFormat::Table => {
            if let Some(status) = result.get("status") {
                println!(
                    "Backup Status: {}",
                    format_status_text(status.as_str().unwrap_or(""))
                );
            }
            if let Some(last_backup) = result.get("lastBackupTime") {
                println!(
                    "Last Backup: {}",
                    format_date(last_backup.as_str().unwrap_or("").to_string())
                );
            }
        }
        _ => print_json_or_yaml(result, output_format)?,
    }

    Ok(())
}

/// Trigger manual database backup
pub async fn backup_database(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: &str,
    async_ops: &AsyncOperationArgs,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let (subscription_id, database_id) = parse_database_id(id)?;
    let client = conn_mgr.create_cloud_client(profile_name).await?;

    let response = client
        .post_raw(
            &format!(
                "/subscriptions/{}/databases/{}/backup",
                subscription_id, database_id
            ),
            json!({}),
        )
        .await
        .context("Failed to trigger backup")?;

    handle_async_response(
        conn_mgr,
        profile_name,
        response,
        async_ops,
        output_format,
        query,
        "Backup initiated successfully",
    )
    .await
}

/// Get database import status
pub async fn get_import_status(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let (subscription_id, database_id) = parse_database_id(id)?;
    let client = conn_mgr.create_cloud_client(profile_name).await?;

    let response = client
        .get_raw(&format!(
            "/subscriptions/{}/databases/{}/import-status",
            subscription_id, database_id
        ))
        .await
        .context("Failed to get import status")?;

    let result = if let Some(q) = query {
        apply_jmespath(&response, q)?
    } else {
        response
    };

    match output_format {
        OutputFormat::Table => {
            if let Some(status) = result.get("status") {
                println!(
                    "Import Status: {}",
                    format_status_text(status.as_str().unwrap_or(""))
                );
            }
            if let Some(progress) = result.get("progress") {
                println!("Progress: {}%", progress);
            }
        }
        _ => print_json_or_yaml(result, output_format)?,
    }

    Ok(())
}

/// Import data into database
pub async fn import_database(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: &str,
    data: &str,
    async_ops: &AsyncOperationArgs,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let (subscription_id, database_id) = parse_database_id(id)?;
    let client = conn_mgr.create_cloud_client(profile_name).await?;
    let request = read_json_data(data)?;

    let response = client
        .post_raw(
            &format!(
                "/subscriptions/{}/databases/{}/import",
                subscription_id, database_id
            ),
            request,
        )
        .await
        .context("Failed to start import")?;

    handle_async_response(
        conn_mgr,
        profile_name,
        response,
        async_ops,
        output_format,
        query,
        "Import initiated successfully",
    )
    .await
}

/// Get database certificate
pub async fn get_certificate(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let (subscription_id, database_id) = parse_database_id(id)?;
    let client = conn_mgr.create_cloud_client(profile_name).await?;

    let response = client
        .get_raw(&format!(
            "/subscriptions/{}/databases/{}/certificate",
            subscription_id, database_id
        ))
        .await
        .context("Failed to get certificate")?;

    let result = if let Some(q) = query {
        apply_jmespath(&response, q)?
    } else {
        response
    };

    match output_format {
        OutputFormat::Table => {
            if let Some(cert) = result.get("certificate") {
                println!("{}", cert.as_str().unwrap_or(""));
            } else {
                println!("No certificate available");
            }
        }
        _ => print_json_or_yaml(result, output_format)?,
    }

    Ok(())
}

/// Slow log entry for table display
#[derive(Tabled)]
struct SlowLogEntry {
    #[tabled(rename = "TIMESTAMP")]
    timestamp: String,
    #[tabled(rename = "DURATION (ms)")]
    duration: String,
    #[tabled(rename = "COMMAND")]
    command: String,
    #[tabled(rename = "CLIENT")]
    client: String,
}

/// Get slow query log
pub async fn get_slow_log(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: &str,
    limit: u32,
    offset: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let (subscription_id, database_id) = parse_database_id(id)?;
    let client = conn_mgr.create_cloud_client(profile_name).await?;

    let response = client
        .get_raw(&format!(
            "/subscriptions/{}/databases/{}/slowlog?limit={}&offset={}",
            subscription_id, database_id, limit, offset
        ))
        .await
        .context("Failed to get slow log")?;

    let result = if let Some(q) = query {
        apply_jmespath(&response, q)?
    } else {
        response
    };

    match output_format {
        OutputFormat::Table => {
            let mut entries = Vec::new();

            if let Some(Value::Array(logs)) = result.get("entries") {
                for entry in logs {
                    entries.push(SlowLogEntry {
                        timestamp: format_date(extract_field(entry, "timestamp", "")),
                        duration: extract_field(entry, "duration", ""),
                        command: truncate_string(&extract_field(entry, "command", ""), 50),
                        client: extract_field(entry, "client", ""),
                    });
                }
            }

            if entries.is_empty() {
                println!("No slow log entries found");
            } else {
                let mut table = Table::new(entries);
                table.with(Style::modern());
                output_with_pager(&table.to_string());
            }
        }
        _ => print_json_or_yaml(result, output_format)?,
    }

    Ok(())
}

/// Tag entry for table display
#[derive(Tabled)]
struct TagEntry {
    #[tabled(rename = "KEY")]
    key: String,
    #[tabled(rename = "VALUE")]
    value: String,
}

/// List database tags
pub async fn list_tags(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let (subscription_id, database_id) = parse_database_id(id)?;
    let client = conn_mgr.create_cloud_client(profile_name).await?;

    let response = client
        .get_raw(&format!(
            "/subscriptions/{}/databases/{}/tags",
            subscription_id, database_id
        ))
        .await
        .context("Failed to get tags")?;

    let result = if let Some(q) = query {
        apply_jmespath(&response, q)?
    } else {
        response
    };

    match output_format {
        OutputFormat::Table => {
            let mut entries = Vec::new();

            if let Some(Value::Object(tags)) = result.get("tags") {
                for (key, value) in tags {
                    entries.push(TagEntry {
                        key: key.clone(),
                        value: value.as_str().unwrap_or("").to_string(),
                    });
                }
            }

            if entries.is_empty() {
                println!("No tags found");
            } else {
                let mut table = Table::new(entries);
                table.with(Style::modern());
                println!("{}", table);
            }
        }
        _ => print_json_or_yaml(result, output_format)?,
    }

    Ok(())
}

/// Add a tag to database
pub async fn add_tag(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: &str,
    key: &str,
    value: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let (subscription_id, database_id) = parse_database_id(id)?;
    let client = conn_mgr.create_cloud_client(profile_name).await?;

    let request = json!({
        "key": key,
        "value": value
    });

    let response = client
        .post_raw(
            &format!(
                "/subscriptions/{}/databases/{}/tags",
                subscription_id, database_id
            ),
            request,
        )
        .await
        .context("Failed to add tag")?;

    let result = if let Some(q) = query {
        apply_jmespath(&response, q)?
    } else {
        response
    };

    match output_format {
        OutputFormat::Table => {
            println!("Tag added successfully: {} = {}", key, value);
        }
        _ => print_json_or_yaml(result, output_format)?,
    }

    Ok(())
}

/// Update database tags
pub async fn update_tags(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: &str,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let (subscription_id, database_id) = parse_database_id(id)?;
    let client = conn_mgr.create_cloud_client(profile_name).await?;
    let request = read_json_data(data)?;

    let response = client
        .put_raw(
            &format!(
                "/subscriptions/{}/databases/{}/tags",
                subscription_id, database_id
            ),
            request,
        )
        .await
        .context("Failed to update tags")?;

    let result = if let Some(q) = query {
        apply_jmespath(&response, q)?
    } else {
        response
    };

    match output_format {
        OutputFormat::Table => {
            println!("Tags updated successfully");
        }
        _ => print_json_or_yaml(result, output_format)?,
    }

    Ok(())
}

/// Delete a tag from database
pub async fn delete_tag(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: &str,
    key: &str,
    output_format: OutputFormat,
    _query: Option<&str>,
) -> CliResult<()> {
    let (subscription_id, database_id) = parse_database_id(id)?;
    let client = conn_mgr.create_cloud_client(profile_name).await?;

    client
        .delete_raw(&format!(
            "/subscriptions/{}/databases/{}/tags/{}",
            subscription_id, database_id, key
        ))
        .await
        .context("Failed to delete tag")?;

    match output_format {
        OutputFormat::Table => {
            println!("Tag '{}' deleted successfully", key);
        }
        _ => {
            let result = json!({"message": format!("Tag '{}' deleted", key)});
            print_json_or_yaml(result, output_format)?;
        }
    }

    Ok(())
}

/// Flush standard (non-Active-Active) database
pub async fn flush_database(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: &str,
    force: bool,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let (subscription_id, database_id) = parse_database_id(id)?;

    // Confirmation prompt unless --force is used
    if !force {
        use dialoguer::Confirm;
        let confirm = Confirm::new()
            .with_prompt(format!(
                "Are you sure you want to flush database {}? This will delete all data!",
                id
            ))
            .default(false)
            .interact()
            .map_err(|e| RedisCtlError::InvalidInput {
                message: format!("Failed to read confirmation: {}", e),
            })?;

        if !confirm {
            println!("Flush operation cancelled");
            return Ok(());
        }
    }

    let client = conn_mgr.create_cloud_client(profile_name).await?;

    let response = client
        .put_raw(
            &format!(
                "/subscriptions/{}/databases/{}/flush",
                subscription_id, database_id
            ),
            json!({}),
        )
        .await
        .context("Failed to flush database")?;

    let result = if let Some(q) = query {
        apply_jmespath(&response, q)?
    } else {
        response
    };

    match output_format {
        OutputFormat::Table => {
            println!("Database flush initiated");
            if let Some(task_id) = result.get("taskId") {
                println!("Task ID: {}", task_id);
            }
        }
        _ => print_json_or_yaml(result, output_format)?,
    }

    Ok(())
}

/// Get available Redis versions for upgrade
pub async fn get_available_versions(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let (subscription_id, database_id) = parse_database_id(id)?;
    let client = conn_mgr.create_cloud_client(profile_name).await?;

    let response = client
        .get_raw(&format!(
            "/subscriptions/{}/databases/{}/available-target-versions",
            subscription_id, database_id
        ))
        .await
        .context("Failed to get available versions")?;

    let result = if let Some(q) = query {
        apply_jmespath(&response, q)?
    } else {
        response
    };

    match output_format {
        OutputFormat::Table => {
            if let Some(versions) = result.as_array() {
                if versions.is_empty() {
                    println!("No upgrade versions available");
                } else {
                    println!("Available Redis versions for upgrade:");
                    for v in versions {
                        if let Some(version) = v.as_str() {
                            println!("  - {}", version);
                        } else {
                            println!("  - {}", v);
                        }
                    }
                }
            } else {
                print_json_or_yaml(result, output_format)?;
            }
        }
        _ => print_json_or_yaml(result, output_format)?,
    }

    Ok(())
}

/// Flush Active-Active database
pub async fn flush_crdb(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: &str,
    force: bool,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let (subscription_id, database_id) = parse_database_id(id)?;

    // Confirmation prompt unless --force is used
    if !force {
        use dialoguer::Confirm;
        let confirm = Confirm::new()
            .with_prompt(format!("Are you sure you want to flush Active-Active database {}? This will delete all data!", id))
            .default(false)
            .interact()
            .map_err(|e| RedisCtlError::InvalidInput {
                message: format!("Failed to read confirmation: {}", e),
            })?;

        if !confirm {
            println!("Flush operation cancelled");
            return Ok(());
        }
    }

    let client = conn_mgr.create_cloud_client(profile_name).await?;

    let response = client
        .post_raw(
            &format!(
                "/subscriptions/{}/databases/{}/flush",
                subscription_id, database_id
            ),
            json!({}),
        )
        .await
        .context("Failed to flush database")?;

    let result = if let Some(q) = query {
        apply_jmespath(&response, q)?
    } else {
        response
    };

    match output_format {
        OutputFormat::Table => {
            println!("Active-Active database flush initiated");
            if let Some(task_id) = result.get("taskId") {
                println!("Task ID: {}", task_id);
            }
        }
        _ => print_json_or_yaml(result, output_format)?,
    }

    Ok(())
}

/// Update Active-Active database regions
pub async fn update_aa_regions(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: &str,
    file: &str,
    async_ops: &AsyncOperationArgs,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let (subscription_id, database_id) = parse_database_id(id)?;
    let client = conn_mgr.create_cloud_client(profile_name).await?;

    // Read the request body from file
    let file_content = read_file_input(file)?;
    let request_body: Value =
        serde_json::from_str(&file_content).context("Failed to parse JSON input")?;

    let response = client
        .put_raw(
            &format!(
                "/subscriptions/{}/databases/{}/regions",
                subscription_id, database_id
            ),
            request_body,
        )
        .await
        .context("Failed to update Active-Active database regions")?;

    handle_async_response(
        conn_mgr,
        profile_name,
        response,
        async_ops,
        output_format,
        query,
        "Update AA regions",
    )
    .await
}

/// Get Redis version upgrade status
pub async fn get_upgrade_status(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let (subscription_id, database_id) = parse_database_id(id)?;
    let client = conn_mgr.create_cloud_client(profile_name).await?;

    let response = client
        .get_raw(&format!(
            "/subscriptions/{}/databases/{}/redis-version-upgrade-status",
            subscription_id, database_id
        ))
        .await
        .context("Failed to get upgrade status")?;

    let result = if let Some(q) = query {
        apply_jmespath(&response, q)?
    } else {
        response
    };

    match output_format {
        OutputFormat::Table => {
            if let Some(status) = result.get("status") {
                println!(
                    "Upgrade Status: {}",
                    format_status_text(status.as_str().unwrap_or(""))
                );
            }
            if let Some(current) = result.get("currentVersion") {
                println!("Current Version: {}", current);
            }
            if let Some(target) = result.get("targetVersion") {
                println!("Target Version: {}", target);
            }
        }
        _ => print_json_or_yaml(result, output_format)?,
    }

    Ok(())
}

/// Upgrade Redis version
pub async fn upgrade_redis(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: &str,
    version: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let (subscription_id, database_id) = parse_database_id(id)?;
    let client = conn_mgr.create_cloud_client(profile_name).await?;

    let request = json!({
        "redisVersion": version
    });

    let response = client
        .post_raw(
            &format!(
                "/subscriptions/{}/databases/{}/upgrade-redis-version",
                subscription_id, database_id
            ),
            request,
        )
        .await
        .context("Failed to upgrade Redis version")?;

    let result = if let Some(q) = query {
        apply_jmespath(&response, q)?
    } else {
        response
    };

    match output_format {
        OutputFormat::Table => {
            println!("Redis version upgrade initiated to {}", version);
            if let Some(task_id) = result.get("taskId") {
                println!("Task ID: {}", task_id);
            }
        }
        _ => print_json_or_yaml(result, output_format)?,
    }

    Ok(())
}
