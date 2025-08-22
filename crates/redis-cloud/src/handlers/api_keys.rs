//! API keys management handler

use crate::{client::CloudClient, Result};
use serde_json::Value;

/// Handler for Cloud API keys management
pub struct CloudApiKeysHandler {
    client: CloudClient,
}

impl CloudApiKeysHandler {
    pub fn new(client: CloudClient) -> Self {
        CloudApiKeysHandler { client }
    }

    /// List all API keys
    pub async fn list(&self) -> Result<Value> {
        self.client.get("/api-keys").await
    }

    /// Get API key by ID
    pub async fn get(&self, key_id: u32) -> Result<Value> {
        self.client.get(&format!("/api-keys/{}", key_id)).await
    }

    /// Create API key
    pub async fn create(&self, request: Value) -> Result<Value> {
        self.client.post("/api-keys", &request).await
    }

    /// Update API key
    pub async fn update(&self, key_id: u32, request: Value) -> Result<Value> {
        self.client
            .put(&format!("/api-keys/{}", key_id), &request)
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

    /// Get API key permissions
    pub async fn get_permissions(&self, key_id: u32) -> Result<Value> {
        self.client
            .get(&format!("/api-keys/{}/permissions", key_id))
            .await
    }

    /// Update API key permissions
    pub async fn update_permissions(&self, key_id: u32, request: Value) -> Result<Value> {
        self.client
            .put(&format!("/api-keys/{}/permissions", key_id), &request)
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

    /// Get API key usage statistics
    pub async fn get_usage(&self, key_id: u32, period: &str) -> Result<Value> {
        self.client
            .get(&format!("/api-keys/{}/usage?period={}", key_id, period))
            .await
    }

    /// List API key audit logs
    pub async fn get_audit_logs(&self, key_id: u32) -> Result<Value> {
        self.client
            .get(&format!("/api-keys/{}/audit", key_id))
            .await
    }
}