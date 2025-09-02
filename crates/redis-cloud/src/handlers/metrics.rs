//! Metrics operations handler

use crate::{Result, client::CloudClient, models::CloudMetrics};
use serde_json::Value;

/// Handler for Cloud metrics operations
pub struct CloudMetricsHandler {
    client: CloudClient,
}

impl CloudMetricsHandler {
    pub fn new(client: CloudClient) -> Self {
        CloudMetricsHandler { client }
    }

    /// Get database metrics
    pub async fn database(
        &self,
        subscription_id: u32,
        database_id: u32,
        metric_names: Vec<String>,
        from: Option<String>,
        to: Option<String>,
    ) -> Result<CloudMetrics> {
        let mut query_params = vec![];

        for name in metric_names {
            query_params.push(format!("metricSpecs={}", name));
        }

        if let Some(from_time) = from {
            query_params.push(format!("from={}", from_time));
        }

        if let Some(to_time) = to {
            query_params.push(format!("to={}", to_time));
        }

        let query_string = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{}", query_params.join("&"))
        };

        self.client
            .get(&format!(
                "/subscriptions/{}/databases/{}/metrics{}",
                subscription_id, database_id, query_string
            ))
            .await
    }

    /// Get subscription metrics
    pub async fn subscription(
        &self,
        subscription_id: u32,
        from: Option<String>,
        to: Option<String>,
    ) -> Result<Value> {
        let mut query_params = vec![];

        if let Some(from_time) = from {
            query_params.push(format!("from={}", from_time));
        }

        if let Some(to_time) = to {
            query_params.push(format!("to={}", to_time));
        }

        let query_string = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{}", query_params.join("&"))
        };

        self.client
            .get(&format!(
                "/subscriptions/{}/metrics{}",
                subscription_id, query_string
            ))
            .await
    }
}
