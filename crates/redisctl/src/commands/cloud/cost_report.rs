//! Cost report command implementations
//!
//! Handles generating and downloading cost reports in FOCUS format.

#![allow(dead_code)] // Functions used from main.rs binary

use crate::cli::{CloudCostReportCommands, OutputFormat};
use crate::commands::cloud::async_utils::{AsyncOperationArgs, handle_async_response};
use crate::connection::ConnectionManager;
use crate::error::Result as CliResult;
use anyhow::Context;
use redis_cloud::cost_report::{CostReportCreateRequest, CostReportFormat, SubscriptionType, Tag};
use serde_json::json;
use std::io::Write;

/// Handle cost report commands
pub async fn handle_cost_report_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    command: CloudCostReportCommands,
    output_format: OutputFormat,
) -> CliResult<()> {
    match command {
        CloudCostReportCommands::Generate {
            start_date,
            end_date,
            format,
            subscription_ids,
            database_ids,
            subscription_type,
            regions,
            tags,
            async_ops,
        } => {
            generate_cost_report(
                conn_mgr,
                profile_name,
                start_date,
                end_date,
                format,
                subscription_ids,
                database_ids,
                subscription_type,
                regions,
                tags,
                async_ops,
                output_format,
            )
            .await
        }
        CloudCostReportCommands::Download {
            cost_report_id,
            file,
        } => {
            download_cost_report(conn_mgr, profile_name, cost_report_id, file, output_format).await
        }
    }
}

/// Generate a cost report
#[allow(clippy::too_many_arguments)]
async fn generate_cost_report(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    start_date: String,
    end_date: String,
    format: String,
    subscription_ids: Vec<i32>,
    database_ids: Vec<i32>,
    subscription_type: Option<String>,
    regions: Vec<String>,
    tags: Vec<String>,
    async_ops: AsyncOperationArgs,
    output_format: OutputFormat,
) -> CliResult<()> {
    let client = conn_mgr.create_cloud_client(profile_name).await?;

    // Build the request
    let mut request = CostReportCreateRequest::new(&start_date, &end_date);

    // Set format
    request.format = Some(match format.as_str() {
        "json" => CostReportFormat::Json,
        _ => CostReportFormat::Csv,
    });

    // Set subscription IDs if provided
    if !subscription_ids.is_empty() {
        request.subscription_ids = Some(subscription_ids);
    }

    // Set database IDs if provided
    if !database_ids.is_empty() {
        request.database_ids = Some(database_ids);
    }

    // Set subscription type if provided
    if let Some(sub_type) = subscription_type {
        request.subscription_type = Some(match sub_type.as_str() {
            "essentials" => SubscriptionType::Essentials,
            _ => SubscriptionType::Pro,
        });
    }

    // Set regions if provided
    if !regions.is_empty() {
        request.regions = Some(regions);
    }

    // Parse and set tags if provided
    if !tags.is_empty() {
        let parsed_tags: Vec<Tag> = tags
            .iter()
            .filter_map(|t| {
                let parts: Vec<&str> = t.splitn(2, ':').collect();
                if parts.len() == 2 {
                    Some(Tag::new(parts[0], parts[1]))
                } else {
                    eprintln!("Warning: Invalid tag format '{}', expected 'key:value'", t);
                    None
                }
            })
            .collect();
        if !parsed_tags.is_empty() {
            request.tags = Some(parsed_tags);
        }
    }

    // Convert to JSON for the raw API call
    let body = serde_json::to_value(&request).context("Failed to serialize request")?;

    // Make the API call
    let response = client
        .post_raw("/cost-report", body)
        .await
        .context("Failed to generate cost report")?;

    // Handle async response
    handle_async_response(
        conn_mgr,
        profile_name,
        response,
        &async_ops,
        output_format,
        None,
        "Cost report generation",
    )
    .await
}

/// Download a generated cost report
async fn download_cost_report(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    cost_report_id: String,
    output: Option<String>,
    output_format: OutputFormat,
) -> CliResult<()> {
    let client = conn_mgr.create_cloud_client(profile_name).await?;

    let bytes = client
        .get_bytes(&format!("/cost-report/{}", cost_report_id))
        .await?;

    match output {
        Some(path) => {
            // Write to file
            std::fs::write(&path, &bytes)
                .with_context(|| format!("Failed to write cost report to '{}'", path))?;

            match output_format {
                OutputFormat::Json => {
                    let result = json!({
                        "success": true,
                        "cost_report_id": cost_report_id,
                        "output_file": path,
                        "bytes_written": bytes.len(),
                    });
                    println!("{}", serde_json::to_string_pretty(&result)?);
                }
                _ => {
                    println!(
                        "Cost report downloaded successfully to '{}' ({} bytes)",
                        path,
                        bytes.len()
                    );
                }
            }
        }
        None => {
            // Write to stdout
            match output_format {
                OutputFormat::Json => {
                    // For JSON output format, try to parse the content as JSON
                    // If it's CSV, wrap it in a JSON structure
                    if let Ok(json_content) = serde_json::from_slice::<serde_json::Value>(&bytes) {
                        println!("{}", serde_json::to_string_pretty(&json_content)?);
                    } else {
                        // It's probably CSV, wrap in JSON
                        let content = String::from_utf8_lossy(&bytes);
                        let result = json!({
                            "cost_report_id": cost_report_id,
                            "format": "csv",
                            "content": content,
                        });
                        println!("{}", serde_json::to_string_pretty(&result)?);
                    }
                }
                _ => {
                    // Write raw content to stdout
                    std::io::stdout()
                        .write_all(&bytes)
                        .context("Failed to write cost report to stdout")?;
                }
            }
        }
    }

    Ok(())
}
