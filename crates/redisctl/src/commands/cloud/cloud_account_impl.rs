#![allow(dead_code)]

use crate::cli::OutputFormat;
use crate::commands::cloud::utils::{
    confirm_action, handle_output, print_formatted_output, read_file_input,
};
use crate::error::Result as CliResult;

use anyhow::Context;
use comfy_table::{Cell, Color, Table};
use redis_cloud::CloudClient;
use serde_json::{Value, json};

pub async fn handle_list(
    client: &CloudClient,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let result = client
        .get_raw("/cloud-accounts")
        .await
        .context("Failed to list cloud accounts")?;

    // For table output, create a formatted table
    if matches!(output_format, OutputFormat::Table)
        && query.is_none()
        && let Some(accounts) = result.get("cloudAccounts").and_then(|a| a.as_array())
    {
        let mut table = Table::new();
        table.set_header(vec!["ID", "Name", "Provider", "Status", "Created"]);

        for account in accounts {
            let id = account.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
            let name = account.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let provider = account
                .get("provider")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let status = account.get("status").and_then(|v| v.as_str()).unwrap_or("");
            let created_timestamp = account
                .get("createdTimestamp")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            let status_cell = match status {
                "active" => Cell::new(status).fg(Color::Green),
                "inactive" => Cell::new(status).fg(Color::Red),
                _ => Cell::new(status),
            };

            table.add_row(vec![
                Cell::new(id),
                Cell::new(name),
                Cell::new(provider),
                status_cell,
                Cell::new(created_timestamp),
            ]);
        }

        println!("{}", table);
        return Ok(());
    }

    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn handle_get(
    client: &CloudClient,
    account_id: i32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let result = client
        .get_raw(&format!("/cloud-accounts/{}", account_id))
        .await
        .context("Failed to get cloud account")?;

    // For table output, create a detailed view
    if matches!(output_format, OutputFormat::Table) && query.is_none() {
        let mut table = Table::new();
        table.set_header(vec!["Field", "Value"]);

        if let Some(obj) = result.as_object() {
            for (key, value) in obj {
                // Mask sensitive fields
                let display_value =
                    if key.contains("secret") || key.contains("password") || key.contains("key") {
                        "***REDACTED***".to_string()
                    } else {
                        match value {
                            Value::String(s) => s.clone(),
                            _ => value.to_string(),
                        }
                    };
                table.add_row(vec![Cell::new(key), Cell::new(display_value)]);
            }
        }

        println!("{}", table);
        return Ok(());
    }

    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn handle_create(
    client: &CloudClient,
    file: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let content = read_file_input(file)?;
    let mut payload: Value =
        serde_json::from_str(&content).context("Failed to parse JSON from file")?;

    // If the input is a GCP service account JSON, convert it to the cloud account format
    if let Some(_project_id) = payload.get("project_id") {
        // This is a GCP service account JSON
        let provider_payload = json!({
            "provider": "GCP",
            "name": payload.get("client_email")
                .and_then(|v| v.as_str())
                .unwrap_or("GCP Cloud Account"),
            "serviceAccountJson": serde_json::to_string(&payload)?
        });
        payload = provider_payload;
    }

    // Validate required fields based on provider
    let provider = payload
        .get("provider")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Missing 'provider' field in JSON"))?;

    match provider {
        "AWS" => {
            if payload.get("accessKeyId").is_none() {
                return Err(anyhow::anyhow!("AWS provider requires 'accessKeyId' field").into());
            }
            if payload.get("accessSecretKey").is_none() {
                return Err(
                    anyhow::anyhow!("AWS provider requires 'accessSecretKey' field").into(),
                );
            }
        }
        "GCP" => {
            if payload.get("serviceAccountJson").is_none() {
                return Err(
                    anyhow::anyhow!("GCP provider requires 'serviceAccountJson' field").into(),
                );
            }
        }
        "Azure" => {
            if payload.get("subscriptionId").is_none() {
                return Err(
                    anyhow::anyhow!("Azure provider requires 'subscriptionId' field").into(),
                );
            }
            if payload.get("tenantId").is_none() {
                return Err(anyhow::anyhow!("Azure provider requires 'tenantId' field").into());
            }
            if payload.get("clientId").is_none() {
                return Err(anyhow::anyhow!("Azure provider requires 'clientId' field").into());
            }
            if payload.get("clientSecret").is_none() {
                return Err(anyhow::anyhow!("Azure provider requires 'clientSecret' field").into());
            }
        }
        _ => {
            return Err(anyhow::anyhow!("Unknown provider: {}", provider).into());
        }
    }

    let result = client
        .post_raw("/cloud-accounts", payload)
        .await
        .context("Failed to create cloud account")?;

    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn handle_update(
    client: &CloudClient,
    account_id: i32,
    file: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let content = read_file_input(file)?;
    let payload: Value =
        serde_json::from_str(&content).context("Failed to parse JSON from file")?;

    let result = client
        .put_raw(&format!("/cloud-accounts/{}", account_id), payload)
        .await
        .context("Failed to update cloud account")?;

    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn handle_delete(client: &CloudClient, account_id: i32, force: bool) -> CliResult<()> {
    if !force {
        let confirmed = confirm_action(&format!("delete cloud account {}", account_id))?;
        if !confirmed {
            println!("Operation cancelled");
            return Ok(());
        }
    }

    client
        .delete_raw(&format!("/cloud-accounts/{}", account_id))
        .await
        .context("Failed to delete cloud account")?;

    println!("Cloud account {} deleted successfully", account_id);
    Ok(())
}
