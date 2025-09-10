//! Implementation of enterprise module commands
#![allow(dead_code)]

use crate::cli::OutputFormat;
use crate::commands::enterprise::module::ModuleCommands;
use crate::connection::ConnectionManager;
use crate::error::Result as CliResult;
use anyhow::Context;
use redis_enterprise::ModuleHandler;
use std::fs;
use std::path::Path;

pub async fn handle_module_commands(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    cmd: &ModuleCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    match cmd {
        ModuleCommands::List => handle_list(conn_mgr, profile_name, output_format, query).await,
        ModuleCommands::Get { uid } => {
            handle_get(conn_mgr, profile_name, uid, output_format, query).await
        }
        ModuleCommands::Upload { file } => {
            handle_upload(conn_mgr, profile_name, file, output_format, query).await
        }
        ModuleCommands::Delete { uid, force } => {
            handle_delete(conn_mgr, profile_name, uid, *force, output_format, query).await
        }
        ModuleCommands::ConfigBdb { bdb_uid, data } => {
            handle_config_bdb(conn_mgr, profile_name, *bdb_uid, data, output_format, query).await
        }
    }
}

async fn handle_list(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = ModuleHandler::new(client);

    let modules = handler.list().await.context("Failed to list modules")?;

    let modules_json = serde_json::to_value(&modules)?;
    let output_data = if let Some(q) = query {
        crate::commands::enterprise::utils::apply_jmespath(&modules_json, q)?
    } else {
        modules_json
    };

    crate::commands::enterprise::utils::print_formatted_output(output_data, output_format)?;
    Ok(())
}

async fn handle_get(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    uid: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = ModuleHandler::new(client);

    let module = handler.get(uid).await.context("Failed to get module")?;

    let module_json = serde_json::to_value(&module)?;
    let output_data = if let Some(q) = query {
        crate::commands::enterprise::utils::apply_jmespath(&module_json, q)?
    } else {
        module_json
    };

    crate::commands::enterprise::utils::print_formatted_output(output_data, output_format)?;
    Ok(())
}

async fn handle_upload(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    file: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    // Handle @file syntax
    let file_path = if let Some(path) = file.strip_prefix('@') {
        path
    } else {
        file
    };

    // Check if file exists
    if !Path::new(file_path).exists() {
        return Err(anyhow::anyhow!("Module file not found: {}", file_path).into());
    }

    // Read file contents
    let module_data = fs::read(file_path)
        .with_context(|| format!("Failed to read module file: {}", file_path))?;

    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = ModuleHandler::new(client);

    // Upload module
    let module = handler
        .upload(module_data)
        .await
        .context("Failed to upload module")?;

    let module_json = serde_json::to_value(&module)?;
    let output_data = if let Some(q) = query {
        crate::commands::enterprise::utils::apply_jmespath(&module_json, q)?
    } else {
        module_json
    };

    crate::commands::enterprise::utils::print_formatted_output(output_data, output_format)?;
    Ok(())
}

async fn handle_delete(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    uid: &str,
    force: bool,
    output_format: OutputFormat,
    _query: Option<&str>,
) -> CliResult<()> {
    // Confirm deletion if not forced
    if !force {
        let message = format!("Are you sure you want to delete module '{}'?", uid);
        if !crate::commands::enterprise::utils::confirm_action(&message)? {
            println!("Module deletion cancelled");
            return Ok(());
        }
    }

    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = ModuleHandler::new(client);

    handler
        .delete(uid)
        .await
        .context("Failed to delete module")?;

    // Print success message
    let result = serde_json::json!({
        "status": "success",
        "message": format!("Module '{}' deleted successfully", uid)
    });

    crate::commands::enterprise::utils::print_formatted_output(result, output_format)?;
    Ok(())
}

async fn handle_config_bdb(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    bdb_uid: u32,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = ModuleHandler::new(client);

    // Parse data (from file or inline JSON)
    let config = crate::commands::enterprise::utils::read_json_data(data)?;

    let result = handler
        .config_bdb(bdb_uid, config)
        .await
        .context("Failed to configure module for database")?;

    let result_json = serde_json::to_value(&result)?;
    let output_data = if let Some(q) = query {
        crate::commands::enterprise::utils::apply_jmespath(&result_json, q)?
    } else {
        result_json
    };

    crate::commands::enterprise::utils::print_formatted_output(output_data, output_format)?;
    Ok(())
}
