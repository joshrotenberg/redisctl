//! Implementation of additional subscription commands

use super::utils::*;
use crate::cli::OutputFormat;
use crate::connection::ConnectionManager;
use crate::error::{RedisCtlError, Result as CliResult};
use crate::output::print_output;
use anyhow::Context;
use serde_json::Value;
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

/// Create a new subscription
pub async fn create_subscription(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_cloud_client(profile_name).await?;
    let request = read_json_data(data)?;

    let response = client
        .post_raw("/subscriptions", request)
        .await
        .context("Failed to create subscription")?;

    let result = if let Some(q) = query {
        apply_jmespath(&response, q)?
    } else {
        response
    };

    match output_format {
        OutputFormat::Table => {
            println!("Subscription created successfully");
            if let Some(task_id) = result.get("taskId") {
                println!("Task ID: {}", task_id);
            }
            if let Some(sub_id) = result.get("resourceId") {
                println!("Subscription ID: {}", sub_id);
            }
        }
        _ => print_json_or_yaml(result, output_format)?,
    }

    Ok(())
}

/// Update subscription configuration
pub async fn update_subscription(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_cloud_client(profile_name).await?;
    let request = read_json_data(data)?;

    let response = client
        .put_raw(&format!("/subscriptions/{}", id), request)
        .await
        .context("Failed to update subscription")?;

    let result = if let Some(q) = query {
        apply_jmespath(&response, q)?
    } else {
        response
    };

    match output_format {
        OutputFormat::Table => {
            println!("Subscription updated successfully");
            if let Some(task_id) = result.get("taskId") {
                println!("Task ID: {}", task_id);
            }
        }
        _ => print_json_or_yaml(result, output_format)?,
    }

    Ok(())
}

/// Delete a subscription
pub async fn delete_subscription(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    force: bool,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    // Confirmation prompt unless --force is used
    if !force {
        use dialoguer::Confirm;
        let confirm = Confirm::new()
            .with_prompt(format!("Are you sure you want to delete subscription {}? This will delete all databases in the subscription!", id))
            .default(false)
            .interact()
            .map_err(|e| RedisCtlError::InvalidInput {
                message: format!("Failed to read confirmation: {}", e),
            })?;

        if !confirm {
            println!("Subscription deletion cancelled");
            return Ok(());
        }
    }

    let client = conn_mgr.create_cloud_client(profile_name).await?;

    let response = client
        .delete_raw(&format!("/subscriptions/{}", id))
        .await
        .context("Failed to delete subscription")?;

    let result = if let Some(q) = query {
        apply_jmespath(&response, q)?
    } else {
        response
    };

    match output_format {
        OutputFormat::Table => {
            println!("Subscription deletion initiated");
            if let Some(task_id) = result.get("taskId") {
                println!("Task ID: {}", task_id);
            }
        }
        _ => print_json_or_yaml(result, output_format)?,
    }

    Ok(())
}

/// Redis version info for table display
#[derive(Tabled)]
struct RedisVersionRow {
    #[tabled(rename = "VERSION")]
    version: String,
    #[tabled(rename = "RELEASE DATE")]
    release_date: String,
    #[tabled(rename = "END OF LIFE")]
    end_of_life: String,
}

/// Get available Redis versions
pub async fn get_redis_versions(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    subscription_id: Option<u32>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_cloud_client(profile_name).await?;

    let path = if let Some(sub_id) = subscription_id {
        format!("/redis-versions?subscription={}", sub_id)
    } else {
        "/redis-versions".to_string()
    };

    let response = client
        .get_raw(&path)
        .await
        .context("Failed to get Redis versions")?;

    let result = if let Some(q) = query {
        apply_jmespath(&response, q)?
    } else {
        response
    };

    match output_format {
        OutputFormat::Table => {
            let mut rows = Vec::new();

            if let Some(Value::Array(versions)) = result.get("versions") {
                for version in versions {
                    rows.push(RedisVersionRow {
                        version: extract_field(version, "version", ""),
                        release_date: format_date(extract_field(version, "releaseDate", "")),
                        end_of_life: format_date(extract_field(version, "endOfLife", "")),
                    });
                }
            }

            if rows.is_empty() {
                println!("No Redis versions found");
            } else {
                let mut table = Table::new(rows);
                table.with(Style::modern());
                println!("{}", table);
            }
        }
        _ => print_json_or_yaml(result, output_format)?,
    }

    Ok(())
}

