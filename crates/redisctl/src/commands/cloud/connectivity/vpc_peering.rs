//! VPC Peering command implementations

#![allow(dead_code)]

use super::ConnectivityOperationParams;
use crate::cli::{OutputFormat, VpcPeeringCommands};
use crate::commands::cloud::async_utils::handle_async_response;
use crate::commands::cloud::utils::{
    confirm_action, handle_output, print_formatted_output, read_file_input,
};
use crate::connection::ConnectionManager;
use crate::error::Result as CliResult;
use anyhow::Context;
use redis_cloud::CloudClient;
use serde_json::Value;

/// Handle VPC peering commands
pub async fn handle_vpc_peering_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    command: &VpcPeeringCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let profile = conn_mgr.get_profile(profile_name)?;
    let client = crate::commands::cloud::utils::create_cloud_client_raw(profile).await?;

    match command {
        VpcPeeringCommands::Get { subscription } => {
            handle_get(&client, *subscription, output_format, query).await
        }
        VpcPeeringCommands::Create {
            subscription,
            data,
            async_ops,
        } => {
            let params = ConnectivityOperationParams {
                conn_mgr,
                profile_name,
                client: &client,
                subscription_id: *subscription,
                async_ops,
                output_format,
                query,
            };
            handle_create(&params, data).await
        }
        VpcPeeringCommands::Update {
            subscription,
            peering_id,
            data,
            async_ops,
        } => {
            let params = ConnectivityOperationParams {
                conn_mgr,
                profile_name,
                client: &client,
                subscription_id: *subscription,
                async_ops,
                output_format,
                query,
            };
            handle_update(&params, *peering_id, data).await
        }
        VpcPeeringCommands::Delete {
            subscription,
            peering_id,
            force,
            async_ops,
        } => {
            let params = ConnectivityOperationParams {
                conn_mgr,
                profile_name,
                client: &client,
                subscription_id: *subscription,
                async_ops,
                output_format,
                query,
            };
            handle_delete(&params, *peering_id, *force).await
        }
        VpcPeeringCommands::ListActiveActive { subscription } => {
            handle_list_active_active(&client, *subscription, output_format, query).await
        }
        VpcPeeringCommands::CreateActiveActive {
            subscription,
            data,
            async_ops,
        } => {
            let params = ConnectivityOperationParams {
                conn_mgr,
                profile_name,
                client: &client,
                subscription_id: *subscription,
                async_ops,
                output_format,
                query,
            };
            handle_create_active_active(&params, data).await
        }
        VpcPeeringCommands::UpdateActiveActive {
            subscription,
            peering_id,
            data,
            async_ops,
        } => {
            let params = ConnectivityOperationParams {
                conn_mgr,
                profile_name,
                client: &client,
                subscription_id: *subscription,
                async_ops,
                output_format,
                query,
            };
            handle_update_active_active(&params, *peering_id, data).await
        }
        VpcPeeringCommands::DeleteActiveActive {
            subscription,
            peering_id,
            force,
            async_ops,
        } => {
            let params = ConnectivityOperationParams {
                conn_mgr,
                profile_name,
                client: &client,
                subscription_id: *subscription,
                async_ops,
                output_format,
                query,
            };
            handle_delete_active_active(&params, *peering_id, *force).await
        }
    }
}

/// Get VPC peering details
async fn handle_get(
    client: &CloudClient,
    subscription_id: i32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let result = client
        .get_raw(&format!("/subscriptions/{}/peerings/vpc", subscription_id))
        .await
        .context("Failed to get VPC peering")?;

    let data = handle_output(result, output_format, query)?;

    if matches!(output_format, OutputFormat::Table) && query.is_none() {
        print_vpc_peering_table(&data)?;
    } else {
        print_formatted_output(data, output_format)?;
    }

    Ok(())
}

/// Create VPC peering
async fn handle_create(params: &ConnectivityOperationParams<'_>, data: &str) -> CliResult<()> {
    let content = read_file_input(data)?;
    let payload: Value = serde_json::from_str(&content).context("Failed to parse JSON input")?;

    let result = params
        .client
        .post_raw(
            &format!("/subscriptions/{}/peerings/vpc", params.subscription_id),
            payload,
        )
        .await
        .context("Failed to create VPC peering")?;

    handle_async_response(
        params.conn_mgr,
        params.profile_name,
        result,
        params.async_ops,
        params.output_format,
        params.query,
        "VPC peering created successfully",
    )
    .await
}

