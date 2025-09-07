//! Fixed database command implementations

#![allow(dead_code)]

use crate::cli::{CloudFixedDatabaseCommands, OutputFormat};
use crate::commands::cloud::async_utils::handle_async_response;
use crate::commands::cloud::utils::{
    confirm_action, handle_output, print_formatted_output, read_file_input,
};
use crate::connection::ConnectionManager;
use crate::error::{RedisCtlError, Result as CliResult};
use anyhow::Context;
use redis_cloud::fixed::databases::{
    DatabaseTagCreateRequest, DatabaseTagUpdateRequest, FixedDatabaseBackupRequest,
    FixedDatabaseCreateRequest, FixedDatabaseHandler, FixedDatabaseImportRequest,
    FixedDatabaseUpdateRequest,
};

/// Parse database ID in format "subscription_id:database_id"
fn parse_fixed_database_id(id: &str) -> CliResult<(i32, i32)> {
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
        .parse::<i32>()
        .with_context(|| format!("Invalid subscription ID: {}", parts[0]))?;
    let database_id = parts[1]
        .parse::<i32>()
        .with_context(|| format!("Invalid database ID: {}", parts[1]))?;

    Ok((subscription_id, database_id))
}

/// Handle fixed database commands
pub async fn handle_fixed_database_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    command: &CloudFixedDatabaseCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr
        .create_cloud_client(profile_name)
        .await
        .context("Failed to create Cloud client")?;

    let handler = FixedDatabaseHandler::new(client);

    match command {
        CloudFixedDatabaseCommands::List { subscription_id } => {
            let databases = handler
                .list(*subscription_id, None, None)
                .await
                .context("Failed to list fixed databases")?;

            let json_response =
                serde_json::to_value(databases).context("Failed to serialize response")?;
            let data = handle_output(json_response, output_format, query)?;
            print_formatted_output(data, output_format)?;
            Ok(())
        }

        CloudFixedDatabaseCommands::Get { id } => {
            let (subscription_id, database_id) = parse_fixed_database_id(id)?;
            let database = handler
                .get_by_id(subscription_id, database_id)
                .await
                .context("Failed to get fixed database")?;

            let json_response =
                serde_json::to_value(database).context("Failed to serialize response")?;
            let data = handle_output(json_response, output_format, query)?;
            print_formatted_output(data, output_format)?;
            Ok(())
        }

        CloudFixedDatabaseCommands::Create {
            subscription_id,
            file,
            async_ops,
        } => {
            let json_string = read_file_input(file)?;
            let request: FixedDatabaseCreateRequest =
                serde_json::from_str(&json_string).context("Invalid database configuration")?;

            let result = handler
                .create(*subscription_id, &request)
                .await
                .context("Failed to create fixed database")?;

            let json_result =
                serde_json::to_value(&result).context("Failed to serialize response")?;

            handle_async_response(
                conn_mgr,
                profile_name,
                json_result,
                async_ops,
                output_format,
                query,
                "Fixed database created successfully",
            )
            .await
        }

        CloudFixedDatabaseCommands::Update {
            id,
            file,
            async_ops,
        } => {
            let (subscription_id, database_id) = parse_fixed_database_id(id)?;
            let json_string = read_file_input(file)?;
            let request: FixedDatabaseUpdateRequest =
                serde_json::from_str(&json_string).context("Invalid update configuration")?;

            let result = handler
                .update(subscription_id, database_id, &request)
                .await
                .context("Failed to update fixed database")?;

            let json_result =
                serde_json::to_value(&result).context("Failed to serialize response")?;

            handle_async_response(
                conn_mgr,
                profile_name,
                json_result,
                async_ops,
                output_format,
                query,
                "Fixed database updated successfully",
            )
            .await
        }

        CloudFixedDatabaseCommands::Delete { id, yes, async_ops } => {
            let (subscription_id, database_id) = parse_fixed_database_id(id)?;

            if !yes {
                let prompt = format!("Delete fixed database {}:{}?", subscription_id, database_id);
                if !confirm_action(&prompt)? {
                    eprintln!("Operation cancelled");
                    return Ok(());
                }
            }

            let result = handler
                .delete_by_id(subscription_id, database_id)
                .await
                .context("Failed to delete fixed database")?;

            let json_result =
                serde_json::to_value(&result).context("Failed to serialize response")?;

            handle_async_response(
                conn_mgr,
                profile_name,
                json_result,
                async_ops,
                output_format,
                query,
                "Fixed database deleted successfully",
            )
            .await
        }

        CloudFixedDatabaseCommands::BackupStatus { id } => {
            let (subscription_id, database_id) = parse_fixed_database_id(id)?;
            let status = handler
                .get_backup_status(subscription_id, database_id)
                .await
                .context("Failed to get backup status")?;

            let json_response =
                serde_json::to_value(status).context("Failed to serialize response")?;
            let data = handle_output(json_response, output_format, query)?;
            print_formatted_output(data, output_format)?;
            Ok(())
        }

        CloudFixedDatabaseCommands::Backup { id, async_ops } => {
            let (subscription_id, database_id) = parse_fixed_database_id(id)?;

            // Create a minimal backup request - most fields are optional
            let backup_request = FixedDatabaseBackupRequest {
                subscription_id: Some(subscription_id),
                database_id: Some(database_id),
                adhoc_backup_path: None,
                command_type: None,
                extra: serde_json::Value::Null,
            };

            let result = handler
                .backup(subscription_id, database_id, &backup_request)
                .await
                .context("Failed to initiate backup")?;

            let json_result =
                serde_json::to_value(&result).context("Failed to serialize response")?;

            handle_async_response(
                conn_mgr,
                profile_name,
                json_result,
                async_ops,
                output_format,
                query,
                "Backup initiated successfully",
            )
            .await
        }

        CloudFixedDatabaseCommands::ImportStatus { id } => {
            let (subscription_id, database_id) = parse_fixed_database_id(id)?;
            let status = handler
                .get_import_status(subscription_id, database_id)
                .await
                .context("Failed to get import status")?;

            let json_response =
                serde_json::to_value(status).context("Failed to serialize response")?;
            let data = handle_output(json_response, output_format, query)?;
            print_formatted_output(data, output_format)?;
            Ok(())
        }

        CloudFixedDatabaseCommands::Import {
            id,
            file,
            async_ops,
        } => {
            let (subscription_id, database_id) = parse_fixed_database_id(id)?;
            let json_string = read_file_input(file)?;
            let request: FixedDatabaseImportRequest =
                serde_json::from_str(&json_string).context("Invalid import configuration")?;

            let result = handler
                .import(subscription_id, database_id, &request)
                .await
                .context("Failed to initiate import")?;

            let json_result =
                serde_json::to_value(&result).context("Failed to serialize response")?;

            handle_async_response(
                conn_mgr,
                profile_name,
                json_result,
                async_ops,
                output_format,
                query,
                "Import initiated successfully",
            )
            .await
        }

        CloudFixedDatabaseCommands::SlowLog {
            id,
            limit: _,
            offset: _,
        } => {
            let (subscription_id, database_id) = parse_fixed_database_id(id)?;
            // Note: The API doesn't currently support limit/offset parameters
            let result = handler
                .get_slow_log(subscription_id, database_id)
                .await
                .context("Failed to get slow log")?;

            let json_result =
                serde_json::to_value(result).context("Failed to serialize response")?;
            let data = handle_output(json_result, output_format, query)?;
            print_formatted_output(data, output_format)?;
            Ok(())
        }

        CloudFixedDatabaseCommands::ListTags { id } => {
            let (subscription_id, database_id) = parse_fixed_database_id(id)?;
            let tags = handler
                .get_tags(subscription_id, database_id)
                .await
                .context("Failed to get tags")?;

            let json_response =
                serde_json::to_value(tags).context("Failed to serialize response")?;
            let data = handle_output(json_response, output_format, query)?;
            print_formatted_output(data, output_format)?;
            Ok(())
        }

        CloudFixedDatabaseCommands::AddTag { id, key, value } => {
            let (subscription_id, database_id) = parse_fixed_database_id(id)?;
            let tag_request = DatabaseTagCreateRequest {
                subscription_id: Some(subscription_id),
                database_id: Some(database_id),
                command_type: None,
                key: key.clone(),
                value: value.clone(),
                extra: serde_json::Value::Null,
            };

            let result = handler
                .create_tag(subscription_id, database_id, &tag_request)
                .await
                .context("Failed to add tag")?;

            let json_result =
                serde_json::to_value(result).context("Failed to serialize response")?;
            let data = handle_output(json_result, output_format, query)?;
            print_formatted_output(data, output_format)?;
            Ok(())
        }

        CloudFixedDatabaseCommands::UpdateTags { id, file } => {
            let (subscription_id, database_id) = parse_fixed_database_id(id)?;
            let json_string = read_file_input(file)?;

            // Parse the JSON directly into the expected format
            let parsed: serde_json::Value =
                serde_json::from_str(&json_string).context("Invalid tags configuration")?;

            // Extract tags array or create from object
            let tags_vec = if let Some(tags_array) = parsed.get("tags").and_then(|v| v.as_array()) {
                tags_array.clone()
            } else if parsed.is_object() {
                // If it's just an object, wrap it in an array
                vec![parsed]
            } else {
                return Err(
                    anyhow::anyhow!("Invalid tags format. Expected object or array.").into(),
                );
            };

            // Build the request with the proper structure
            let tags_request = serde_json::json!({
                "subscription_id": subscription_id,
                "database_id": database_id,
                "tags": tags_vec
            });

            // Use raw API call since the types don't match exactly
            let client = conn_mgr
                .create_cloud_client(profile_name)
                .await
                .context("Failed to create Cloud client")?;

            let result = client
                .put_raw(
                    &format!(
                        "/fixed/subscriptions/{}/databases/{}/tags",
                        subscription_id, database_id
                    ),
                    tags_request,
                )
                .await
                .context("Failed to update tags")?;

            let data = handle_output(result, output_format, query)?;
            print_formatted_output(data, output_format)?;
            Ok(())
        }

        CloudFixedDatabaseCommands::UpdateTag { id, key, value } => {
            let (subscription_id, database_id) = parse_fixed_database_id(id)?;
            let tag_request = DatabaseTagUpdateRequest {
                subscription_id: Some(subscription_id),
                database_id: Some(database_id),
                command_type: None,
                key: Some(key.clone()),
                value: value.clone(),
                extra: serde_json::Value::Null,
            };

            let result = handler
                .update_tag(subscription_id, database_id, key.clone(), &tag_request)
                .await
                .context("Failed to update tag")?;

            let json_result =
                serde_json::to_value(result).context("Failed to serialize response")?;
            let data = handle_output(json_result, output_format, query)?;
            print_formatted_output(data, output_format)?;
            Ok(())
        }

        CloudFixedDatabaseCommands::DeleteTag { id, key } => {
            let (subscription_id, database_id) = parse_fixed_database_id(id)?;

            let _result = handler
                .delete_tag(subscription_id, database_id, key.clone())
                .await
                .context("Failed to delete tag")?;

            eprintln!("Tag '{}' deleted successfully", key);
            Ok(())
        }
    }
}
