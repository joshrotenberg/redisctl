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
    /// License key - the actual field name returned by API
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,

    /// License string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,

    /// License type (trial, commercial, etc.)
    #[serde(rename = "type")]
    pub type_: Option<String>,

    /// Mark license expired or not
    pub expired: bool,

    /// License activation date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_date: Option<String>,

    /// License expiration date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,

    /// The cluster name as appears in the license
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,

    /// Owner of license
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,

    /// Shards limit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shards_limit: Option<u32>,

    /// Amount of RAM shards in use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram_shards_in_use: Option<u32>,

    /// Amount of RAM shards allowed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram_shards_limit: Option<u32>,

    /// Amount of flash shards in use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flash_shards_in_use: Option<u32>,

    /// Amount of flash shards allowed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flash_shards_limit: Option<u32>,

    /// Node limit (deprecated in favor of shards_limit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_limit: Option<u32>,

    /// List of features supported by license
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,

    #[serde(flatten)]
    pub extra: Value,
}

/// License update request
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct LicenseUpdateRequest {
    /// New license key to install
    #[builder(setter(into))]
    pub license: String,
}

/// License usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseUsage {
    /// Number of shards currently in use
    pub shards_used: u32,
    /// Maximum number of shards allowed by license
    pub shards_limit: u32,
    /// Number of nodes currently in use
    pub nodes_used: u32,
    /// Maximum number of nodes allowed by license
    pub nodes_limit: u32,
    /// Amount of RAM currently in use (bytes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram_used: Option<u64>,
    /// Maximum amount of RAM allowed by license (bytes)
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
