//! Account-related data models

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
