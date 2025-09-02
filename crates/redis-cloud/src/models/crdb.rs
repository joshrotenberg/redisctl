//! Active-Active (CRDB) models for Redis Cloud

use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudCrdb {
    pub id: u32,
    pub name: String,
    pub status: Option<String>,
    pub memory_limit_in_gb: Option<f64>,
    pub regions: Option<Vec<CloudCrdbRegion>>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudCrdbRegion {
    pub id: Option<u32>,
    pub provider: Option<String>,
    pub region: Option<String>,
    pub status: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrdbTask {
    pub id: String,
    pub status: String,
    pub created_at: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrdbMetrics {
    pub measurements: Vec<CrdbMeasurement>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrdbMeasurement {
    pub name: String,
    pub values: Vec<CrdbMetricValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrdbMetricValue {
    pub timestamp: String,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CreateCrdbRequest {
    pub name: String,
    #[builder(default, setter(strip_option))]
    pub memory_limit_in_gb: Option<f64>,
    #[builder(default, setter(strip_option))]
    pub regions: Option<Vec<CloudCrdbRegion>>, // minimal
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UpdateCrdbRequest {
    #[builder(default, setter(into, strip_option))]
    pub name: Option<String>,
    #[builder(default, setter(strip_option))]
    pub memory_limit_in_gb: Option<f64>,
}
