//! Region operations handler

use crate::{Result, client::CloudClient};
use serde_json::Value;

/// Handler for Cloud regions
pub struct CloudRegionHandler {
    client: CloudClient,
}

impl CloudRegionHandler {
    pub fn new(client: CloudClient) -> Self {
        CloudRegionHandler { client }
    }

    /// List available regions for a cloud provider
    pub async fn list(&self, provider: &str) -> Result<Value> {
        self.client
            .get(&format!("/cloud-providers/{}/regions", provider))
            .await
    }

    /// Get region details
    pub async fn get(&self, provider: &str, region: &str) -> Result<Value> {
        self.client
            .get(&format!("/cloud-providers/{}/regions/{}", provider, region))
            .await
    }
}
