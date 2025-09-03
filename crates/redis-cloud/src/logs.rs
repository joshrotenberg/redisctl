//! Log management for Redis Cloud
//!
//! ## Overview
//! - Retrieve system and session logs
//! - Access database slow logs

use crate::client::CloudClient;
use anyhow::Result;
use serde_json::Value;

/// Log handler
pub struct LogHandler {
    client: CloudClient,
}

impl LogHandler {
    pub fn new(client: CloudClient) -> Self {
        LogHandler { client }
    }
    
    /// Get system logs
    pub async fn get_logs(&self) -> Result<Value> {
        self.client.get_raw("/logs").await
    }
    
    /// Get session logs
    pub async fn get_session_logs(&self) -> Result<Value> {
        self.client.get_raw("/session-logs").await
    }
    
    /// Get database slow log
    pub async fn get_database_slow_log(
        &self,
        subscription_id: u32,
        database_id: u32,
    ) -> Result<Value> {
        self.client
            .get_raw(&format!(
                "/subscriptions/{}/databases/{}/slow-log",
                subscription_id, database_id
            ))
            .await
    }
    
    /// Get fixed subscription database slow log
    pub async fn get_fixed_database_slow_log(
        &self,
        subscription_id: u32,
        database_id: u32,
    ) -> Result<Value> {
        self.client
            .get_raw(&format!(
                "/fixed/subscriptions/{}/databases/{}/slow-log",
                subscription_id, database_id
            ))
            .await
    }
}