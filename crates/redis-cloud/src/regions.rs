//! Region management for Redis Cloud
//!
//! ## Overview
//! - Query available regions
//! - Get region details and capabilities

use crate::client::CloudClient;
use anyhow::Result;
use serde_json::Value;

/// Region handler
pub struct RegionHandler {
    client: CloudClient,
}

impl RegionHandler {
    pub fn new(client: CloudClient) -> Self {
        RegionHandler { client }
    }
    
    /// List all available regions
    /// 
    /// Note: The /regions endpoint returns a complex nested structure
    /// that varies by provider, so we return it as a Value
    pub async fn list(&self) -> Result<Value> {
        self.client.get_raw("/regions").await
    }
}