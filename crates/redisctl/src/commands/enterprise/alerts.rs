use crate::error::RedisCtlError;
use anyhow::Result as AnyhowResult;
use clap::Subcommand;
use serde_json::Value;

use crate::{cli::OutputFormat, config::Config};

#[derive(Debug, Subcommand)]
pub enum AlertsCommands {
    /// List all alerts
    List {
        /// Filter by alert type (cluster, node, bdb)
        #[arg(long)]
        filter_type: Option<String>,
        /// Filter by severity (info, warning, error, critical)
        #[arg(long)]
        severity: Option<String>,
    },
    /// Get specific alert
    Get {
        /// Alert UID
        uid: u64,
    },
    /// Get cluster alerts
    Cluster {
        /// Specific alert name
        #[arg(long)]
        alert: Option<String>,
    },
    /// Get node alerts
    Node {
        /// Node UID (optional, defaults to all nodes)
        node_uid: Option<u64>,
        /// Specific alert name
        #[arg(long)]
        alert: Option<String>,
    },
    /// Get database alerts
    Database {
        /// Database UID (optional, defaults to all databases)
        bdb_uid: Option<u64>,
        /// Specific alert name
        #[arg(long)]
        alert: Option<String>,
    },
    /// Get alert settings
    #[command(name = "settings-get")]
    SettingsGet,
    /// Update alert settings
    #[command(name = "settings-update")]
    SettingsUpdate {
        /// JSON data for alert settings (use '-' for stdin)
        #[arg(long)]
        data: String,
    },
}

impl AlertsCommands {
    #[allow(dead_code)]
    pub async fn execute(
        &self,
        config: &Config,
        profile_name: Option<&str>,
        output_format: OutputFormat,
        query: Option<&str>,
    ) -> AnyhowResult<()> {
        let conn_manager = crate::connection::ConnectionManager::new(config.clone());

        match self {
            Self::List {
                filter_type,
                severity,
            } => {
                handle_list_alerts(
                    &conn_manager,
                    profile_name,
                    filter_type.as_deref(),
                    severity.as_deref(),
                    output_format,
                    query,
                )
                .await
            }
            Self::Get { uid } => {
                handle_get_alert(&conn_manager, profile_name, *uid, output_format, query).await
            }
            Self::Cluster { alert } => {
                handle_cluster_alerts(
                    &conn_manager,
                    profile_name,
                    alert.as_deref(),
                    output_format,
                    query,
                )
                .await
            }
            Self::Node { node_uid, alert } => {
                handle_node_alerts(
                    &conn_manager,
                    profile_name,
                    *node_uid,
                    alert.as_deref(),
                    output_format,
                    query,
                )
                .await
            }
            Self::Database { bdb_uid, alert } => {
                handle_database_alerts(
                    &conn_manager,
                    profile_name,
                    *bdb_uid,
                    alert.as_deref(),
                    output_format,
                    query,
                )
                .await
            }
            Self::SettingsGet => {
                handle_get_alert_settings(&conn_manager, profile_name, output_format, query).await
            }
            Self::SettingsUpdate { data } => {
                handle_update_alert_settings(
                    &conn_manager,
                    profile_name,
                    data,
                    output_format,
                    query,
                )
                .await
            }
        }
    }
}

