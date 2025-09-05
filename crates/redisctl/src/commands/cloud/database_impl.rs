//! Implementation of additional database commands

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

/// Create a new database
pub async fn create_database(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    subscription_id: u32,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_cloud_client(profile_name).await?;
    let request = read_json_data(data)?;

    let response = client
        .post_raw(
            &format!("/subscriptions/{}/databases", subscription_id),
            request,
        )
        .await
        .context("Failed to create database")?;

    let result = if let Some(q) = query {
        apply_jmespath(&response, q)?
    } else {
        response
    };

    match output_format {
        OutputFormat::Table => {
            println!("Database created successfully");
            if let Some(task_id) = result.get("taskId") {
                println!("Task ID: {}", task_id);
            }
        }
        _ => print_json_or_yaml(result, output_format)?,
    }

    Ok(())
}

/// Update database configuration
pub async fn update_database(
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
                "/subscriptions/{}/databases/{}",
                subscription_id, database_id
            ),
            request,
        )
        .await
        .context("Failed to update database")?;

    let result = if let Some(q) = query {
        apply_jmespath(&response, q)?
    } else {
        response
    };

    match output_format {
        OutputFormat::Table => {
            println!("Database updated successfully");
            if let Some(task_id) = result.get("taskId") {
                println!("Task ID: {}", task_id);
            }
        }
        _ => print_json_or_yaml(result, output_format)?,
    }

    Ok(())
}

/// Delete a database
pub async fn delete_database(
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

    let result = if let Some(q) = query {
        apply_jmespath(&response, q)?
    } else {
        response
    };

    match output_format {
        OutputFormat::Table => {
            println!("Database deletion initiated");
            if let Some(task_id) = result.get("taskId") {
                println!("Task ID: {}", task_id);
            }
        }
        _ => print_json_or_yaml(result, output_format)?,
    }

    Ok(())
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

    let result = if let Some(q) = query {
        apply_jmespath(&response, q)?
    } else {
        response
    };

    match output_format {
        OutputFormat::Table => {
            println!("Backup initiated successfully");
            if let Some(task_id) = result.get("taskId") {
                println!("Task ID: {}", task_id);
            }
        }
        _ => print_json_or_yaml(result, output_format)?,
    }

    Ok(())
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

    let result = if let Some(q) = query {
        apply_jmespath(&response, q)?
    } else {
        response
    };

    match output_format {
        OutputFormat::Table => {
            println!("Import initiated successfully");
            if let Some(task_id) = result.get("taskId") {
                println!("Task ID: {}", task_id);
            }
        }
        _ => print_json_or_yaml(result, output_format)?,
    }

    Ok(())
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
