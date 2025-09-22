//! Enterprise statistics and metrics commands
#![allow(dead_code)]


use crate::cli::{EnterpriseStatsCommands, OutputFormat};
use crate::connection::ConnectionManager;
use crate::error::Result as CliResult;
use anyhow::Context;
use redis_enterprise::stats::StatsHandler;

use super::utils::*;

/// Handle enterprise stats commands
pub async fn handle_stats_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    cmd: &EnterpriseStatsCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    match cmd {
        EnterpriseStatsCommands::Database { id } => {
            handle_database_stats(conn_mgr, profile_name, *id, output_format, query).await
        }
        EnterpriseStatsCommands::DatabaseShards { id } => {
            handle_database_shard_stats(conn_mgr, profile_name, *id, output_format, query).await
        }
        EnterpriseStatsCommands::DatabaseMetrics { id, interval } => {
            handle_database_metrics(conn_mgr, profile_name, *id, interval, output_format, query)
                .await
        }
        EnterpriseStatsCommands::Node { id } => {
            handle_node_stats(conn_mgr, profile_name, *id, output_format, query).await
        }
        EnterpriseStatsCommands::NodeMetrics { id, interval } => {
            handle_node_metrics(conn_mgr, profile_name, *id, interval, output_format, query).await
        }
        EnterpriseStatsCommands::Cluster => {
            handle_cluster_stats(conn_mgr, profile_name, output_format, query).await
        }
        EnterpriseStatsCommands::ClusterMetrics { interval } => {
            handle_cluster_metrics(conn_mgr, profile_name, interval, output_format, query).await
        }
        EnterpriseStatsCommands::Listener => {
            handle_listener_stats(conn_mgr, profile_name, output_format, query).await
        }
        EnterpriseStatsCommands::Export { format, interval } => {
            handle_stats_export(
                conn_mgr,
                profile_name,
                format,
                interval.as_deref(),
                output_format,
                query,
            )
            .await
        }
    }
}

/// Handle database statistics
async fn handle_database_stats(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    database_id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let stats_handler = StatsHandler::new(client);
    let response = stats_handler.database_last(database_id).await?;
    let stats_json = serde_json::to_value(response).context("Failed to serialize stats")?;
    let data = handle_output(stats_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Handle database shard statistics
async fn handle_database_shard_stats(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    database_id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let stats_handler = StatsHandler::new(client);
    let response = stats_handler.shard(database_id, None).await?;
    let stats_json = serde_json::to_value(response).context("Failed to serialize stats")?;
    let data = handle_output(stats_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Handle database metrics over time
async fn handle_database_metrics(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    database_id: u32,
    interval: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let stats_handler = StatsHandler::new(client);
    let stats_query = redis_enterprise::stats::StatsQuery {
        interval: Some(interval.to_string()),
        stime: None,
        etime: None,
        metrics: None,
    };
    let response = stats_handler
        .database(database_id, Some(stats_query))
        .await?;
    let stats_json = serde_json::to_value(response).context("Failed to serialize stats")?;
    let data = handle_output(stats_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Handle node statistics
async fn handle_node_stats(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    node_id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let stats_handler = StatsHandler::new(client);
    let response = stats_handler.node_last(node_id).await?;
    let stats_json = serde_json::to_value(response).context("Failed to serialize stats")?;
    let data = handle_output(stats_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Handle node metrics over time
async fn handle_node_metrics(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    node_id: u32,
    interval: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let stats_handler = StatsHandler::new(client);
    let stats_query = redis_enterprise::stats::StatsQuery {
        interval: Some(interval.to_string()),
        stime: None,
        etime: None,
        metrics: None,
    };
    let response = stats_handler.node(node_id, Some(stats_query)).await?;
    let stats_json = serde_json::to_value(response).context("Failed to serialize stats")?;
    let data = handle_output(stats_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Handle cluster statistics
async fn handle_cluster_stats(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let stats_handler = StatsHandler::new(client);
    let response = stats_handler.cluster_last().await?;
    let stats_json = serde_json::to_value(response).context("Failed to serialize stats")?;
    let data = handle_output(stats_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Handle cluster metrics over time
async fn handle_cluster_metrics(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    interval: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let stats_handler = StatsHandler::new(client);
    let stats_query = redis_enterprise::stats::StatsQuery {
        interval: Some(interval.to_string()),
        stime: None,
        etime: None,
        metrics: None,
    };
    let response = stats_handler.cluster(Some(stats_query)).await?;
    let stats_json = serde_json::to_value(response).context("Failed to serialize stats")?;
    let data = handle_output(stats_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Handle listener statistics
async fn handle_listener_stats(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    // Note: Listener stats might need to be implemented via raw API call
    // For now, using cluster stats as fallback
    let stats_handler = StatsHandler::new(client);
    let response = stats_handler.cluster_last().await?;
    let stats_json = serde_json::to_value(response).context("Failed to serialize stats")?;
    let data = handle_output(stats_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

/// Handle statistics export
async fn handle_stats_export(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    format: &str,
    interval: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let stats_handler = StatsHandler::new(client);

    // For export, collect all relevant stats based on format
    let response = match format.to_lowercase().as_str() {
        "prometheus" | "json" | "csv" => {
            // Create separate query instances since StatsQuery doesn't implement Clone
            let create_query = || {
                interval.map(|i| redis_enterprise::stats::StatsQuery {
                    interval: Some(i.to_string()),
                    stime: None,
                    etime: None,
                    metrics: None,
                })
            };

            // Collect cluster, nodes, and databases stats
            let cluster_stats = stats_handler.cluster(create_query()).await?;
            let nodes_stats = stats_handler.nodes(create_query()).await?;
            let databases_stats = stats_handler.databases(create_query()).await?;

            serde_json::json!({
                "cluster": cluster_stats,
                "nodes": nodes_stats,
                "databases": databases_stats,
                "export_format": format
            })
        }
        _ => {
            return Err(anyhow::anyhow!(
                "Unsupported export format: {}. Use json, prometheus, or csv",
                format
            )
            .into());
        }
    };

    let data = handle_output(response, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_stats_command_structure() {
        // Test that all stats commands can be constructed

        // Database stats
        let _cmd = EnterpriseStatsCommands::Database { id: 1 };

        // Node stats
        let _cmd = EnterpriseStatsCommands::Node { id: 1 };

        // Cluster stats
        let _cmd = EnterpriseStatsCommands::Cluster;

        // Export stats
        let _cmd = EnterpriseStatsCommands::Export {
            format: "prometheus".to_string(),
            interval: Some("1h".to_string()),
        };
    }
}