/// Update VPC peering
async fn handle_update(
    params: &ConnectivityOperationParams<'_>,
    peering_id: i32,
    data: &str,
) -> CliResult<()> {
    let content = read_file_input(data)?;
    let payload: Value = serde_json::from_str(&content).context("Failed to parse JSON input")?;

    let result = params
        .client
        .put_raw(
            &format!(
                "/subscriptions/{}/peerings/vpc/{}",
                params.subscription_id, peering_id
            ),
            payload,
        )
        .await
        .context("Failed to update VPC peering")?;

    handle_async_response(
        params.conn_mgr,
        params.profile_name,
        result,
        params.async_ops,
        params.output_format,
        params.query,
        "VPC peering updated successfully",
    )
    .await
}

/// Delete VPC peering
async fn handle_delete(
    params: &ConnectivityOperationParams<'_>,
    peering_id: i32,
    force: bool,
) -> CliResult<()> {
    if !force {
        let confirmed = confirm_action(&format!(
            "delete VPC peering {} for subscription {}",
            peering_id, params.subscription_id
        ))?;
        if !confirmed {
            println!("Operation cancelled");
            return Ok(());
        }
    }

    let result = params
        .client
        .delete_raw(&format!(
            "/subscriptions/{}/peerings/vpc/{}",
            params.subscription_id, peering_id
        ))
        .await
        .context("Failed to delete VPC peering")?;

    handle_async_response(
        params.conn_mgr,
        params.profile_name,
        result,
        params.async_ops,
        params.output_format,
        params.query,
        "VPC peering deleted successfully",
    )
    .await
}

/// List Active-Active VPC peerings
async fn handle_list_active_active(
    client: &CloudClient,
    subscription_id: i32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let result = client
        .get_raw(&format!(
            "/subscriptions/{}/peerings/vpc/active-active",
            subscription_id
        ))
        .await
        .context("Failed to list Active-Active VPC peerings")?;

    let data = handle_output(result, output_format, query)?;

    if matches!(output_format, OutputFormat::Table) && query.is_none() {
        print_vpc_peering_list_table(&data)?;
    } else {
        print_formatted_output(data, output_format)?;
    }

    Ok(())
}

/// Create Active-Active VPC peering
async fn handle_create_active_active(
    params: &ConnectivityOperationParams<'_>,
    data: &str,
) -> CliResult<()> {
    let content = read_file_input(data)?;
    let payload: Value = serde_json::from_str(&content).context("Failed to parse JSON input")?;

    let result = params
        .client
        .post_raw(
            &format!(
                "/subscriptions/{}/peerings/vpc/active-active",
                params.subscription_id
            ),
            payload,
        )
        .await
        .context("Failed to create Active-Active VPC peering")?;

    handle_async_response(
        params.conn_mgr,
        params.profile_name,
        result,
        params.async_ops,
        params.output_format,
        params.query,
        "Active-Active VPC peering created successfully",
    )
    .await
}

/// Update Active-Active VPC peering
async fn handle_update_active_active(
    params: &ConnectivityOperationParams<'_>,
    peering_id: i32,
    data: &str,
) -> CliResult<()> {
    let content = read_file_input(data)?;
    let payload: Value = serde_json::from_str(&content).context("Failed to parse JSON input")?;

    let result = params
        .client
        .put_raw(
            &format!(
                "/subscriptions/{}/peerings/vpc/active-active/{}",
                params.subscription_id, peering_id
            ),
            payload,
        )
        .await
        .context("Failed to update Active-Active VPC peering")?;

    handle_async_response(
        params.conn_mgr,
        params.profile_name,
        result,
        params.async_ops,
        params.output_format,
        params.query,
        "Active-Active VPC peering updated successfully",
    )
    .await
}

/// Delete Active-Active VPC peering
async fn handle_delete_active_active(
    params: &ConnectivityOperationParams<'_>,
    peering_id: i32,
    force: bool,
) -> CliResult<()> {
    if !force {
        let confirmed = confirm_action(&format!(
            "delete Active-Active VPC peering {} for subscription {}",
            peering_id, params.subscription_id
        ))?;
        if !confirmed {
            println!("Operation cancelled");
            return Ok(());
        }
    }

    let result = params
        .client
        .delete_raw(&format!(
            "/subscriptions/{}/peerings/vpc/active-active/{}",
            params.subscription_id, peering_id
        ))
        .await
        .context("Failed to delete Active-Active VPC peering")?;

    handle_async_response(
        params.conn_mgr,
        params.profile_name,
        result,
        params.async_ops,
        params.output_format,
        params.query,
        "Active-Active VPC peering deleted successfully",
    )
    .await
}

