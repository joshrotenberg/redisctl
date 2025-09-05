//! Cloud account command implementations

#![allow(dead_code)] // Used by binary target

use anyhow::Context;
use serde_json::Value;
use tabled::{Table, settings::Style};

use crate::cli::{CloudAccountCommands, OutputFormat};
use crate::connection::ConnectionManager;
use crate::error::Result as CliResult;

use super::utils::*;

/// Handle cloud account commands
pub async fn handle_account_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    command: &CloudAccountCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    match command {
        CloudAccountCommands::Get => {
            get_account(conn_mgr, profile_name, output_format, query).await
        }
    }
}

/// Get account information
async fn get_account(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_cloud_client(profile_name).await?;

    let response = client
        .get_raw("/")
        .await
        .context("Failed to fetch account")?;

    let data = handle_output(response, output_format, query)?;

    match output_format {
        OutputFormat::Auto | OutputFormat::Table => {
            print_account_table(&data)?;
        }
        _ => print_formatted_output(data, output_format)?,
    }

    Ok(())
}

/// Print account info in clean table format
fn print_account_table(data: &Value) -> CliResult<()> {
    let mut rows = Vec::new();

    // Extract account object from response
    let account = data.get("account").unwrap_or(data);

    // Add account fields in a logical order
    if let Some(id) = account.get("id") {
        rows.push(DetailRow {
            field: "Account ID".to_string(),
            value: id.to_string().trim_matches('"').to_string(),
        });
    }

    if let Some(name) = account.get("name").and_then(|n| n.as_str()) {
        rows.push(DetailRow {
            field: "Name".to_string(),
            value: name.to_string(),
        });
    }

    // Try to get email from key.owner.email or direct email field
    let email = account
        .get("key")
        .and_then(|k| k.get("owner"))
        .and_then(|o| o.get("email"))
        .and_then(|e| e.as_str())
        .or_else(|| account.get("email").and_then(|e| e.as_str()));

    if let Some(email) = email {
        rows.push(DetailRow {
            field: "Email".to_string(),
            value: email.to_string(),
        });
    }

    if let Some(company) = account.get("companyName").and_then(|c| c.as_str()) {
        rows.push(DetailRow {
            field: "Company".to_string(),
            value: company.to_string(),
        });
    }

    // Use createdTimestamp field
    if let Some(created) = account
        .get("createdTimestamp")
        .and_then(|c| c.as_str())
        .or_else(|| account.get("created").and_then(|c| c.as_str()))
    {
        rows.push(DetailRow {
            field: "Created".to_string(),
            value: format_date(created.to_string()),
        });
    }

    if let Some(status) = account.get("status").and_then(|s| s.as_str()) {
        rows.push(DetailRow {
            field: "Status".to_string(),
            value: format_status_text(status),
        });
    }

    if let Some(payment) = account.get("paymentMethods").and_then(|p| p.as_array()) {
        rows.push(DetailRow {
            field: "Payment Methods".to_string(),
            value: format!("{} configured", payment.len()),
        });
    }

    // Add API key info if present
    if let Some(key) = account.get("key").and_then(|k| k.as_object()) {
        if let Some(key_name) = key.get("name").and_then(|n| n.as_str()) {
            rows.push(DetailRow {
                field: "API Key".to_string(),
                value: key_name.to_string(),
            });
        }
        if let Some(allowed_ips) = key.get("allowedSourceIps").and_then(|i| i.as_array()) {
            let ips_str = allowed_ips
                .iter()
                .filter_map(|v| v.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            rows.push(DetailRow {
                field: "Allowed IPs".to_string(),
                value: ips_str,
            });
        }
    }

    if rows.is_empty() {
        println!("No account information available");
        return Ok(());
    }

    let mut table = Table::new(&rows);
    table.with(Style::blank());

    output_with_pager(&table.to_string());
    Ok(())
}
