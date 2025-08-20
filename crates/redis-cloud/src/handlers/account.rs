//! Account operations handler

use crate::{
    Result,
    client::CloudClient,
    models::{AccountResponse, CloudAccount},
};
use serde_json::Value;

/// Handler for Cloud account operations
pub struct CloudAccountHandler {
    client: CloudClient,
}

impl CloudAccountHandler {
    pub fn new(client: CloudClient) -> Self {
        CloudAccountHandler { client }
    }

    /// Get current account information
    pub async fn info(&self) -> Result<CloudAccount> {
        let response: AccountResponse = self.client.get("/").await?;
        Ok(response.account)
    }

    /// Get account owner information
    pub async fn owner(&self) -> Result<Value> {
        self.client.get("/users/owners").await
    }

    /// Get account users
    pub async fn users(&self) -> Result<Value> {
        self.client.get("/users").await
    }
    
    // Aliases for CLI compatibility
    pub async fn get_account(&self) -> Result<Value> {
        self.client.get("/").await
    }
    
    pub async fn get_users(&self) -> Result<Value> {
        self.users().await
    }
    
    pub async fn get_owner(&self) -> Result<Value> {
        self.owner().await
    }
    
    pub async fn get_payment_methods(&self) -> Result<Value> {
        self.client.get("/payment-methods").await
    }
}
