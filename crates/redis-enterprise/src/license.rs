//! License management and validation
//!
//! ## Overview
//! - Query license status
//! - Update license keys
//! - Monitor license expiration

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

/// License information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    pub license_key: String,
    pub type_: String,
    pub expired: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shards_limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

/// License update request
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct LicenseUpdateRequest {
    #[builder(setter(into))]
    pub license: String,
}

/// License usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseUsage {
    pub shards_used: u32,
    pub shards_limit: u32,
    pub nodes_used: u32,
    pub nodes_limit: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram_used: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram_limit: Option<u64>,

    #[serde(flatten)]
    pub extra: Value,
}

/// License handler
pub struct LicenseHandler {
    client: RestClient,
}

impl LicenseHandler {
    pub fn new(client: RestClient) -> Self {
        LicenseHandler { client }
    }

    /// Get current license information
    pub async fn get(&self) -> Result<License> {
        self.client.get("/v1/license").await
    }

    /// Update license
    pub async fn update(&self, request: LicenseUpdateRequest) -> Result<License> {
        self.client.put("/v1/license", &request).await
    }

    /// Get license usage statistics
    pub async fn usage(&self) -> Result<LicenseUsage> {
        self.client.get("/v1/license/usage").await
    }

    /// Validate a license key
    pub async fn validate(&self, license_key: &str) -> Result<License> {
        let request = LicenseUpdateRequest {
            license: license_key.to_string(),
        };
        self.client.post("/v1/license/validate", &request).await
    }

    /// Get license from cluster
    pub async fn cluster_license(&self) -> Result<License> {
        self.client.get("/v1/cluster/license").await
    }
}
