//! API keys management handler
//!
//! Provides both typed and raw accessors for working with API keys. Prefer the
//! typed helpers for most application code; raw methods are kept for CLI and
//! power-user scenarios where arbitrary JSON shape is desirable.

use crate::{
    Result,
    client::CloudClient,
    models::{ApiKey, ApiKeyAuditLogsResponse, ApiKeyPermissions, ApiKeysResponse},
};
use serde_json::Value;

/// Handler for Cloud API key management
pub struct CloudApiKeyHandler {
    client: CloudClient,
}

impl CloudApiKeyHandler {
    pub fn new(client: CloudClient) -> Self {
        CloudApiKeyHandler { client }
    }

    /// List all API keys (typed)
    pub async fn list(&self) -> Result<Vec<ApiKey>> {
        let resp: ApiKeysResponse = self.client.get("/api-keys").await?;
        Ok(resp.api_keys)
    }

    /// Get API key by ID (typed)
    pub async fn get(&self, key_id: u32) -> Result<ApiKey> {
        let v: Value = self.client.get(&format!("/api-keys/{}", key_id)).await?;
        if let Some(obj) = v.get("apiKey") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
    }

    /// Create API key (typed)
    pub async fn create(&self, request: &serde_json::Value) -> Result<ApiKey> {
        let v: Value = self.client.post("/api-keys", request).await?;
        if let Some(obj) = v.get("apiKey") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
    }

    /// Update API key (typed)
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

    /// Get API key permissions (typed)
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

    /// Update API key permissions (typed)
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

    /// Get API key usage statistics (typed)
    pub async fn get_usage(
        &self,
        key_id: u32,
        period: &str,
    ) -> Result<crate::models::api_keys::ApiKeyUsageSummary> {
        let v: Value = self
            .client
            .get(&format!("/api-keys/{}/usage?period={}", key_id, period))
            .await?;
        let inner = v.get("usage").cloned().unwrap_or(v);
        serde_json::from_value(inner).map_err(Into::into)
    }

    /// List API key audit logs (typed)
    pub async fn get_audit_logs(&self, key_id: u32) -> Result<ApiKeyAuditLogsResponse> {
        let v: Value = self
            .client
            .get(&format!("/api-keys/{}/audit", key_id))
            .await?;
        if let Some(arr) = v.get("auditLogs") {
            let logs: Vec<crate::models::api_keys::ApiKeyAuditLogEntry> =
                serde_json::from_value(arr.clone())?;
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