async fn handle_list_alerts(
    conn_mgr: &crate::connection::ConnectionManager,
    profile_name: Option<&str>,
    filter_type: Option<&str>,
    severity: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> AnyhowResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    // Get alerts from all sources and combine
    let mut all_alerts = Vec::new();

    // Get cluster alerts
    if (filter_type.is_none() || filter_type == Some("cluster"))
        && let Ok(cluster_alerts) = client.get::<Value>("/v1/cluster/alerts").await
        && let Some(alerts) = cluster_alerts.as_array()
    {
        for alert in alerts {
            let mut alert = alert.clone();
            if let Some(obj) = alert.as_object_mut() {
                obj.insert("type".to_string(), Value::String("cluster".to_string()));
            }
            all_alerts.push(alert);
        }
    }

    // Get node alerts
    if (filter_type.is_none() || filter_type == Some("node"))
        && let Ok(node_alerts) = client.get::<Value>("/v1/nodes/alerts").await
        && let Some(alerts) = node_alerts.as_array()
    {
        for alert in alerts {
            let mut alert = alert.clone();
            if let Some(obj) = alert.as_object_mut() {
                obj.insert("type".to_string(), Value::String("node".to_string()));
            }
            all_alerts.push(alert);
        }
    }

    // Get database alerts
    if (filter_type.is_none() || filter_type == Some("bdb"))
        && let Ok(bdb_alerts) = client.get::<Value>("/v1/bdbs/alerts").await
        && let Some(alerts) = bdb_alerts.as_array()
    {
        for alert in alerts {
            let mut alert = alert.clone();
            if let Some(obj) = alert.as_object_mut() {
                obj.insert("type".to_string(), Value::String("database".to_string()));
            }
            all_alerts.push(alert);
        }
    }

    // Filter by severity if specified
    if let Some(severity) = severity {
        all_alerts.retain(|alert| {
            alert
                .get("severity")
                .and_then(|s| s.as_str())
                .map(|s| s.eq_ignore_ascii_case(severity))
                .unwrap_or(false)
        });
    }

    let response = Value::Array(all_alerts);

    // Apply JMESPath query if provided
    let response = if let Some(q) = query {
        super::utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    super::utils::print_formatted_output(response, output_format).map_err(|e| anyhow::anyhow!(e))
}

async fn handle_get_alert(
    conn_mgr: &crate::connection::ConnectionManager,
    profile_name: Option<&str>,
    uid: u64,
    output_format: OutputFormat,
    query: Option<&str>,
) -> AnyhowResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    // Try to find the alert in different endpoints
    // First try cluster alerts
    if let Ok(alerts) = client
        .get::<Value>(&format!("/v1/cluster/alerts/{}", uid))
        .await
    {
        let response = if let Some(q) = query {
            super::utils::apply_jmespath(&alerts, q)?
        } else {
            alerts
        };
        return super::utils::print_formatted_output(response, output_format)
            .map_err(|e| anyhow::anyhow!(e));
    }

    // Try node alerts
    if let Ok(alerts) = client
        .get::<Value>(&format!("/v1/nodes/alerts/{}", uid))
        .await
    {
        let response = if let Some(q) = query {
            super::utils::apply_jmespath(&alerts, q)?
        } else {
            alerts
        };
        return super::utils::print_formatted_output(response, output_format)
            .map_err(|e| anyhow::anyhow!(e));
    }

    // Try database alerts
    if let Ok(alerts) = client
        .get::<Value>(&format!("/v1/bdbs/alerts/{}", uid))
        .await
    {
        let response = if let Some(q) = query {
            super::utils::apply_jmespath(&alerts, q)?
        } else {
            alerts
        };
        return super::utils::print_formatted_output(response, output_format)
            .map_err(|e| anyhow::anyhow!(e));
    }

    anyhow::bail!("Alert with UID {} not found", uid)
}

