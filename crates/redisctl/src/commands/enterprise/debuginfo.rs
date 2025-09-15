use crate::cli::OutputFormat;
use crate::commands::enterprise::utils;
use crate::connection::ConnectionManager;
use crate::error::RedisCtlError;
use anyhow::Context;
use clap::Subcommand;
use serde_json::Value;

#[derive(Debug, Clone, Subcommand)]
pub enum DebugInfoCommands {
    /// Collect all debug info
    All,

    /// Collect node debug info
    Node,

    /// Collect database-specific debug info
    Database {
        /// Database UID
        bdb_uid: u64,
    },
}

#[allow(dead_code)]
pub async fn handle_debuginfo_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    cmd: DebugInfoCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    match cmd {
        DebugInfoCommands::All => {
            handle_debuginfo_all(conn_mgr, profile_name, output_format, query).await
        }
        DebugInfoCommands::Node => {
            handle_debuginfo_node(conn_mgr, profile_name, output_format, query).await
        }
        DebugInfoCommands::Database { bdb_uid } => {
            handle_debuginfo_database(conn_mgr, profile_name, bdb_uid, output_format, query).await
        }
    }
}

#[allow(dead_code)]
async fn handle_debuginfo_all(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let response = client
        .get::<Value>("/v1/debuginfo/all")
        .await
        .context("Failed to collect all debug info")?;

    let result = if let Some(q) = query {
        utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    utils::print_formatted_output(result, output_format)
}

#[allow(dead_code)]
async fn handle_debuginfo_node(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let response = client
        .get::<Value>("/v1/debuginfo/node")
        .await
        .context("Failed to collect node debug info")?;

    let result = if let Some(q) = query {
        utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    utils::print_formatted_output(result, output_format)
}

#[allow(dead_code)]
async fn handle_debuginfo_database(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    bdb_uid: u64,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let endpoint = format!("/v1/debuginfo/node/bdb/{}", bdb_uid);
    let response = client.get::<Value>(&endpoint).await.context(format!(
        "Failed to collect debug info for database {}",
        bdb_uid
    ))?;

    let result = if let Some(q) = query {
        utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    utils::print_formatted_output(result, output_format)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debuginfo_commands() {
        use clap::CommandFactory;

        #[derive(clap::Parser)]
        struct TestCli {
            #[command(subcommand)]
            cmd: DebugInfoCommands,
        }

        TestCli::command().debug_assert();
    }
}
