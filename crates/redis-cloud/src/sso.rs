//! SSO/SAML configuration handler

use crate::{Result, client::CloudClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsoConfig {
    pub enabled: bool,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamlConfig {
    pub enabled: bool,
    pub metadata: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamlMetadata {
    pub metadata: String,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsoUserMapping {
    pub user_id: String,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsoGroupMapping {
    pub group_name: String,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsoTestResponse {
    pub success: bool,
    #[serde(flatten)]
    pub extra: Value,
}

/// Handler for Cloud SSO/SAML operations
pub struct CloudSsoHandler {
    client: CloudClient,
}

impl CloudSsoHandler {
    pub fn new(client: CloudClient) -> Self {
        CloudSsoHandler { client }
    }

    /// Get SSO configuration (typed)
    pub async fn get(&self) -> Result<SsoConfig> {
        let v: serde_json::Value = self.client.get("/sso").await?;
        if let Some(obj) = v.get("sso") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
    }

    /// Update SSO configuration (typed)
    pub async fn update(&self, request: serde_json::Value) -> Result<SsoConfig> {
        let v: serde_json::Value = self.client.put("/sso", &request).await?;
        if let Some(obj) = v.get("sso") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
    }

    /// Delete SSO configuration
    pub async fn delete(&self) -> Result<()> {
        self.client.delete("/sso").await
    }

    /// Test SSO configuration
    pub async fn test(&self, request: serde_json::Value) -> Result<SsoTestResponse> {
        let v: serde_json::Value = self.client.post("/sso/test", &request).await?;
        if let Some(obj) = v.get("test") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
    }

    /// Get SAML configuration
    pub async fn get_saml(&self) -> Result<SamlConfig> {
        let v: serde_json::Value = self.client.get("/sso/saml").await?;
        if let Some(obj) = v.get("saml") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
    }

    /// Update SAML configuration
    pub async fn update_saml(&self, request: serde_json::Value) -> Result<SamlConfig> {
        let v: serde_json::Value = self.client.put("/sso/saml", &request).await?;
        if let Some(obj) = v.get("saml") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
    }

    /// Get SAML metadata
    pub async fn get_saml_metadata(&self) -> Result<SamlMetadata> {
        let v: serde_json::Value = self.client.get("/sso/saml/metadata").await?;
        if let Some(obj) = v.get("metadata") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
    }

    /// Upload SAML certificate
    pub async fn upload_saml_cert(&self, payload: serde_json::Value) -> Result<serde_json::Value> {
        self.client.post("/sso/saml/certificate", &payload).await
    }

    /// Get SSO users
    pub async fn list_users(&self) -> Result<Vec<SsoUserMapping>> {
        let v: serde_json::Value = self.client.get("/sso/users").await?;
        if v.is_array() {
            serde_json::from_value(v).map_err(Into::into)
        } else if let Some(arr) = v.get("users") {
            serde_json::from_value(arr.clone()).map_err(Into::into)
        } else {
            Ok(vec![])
        }
    }

    /// Get SSO user by ID
    pub async fn get_user(&self, user_id: u32) -> Result<SsoUserMapping> {
        let v: serde_json::Value = self.client.get(&format!("/sso/users/{}", user_id)).await?;
        if let Some(obj) = v.get("user") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
    }

    /// Create SSO user mapping
    pub async fn create_user_mapping(&self, mapping: serde_json::Value) -> Result<SsoUserMapping> {
        let v: serde_json::Value = self.client.post("/sso/users", &mapping).await?;
        if let Some(obj) = v.get("user") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
    }

    /// Update SSO user mapping
    pub async fn update_user_mapping(
        &self,
        user_id: u32,
        mapping: serde_json::Value,
    ) -> Result<SsoUserMapping> {
        let v: serde_json::Value = self
            .client
            .put(&format!("/sso/users/{}", user_id), &mapping)
            .await?;
        if let Some(obj) = v.get("user") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
    }

    /// Delete SSO user mapping
    pub async fn delete_user_mapping(&self, user_id: u32) -> Result<()> {
        self.client.delete(&format!("/sso/users/{}", user_id)).await
    }

    /// Get SSO groups
    pub async fn list_groups(&self) -> Result<Vec<SsoGroupMapping>> {
        let v: serde_json::Value = self.client.get("/sso/groups").await?;
        if v.is_array() {
            serde_json::from_value(v).map_err(Into::into)
        } else if let Some(arr) = v.get("groups") {
            serde_json::from_value(arr.clone()).map_err(Into::into)
        } else {
            Ok(vec![])
        }
    }

    /// Map SSO group to role
    pub async fn map_group(&self, mapping: serde_json::Value) -> Result<SsoGroupMapping> {
        let v: serde_json::Value = self.client.post("/sso/groups", &mapping).await?;
        if let Some(obj) = v.get("group") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
    }

    /// Update SSO group mapping
    pub async fn update_group_mapping(
        &self,
        group_id: u32,
        mapping: serde_json::Value,
    ) -> Result<SsoGroupMapping> {
        let v: serde_json::Value = self
            .client
            .put(&format!("/sso/groups/{}", group_id), &mapping)
            .await?;
        if let Some(obj) = v.get("group") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
    }

    /// Delete SSO group mapping
    pub async fn delete_group_mapping(&self, group_id: u32) -> Result<serde_json::Value> {
        self.client
            .delete(&format!("/sso/groups/{}", group_id))
            .await?;
        Ok(serde_json::json!({"message": format!("SSO group mapping {} deleted", group_id)}))
    }
}
