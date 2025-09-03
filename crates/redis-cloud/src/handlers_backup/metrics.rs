//! Metrics operations handler

use crate::{Result, client::CloudClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudMetrics {
    pub database_id: u32,
    pub subscription_id: u32,
    pub measurements: Vec<Measurement>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Measurement {
    pub name: String,
    pub values: Vec<MetricDataPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDataPoint {
    pub timestamp: String,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MetricValue {
    Number(f64),
    String(String),
    Array(Vec<f64>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionMetrics {
    pub subscription_id: u32,
    pub metrics: Vec<CloudMetrics>,
    #[serde(flatten)]
    pub extra: Value,
}

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
    ) -> Result<SubscriptionMetrics> {
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

        let v: Value = self
            .client
            .get(&format!(
                "/subscriptions/{}/metrics{}",
                subscription_id, query_string
            ))
            .await?;
        if let Some(obj) = v.get("subscriptionMetrics") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
    }
}
