//! Implementation of enterprise logs commands
#![allow(dead_code)]

use crate::cli::OutputFormat;
use crate::commands::enterprise::logs::LogsCommands;
use crate::connection::ConnectionManager;
use crate::error::Result as CliResult;
use anyhow::Context;
use redis_enterprise::logs::LogsQuery;

/// Parameters for log list operation
struct LogListParams {
    since: Option<String>,
    until: Option<String>,
    order: Option<String>,
    limit: Option<u32>,
    offset: Option<u32>,
}

pub async fn handle_logs_commands(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    cmd: &LogsCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    match cmd {
        LogsCommands::List {
            since,
            until,
            order,
            limit,
            offset,
        } => {
            let params = LogListParams {
                since: since.clone(),
                until: until.clone(),
                order: order.clone(),
                limit: *limit,
                offset: *offset,
            };
            handle_list_logs(conn_mgr, profile_name, params, output_format, query).await
        }
    }
}

async fn handle_list_logs(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    params: LogListParams,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = redis_enterprise::LogsHandler::new(client);

    let logs_query = if params.since.is_some()
        || params.until.is_some()
        || params.order.is_some()
        || params.limit.is_some()
        || params.offset.is_some()
    {
        Some(LogsQuery {
            stime: params.since,
            etime: params.until,
            order: params.order,
            limit: params.limit,
            offset: params.offset,
        })
    } else {
        None
    };

    let logs = handler
        .list(logs_query)
        .await
        .context("Failed to retrieve cluster logs")?;

    // Convert to JSON value for output
    let logs_json = serde_json::to_value(&logs)?;

    // Handle output with optional JMESPath query
    let output_data = if let Some(q) = query {
        crate::commands::enterprise::utils::apply_jmespath(&logs_json, q)?
    } else {
        logs_json
    };

    // Print the output
    crate::commands::enterprise::utils::print_formatted_output(output_data, output_format)?;

    Ok(())
}
