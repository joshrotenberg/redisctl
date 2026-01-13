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

/// Parameters for PrivateLink create operation
#[derive(Debug, Default)]
pub struct PrivateLinkCreateParams {
    pub share_name: Option<String>,
    pub principal: Option<String>,
    pub principal_type: Option<String>,
    pub alias: Option<String>,
    pub data: Option<String>,
}

/// Parameters for PrivateLink principal operations
#[derive(Debug, Default)]
pub struct PrivateLinkPrincipalParams {
    pub principal: Option<String>,
    pub principal_type: Option<String>,
    pub alias: Option<String>,
    pub data: Option<String>,
}

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
            share_name,
            principal,
            principal_type,
            alias,
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
            let create_params = PrivateLinkCreateParams {
                share_name: share_name.clone(),
                principal: principal.clone(),
                principal_type: principal_type.clone(),
                alias: alias.clone(),
                data: data.clone(),
            };
            handle_create(&handler, &params, *region, &create_params).await
        }
        PrivateLinkCommands::AddPrincipal {
            subscription,
            region,
            principal,
            principal_type,
            alias,
            data,
        } => {
            let principal_params = PrivateLinkPrincipalParams {
                principal: principal.clone(),
                principal_type: principal_type.clone(),
                alias: alias.clone(),
                data: data.clone(),
            };
            handle_add_principal(
                &handler,
                *subscription,
                *region,
                &principal_params,
                output_format,
                query,
            )
            .await
        }
        PrivateLinkCommands::RemovePrincipal {
            subscription,
            region,
            principal,
            principal_type,
            alias,
            data,
        } => {
            let principal_params = PrivateLinkPrincipalParams {
                principal: principal.clone(),
                principal_type: principal_type.clone(),
                alias: alias.clone(),
                data: data.clone(),
            };
            handle_remove_principal(
                &handler,
                *subscription,
                *region,
                &principal_params,
                output_format,
                query,
            )
            .await
        }
        PrivateLinkCommands::GetScript {
            subscription,
            region,
        } => handle_get_script(&handler, *subscription, *region, output_format, query).await,
        PrivateLinkCommands::Delete {
            subscription,
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
            handle_delete(&handler, &params, *force).await
        }
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

/// Normalize principal type (convert aws-account to aws_account for API)
fn normalize_principal_type(t: &str) -> String {
    t.replace('-', "_")
}

/// Build PrivateLink create payload from parameters
fn build_create_payload(create_params: &PrivateLinkCreateParams) -> CliResult<Value> {
    // If --data is provided, use it as the base (escape hatch)
    if let Some(data) = &create_params.data {
        let content = read_file_input(data)?;
        return Ok(serde_json::from_str(&content).context("Failed to parse JSON input")?);
    }

    // Build payload from first-class parameters
    let mut payload = serde_json::Map::new();

    if let Some(share_name) = &create_params.share_name {
        payload.insert("shareName".to_string(), Value::String(share_name.clone()));
    }
    if let Some(principal) = &create_params.principal {
        payload.insert("principal".to_string(), Value::String(principal.clone()));
    }
    if let Some(principal_type) = &create_params.principal_type {
        payload.insert(
            "type".to_string(),
            Value::String(normalize_principal_type(principal_type)),
        );
    }
    if let Some(alias) = &create_params.alias {
        payload.insert("alias".to_string(), Value::String(alias.clone()));
    }

    Ok(Value::Object(payload))
}

/// Build PrivateLink principal payload from parameters
fn build_principal_payload(principal_params: &PrivateLinkPrincipalParams) -> CliResult<Value> {
    // If --data is provided, use it as the base (escape hatch)
    if let Some(data) = &principal_params.data {
        let content = read_file_input(data)?;
        return Ok(serde_json::from_str(&content).context("Failed to parse JSON input")?);
    }

    // Build payload from first-class parameters
    let mut payload = serde_json::Map::new();

    if let Some(principal) = &principal_params.principal {
        payload.insert("principal".to_string(), Value::String(principal.clone()));
    }
    if let Some(principal_type) = &principal_params.principal_type {
        payload.insert(
            "type".to_string(),
            Value::String(normalize_principal_type(principal_type)),
        );
    }
    if let Some(alias) = &principal_params.alias {
        payload.insert("alias".to_string(), Value::String(alias.clone()));
    }

    Ok(Value::Object(payload))
}

/// Create PrivateLink
async fn handle_create(
    handler: &PrivateLinkHandler,
    params: &ConnectivityOperationParams<'_>,
    region_id: Option<i32>,
    create_params: &PrivateLinkCreateParams,
) -> CliResult<()> {
    let request = build_create_payload(create_params)?;

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
    principal_params: &PrivateLinkPrincipalParams,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let request = build_principal_payload(principal_params)?;

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
    principal_params: &PrivateLinkPrincipalParams,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let request = build_principal_payload(principal_params)?;

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

/// Delete PrivateLink configuration
async fn handle_delete(
    handler: &PrivateLinkHandler,
    params: &ConnectivityOperationParams<'_>,
    force: bool,
) -> CliResult<()> {
    // Confirmation prompt unless --force is used
    if !force {
        use dialoguer::Confirm;
        let confirm = Confirm::new()
            .with_prompt(format!(
                "Are you sure you want to delete PrivateLink for subscription {}?",
                params.subscription_id
            ))
            .default(false)
            .interact()
            .map_err(|e| anyhow::anyhow!("Failed to read confirmation: {}", e))?;

        if !confirm {
            println!("Delete operation cancelled");
            return Ok(());
        }
    }

    let result = handler
        .delete(params.subscription_id)
        .await
        .context("Failed to delete PrivateLink")?;

    handle_async_response(
        params.conn_mgr,
        params.profile_name,
        result,
        params.async_ops,
        params.output_format,
        params.query,
        "Delete PrivateLink",
    )
    .await
}
