//! Cloud-specific command implementations with human-friendly output

#![allow(dead_code)] // Used by binary target

use crate::cli::{CloudSubscriptionCommands, OutputFormat};
use crate::connection::ConnectionManager;
use crate::error::Result as CliResult;
use crate::output::print_output;
use anyhow::Context;
use chrono::{DateTime, Utc};
use colored::Colorize;
use comfy_table::{
    Cell, ContentArrangement, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL,
};
use serde_json::Value;

/// Handle cloud subscription commands
pub async fn handle_subscription_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    command: &CloudSubscriptionCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    match command {
        CloudSubscriptionCommands::List => {
            list_subscriptions(conn_mgr, profile_name, output_format, query).await
        }
    }
}

/// List all cloud subscriptions with human-friendly output
async fn list_subscriptions(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_cloud_client(profile_name).await?;

    // Get raw subscription data
    let response = client
        .get_raw("/subscriptions")
        .await
        .context("Failed to fetch subscriptions")?;

    // Apply JMESPath query if provided
    let data = if let Some(q) = query {
        apply_jmespath(&response, q)?
    } else {
        response
    };

    // Format output based on requested format
    match output_format {
        OutputFormat::Auto | OutputFormat::Table => {
            print_subscriptions_table(&data)?;
        }
        OutputFormat::Json => {
            print_output(data, crate::output::OutputFormat::Json, None)?;
        }
        OutputFormat::Yaml => {
            print_output(data, crate::output::OutputFormat::Yaml, None)?;
        }
    }

    Ok(())
}

/// Print subscriptions in a human-friendly table format
fn print_subscriptions_table(data: &Value) -> CliResult<()> {
    // Handle both array response and filtered object
    let subscriptions = match data {
        Value::Array(arr) => arr.clone(),
        Value::Object(_) => vec![data.clone()],
        _ => {
            println!("No subscriptions found");
            return Ok(());
        }
    };

    if subscriptions.is_empty() {
        println!("No subscriptions found");
        return Ok(());
    }

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic);

    // Set headers
    table.set_header(vec![
        "ID",
        "NAME",
        "STATUS",
        "PLAN",
        "MEMORY",
        "DATABASES",
        "REGION",
        "CREATED",
    ]);

    // Add rows
    for sub in subscriptions {
        let id = extract_field(&sub, "id", "—");
        let name = extract_field(&sub, "name", "—");
        let status = format_status(extract_field(&sub, "status", "unknown"));
        let plan = extract_plan_info(&sub);
        let memory = format_memory(&sub);
        let databases = count_databases(&sub);
        let region = extract_region(&sub);
        let created = format_date(extract_field(&sub, "created", ""));

        table.add_row(vec![
            Cell::new(id),
            Cell::new(name),
            Cell::new(status),
            Cell::new(plan),
            Cell::new(memory),
            Cell::new(databases),
            Cell::new(region),
            Cell::new(created),
        ]);
    }

    println!("{}", table);
    Ok(())
}

/// Extract field from JSON value with fallback
fn extract_field(value: &Value, field: &str, default: &str) -> String {
    value
        .get(field)
        .and_then(|v| match v {
            Value::String(s) => Some(s.clone()),
            Value::Number(n) => Some(n.to_string()),
            Value::Bool(b) => Some(b.to_string()),
            _ => None,
        })
        .unwrap_or_else(|| default.to_string())
}

/// Format status with color coding
fn format_status(status: String) -> String {
    match status.to_lowercase().as_str() {
        "active" => status.green().to_string(),
        "pending" => status.yellow().to_string(),
        "error" | "failed" => status.red().to_string(),
        _ => status,
    }
}

/// Extract plan information (Pro/Fixed, pricing info)
fn extract_plan_info(sub: &Value) -> String {
    // Check if it's a fixed or flexible subscription
    if sub.get("planId").is_some() {
        let plan_id = extract_field(sub, "planId", "");
        let plan_name = extract_field(sub, "planName", "Fixed");
        format!("{} ({})", plan_name, plan_id)
    } else if sub.get("paymentMethod").is_some() {
        "Pro".to_string()
    } else {
        "Unknown".to_string()
    }
}

