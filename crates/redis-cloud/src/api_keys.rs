//! API keys management handler
//!
//! Provides both typed and raw accessors for working with API keys. Prefer the
//! typed helpers for most application code; raw methods are kept for CLI and
//! power-user scenarios where arbitrary JSON shape is desirable.

use crate::{Result, client::CloudClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Single API key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    /// Unique identifier
    pub id: u32,
    /// Human-friendly name
    pub name: String,
    /// Status string (e.g. "enabled"/"disabled")
    pub status: Option<String>,
    /// Prefix used in key display
    pub prefix: Option<String>,
    /// ISO 8601 created timestamp
    #[serde(rename = "createdTimestamp")]
    pub created_at: Option<String>,
    /// ISO 8601 updated timestamp
    #[serde(rename = "updatedTimestamp")]
    pub updated_at: Option<String>,
    /// ISO 8601 timestamp of last use
    pub last_used_at: Option<String>,

    /// Extra fields not explicitly modeled
    #[serde(flatten)]
    pub extra: Value,
}

/// List API keys response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeysResponse {
    #[serde(rename = "apiKeys")]
    pub api_keys: Vec<ApiKey>,

    /// Extra fields not explicitly modeled
    #[serde(flatten)]
    pub extra: Value,
}

/// API key create/update request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyRequest {
    pub name: String,
    /// Optional status to set (e.g. "enabled"/"disabled")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// API key permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyPermissions {
    /// A structured permissions document. The exact shape is subject to change
    /// and may include role bindings or resource/action pairs.
    /// Keeping as a typed wrapper over JSON preserves forward compatibility.
    #[serde(flatten)]
    pub document: Value,
}

/// API key usage response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyUsageResponse {
    /// Selected aggregation period (e.g. "7d", "30d")
    pub period: Option<String>,
    /// Aggregated usage stats by time bucket
    pub usage: Vec<ApiKeyUsagePoint>,

    /// Extra fields not explicitly modeled
    #[serde(flatten)]
    pub extra: Value,
}

/// Single usage point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyUsagePoint {
    pub timestamp: String,
    pub count: u64,

    /// Extra fields not explicitly modeled
    #[serde(flatten)]
    pub extra: Value,
}

/// API key audit logs response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyAuditLogsResponse {
    pub logs: Vec<ApiKeyAuditLogEntry>,
    pub total: Option<u32>,
    pub offset: Option<u32>,
    pub limit: Option<u32>,

    /// Extra fields not explicitly modeled
    #[serde(flatten)]
    pub extra: Value,
}

/// Single audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyAuditLogEntry {
    pub timestamp: String,
    pub action: String,
    pub user_id: Option<u32>,
    pub request_id: Option<String>,
    pub ip_address: Option<String>,
    pub metadata: Option<Value>,

    /// Extra fields not explicitly modeled
    #[serde(flatten)]
    pub extra: Value,
}