/// Print VPC peering details in table format
fn print_vpc_peering_table(data: &Value) -> CliResult<()> {
    use super::super::utils::DetailRow;
    use tabled::{Table, settings::Style};

    let mut rows = Vec::new();

    // Basic info
    if let Some(id) = data.get("peeringId") {
        rows.push(DetailRow {
            field: "Peering ID".to_string(),
            value: id.to_string().trim_matches('"').to_string(),
        });
    }

    if let Some(status) = data.get("status").and_then(|s| s.as_str()) {
        rows.push(DetailRow {
            field: "Status".to_string(),
            value: super::super::utils::format_status_text(status),
        });
    }

    // AWS VPC info
    if let Some(vpc_id) = data.get("awsVpcId").and_then(|v| v.as_str()) {
        rows.push(DetailRow {
            field: "AWS VPC ID".to_string(),
            value: vpc_id.to_string(),
        });
    }

    if let Some(account_id) = data.get("awsAccountId").and_then(|a| a.as_str()) {
        rows.push(DetailRow {
            field: "AWS Account ID".to_string(),
            value: account_id.to_string(),
        });
    }

    if let Some(region) = data.get("region").and_then(|r| r.as_str()) {
        rows.push(DetailRow {
            field: "Region".to_string(),
            value: region.to_string(),
        });
    }

    // CIDR blocks
    if let Some(cidrs) = data.get("vpcCidrs").and_then(|c| c.as_array()) {
        let cidr_list: Vec<String> = cidrs
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
        if !cidr_list.is_empty() {
            rows.push(DetailRow {
                field: "VPC CIDRs".to_string(),
                value: cidr_list.join(", "),
            });
        }
    }

    // Connection details
    if let Some(connection_id) = data.get("connectionId").and_then(|c| c.as_str()) {
        rows.push(DetailRow {
            field: "Connection ID".to_string(),
            value: connection_id.to_string(),
        });
    }

    if rows.is_empty() {
        println!("No VPC peering information available");
        return Ok(());
    }

    let mut table = Table::new(&rows);
    table.with(Style::blank());

    println!("{}", table);
    Ok(())
}

/// Print VPC peering list in table format
fn print_vpc_peering_list_table(data: &Value) -> CliResult<()> {
    use comfy_table::{Cell, Color, Table};

    let peerings = if let Some(arr) = data.as_array() {
        arr.clone()
    } else if let Some(peerings) = data.get("peerings").and_then(|p| p.as_array()) {
        peerings.clone()
    } else {
        println!("No VPC peerings found");
        return Ok(());
    };

    if peerings.is_empty() {
        println!("No VPC peerings found");
        return Ok(());
    }

    let mut table = Table::new();
    table.set_header(vec![
        "ID",
        "Status",
        "VPC ID",
        "Account ID",
        "Region",
        "CIDRs",
    ]);

    for peering in peerings {
        let id = peering
            .get("peeringId")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);

        let status = peering
            .get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        let vpc_id = peering
            .get("awsVpcId")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let account_id = peering
            .get("awsAccountId")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let region = peering.get("region").and_then(|v| v.as_str()).unwrap_or("");

        let cidrs = if let Some(cidr_array) = peering.get("vpcCidrs").and_then(|c| c.as_array()) {
            cidr_array
                .iter()
                .filter_map(|v| v.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            String::new()
        };

        let status_cell = match status.to_lowercase().as_str() {
            "active" => Cell::new(status).fg(Color::Green),
            "pending" => Cell::new(status).fg(Color::Yellow),
            "failed" | "error" => Cell::new(status).fg(Color::Red),
            _ => Cell::new(status),
        };

        table.add_row(vec![
            Cell::new(id),
            status_cell,
            Cell::new(vpc_id),
            Cell::new(account_id),
            Cell::new(region),
            Cell::new(cidrs),
        ]);
    }

    println!("{}", table);
    Ok(())
}
