//! Cloud account operations handler

use crate::{
    Result,
    client::CloudClient,
    models::{
        CloudProviderAccount, CreateCloudProviderAccountRequest, UpdateCloudProviderAccountRequest,
    },
};

/// Handler for Cloud account operations
pub struct CloudAccountsHandler {
    client: CloudClient,
}

impl CloudAccountsHandler {
    pub fn new(client: CloudClient) -> Self {
        CloudAccountsHandler { client }
    }

    /// List all cloud accounts (typed)
    pub async fn list(&self) -> Result<Vec<CloudProviderAccount>> {
        let v: serde_json::Value = self.client.get("/cloud-accounts").await?;
        if v.is_array() {
            serde_json::from_value(v).map_err(Into::into)
        } else if let Some(arr) = v.get("accounts") {
            serde_json::from_value(arr.clone()).map_err(Into::into)
        } else {
            Ok(vec![])
        }
    }

    /// Get cloud account by ID
    pub async fn get(&self, account_id: u32) -> Result<CloudProviderAccount> {
        self.client.get(&format!("/cloud-accounts/{}", account_id)).await
    }

    /// Create a new cloud account
    pub async fn create(
        &self,
        request: CreateCloudProviderAccountRequest,
    ) -> Result<CloudProviderAccount> {
        self.client.post("/cloud-accounts", &request).await
    }

    /// Update cloud account
    pub async fn update(
        &self,
        account_id: u32,
        request: UpdateCloudProviderAccountRequest,
    ) -> Result<CloudProviderAccount> {
        self.client
            .put(&format!("/cloud-accounts/{}", account_id), &request)
            .await
    }

    /// Delete cloud account
    pub async fn delete(&self, account_id: u32) -> Result<()> {
        self.client
            .delete(&format!("/cloud-accounts/{}", account_id))
            .await
    }
}
