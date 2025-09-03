//! Cloud account management for Redis Cloud
//!
//! ## Overview
//! - Link AWS, GCP, and Azure accounts
//! - Configure cloud provider settings
//! - Manage cloud account credentials

use crate::client::CloudClient;
use crate::error::Result;
use crate::types::CloudProvider;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

/// Cloud account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudAccount {
    pub id: u32,
    pub name: String,
    pub provider: CloudProvider,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(rename = "accessKeyId", skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,

    #[serde(rename = "iamRoleArn", skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,

    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,

    #[serde(rename = "subscriptionId", skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,

    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,

    #[serde(rename = "clientId", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Create cloud account request
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CreateCloudAccountRequest {
    #[builder(setter(into))]
    pub name: String,

    pub provider: CloudProvider,

    // AWS fields
    #[serde(rename = "accessKeyId", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub access_key_id: Option<String>,

    #[serde(rename = "accessSecretKey", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub access_secret_key: Option<String>,

    #[serde(rename = "iamRoleArn", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub iam_role_arn: Option<String>,

    // GCP fields
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub project_id: Option<String>,

    #[serde(rename = "serviceAccountJson", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub service_account_json: Option<String>,

    // Azure fields
    #[serde(rename = "subscriptionId", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub subscription_id: Option<String>,

    #[serde(rename = "tenantId", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub tenant_id: Option<String>,

    #[serde(rename = "clientId", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub client_id: Option<String>,

    #[serde(rename = "clientSecret", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub client_secret: Option<String>,
}

/// Update cloud account request
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UpdateCloudAccountRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub name: Option<String>,

    // AWS fields
    #[serde(rename = "accessKeyId", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub access_key_id: Option<String>,

    #[serde(rename = "accessSecretKey", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub access_secret_key: Option<String>,

    // GCP fields
    #[serde(rename = "serviceAccountJson", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub service_account_json: Option<String>,

    // Azure fields
    #[serde(rename = "clientSecret", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub client_secret: Option<String>,
}

/// Cloud account handler
pub struct CloudAccountHandler {
    client: CloudClient,
}

impl CloudAccountHandler {
    pub fn new(client: CloudClient) -> Self {
        CloudAccountHandler { client }
    }

    /// List all cloud accounts
    pub async fn list(&self) -> Result<Vec<CloudAccount>> {
        self.client.get("/cloud-accounts").await
    }

    /// Get a specific cloud account
    pub async fn get(&self, account_id: u32) -> Result<CloudAccount> {
        self.client
            .get(&format!("/cloud-accounts/{}", account_id))
            .await
    }

    /// Create a new cloud account
    pub async fn create(&self, request: CreateCloudAccountRequest) -> Result<CloudAccount> {
        self.client.post("/cloud-accounts", &request).await
    }

    /// Update a cloud account
    pub async fn update(
        &self,
        account_id: u32,
        request: UpdateCloudAccountRequest,
    ) -> Result<CloudAccount> {
        self.client
            .put(&format!("/cloud-accounts/{}", account_id), &request)
            .await
    }

    /// Delete a cloud account
    pub async fn delete(&self, account_id: u32) -> Result<Value> {
        self.client
            .delete(&format!("/cloud-accounts/{}", account_id))
            .await
    }
}
