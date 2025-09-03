//! Database management for Redis Cloud Essentials
//!
//! ## Overview
//! - Manage Essentials tier databases
//! - Configure fixed plan databases

use crate::client::CloudClient;

/// Fixed database handler
pub struct FixedDatabaseHandler {
    #[allow(dead_code)]
    client: CloudClient,
}

impl FixedDatabaseHandler {
    pub fn new(client: CloudClient) -> Self {
        FixedDatabaseHandler { client }
    }
}