/// Summary usage model for API key (matches tests' expected keys)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyEndpointUsage {
    pub path: String,
    pub method: String,
    pub requests: u64,
    #[serde(rename = "averageResponseTime")]
    pub average_response_time: f64,

    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyUsageSummary {
    pub period: String,
    #[serde(rename = "totalRequests")]
    pub total_requests: u64,
    #[serde(rename = "successfulRequests")]
    pub successful_requests: u64,
    #[serde(rename = "failedRequests")]
    pub failed_requests: u64,
    #[serde(rename = "averageResponseTime")]
    pub average_response_time: f64,
    #[serde(rename = "peakRequestsPerHour")]
    pub peak_requests_per_hour: u64,
    pub endpoints: Vec<ApiKeyEndpointUsage>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Handler for Cloud API key management
pub struct CloudApiKeyHandler {
    client: CloudClient,
}

impl CloudApiKeyHandler {
    pub fn new(client: CloudClient) -> Self {
        CloudApiKeyHandler { client }
    }

    /// List all API keys
    pub async fn list(&self) -> Result<Vec<ApiKey>> {
        let resp: ApiKeysResponse = self.client.get("/api-keys").await?;
        Ok(resp.api_keys)
    }

    /// Get API key by ID
    pub async fn get(&self, key_id: u32) -> Result<ApiKey> {
        let v: Value = self.client.get(&format!("/api-keys/{}", key_id)).await?;
        if let Some(obj) = v.get("apiKey") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
    }

    /// Create API key
    pub async fn create(&self, request: &serde_json::Value) -> Result<ApiKey> {
        let v: Value = self.client.post("/api-keys", request).await?;
        if let Some(obj) = v.get("apiKey") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
    }

    /// Update API key
    pub async fn update(&self, key_id: u32, request: &serde_json::Value) -> Result<ApiKey> {
        let v: Value = self
            .client
            .put(&format!("/api-keys/{}", key_id), request)
            .await?;
        if let Some(obj) = v.get("apiKey") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
    }

    /// Delete API key
    pub async fn delete(&self, key_id: u32) -> Result<Value> {
        self.client.delete(&format!("/api-keys/{}", key_id)).await?;
        Ok(serde_json::json!({"message": format!("API key {} deleted", key_id)}))
    }

    /// Regenerate API key secret
    pub async fn regenerate(&self, key_id: u32) -> Result<Value> {
        let v: Value = self
            .client
            .post(&format!("/api-keys/{}/regenerate", key_id), &Value::Null)
            .await?;
        Ok(v.get("apiKey").cloned().unwrap_or(v))
    }

    /// Get API key permissions
    pub async fn get_permissions(&self, key_id: u32) -> Result<ApiKeyPermissions> {
        let v: Value = self
            .client
            .get(&format!("/api-keys/{}/permissions", key_id))
            .await?;
        if let Some(obj) = v.get("permissions") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
    }

    /// Update API key permissions
    pub async fn update_permissions(
        &self,
        key_id: u32,
        request: &serde_json::Value,
    ) -> Result<ApiKeyPermissions> {
        let v: Value = self
            .client
            .put(&format!("/api-keys/{}/permissions", key_id), request)
            .await?;
        if let Some(obj) = v.get("permissions") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
    }

    /// Enable API key
    pub async fn enable(&self, key_id: u32) -> Result<Value> {
        let v: Value = self
            .client
            .post(&format!("/api-keys/{}/enable", key_id), &Value::Null)
            .await?;
        Ok(v.get("apiKey").cloned().unwrap_or(v))
    }

    /// Disable API key
    pub async fn disable(&self, key_id: u32) -> Result<Value> {
        let v: Value = self
            .client
            .post(&format!("/api-keys/{}/disable", key_id), &Value::Null)
            .await?;
        Ok(v.get("apiKey").cloned().unwrap_or(v))
    }

    /// Get API key usage statistics
    pub async fn get_usage(&self, key_id: u32, period: &str) -> Result<ApiKeyUsageSummary> {
        let v: Value = self
            .client
            .get(&format!("/api-keys/{}/usage?period={}", key_id, period))
            .await?;
        let inner = v.get("usage").cloned().unwrap_or(v);
        serde_json::from_value(inner).map_err(Into::into)
    }

    /// List API key audit logs
    pub async fn get_audit_logs(&self, key_id: u32) -> Result<ApiKeyAuditLogsResponse> {
        let v: Value = self
            .client
            .get(&format!("/api-keys/{}/audit", key_id))
            .await?;
        if let Some(arr) = v.get("auditLogs") {
            let logs: Vec<ApiKeyAuditLogEntry> = serde_json::from_value(arr.clone())?;
            Ok(ApiKeyAuditLogsResponse {
                logs,
                total: None,
                offset: None,
                limit: None,
                extra: serde_json::json!({}),
            })
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
    }
}