/// Format memory information
fn format_memory(sub: &Value) -> String {
    // Try to get memory from cloudProviders[].regions[].networking
    if let Some(providers) = sub.get("cloudProviders").and_then(|p| p.as_array()) {
        let total_memory_gb: f64 = providers
            .iter()
            .filter_map(|provider| provider.get("regions").and_then(|r| r.as_array()))
            .flatten()
            .filter_map(|region| {
                region
                    .get("memoryStorage")
                    .and_then(|m| m.get("quantity").and_then(|q| q.as_f64()))
            })
            .sum();

        if total_memory_gb > 0.0 {
            return format_memory_size(total_memory_gb);
        }
    }

    // Fallback to memoryStorage field
    if let Some(memory) = sub
        .get("memoryStorage")
        .and_then(|m| m.get("quantity").and_then(|q| q.as_f64()))
    {
        return format_memory_size(memory);
    }

    "—".to_string()
}

/// Format memory size in human-readable format
fn format_memory_size(gb: f64) -> String {
    if gb < 1.0 {
        format!("{:.0}MB", gb * 1024.0)
    } else {
        format!("{:.1}GB", gb)
    }
}

/// Count number of databases in subscription
fn count_databases(sub: &Value) -> String {
    if let Some(dbs) = sub.get("numberOfDatabases").and_then(|n| n.as_u64()) {
        dbs.to_string()
    } else if let Some(dbs) = sub.get("databases").and_then(|d| d.as_array()) {
        dbs.len().to_string()
    } else {
        "0".to_string()
    }
}

/// Extract primary region from subscription
fn extract_region(sub: &Value) -> String {
    // Try to get from cloudProviders[].regions[]
    if let Some(providers) = sub.get("cloudProviders").and_then(|p| p.as_array())
        && let Some(first_provider) = providers.first()
    {
        let provider_name = extract_field(first_provider, "provider", "");
        if let Some(regions) = first_provider.get("regions").and_then(|r| r.as_array())
            && let Some(first_region) = regions.first()
        {
            let region_name = extract_field(first_region, "region", "");
            if !provider_name.is_empty() && !region_name.is_empty() {
                return format!("{}/{}", provider_short_name(&provider_name), region_name);
            }
        }
    }

    "—".to_string()
}

/// Get short provider name for display
fn provider_short_name(provider: &str) -> &str {
    match provider.to_lowercase().as_str() {
        "aws" => "AWS",
        "gcp" | "google" => "GCP",
        "azure" => "Azure",
        _ => provider,
    }
}

/// Format date in human-readable format
fn format_date(date_str: String) -> String {
    if date_str.is_empty() || date_str == "—" {
        return "—".to_string();
    }

    // Try to parse as ISO8601/RFC3339
    if let Ok(dt) = DateTime::parse_from_rfc3339(&date_str) {
        let utc: DateTime<Utc> = dt.into();
        let now = Utc::now();
        let duration = now.signed_duration_since(utc);

        // Show relative time for recent items
        if duration.num_days() == 0 {
            if duration.num_hours() == 0 {
                return format!("{} min ago", duration.num_minutes());
            }
            return format!("{} hours ago", duration.num_hours());
        } else if duration.num_days() < 7 {
            return format!("{} days ago", duration.num_days());
        }

        // Show date for older items
        return utc.format("%Y-%m-%d").to_string();
    }

    // Fallback to original string
    date_str
}

/// Apply JMESPath query to JSON data
fn apply_jmespath(data: &Value, query: &str) -> CliResult<Value> {
    let expr = jmespath::compile(query)
        .with_context(|| format!("Invalid JMESPath expression: {}", query))?;

    let result = expr
        .search(data)
        .with_context(|| format!("Failed to apply JMESPath query: {}", query))?;

    // Convert jmespath Variable to serde_json Value
    let json_str = serde_json::to_string(&result).context("Failed to serialize JMESPath result")?;
    let value =
        serde_json::from_str(&json_str).context("Failed to parse JMESPath result as JSON")?;

    Ok(value)
}
