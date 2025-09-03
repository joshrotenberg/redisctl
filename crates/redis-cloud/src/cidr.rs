//! CIDR allowlist management for Redis Cloud
//!
//! ## Overview
//! - Configure CIDR allowlists for subscriptions
//! - Manage network access control

use crate::client::CloudClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// CIDR allowlist information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CidrAllowlist {
    pub cidrs: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Update CIDR allowlist request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCidrRequest {
    pub cidrs: Vec<String>,
}

/// CIDR handler
pub struct CidrHandler {
    client: CloudClient,
}

impl CidrHandler {
    pub fn new(client: CloudClient) -> Self {
        CidrHandler { client }
    }

    /// Get CIDR allowlist for a subscription
    pub async fn get(&self, subscription_id: u32) -> Result<CidrAllowlist> {
        self.client
            .get(&format!("/subscriptions/{}/cidr", subscription_id))
            .await
    }

    /// Update CIDR allowlist for a subscription
    pub async fn update(
        &self,
        subscription_id: u32,
        request: UpdateCidrRequest,
    ) -> Result<CidrAllowlist> {
        self.client
            .put(
                &format!("/subscriptions/{}/cidr", subscription_id),
                &request,
            )
            .await
    }
}
