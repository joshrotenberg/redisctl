//! Account management for Redis Cloud
//!
//! ## Overview
//! - Get current account information
//! - Query account limits and usage
//! - Manage account settings

use crate::client::CloudClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: u32,
    pub name: String,

    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "createdTimestamp", skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<AccountKey>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub providers: Option<Vec<ProviderInfo>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<SubscriptionSummary>>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Account API key information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountKey {
    pub name: String,

    #[serde(rename = "accountId")]
    pub account_id: u32,

    #[serde(rename = "accountName", skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,

    #[serde(rename = "allowedSourceIPs", skip_serializing_if = "Option::is_none")]
    pub allowed_source_ips: Option<Vec<String>>,

    #[serde(rename = "createdTimestamp", skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,

    #[serde(rename = "lastUsedTimestamp", skip_serializing_if = "Option::is_none")]
    pub last_used_timestamp: Option<String>,
}

/// Provider information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderInfo {
    pub provider: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,

    #[serde(rename = "cloudAccountId", skip_serializing_if = "Option::is_none")]
    pub cloud_account_id: Option<u32>,
}

/// Subscription summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionSummary {
    pub id: u32,
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(rename = "numberOfDatabases", skip_serializing_if = "Option::is_none")]
    pub number_of_databases: Option<u32>,
}

/// Account handler
pub struct AccountHandler {
    client: CloudClient,
}

impl AccountHandler {
    pub fn new(client: CloudClient) -> Self {
        AccountHandler { client }
    }

    /// Get current account information
    pub async fn get(&self) -> Result<Account> {
        self.client.get("/").await
    }

    /// Get account by ID
    pub async fn get_by_id(&self, account_id: u32) -> Result<Account> {
        self.client.get(&format!("/accounts/{}", account_id)).await
    }
}
