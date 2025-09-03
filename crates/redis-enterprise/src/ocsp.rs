//! OCSP (Online Certificate Status Protocol) management for Redis Enterprise
//!
//! Overview
//! - Query and configure OCSP settings
//! - Status check and test endpoints

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// OCSP configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcspConfig {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responder_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_timeout: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_frequency: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_frequency: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_max_tries: Option<u32>,

    #[serde(flatten)]
    pub extra: Value,
}

/// OCSP status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcspStatus {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_update: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_reason: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

/// OCSP test result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcspTestResult {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_time_ms: Option<u32>,

    #[serde(flatten)]
    pub extra: Value,
}

/// OCSP handler for managing OCSP configuration
pub struct OcspHandler {
    client: RestClient,
}

impl OcspHandler {
    pub fn new(client: RestClient) -> Self {
        OcspHandler { client }
    }

    /// Get OCSP configuration
    pub async fn get_config(&self) -> Result<OcspConfig> {
        self.client.get("/v1/ocsp").await
    }

    /// Update OCSP configuration
    pub async fn update_config(&self, config: OcspConfig) -> Result<OcspConfig> {
        self.client.put("/v1/ocsp", &config).await
    }

    /// Get OCSP status
    pub async fn get_status(&self) -> Result<OcspStatus> {
        self.client.get("/v1/ocsp/status").await
    }

    /// Test OCSP connectivity
    pub async fn test(&self) -> Result<OcspTestResult> {
        self.client.get("/v1/ocsp/test").await
    }

    /// Test OCSP via POST
    pub async fn test_post(&self) -> Result<OcspTestResult> {
        self.client
            .post("/v1/ocsp/test", &serde_json::Value::Null)
            .await
    }

    /// Trigger OCSP query
    pub async fn query(&self) -> Result<()> {
        self.client
            .post_action("/v1/ocsp/query", &Value::Null)
            .await
    }

    /// Clear OCSP cache
    pub async fn clear_cache(&self) -> Result<()> {
        self.client.delete("/v1/ocsp/cache").await
    }
}
