//! Logs operations handler

use crate::{Result, client::CloudClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogsResponse {
    pub logs: Vec<LogEntry>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemLogsResponse {
    pub logs: Vec<LogEntry>,
    pub total: Option<u32>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub pagination: Option<Value>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionLogsResponse {
    pub logs: Vec<LogEntry>,
    pub total: Option<u32>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub pagination: Option<Value>,
    #[serde(flatten)]
    pub extra: Value,
}

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

        let mut resp: SystemLogsResponse =
            self.client.get(&format!("/logs{}", query_string)).await?;
        if resp.total.is_none()
            && let Some(p) = &resp.pagination
        {
            resp.total = p.get("total").and_then(|v| v.as_u64()).map(|v| v as u32);
        }
        if resp.limit.is_none()
            && let Some(p) = &resp.pagination
        {
            resp.limit = p.get("limit").and_then(|v| v.as_u64()).map(|v| v as u32);
        }
        if resp.offset.is_none()
            && let Some(p) = &resp.pagination
        {
            resp.offset = p.get("offset").and_then(|v| v.as_u64()).map(|v| v as u32);
        }
        Ok(resp)
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

        let mut resp: SessionLogsResponse = self
            .client
            .get(&format!("/session-logs{}", query_string))
            .await?;
        if resp.total.is_none()
            && let Some(p) = &resp.pagination
        {
            resp.total = p.get("total").and_then(|v| v.as_u64()).map(|v| v as u32);
        }
        if resp.limit.is_none()
            && let Some(p) = &resp.pagination
        {
            resp.limit = p.get("limit").and_then(|v| v.as_u64()).map(|v| v as u32);
        }
        if resp.offset.is_none()
            && let Some(p) = &resp.pagination
        {
            resp.offset = p.get("offset").and_then(|v| v.as_u64()).map(|v| v as u32);
        }
        Ok(resp)
    }
}