async fn handle_cluster_alerts(
    conn_mgr: &crate::connection::ConnectionManager,
    profile_name: Option<&str>,
    alert: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> AnyhowResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let endpoint = if let Some(alert_name) = alert {
        format!("/v1/cluster/alerts/{}", alert_name)
    } else {
        "/v1/cluster/alerts".to_string()
    };

    let response = client
        .get::<Value>(&endpoint)
        .await
        .map_err(|e| RedisCtlError::from(e))?;

    let response = if let Some(q) = query {
        super::utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    super::utils::print_formatted_output(response, output_format).map_err(|e| anyhow::anyhow!(e))
}

async fn handle_node_alerts(
    conn_mgr: &crate::connection::ConnectionManager,
    profile_name: Option<&str>,
    node_uid: Option<u64>,
    alert: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> AnyhowResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let endpoint = match (node_uid, alert) {
        (Some(uid), Some(alert_name)) => format!("/v1/nodes/alerts/{}/{}", uid, alert_name),
        (Some(uid), None) => format!("/v1/nodes/alerts/{}", uid),
        (None, None) => "/v1/nodes/alerts".to_string(),
        (None, Some(_)) => anyhow::bail!("Cannot specify alert without node_uid"),
    };

    let response = client
        .get::<Value>(&endpoint)
        .await
        .map_err(|e| RedisCtlError::from(e))?;

    let response = if let Some(q) = query {
        super::utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    super::utils::print_formatted_output(response, output_format).map_err(|e| anyhow::anyhow!(e))
}

async fn handle_database_alerts(
    conn_mgr: &crate::connection::ConnectionManager,
    profile_name: Option<&str>,
    bdb_uid: Option<u64>,
    alert: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> AnyhowResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let endpoint = match (bdb_uid, alert) {
        (Some(uid), Some(alert_name)) => format!("/v1/bdbs/alerts/{}/{}", uid, alert_name),
        (Some(uid), None) => format!("/v1/bdbs/alerts/{}", uid),
        (None, None) => "/v1/bdbs/alerts".to_string(),
        (None, Some(_)) => anyhow::bail!("Cannot specify alert without bdb_uid"),
    };

    let response = client
        .get::<Value>(&endpoint)
        .await
        .map_err(|e| RedisCtlError::from(e))?;

    let response = if let Some(q) = query {
        super::utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    super::utils::print_formatted_output(response, output_format).map_err(|e| anyhow::anyhow!(e))
}

async fn handle_get_alert_settings(
    conn_mgr: &crate::connection::ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> AnyhowResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    // Alert settings are part of cluster configuration
    let response = client
        .get::<Value>("/v1/cluster")
        .await
        .map_err(|e| RedisCtlError::from(e))?;

    // Extract alert_settings from the cluster config
    let alert_settings = response
        .get("alert_settings")
        .cloned()
        .unwrap_or_else(|| Value::Object(serde_json::Map::new()));

    let response = if let Some(q) = query {
        super::utils::apply_jmespath(&alert_settings, q)?
    } else {
        alert_settings
    };

    super::utils::print_formatted_output(response, output_format).map_err(|e| anyhow::anyhow!(e))
}

async fn handle_update_alert_settings(
    conn_mgr: &crate::connection::ConnectionManager,
    profile_name: Option<&str>,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> AnyhowResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let json_data = super::utils::read_json_data(data)?;

    // Alert settings are updated through cluster configuration
    let update_payload = serde_json::json!({
        "alert_settings": json_data
    });

    let response = client
        .put::<_, Value>("/v1/cluster", &update_payload)
        .await
        .map_err(|e| RedisCtlError::from(e))?;

    let response = if let Some(q) = query {
        super::utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    super::utils::print_formatted_output(response, output_format).map_err(|e| anyhow::anyhow!(e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_alerts_command_structure() {
        // Test that all alerts commands can be constructed

        // List command
        let _cmd = AlertsCommands::List {
            filter_type: None,
            severity: None,
        };

        let _cmd = AlertsCommands::List {
            filter_type: Some("cluster".to_string()),
            severity: Some("error".to_string()),
        };

        // Get command
        let _cmd = AlertsCommands::Get { uid: 1 };

        // Cluster alerts
        let _cmd = AlertsCommands::Cluster { alert: None };
        let _cmd = AlertsCommands::Cluster {
            alert: Some("test_alert".to_string()),
        };

        // Node alerts
        let _cmd = AlertsCommands::Node {
            node_uid: None,
            alert: None,
        };
        let _cmd = AlertsCommands::Node {
            node_uid: Some(1),
            alert: Some("test".to_string()),
        };

        // Database alerts
        let _cmd = AlertsCommands::Database {
            bdb_uid: None,
            alert: None,
        };
        let _cmd = AlertsCommands::Database {
            bdb_uid: Some(1),
            alert: Some("test".to_string()),
        };

        // Settings commands
        let _cmd = AlertsCommands::SettingsGet;
        let _cmd = AlertsCommands::SettingsUpdate {
            data: "{}".to_string(),
        };
    }
}
