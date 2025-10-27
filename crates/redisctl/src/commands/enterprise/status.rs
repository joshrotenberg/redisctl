//! Comprehensive status command implementation for Redis Enterprise
//!
//! Provides a single command to view cluster, nodes, databases, and shards status,
//! similar to `rladmin status extra all`.

use crate::cli::OutputFormat;
use crate::connection::ConnectionManager;
use crate::error::Result as CliResult;
use anyhow::Context;
use redis_enterprise::bdb::BdbHandler;
use redis_enterprise::cluster::ClusterHandler;
use redis_enterprise::nodes::NodeHandler;
use redis_enterprise::shards::ShardHandler;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::utils::*;

/// Comprehensive cluster status information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ClusterStatus {
    /// Cluster information
    pub cluster: Value,
    /// List of nodes
    pub nodes: Value,
    /// List of databases
    pub databases: Value,
    /// List of shards
    pub shards: Value,
    /// Summary statistics
    pub summary: StatusSummary,
}

/// Summary statistics for cluster health
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct StatusSummary {
    /// Total number of nodes
    pub total_nodes: usize,
    /// Number of healthy nodes
    pub healthy_nodes: usize,
    /// Total number of databases
    pub total_databases: usize,
    /// Number of active databases
    pub active_databases: usize,
    /// Total number of shards
    pub total_shards: usize,
    /// Cluster health status
    pub cluster_health: String,
}

/// Sections to display in status output
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct StatusSections {
    /// Show cluster information
    pub cluster: bool,
    /// Show nodes information
    pub nodes: bool,
    /// Show databases information
    pub databases: bool,
    /// Show shards information
    pub shards: bool,
}

impl StatusSections {
    /// Create sections showing all information
    #[allow(dead_code)]
    pub fn all() -> Self {
        Self {
            cluster: true,
            nodes: true,
            databases: true,
            shards: true,
        }
    }

    /// Check if any section is enabled
    #[allow(dead_code)]
    pub fn any_enabled(&self) -> bool {
        self.cluster || self.nodes || self.databases || self.shards
    }
}

/// Get comprehensive cluster status
#[allow(dead_code)]
pub async fn get_status(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    sections: StatusSections,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    // Use provided sections, or default to all if none specified
    let sections = if sections.any_enabled() {
        sections
    } else {
        StatusSections::all()
    };

    // Collect cluster info
    let cluster_result = if sections.cluster {
        ClusterHandler::new(client.clone())
            .info()
            .await
            .map(|v| serde_json::to_value(v).unwrap_or(json!({})))
            .context("Failed to get cluster info")?
    } else {
        json!({})
    };

    // Collect nodes
    let nodes_result = if sections.nodes {
        NodeHandler::new(client.clone())
            .list()
            .await
            .map(|v| serde_json::to_value(v).unwrap_or(json!([])))
            .context("Failed to list nodes")?
    } else {
        json!([])
    };

    // Collect databases
    let databases_result = if sections.databases {
        BdbHandler::new(client.clone())
            .list()
            .await
            .map(|v| serde_json::to_value(v).unwrap_or(json!([])))
            .context("Failed to list databases")?
    } else {
        json!([])
    };

    // Collect shards
    let shards_result = if sections.shards {
        ShardHandler::new(client.clone())
            .list()
            .await
            .map(|v| serde_json::to_value(v).unwrap_or(json!([])))
            .context("Failed to list shards")?
    } else {
        json!([])
    };

    // Calculate summary statistics
    let summary = calculate_summary(&nodes_result, &databases_result, &shards_result);

    // Build comprehensive status
    let status = ClusterStatus {
        cluster: cluster_result,
        nodes: nodes_result,
        databases: databases_result,
        shards: shards_result,
        summary,
    };

    let status_json = serde_json::to_value(status).context("Failed to serialize cluster status")?;

    // Apply query if provided
    let data = handle_output(status_json, output_format, query)?;

    // Format and display
    print_formatted_output(data, output_format)?;

    Ok(())
}

/// Calculate summary statistics from collected data
#[allow(dead_code)]
fn calculate_summary(nodes: &Value, databases: &Value, shards: &Value) -> StatusSummary {
    let empty_vec = vec![];
    let nodes_array = nodes.as_array().unwrap_or(&empty_vec);
    let databases_array = databases.as_array().unwrap_or(&empty_vec);
    let shards_array = shards.as_array().unwrap_or(&empty_vec);

    let total_nodes = nodes_array.len();
    let healthy_nodes = nodes_array
        .iter()
        .filter(|n| {
            n.get("status")
                .and_then(|s| s.as_str())
                .map(|s| s == "active" || s == "ok")
                .unwrap_or(false)
        })
        .count();

    let total_databases = databases_array.len();
    let active_databases = databases_array
        .iter()
        .filter(|db| {
            db.get("status")
                .and_then(|s| s.as_str())
                .map(|s| s == "active")
                .unwrap_or(false)
        })
        .count();

    let total_shards = shards_array.len();

    // Determine cluster health
    let cluster_health = if healthy_nodes == total_nodes && active_databases == total_databases {
        "healthy".to_string()
    } else if healthy_nodes == 0 || active_databases == 0 {
        "critical".to_string()
    } else {
        "degraded".to_string()
    };

    StatusSummary {
        total_nodes,
        healthy_nodes,
        total_databases,
        active_databases,
        total_shards,
        cluster_health,
    }
}
