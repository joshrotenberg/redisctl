//! Region operations handler

use crate::{Result, client::CloudClient, models::RegionInfo};
use serde_json::Value;

/// Handler for Cloud regions
pub struct CloudRegionHandler {
    client: CloudClient,
}

impl CloudRegionHandler {
    pub fn new(client: CloudClient) -> Self {
        CloudRegionHandler { client }
    }

    /// List available regions for a cloud provider (typed)
    pub async fn list(&self, provider: &str) -> Result<Vec<RegionInfo>> {
        let v: Value = self
            .client
            .get(&format!("/cloud-providers/{}/regions", provider))
            .await?;
        if let Some(arr) = v.get("regions").and_then(|x| x.as_array()) {
            serde_json::from_value::<Vec<RegionInfo>>(serde_json::Value::Array(arr.clone()))
                .map_err(Into::into)
        } else if v.is_array() {
            serde_json::from_value::<Vec<RegionInfo>>(v).map_err(Into::into)
        } else {
            Ok(vec![])
        }
    }

    /// Get region details (typed)
    pub async fn get(&self, provider: &str, region: &str) -> Result<RegionInfo> {
        self.client
            .get(&format!("/cloud-providers/{}/regions/{}", provider, region))
            .await
    }

    /// List all regions (typed)
    pub async fn list_all(&self) -> Result<Vec<RegionInfo>> {
        let v: Value = self.client.get("/regions").await?;
        if let Some(arr) = v.get("regions").and_then(|x| x.as_array()) {
            serde_json::from_value::<Vec<RegionInfo>>(serde_json::Value::Array(arr.clone()))
                .map_err(Into::into)
        } else if v.is_array() {
            serde_json::from_value::<Vec<RegionInfo>>(v).map_err(Into::into)
        } else {
            Ok(vec![])
        }
    }
}
