//! SSO/SAML configuration handler

use crate::{client::CloudClient, Result};
use serde_json::Value;

/// Handler for Cloud SSO/SAML operations
pub struct CloudSsoHandler {
    client: CloudClient,
}

impl CloudSsoHandler {
    pub fn new(client: CloudClient) -> Self {
        CloudSsoHandler { client }
    }

    /// Get SSO configuration
    pub async fn get(&self) -> Result<Value> {
        self.client.get("/sso").await
    }

    /// Update SSO configuration
    pub async fn update(&self, request: Value) -> Result<Value> {
        self.client.put("/sso", &request).await
    }

    /// Delete SSO configuration
    pub async fn delete(&self) -> Result<Value> {
        self.client.delete("/sso").await?;
        Ok(serde_json::json!({"message": "SSO configuration deleted"}))
    }

    /// Test SSO configuration
    pub async fn test(&self, request: Value) -> Result<Value> {
        self.client.post("/sso/test", &request).await
    }

    /// Get SAML configuration
    pub async fn get_saml(&self) -> Result<Value> {
        self.client.get("/sso/saml").await
    }

    /// Update SAML configuration
    pub async fn update_saml(&self, request: Value) -> Result<Value> {
        self.client.put("/sso/saml", &request).await
    }

    /// Get SAML metadata
    pub async fn get_saml_metadata(&self) -> Result<Value> {
        self.client.get("/sso/saml/metadata").await
    }

    /// Upload SAML certificate
    pub async fn upload_saml_cert(&self, request: Value) -> Result<Value> {
        self.client.post("/sso/saml/certificate", &request).await
    }

    /// Get SSO users
    pub async fn list_users(&self) -> Result<Value> {
        self.client.get("/sso/users").await
    }

    /// Get SSO user by ID
    pub async fn get_user(&self, user_id: u32) -> Result<Value> {
        self.client.get(&format!("/sso/users/{}", user_id)).await
    }

    /// Create SSO user mapping
    pub async fn create_user_mapping(&self, request: Value) -> Result<Value> {
        self.client.post("/sso/users", &request).await
    }

    /// Update SSO user mapping
    pub async fn update_user_mapping(&self, user_id: u32, request: Value) -> Result<Value> {
        self.client
            .put(&format!("/sso/users/{}", user_id), &request)
            .await
    }

    /// Delete SSO user mapping
    pub async fn delete_user_mapping(&self, user_id: u32) -> Result<Value> {
        self.client
            .delete(&format!("/sso/users/{}", user_id))
            .await?;
        Ok(serde_json::json!({"message": format!("SSO user mapping {} deleted", user_id)}))
    }

    /// Get SSO groups
    pub async fn list_groups(&self) -> Result<Value> {
        self.client.get("/sso/groups").await
    }

    /// Map SSO group to role
    pub async fn map_group(&self, request: Value) -> Result<Value> {
        self.client.post("/sso/groups", &request).await
    }

    /// Update SSO group mapping
    pub async fn update_group_mapping(&self, group_id: u32, request: Value) -> Result<Value> {
        self.client
            .put(&format!("/sso/groups/{}", group_id), &request)
            .await
    }

    /// Delete SSO group mapping
    pub async fn delete_group_mapping(&self, group_id: u32) -> Result<Value> {
        self.client
            .delete(&format!("/sso/groups/{}", group_id))
            .await?;
        Ok(serde_json::json!({"message": format!("SSO group mapping {} deleted", group_id)}))
    }
}
