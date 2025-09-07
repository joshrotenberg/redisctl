//! Transit Gateway (TGW) command implementations

#![allow(dead_code)]

use crate::cli::{OutputFormat, TgwCommands};
use crate::commands::cloud::async_utils::{AsyncOperationArgs, handle_async_response};
use crate::commands::cloud::utils::{
    confirm_action, handle_output, print_formatted_output, read_file_input,
};
use crate::connection::ConnectionManager;
use crate::error::Result as CliResult;
use anyhow::Context;
use redis_cloud::CloudClient;
use redis_cloud::connectivity::transit_gateway::{TgwAttachmentRequest, TransitGatewayHandler};

/// Handle TGW commands
pub async fn handle_tgw_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    command: &TgwCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr
        .create_cloud_client(profile_name)
        .await
        .context("Failed to create Cloud client")?;

    match command {
        // Standard TGW operations
        TgwCommands::AttachmentsList { subscription_id } => {
            list_attachments(&client, *subscription_id, output_format, query).await
        }
        TgwCommands::AttachmentCreate {
            subscription_id,
            file,
            async_ops,
        } => {
            create_attachment(
                conn_mgr,
                profile_name,
                &client,
                *subscription_id,
                file,
                async_ops,
                output_format,
                query,
            )
            .await
        }
        TgwCommands::AttachmentCreateWithId {
            subscription_id,
            tgw_id,
            async_ops,
        } => {
            create_attachment_with_id(
                &client,
                *subscription_id,
                tgw_id,
                async_ops,
                output_format,
                query,
            )
            .await
        }
        TgwCommands::AttachmentUpdate {
            subscription_id,
            attachment_id,
            file,
            async_ops,
        } => {
            update_attachment_cidrs(
                &client,
                *subscription_id,
                attachment_id,
                file,
                async_ops,
                output_format,
                query,
            )
            .await
        }
        TgwCommands::AttachmentDelete {
            subscription_id,
            attachment_id,
            yes,
            async_ops,
        } => {
            delete_attachment(
                &client,
                *subscription_id,
                attachment_id,
                *yes,
                async_ops,
                output_format,
                query,
            )
            .await
        }
        TgwCommands::InvitationsList { subscription_id } => {
            list_invitations(&client, *subscription_id, output_format, query).await
        }
        TgwCommands::InvitationAccept {
            subscription_id,
            invitation_id,
        } => {
            accept_invitation(
                &client,
                *subscription_id,
                invitation_id,
                output_format,
                query,
            )
            .await
        }
        TgwCommands::InvitationReject {
            subscription_id,
            invitation_id,
        } => {
            reject_invitation(
                &client,
                *subscription_id,
                invitation_id,
                output_format,
                query,
            )
            .await
        }

        // Active-Active TGW operations
        TgwCommands::AaAttachmentsList { subscription_id } => {
            list_attachments_aa(&client, *subscription_id, output_format, query).await
        }
        TgwCommands::AaAttachmentCreate {
            subscription_id,
            region_id,
            file,
            async_ops,
        } => {
            create_attachment_aa(
                &client,
                *subscription_id,
                *region_id,
                file,
                async_ops,
                output_format,
                query,
            )
            .await
        }
        TgwCommands::AaAttachmentUpdate {
            subscription_id,
            region_id,
            attachment_id,
            file,
            async_ops,
        } => {
            update_attachment_cidrs_aa(
                &client,
                *subscription_id,
                *region_id,
                attachment_id,
                file,
                async_ops,
                output_format,
                query,
            )
            .await
        }
        TgwCommands::AaAttachmentDelete {
            subscription_id,
            region_id,
            attachment_id,
            yes,
            async_ops,
        } => {
            delete_attachment_aa(
                &client,
                *subscription_id,
                *region_id,
                attachment_id,
                *yes,
                async_ops,
                output_format,
                query,
            )
            .await
        }
        TgwCommands::AaInvitationsList { subscription_id } => {
            list_invitations_aa(&client, *subscription_id, output_format, query).await
        }
        TgwCommands::AaInvitationAccept {
            subscription_id,
            region_id,
            invitation_id,
        } => {
            accept_invitation_aa(
                &client,
                *subscription_id,
                *region_id,
                invitation_id,
                output_format,
                query,
            )
            .await
        }
        TgwCommands::AaInvitationReject {
            subscription_id,
            region_id,
            invitation_id,
        } => {
            reject_invitation_aa(
                &client,
                *subscription_id,
                *region_id,
                invitation_id,
                output_format,
                query,
            )
            .await
        }
    }
}

// ============================================================================
// Standard TGW Operations
// ============================================================================

