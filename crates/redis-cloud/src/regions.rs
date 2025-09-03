//! Region management for Redis Cloud
//!
//! ## Overview
//! - Query available regions
//! - Get region details and capabilities

use crate::client::CloudClient;

/// Region handler
pub struct RegionHandler {
    #[allow(dead_code)]
    client: CloudClient,
}

impl RegionHandler {
    pub fn new(client: CloudClient) -> Self {
        RegionHandler { client }
    }
}
