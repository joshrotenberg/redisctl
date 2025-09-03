//! API key models for Redis Cloud
//!
//! These models represent API keys, permissions, usage, and audit logs used by
//! the Redis Cloud API. Unknown fields are preserved via `#[serde(flatten)]` to
//! keep compatibility with future API changes while offering typed access to
//! common fields.

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_api_key() {
        let raw = serde_json::json!({
            "id": 42,
            "name": "ci-bot",
            "status": "enabled",
            "prefix": "rk_abc",
            "createdTimestamp": "2024-08-31T10:00:00Z"
        });
        let k: ApiKey = serde_json::from_value(raw).unwrap();
        assert_eq!(k.id, 42);
        assert_eq!(k.name, "ci-bot");
        assert_eq!(k.status.as_deref(), Some("enabled"));
        assert_eq!(k.prefix.as_deref(), Some("rk_abc"));
        assert!(k.extra.is_object());
    }

    #[test]
    fn deserialize_permissions_wrapper() {
        let raw = serde_json::json!({
            "roles": ["admin"],
            "resources": [{"type": "subscriptions", "actions": ["read"]}]
        });
        let p: ApiKeyPermissions = serde_json::from_value(raw).unwrap();
        assert!(p.document.is_object());
    }

    #[test]
    fn deserialize_usage_response() {
        let raw = serde_json::json!({
            "period": "7d",
            "usage": [
                {"timestamp": "2024-08-31T00:00:00Z", "count": 12},
                {"timestamp": "2024-09-01T00:00:00Z", "count": 8}
            ]
        });
        let u: ApiKeyUsageResponse = serde_json::from_value(raw).unwrap();
        assert_eq!(u.period.as_deref(), Some("7d"));
        assert_eq!(u.usage.len(), 2);
    }
}