/// Get subscription pricing
pub async fn get_pricing(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_cloud_client(profile_name).await?;

    let response = client
        .get_raw(&format!("/subscriptions/{}/pricing", id))
        .await
        .context("Failed to get subscription pricing")?;

    let result = if let Some(q) = query {
        apply_jmespath(&response, q)?
    } else {
        response
    };

    match output_format {
        OutputFormat::Table => {
            if let Some(price) = result.get("estimatedMonthlyTotal") {
                println!("Estimated Monthly Total: ${}", price);
            }
            if let Some(currency) = result.get("currency") {
                println!("Currency: {}", currency);
            }
            if let Some(details) = result.get("shards") {
                println!(
                    "Shard Pricing Details: {}",
                    serde_json::to_string_pretty(details)?
                );
            }
        }
        _ => print_json_or_yaml(result, output_format)?,
    }

    Ok(())
}

/// CIDR entry for table display
#[derive(Tabled)]
struct CidrEntry {
    #[tabled(rename = "CIDR")]
    cidr: String,
    #[tabled(rename = "DESCRIPTION")]
    description: String,
}

/// Get CIDR allowlist
pub async fn get_cidr_allowlist(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_cloud_client(profile_name).await?;

    let response = client
        .get_raw(&format!("/subscriptions/{}/cidr", id))
        .await
        .context("Failed to get CIDR allowlist")?;

    let result = if let Some(q) = query {
        apply_jmespath(&response, q)?
    } else {
        response
    };

    match output_format {
        OutputFormat::Table => {
            let mut entries = Vec::new();

            if let Some(Value::Array(cidrs)) = result.get("cidrs") {
                for cidr in cidrs {
                    entries.push(CidrEntry {
                        cidr: extract_field(cidr, "cidr", ""),
                        description: extract_field(cidr, "description", ""),
                    });
                }
            }

            if entries.is_empty() {
                println!("No CIDR blocks configured");
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

/// Update CIDR allowlist
pub async fn update_cidr_allowlist(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    cidrs: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_cloud_client(profile_name).await?;
    let request = read_json_data(cidrs)?;

    let response = client
        .put_raw(&format!("/subscriptions/{}/cidr", id), request)
        .await
        .context("Failed to update CIDR allowlist")?;

    let result = if let Some(q) = query {
        apply_jmespath(&response, q)?
    } else {
        response
    };

    match output_format {
        OutputFormat::Table => {
            println!("CIDR allowlist updated successfully");
            if let Some(task_id) = result.get("taskId") {
                println!("Task ID: {}", task_id);
            }
        }
        _ => print_json_or_yaml(result, output_format)?,
    }

    Ok(())
}

/// Maintenance window for table display
#[derive(Tabled)]
struct MaintenanceWindowRow {
    #[tabled(rename = "MODE")]
    mode: String,
    #[tabled(rename = "WINDOW")]
    window: String,
}

/// Get maintenance windows
pub async fn get_maintenance_windows(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_cloud_client(profile_name).await?;

    let response = client
        .get_raw(&format!("/subscriptions/{}/maintenance-windows", id))
        .await
        .context("Failed to get maintenance windows")?;

    let result = if let Some(q) = query {
        apply_jmespath(&response, q)?
    } else {
        response
    };

    match output_format {
        OutputFormat::Table => {
            let mut rows = Vec::new();

            if let Some(mode) = result.get("mode") {
                let window_text = if let Some(windows) = result.get("windows") {
                    serde_json::to_string(windows).unwrap_or_else(|_| "N/A".to_string())
                } else {
                    "N/A".to_string()
                };

                rows.push(MaintenanceWindowRow {
                    mode: mode.as_str().unwrap_or("").to_string(),
                    window: window_text,
                });
            }

            if rows.is_empty() {
                println!("No maintenance windows configured");
            } else {
                let mut table = Table::new(rows);
                table.with(Style::modern());
                println!("{}", table);
            }
        }
        _ => print_json_or_yaml(result, output_format)?,
    }

    Ok(())
}

/// Update maintenance windows
pub async fn update_maintenance_windows(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_cloud_client(profile_name).await?;
    let request = read_json_data(data)?;

    let response = client
        .put_raw(
            &format!("/subscriptions/{}/maintenance-windows", id),
            request,
        )
        .await
        .context("Failed to update maintenance windows")?;

    let result = if let Some(q) = query {
        apply_jmespath(&response, q)?
    } else {
        response
    };

    match output_format {
        OutputFormat::Table => {
            println!("Maintenance windows updated successfully");
            if let Some(task_id) = result.get("taskId") {
                println!("Task ID: {}", task_id);
            }
        }
        _ => print_json_or_yaml(result, output_format)?,
    }

    Ok(())
}

/// Active-Active region for table display
#[derive(Tabled)]
struct AaRegionRow {
    #[tabled(rename = "REGION")]
    region: String,
    #[tabled(rename = "PROVIDER")]
    provider: String,
    #[tabled(rename = "STATUS")]
    status: String,
}

/// List Active-Active regions
pub async fn list_aa_regions(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_cloud_client(profile_name).await?;

    let response = client
        .get_raw(&format!("/subscriptions/{}/regions", id))
        .await
        .context("Failed to get Active-Active regions")?;

    let result = if let Some(q) = query {
        apply_jmespath(&response, q)?
    } else {
        response
    };

    match output_format {
        OutputFormat::Table => {
            let mut rows = Vec::new();

            if let Some(Value::Array(regions)) = result.get("regions") {
                for region in regions {
                    rows.push(AaRegionRow {
                        region: extract_field(region, "region", ""),
                        provider: extract_field(region, "provider", ""),
                        status: format_status_text(&extract_field(region, "status", "")),
                    });
                }
            }

            if rows.is_empty() {
                println!("No Active-Active regions found");
            } else {
                let mut table = Table::new(rows);
                table.with(Style::modern());
                println!("{}", table);
            }
        }
        _ => print_json_or_yaml(result, output_format)?,
    }

    Ok(())
}

/// Add region to Active-Active subscription
pub async fn add_aa_region(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_cloud_client(profile_name).await?;
    let request = read_json_data(data)?;

    let response = client
        .post_raw(&format!("/subscriptions/{}/regions", id), request)
        .await
        .context("Failed to add Active-Active region")?;

    let result = if let Some(q) = query {
        apply_jmespath(&response, q)?
    } else {
        response
    };

    match output_format {
        OutputFormat::Table => {
            println!("Active-Active region added successfully");
            if let Some(task_id) = result.get("taskId") {
                println!("Task ID: {}", task_id);
            }
        }
        _ => print_json_or_yaml(result, output_format)?,
    }

    Ok(())
}

/// Delete regions from Active-Active subscription
pub async fn delete_aa_regions(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    regions: &str,
    force: bool,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    // Confirmation prompt unless --force is used
    if !force {
        use dialoguer::Confirm;
        let confirm = Confirm::new()
            .with_prompt(format!(
                "Are you sure you want to delete regions from Active-Active subscription {}?",
                id
            ))
            .default(false)
            .interact()
            .map_err(|e| RedisCtlError::InvalidInput {
                message: format!("Failed to read confirmation: {}", e),
            })?;

        if !confirm {
            println!("Region deletion cancelled");
            return Ok(());
        }
    }

    let client = conn_mgr.create_cloud_client(profile_name).await?;
    let _request = read_json_data(regions)?;

    let response = client
        .delete_raw(&format!("/subscriptions/{}/regions", id))
        .await
        .context("Failed to delete Active-Active regions")?;

    let result = if let Some(q) = query {
        apply_jmespath(&response, q)?
    } else {
        response
    };

    match output_format {
        OutputFormat::Table => {
            println!("Active-Active regions deletion initiated");
            if let Some(task_id) = result.get("taskId") {
                println!("Task ID: {}", task_id);
            }
        }
        _ => print_json_or_yaml(result, output_format)?,
    }

    Ok(())
}
