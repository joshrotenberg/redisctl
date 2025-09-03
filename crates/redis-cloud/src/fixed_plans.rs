//! Fixed plan management for Redis Cloud Essentials
//!
//! ## Overview
//! - Query available fixed plans
//! - Get plan details and pricing

use crate::client::CloudClient;

/// Fixed plan handler
pub struct FixedPlanHandler {
    #[allow(dead_code)]
    client: CloudClient,
}

impl FixedPlanHandler {
    pub fn new(client: CloudClient) -> Self {
        FixedPlanHandler { client }
    }
}
