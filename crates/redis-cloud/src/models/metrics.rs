//! Metrics-related data models

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Database metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudMetrics {
    pub database_id: u32,
    pub measurements: Vec<Measurement>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Measurement {
    pub name: String,
    pub values: Vec<MetricValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricValue {
    pub timestamp: String,
    pub value: f64,
}

/// Subscription-level metrics summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionMetrics {
    #[serde(rename = "subscriptionId")]
    pub subscription_id: u32,
    #[serde(rename = "totalMemoryUsage")]
    pub total_memory_usage: f64,
    #[serde(rename = "totalRequests")]
    pub total_requests: f64,
    pub databases: Option<Vec<SubscriptionDatabaseMetrics>>,

    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionDatabaseMetrics {
    #[serde(rename = "databaseId")]
    pub database_id: u32,
    #[serde(rename = "memoryUsage")]
    pub memory_usage: f64,
    pub requests: f64,

    #[serde(flatten)]
    pub extra: Value,
}
