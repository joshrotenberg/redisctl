//! Account operations handler

use crate::{Result, client::CloudClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Account response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountResponse {
    pub account: CloudAccount,
}

/// Cloud account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudAccount {
    pub id: u32,
    pub name: String,
    #[serde(rename = "createdTimestamp")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedTimestamp")]
    pub updated_at: Option<String>,
    pub key: Option<AccountKey>,

    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountKey {
    pub name: String,
    #[serde(rename = "accountId")]
    pub account_id: Option<u32>,
    #[serde(rename = "createdTimestamp")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedTimestamp")]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

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

    /// Get current account information (alias for info())
    pub async fn get(&self) -> Result<CloudAccount> {
        self.info().await
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
