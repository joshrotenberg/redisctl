//! AWS PrivateLink command implementations

#![allow(dead_code)]

use super::ConnectivityOperationParams;
use crate::cli::{OutputFormat, PrivateLinkCommands};
use crate::commands::cloud::async_utils::handle_async_response;
use crate::commands::cloud::utils::{handle_output, print_formatted_output, read_file_input};
use crate::connection::ConnectionManager;
use crate::error::Result as CliResult;
use anyhow::Context;
use redis_cloud::PrivateLinkHandler;
use serde_json::Value;

/// Handle PrivateLink commands
pub async fn handle_private_link_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    command: &PrivateLinkCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_cloud_client(profile_name).await?;
    let handler = PrivateLinkHandler::new(client.clone());

    match command {
        PrivateLinkCommands::Get {
            subscription,
            region,
        } => handle_get(&handler, *subscription, *region, output_format, query).await,
        PrivateLinkCommands::Create {
            subscription,
            region,
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
            handle_create(&handler, &params, *region, data).await
        }
        PrivateLinkCommands::AddPrincipal {
            subscription,
            region,
            data,
        } => {
            handle_add_principal(&handler, *subscription, *region, data, output_format, query).await
        }
        PrivateLinkCommands::RemovePrincipal {
            subscription,
            region,
            data,
        } => {
            handle_remove_principal(&handler, *subscription, *region, data, output_format, query)
                .await
        }
        PrivateLinkCommands::GetScript {
            subscription,
            region,
        } => handle_get_script(&handler, *subscription, *region, output_format, query).await,
    }
}

/// Get PrivateLink configuration
async fn handle_get(
    handler: &PrivateLinkHandler,
    subscription_id: i32,
    region_id: Option<i32>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let result = if let Some(region) = region_id {
        handler
            .get_active_active(subscription_id, region)
            .await
            .context("Failed to get Active-Active PrivateLink configuration")?
    } else {
        handler
            .get(subscription_id)
            .await
            .context("Failed to get PrivateLink configuration")?
    };

    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Create PrivateLink
async fn handle_create(
    handler: &PrivateLinkHandler,
    params: &ConnectivityOperationParams<'_>,
    region_id: Option<i32>,
    data: &str,
) -> CliResult<()> {
    let content = read_file_input(data)?;
    let request: Value = serde_json::from_str(&content).context("Failed to parse JSON input")?;

    let result = if let Some(region) = region_id {
        handler
            .create_active_active(params.subscription_id, region, request)
            .await
            .context("Failed to create Active-Active PrivateLink")?
    } else {
        handler
            .create(params.subscription_id, request)
            .await
            .context("Failed to create PrivateLink")?
    };

    handle_async_response(
        params.conn_mgr,
        params.profile_name,
        result,
        params.async_ops,
        params.output_format,
        params.query,
        "Create PrivateLink",
    )
    .await
}

/// Add principals to PrivateLink
async fn handle_add_principal(
    handler: &PrivateLinkHandler,
    subscription_id: i32,
    region_id: Option<i32>,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let content = read_file_input(data)?;
    let request: Value = serde_json::from_str(&content).context("Failed to parse JSON input")?;

    let result = if let Some(region) = region_id {
        handler
            .add_principals_active_active(subscription_id, region, request)
            .await
            .context("Failed to add principals to Active-Active PrivateLink")?
    } else {
        handler
            .add_principals(subscription_id, request)
            .await
            .context("Failed to add principals to PrivateLink")?
    };

    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Remove principals from PrivateLink
async fn handle_remove_principal(
    handler: &PrivateLinkHandler,
    subscription_id: i32,
    region_id: Option<i32>,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let content = read_file_input(data)?;
    let request: Value = serde_json::from_str(&content).context("Failed to parse JSON input")?;

    let result = if let Some(region) = region_id {
        handler
            .remove_principals_active_active(subscription_id, region, request)
            .await
            .context("Failed to remove principals from Active-Active PrivateLink")?
    } else {
        handler
            .remove_principals(subscription_id, request)
            .await
            .context("Failed to remove principals from PrivateLink")?
    };

    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Get VPC endpoint creation script
async fn handle_get_script(
    handler: &PrivateLinkHandler,
    subscription_id: i32,
    region_id: Option<i32>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let result = if let Some(region) = region_id {
        handler
            .get_endpoint_script_active_active(subscription_id, region)
            .await
            .context("Failed to get Active-Active endpoint script")?
    } else {
        handler
            .get_endpoint_script(subscription_id)
            .await
            .context("Failed to get endpoint script")?
    };

    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}
