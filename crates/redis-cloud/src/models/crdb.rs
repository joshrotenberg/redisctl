//! Active-Active (CRDB) models for Redis Cloud

use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudCrdb {
    #[serde(rename = "crdbId", alias = "id")]
    pub crdb_id: Option<u32>,
    pub name: String,
    pub status: Option<String>,
    pub protocol: Option<String>,
    #[serde(rename = "memoryLimitInGb")]
    pub memory_limit_in_gb: Option<f64>,
    #[serde(rename = "createdTimestamp")]
    pub created_timestamp: Option<String>,
    #[serde(rename = "updatedTimestamp")]
    pub updated_timestamp: Option<String>,
    pub regions: Option<Vec<CloudCrdbRegion>>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudCrdbRegion {
    #[serde(rename = "regionId", alias = "id")]
    pub region_id: Option<u32>,
    #[serde(rename = "regionName", alias = "region")]
    pub region_name: Option<String>,
    #[serde(rename = "subscriptionId")]
    pub subscription_id: Option<u32>,
    pub endpoint: Option<String>,
    pub status: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrdbTask {
    #[serde(rename = "taskId", alias = "id")]
    pub task_id: String,
    #[serde(rename = "commandType")]
    pub command_type: Option<String>,
    pub status: String,
    pub description: Option<String>,
    #[serde(rename = "timestamp", alias = "created_at")]
    pub timestamp: Option<String>,
    pub response: Option<Value>,
    pub progress: Option<Value>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrdbMetrics {
    #[serde(rename = "crdbId")]
    pub crdb_id: Option<u32>,
    pub period: Option<String>,
    #[serde(rename = "sampleRate")]
    pub sample_rate: Option<String>,
    pub data: Value,
    #[serde(rename = "regionMetrics")]
    pub region_metrics: Option<Vec<CrdbRegionMetric>>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrdbRegionMetric {
    #[serde(rename = "regionId")]
    pub region_id: Option<u32>,
    #[serde(rename = "regionName")]
    pub region_name: Option<String>,
    pub memory: Option<f64>,
    pub ops: Option<u64>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CreateCrdbRequest {
    pub name: String,
    #[builder(default, setter(strip_option))]
    #[serde(rename = "memoryLimitInGb")]
    pub memory_limit_in_gb: Option<f64>,
    #[builder(default, setter(strip_option))]
    pub regions: Option<Vec<CloudCrdbRegion>>, // minimal
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UpdateCrdbRequest {
    #[builder(default, setter(into, strip_option))]
    pub name: Option<String>,
    #[builder(default, setter(strip_option))]
    #[serde(rename = "memoryLimitInGb")]
    pub memory_limit_in_gb: Option<f64>,
}