async fn list_attachments(
    client: &CloudClient,
    subscription_id: i32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let handler = TransitGatewayHandler::new(client.clone());
    let response = handler
        .get_attachments(subscription_id)
        .await
        .context("Failed to get TGW attachments")?;

    let json_response = serde_json::to_value(response).context("Failed to serialize response")?;
    let data = handle_output(json_response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

async fn create_attachment(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    client: &CloudClient,
    subscription_id: i32,
    file: &str,
    async_ops: &AsyncOperationArgs,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let json_string = read_file_input(file)?;
    let request: TgwAttachmentRequest =
        serde_json::from_str(&json_string).context("Invalid TGW attachment configuration")?;

    let handler = TransitGatewayHandler::new(client.clone());
    let response = handler
        .create_attachment(subscription_id, &request)
        .await
        .context("Failed to create TGW attachment")?;

    let json_response = serde_json::to_value(&response).context("Failed to serialize response")?;

    handle_async_response(
        conn_mgr,
        profile_name,
        json_response,
        async_ops,
        output_format,
        query,
        "TGW attachment created successfully",
    )
    .await
}

async fn create_attachment_with_id(
    client: &CloudClient,
    subscription_id: i32,
    tgw_id: &str,
    async_ops: &AsyncOperationArgs,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let handler = TransitGatewayHandler::new(client.clone());
    let response = handler
        .create_attachment_with_id(subscription_id, tgw_id)
        .await
        .context("Failed to create TGW attachment")?;

    // Convert response to JSON and check for task ID
    let json_response = serde_json::to_value(&response).context("Failed to serialize response")?;
    if let Some(task_id) = json_response.get("taskId").and_then(|v| v.as_str()) {
        eprintln!("TGW attachment creation initiated. Task ID: {}", task_id);
        eprintln!(
            "Use 'redisctl cloud task wait {}' to monitor progress",
            task_id
        );
    }

    let data = handle_output(json_response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

async fn update_attachment_cidrs(
    client: &CloudClient,
    subscription_id: i32,
    attachment_id: &str,
    file: &str,
    async_ops: &AsyncOperationArgs,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let json_string = read_file_input(file)?;
    let request: TgwAttachmentRequest = serde_json::from_str(&json_string)
        .context("Invalid TGW attachment update configuration")?;

    let handler = TransitGatewayHandler::new(client.clone());
    let response = handler
        .update_attachment_cidrs(subscription_id, attachment_id.to_string(), &request)
        .await
        .context("Failed to update TGW attachment CIDRs")?;

    // Convert response to JSON and check for task ID
    let json_response = serde_json::to_value(&response).context("Failed to serialize response")?;
    if let Some(task_id) = json_response.get("taskId").and_then(|v| v.as_str()) {
        eprintln!("TGW attachment update initiated. Task ID: {}", task_id);
        eprintln!(
            "Use 'redisctl cloud task wait {}' to monitor progress",
            task_id
        );
    }

    let data = handle_output(json_response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

async fn delete_attachment(
    client: &CloudClient,
    subscription_id: i32,
    attachment_id: &str,
    yes: bool,
    async_ops: &AsyncOperationArgs,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    if !yes {
        let prompt = format!(
            "Delete TGW attachment {} for subscription {}?",
            attachment_id, subscription_id
        );
        if !confirm_action(&prompt)? {
            eprintln!("Operation cancelled");
            return Ok(());
        }
    }

    let handler = TransitGatewayHandler::new(client.clone());
    handler
        .delete_attachment(subscription_id, attachment_id.to_string())
        .await
        .context("Failed to delete TGW attachment")?;

    eprintln!("TGW attachment deleted successfully");
    Ok(())
}

async fn list_invitations(
    client: &CloudClient,
    subscription_id: i32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let handler = TransitGatewayHandler::new(client.clone());
    let response = handler
        .get_shared_invitations(subscription_id)
        .await
        .context("Failed to get TGW invitations")?;

    let json_response = serde_json::to_value(response).context("Failed to serialize response")?;
    let data = handle_output(json_response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

async fn accept_invitation(
    client: &CloudClient,
    subscription_id: i32,
    invitation_id: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let handler = TransitGatewayHandler::new(client.clone());
    let response = handler
        .accept_resource_share(subscription_id, invitation_id.to_string())
        .await
        .context("Failed to accept TGW invitation")?;

    // Convert response to JSON and check for task ID
    let json_response = serde_json::to_value(&response).context("Failed to serialize response")?;
    if let Some(task_id) = json_response.get("taskId").and_then(|v| v.as_str()) {
        eprintln!("TGW invitation acceptance initiated. Task ID: {}", task_id);
        eprintln!(
            "Use 'redisctl cloud task wait {}' to monitor progress",
            task_id
        );
    }

    let data = handle_output(json_response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

async fn reject_invitation(
    client: &CloudClient,
    subscription_id: i32,
    invitation_id: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let handler = TransitGatewayHandler::new(client.clone());
    let response = handler
        .reject_resource_share(subscription_id, invitation_id.to_string())
        .await
        .context("Failed to reject TGW invitation")?;

    let json_response = serde_json::to_value(&response).context("Failed to serialize response")?;
    let data = handle_output(json_response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

// ============================================================================
// Active-Active TGW Operations
// ============================================================================

async fn list_attachments_aa(
    client: &CloudClient,
    subscription_id: i32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let handler = TransitGatewayHandler::new(client.clone());
    let response = handler
        .get_attachments_active_active(subscription_id)
        .await
        .context("Failed to get Active-Active TGW attachments")?;

    let json_response = serde_json::to_value(response).context("Failed to serialize response")?;
    let data = handle_output(json_response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

async fn create_attachment_aa(
    client: &CloudClient,
    subscription_id: i32,
    region_id: i32,
    file: &str,
    async_ops: &AsyncOperationArgs,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let json_string = read_file_input(file)?;
    let request: TgwAttachmentRequest = serde_json::from_str(&json_string)
        .context("Invalid Active-Active TGW attachment configuration")?;

    let handler = TransitGatewayHandler::new(client.clone());
    let response = handler
        .create_attachment_active_active(subscription_id, region_id, &request)
        .await
        .context("Failed to create Active-Active TGW attachment")?;

    // Convert response to JSON and check for task ID
    let json_response = serde_json::to_value(&response).context("Failed to serialize response")?;
    if let Some(task_id) = json_response.get("taskId").and_then(|v| v.as_str()) {
        eprintln!(
            "Active-Active TGW attachment creation initiated. Task ID: {}",
            task_id
        );
        eprintln!(
            "Use 'redisctl cloud task wait {}' to monitor progress",
            task_id
        );
    }

    let data = handle_output(json_response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

async fn update_attachment_cidrs_aa(
    client: &CloudClient,
    subscription_id: i32,
    region_id: i32,
    attachment_id: &str,
    file: &str,
    async_ops: &AsyncOperationArgs,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let json_string = read_file_input(file)?;
    let request: TgwAttachmentRequest = serde_json::from_str(&json_string)
        .context("Invalid Active-Active TGW attachment update configuration")?;

    let handler = TransitGatewayHandler::new(client.clone());
    let response = handler
        .update_attachment_cidrs_active_active(
            subscription_id,
            region_id,
            attachment_id.to_string(),
            &request,
        )
        .await
        .context("Failed to update Active-Active TGW attachment CIDRs")?;

    // Convert response to JSON and check for task ID
    let json_response = serde_json::to_value(&response).context("Failed to serialize response")?;
    if let Some(task_id) = json_response.get("taskId").and_then(|v| v.as_str()) {
        eprintln!(
            "Active-Active TGW attachment update initiated. Task ID: {}",
            task_id
        );
        eprintln!(
            "Use 'redisctl cloud task wait {}' to monitor progress",
            task_id
        );
    }

    let data = handle_output(json_response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

async fn delete_attachment_aa(
    client: &CloudClient,
    subscription_id: i32,
    region_id: i32,
    attachment_id: &str,
    yes: bool,
    async_ops: &AsyncOperationArgs,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    if !yes {
        let prompt = format!(
            "Delete Active-Active TGW attachment {} in region {} for subscription {}?",
            attachment_id, region_id, subscription_id
        );
        if !confirm_action(&prompt)? {
            eprintln!("Operation cancelled");
            return Ok(());
        }
    }

    let handler = TransitGatewayHandler::new(client.clone());
    handler
        .delete_attachment_active_active(subscription_id, region_id, attachment_id.to_string())
        .await
        .context("Failed to delete Active-Active TGW attachment")?;

    eprintln!("Active-Active TGW attachment deleted successfully");
    Ok(())
}

async fn list_invitations_aa(
    client: &CloudClient,
    subscription_id: i32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let handler = TransitGatewayHandler::new(client.clone());
    let response = handler
        .get_shared_invitations_active_active(subscription_id)
        .await
        .context("Failed to get Active-Active TGW invitations")?;

    let json_response = serde_json::to_value(response).context("Failed to serialize response")?;
    let data = handle_output(json_response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

async fn accept_invitation_aa(
    client: &CloudClient,
    subscription_id: i32,
    region_id: i32,
    invitation_id: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let handler = TransitGatewayHandler::new(client.clone());
    let response = handler
        .accept_resource_share_active_active(subscription_id, region_id, invitation_id.to_string())
        .await
        .context("Failed to accept Active-Active TGW invitation")?;

    // Convert response to JSON and check for task ID
    let json_response = serde_json::to_value(&response).context("Failed to serialize response")?;
    if let Some(task_id) = json_response.get("taskId").and_then(|v| v.as_str()) {
        eprintln!(
            "Active-Active TGW invitation acceptance initiated. Task ID: {}",
            task_id
        );
        eprintln!(
            "Use 'redisctl cloud task wait {}' to monitor progress",
            task_id
        );
    }

    let data = handle_output(json_response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

async fn reject_invitation_aa(
    client: &CloudClient,
    subscription_id: i32,
    region_id: i32,
    invitation_id: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let handler = TransitGatewayHandler::new(client.clone());
    let response = handler
        .reject_resource_share_active_active(subscription_id, region_id, invitation_id.to_string())
        .await
        .context("Failed to reject Active-Active TGW invitation")?;

    let json_response = serde_json::to_value(&response).context("Failed to serialize response")?;
    let data = handle_output(json_response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}
