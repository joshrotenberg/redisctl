//! API keys management handler
//!
//! Provides both typed and raw accessors for working with API keys. Prefer the
//! typed helpers for most application code; raw methods are kept for CLI and
//! power-user scenarios where arbitrary JSON shape is desirable.

use crate::{
    client::CloudClient,
    models::{
        ApiKey, ApiKeyAuditLogsResponse, ApiKeyPermissions, ApiKeyRequest, ApiKeyUsageResponse,
        ApiKeysResponse,
    },
    Result,
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
        self.client.get(&format!("/api-keys/{}", key_id)).await
    }

    /// Create API key (typed)
    pub async fn create(&self, request: &ApiKeyRequest) -> Result<ApiKey> {
        self.client.post("/api-keys", request).await
    }

    /// Update API key (typed)
    pub async fn update(&self, key_id: u32, request: &ApiKeyRequest) -> Result<ApiKey> {
        self.client
            .put(&format!("/api-keys/{}", key_id), request)
            .await
    }

    /// Delete API key
    pub async fn delete(&self, key_id: u32) -> Result<Value> {
        self.client.delete(&format!("/api-keys/{}", key_id)).await?;
        Ok(serde_json::json!({"message": format!("API key {} deleted", key_id)}))
    }

    /// Regenerate API key secret
    pub async fn regenerate(&self, key_id: u32) -> Result<Value> {
        self.client
            .post(&format!("/api-keys/{}/regenerate", key_id), &Value::Null)
            .await
    }

    /// Get API key permissions (typed)
    pub async fn get_permissions(&self, key_id: u32) -> Result<ApiKeyPermissions> {
        self.client
            .get(&format!("/api-keys/{}/permissions", key_id))
            .await
    }

    /// Update API key permissions (typed)
    pub async fn update_permissions(
        &self,
        key_id: u32,
        request: &ApiKeyPermissions,
    ) -> Result<ApiKeyPermissions> {
        self.client
            .put(&format!("/api-keys/{}/permissions", key_id), request)
            .await
    }

    /// Enable API key
    pub async fn enable(&self, key_id: u32) -> Result<Value> {
        self.client
            .post(&format!("/api-keys/{}/enable", key_id), &Value::Null)
            .await
    }

    /// Disable API key
    pub async fn disable(&self, key_id: u32) -> Result<Value> {
        self.client
            .post(&format!("/api-keys/{}/disable", key_id), &Value::Null)
            .await
    }

    /// Get API key usage statistics (typed)
    pub async fn get_usage(&self, key_id: u32, period: &str) -> Result<ApiKeyUsageResponse> {
        self.client
            .get(&format!("/api-keys/{}/usage?period={}", key_id, period))
            .await
    }

    /// List API key audit logs (typed)
    pub async fn get_audit_logs(&self, key_id: u32) -> Result<ApiKeyAuditLogsResponse> {
        self.client
            .get(&format!("/api-keys/{}/audit", key_id))
            .await
    }
}
