//! Database-related data models

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Cloud database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudDatabase {
    pub db_id: u32,
    pub name: String,
    pub protocol: String,
    pub provider: String,
    pub region: String,
    pub status: String,
    pub memory_limit_in_gb: f64,
    pub memory_used_in_mb: Option<f64>,
    pub memory_usage: Option<f64>,
    pub data_persistence: String,
    pub replication: bool,
    pub data_eviction: Option<String>,
    pub throughput_measurement: Option<ThroughputMeasurement>,
    pub activated_on: Option<String>,
    pub last_modified: Option<String>,
    pub public_endpoint: Option<String>,
    pub private_endpoint: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThroughputMeasurement {
    pub by: String,
    pub value: u32,
}

/// Create database request
#[derive(Debug, Serialize)]
pub struct CreateDatabaseRequest {
    pub name: String,
    pub memory_limit_in_gb: f64,
    pub data_persistence: String,
    pub replication: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_eviction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_oss_cluster_api: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_external_endpoint_for_oss_cluster_api: Option<bool>,
}

/// Update database request
#[derive(Debug, Serialize)]
pub struct UpdateDatabaseRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_limit_in_gb: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_persistence: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_eviction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
