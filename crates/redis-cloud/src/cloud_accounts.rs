//! Cloud provider account management operations and models
//!
//! This module handles the integration between Redis Cloud and your cloud provider
//! accounts (AWS, GCP, Azure). It manages cloud account credentials, access keys,
//! and provider-specific configurations.
//!
//! # Overview
//!
//! Cloud accounts are the bridge between Redis Cloud and your infrastructure provider.
//! They store the credentials and permissions needed for Redis Cloud to provision
//! resources in your cloud environment.
//!
//! # Supported Providers
//!
//! - **AWS**: Amazon Web Services accounts with IAM roles or access keys
//! - **GCP**: Google Cloud Platform projects with service accounts
//! - **Azure**: Microsoft Azure subscriptions with service principals
//!
//! # Key Features
//!
//! - **Account Registration**: Register cloud provider accounts with Redis Cloud
//! - **Credential Management**: Securely store and manage cloud credentials
//! - **Access Key Operations**: Create, update, and delete cloud access keys
//! - **Provider Details**: Retrieve provider-specific account information
//! - **Multi-cloud Support**: Manage accounts across different cloud providers
//!
//! # Example Usage
//!
//! ```no_run
//! use redis_cloud::{CloudClient, CloudAccountHandler};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = CloudClient::builder()
//!     .api_key("your-api-key")
//!     .api_secret("your-api-secret")
//!     .build()?;
//!
//! let handler = CloudAccountHandler::new(client);
//!
//! // List all cloud accounts
//! let accounts = handler.get_cloud_accounts().await?;
//!
//! // Get specific account details (account ID 123)
//! let account = handler.get_cloud_account_by_id(123).await?;
//! # Ok(())
//! # }
//! ```

use crate::{CloudClient, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// ============================================================================
// Models
// ============================================================================

/// Cloud Account definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudAccountUpdateRequest {
    /// name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_account_id: Option<i32>,

    /// Cloud provider access key.
    pub access_key_id: String,

    /// Cloud provider secret key.
    pub access_secret_key: String,

    /// Cloud provider management console username.
    pub console_username: String,

    /// Cloud provider management console password.
    pub console_password: String,

    /// Optional. Cloud provider management console login URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_in_login_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// RedisLabs Cloud Account information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_in_login_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,

    /// HATEOAS links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<HashMap<String, Value>>>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Cloud Account definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudAccountCreateRequest {
    /// Cloud account display name.
    pub name: String,

    /// Optional. Cloud provider. Default: 'AWS'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,

    /// Cloud provider access key.
    pub access_key_id: String,

    /// Cloud provider secret key.
    pub access_secret_key: String,

    /// Cloud provider management console username.
    pub console_username: String,

    /// Cloud provider management console password.
    pub console_password: String,

    /// Cloud provider management console login URL.
    pub sign_in_login_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// RedisLabs Cloud Accounts information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudAccounts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<i32>,

    /// HATEOAS links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<HashMap<String, Value>>>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// ProcessorResponse
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_resource_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<HashMap<String, Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// TaskStateUpdate
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskStateUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<ProcessorResponse>,

    /// HATEOAS links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<HashMap<String, Value>>>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

// ============================================================================
// Handler
// ============================================================================

/// Handler for cloud provider account operations
///
/// Manages integration with AWS, GCP, and Azure accounts, including
/// credential management and provider-specific configurations.
pub struct CloudAccountsHandler {
    client: CloudClient,
}

impl CloudAccountsHandler {
    /// Create a new handler
    pub fn new(client: CloudClient) -> Self {
        Self { client }
    }

    /// Get cloud accounts
    /// Gets a list of all configured cloud accounts.
    ///
    /// GET /cloud-accounts
    pub async fn get_cloud_accounts(&self) -> Result<CloudAccounts> {
        self.client.get("/cloud-accounts").await
    }

    /// Create cloud account
    /// Creates a cloud account.
    ///
    /// POST /cloud-accounts
    pub async fn create_cloud_account(
        &self,
        request: &CloudAccountCreateRequest,
    ) -> Result<TaskStateUpdate> {
        self.client.post("/cloud-accounts", request).await
    }

    /// Delete cloud account
    /// Deletes a cloud account.
    ///
    /// DELETE /cloud-accounts/{cloudAccountId}
    pub async fn delete_cloud_account(&self, cloud_account_id: i32) -> Result<TaskStateUpdate> {
        let response = self
            .client
            .delete_raw(&format!("/cloud-accounts/{}", cloud_account_id))
            .await?;
        serde_json::from_value(response).map_err(Into::into)
    }

    /// Get a single cloud account
    /// Gets details on a single cloud account.
    ///
    /// GET /cloud-accounts/{cloudAccountId}
    pub async fn get_cloud_account_by_id(&self, cloud_account_id: i32) -> Result<CloudAccount> {
        self.client
            .get(&format!("/cloud-accounts/{}", cloud_account_id))
            .await
    }

    /// Update cloud account
    /// Updates cloud account details.
    ///
    /// PUT /cloud-accounts/{cloudAccountId}
    pub async fn update_cloud_account(
        &self,
        cloud_account_id: i32,
        request: &CloudAccountUpdateRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .put(&format!("/cloud-accounts/{}", cloud_account_id), request)
            .await
    }
}
