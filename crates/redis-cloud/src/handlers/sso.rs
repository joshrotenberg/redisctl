//! SSO/SAML configuration handler

use crate::{
    Result,
    client::CloudClient,
    models::{
        SamlConfig, SamlMetadata, SsoConfig, SsoGroupMapping, SsoTestResponse, SsoUserMapping,
        UpdateSamlConfigRequest, UpdateSsoConfigRequest,
    },
};

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
        self.client.get("/sso").await
    }

    /// Update SSO configuration (typed)
    pub async fn update(&self, request: UpdateSsoConfigRequest) -> Result<SsoConfig> {
        self.client.put("/sso", &request).await
    }

    /// Delete SSO configuration
    pub async fn delete(&self) -> Result<()> {
        self.client.delete("/sso").await
    }

    /// Test SSO configuration
    pub async fn test(&self, request: UpdateSsoConfigRequest) -> Result<SsoTestResponse> {
        self.client.post("/sso/test", &request).await
    }

    /// Get SAML configuration
    pub async fn get_saml(&self) -> Result<SamlConfig> {
        self.client.get("/sso/saml").await
    }

    /// Update SAML configuration
    pub async fn update_saml(&self, request: UpdateSamlConfigRequest) -> Result<SamlConfig> {
        self.client.put("/sso/saml", &request).await
    }

    /// Get SAML metadata
    pub async fn get_saml_metadata(&self) -> Result<SamlMetadata> {
        self.client.get("/sso/saml/metadata").await
    }

    /// Upload SAML certificate
    pub async fn upload_saml_cert(&self, certificate_pem: String) -> Result<SamlConfig> {
        let payload = serde_json::json!({"certificate": certificate_pem});
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
        self.client.get(&format!("/sso/users/{}", user_id)).await
    }

    /// Create SSO user mapping
    pub async fn create_user_mapping(&self, mapping: SsoUserMapping) -> Result<SsoUserMapping> {
        self.client.post("/sso/users", &mapping).await
    }

    /// Update SSO user mapping
    pub async fn update_user_mapping(
        &self,
        user_id: u32,
        mapping: SsoUserMapping,
    ) -> Result<SsoUserMapping> {
        self.client
            .put(&format!("/sso/users/{}", user_id), &mapping)
            .await
    }

    /// Delete SSO user mapping
    pub async fn delete_user_mapping(&self, user_id: u32) -> Result<()> {
        self.client
            .delete(&format!("/sso/users/{}", user_id))
            .await
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
    pub async fn map_group(&self, mapping: SsoGroupMapping) -> Result<SsoGroupMapping> {
        self.client.post("/sso/groups", &mapping).await
    }

    /// Update SSO group mapping
    pub async fn update_group_mapping(
        &self,
        group_id: u32,
        mapping: SsoGroupMapping,
    ) -> Result<SsoGroupMapping> {
        self.client
            .put(&format!("/sso/groups/{}", group_id), &mapping)
            .await
    }

    /// Delete SSO group mapping
    pub async fn delete_group_mapping(&self, group_id: u32) -> Result<()> {
        self.client
            .delete(&format!("/sso/groups/{}", group_id))
            .await
    }
}
