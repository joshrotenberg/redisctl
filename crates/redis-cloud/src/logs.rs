//! Log management for Redis Cloud
//!
//! ## Overview
//! - Query system logs
//! - Access session logs

use crate::client::CloudClient;

/// Log handler
pub struct LogHandler {
    #[allow(dead_code)]
    client: CloudClient,
}

impl LogHandler {
    pub fn new(client: CloudClient) -> Self {
        LogHandler { client }
    }
}
