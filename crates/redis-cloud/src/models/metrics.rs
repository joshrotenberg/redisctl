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
