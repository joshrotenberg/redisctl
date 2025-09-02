//! Logs operations handler

use crate::models::logs::*;
use crate::{Result, client::CloudClient};
use serde_json::Value;

/// Handler for Cloud logs operations
pub struct CloudLogsHandler {
    client: CloudClient,
}

impl CloudLogsHandler {
    pub fn new(client: CloudClient) -> Self {
        CloudLogsHandler { client }
    }

    /// Get database logs
    pub async fn database(
        &self,
        subscription_id: u32,
        database_id: u32,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<LogsResponse> {
        let mut query_params = vec![];

        if let Some(limit_val) = limit {
            query_params.push(format!("limit={}", limit_val));
        }

        if let Some(offset_val) = offset {
            query_params.push(format!("offset={}", offset_val));
        }

        let query_string = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{}", query_params.join("&"))
        };

        self.client
            .get(&format!(
                "/subscriptions/{}/databases/{}/logs{}",
                subscription_id, database_id, query_string
            ))
            .await
    }

    /// Get database logs - raw version
    pub async fn database_raw(
        &self,
        subscription_id: u32,
        database_id: u32,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Value> {
        let mut query_params = vec![];

        if let Some(limit_val) = limit {
            query_params.push(format!("limit={}", limit_val));
        }

        if let Some(offset_val) = offset {
            query_params.push(format!("offset={}", offset_val));
        }

        let query_string = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{}", query_params.join("&"))
        };

        self.client
            .get(&format!(
                "/subscriptions/{}/databases/{}/logs{}",
                subscription_id, database_id, query_string
            ))
            .await
    }

    /// Get system logs
    pub async fn system(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<SystemLogsResponse> {
        let mut query_params = vec![];

        if let Some(limit_val) = limit {
            query_params.push(format!("limit={}", limit_val));
        }

        if let Some(offset_val) = offset {
            query_params.push(format!("offset={}", offset_val));
        }

        let query_string = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{}", query_params.join("&"))
        };

        self.client.get(&format!("/logs{}", query_string)).await
    }

    /// Get system logs - raw version
    pub async fn system_raw(&self, limit: Option<u32>, offset: Option<u32>) -> Result<Value> {
        let mut query_params = vec![];

        if let Some(limit_val) = limit {
            query_params.push(format!("limit={}", limit_val));
        }

        if let Some(offset_val) = offset {
            query_params.push(format!("offset={}", offset_val));
        }

        let query_string = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{}", query_params.join("&"))
        };

        self.client.get(&format!("/logs{}", query_string)).await
    }

    /// Get session logs
    pub async fn session(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<SessionLogsResponse> {
        let mut query_params = vec![];

        if let Some(limit_val) = limit {
            query_params.push(format!("limit={}", limit_val));
        }

        if let Some(offset_val) = offset {
            query_params.push(format!("offset={}", offset_val));
        }

        let query_string = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{}", query_params.join("&"))
        };

        self.client
            .get(&format!("/session-logs{}", query_string))
            .await
    }

    /// Get session logs - raw version
    pub async fn session_raw(&self, limit: Option<u32>, offset: Option<u32>) -> Result<Value> {
        let mut query_params = vec![];

        if let Some(limit_val) = limit {
            query_params.push(format!("limit={}", limit_val));
        }

        if let Some(offset_val) = offset {
            query_params.push(format!("offset={}", offset_val));
        }

        let query_string = if query_params.is_empty() {
            String::new()
        } else {
            format!("?{}", query_params.join("&"))
        };

        self.client
            .get(&format!("/session-logs{}", query_string))
            .await
    }
}
