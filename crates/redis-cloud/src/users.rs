//! Users operations and models

use crate::{CloudClient, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// ============================================================================
// Models
// ============================================================================

/// User update request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountUserUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,

    /// The account user's name.
    pub name: String,

    /// Changes the account user's role. See [Team management roles](https://redis.io/docs/latest/operate/rc/security/access-control/access-management/#team-management-roles) to learn about available account roles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// RedisLabs list of users in current account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountUsers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<i32>,

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

/// RedisLabs User options information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountUserOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_alerts: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub operational_emails: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_enabled: Option<bool>,

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

/// RedisLabs User information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_up: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_api_key: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<AccountUserOptions>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

// ============================================================================
// Handler
// ============================================================================

/// Users operations handler
pub struct UsersHandler {
    client: CloudClient,
}

impl UsersHandler {
    /// Create a new handler
    pub fn new(client: CloudClient) -> Self {
        Self { client }
    }

    /// Get users
    /// Gets a list of all account users.
    ///
    /// GET /users
    pub async fn get_all_users(&self) -> Result<AccountUsers> {
        self.client.get("/users").await
    }

    /// Delete user
    /// Deletes a user from this account.
    ///
    /// DELETE /users/{userId}
    pub async fn delete_user_by_id(&self, user_id: i32) -> Result<TaskStateUpdate> {
        let response = self
            .client
            .delete_raw(&format!("/users/{}", user_id))
            .await?;
        serde_json::from_value(response).map_err(Into::into)
    }

    /// Get a single user
    /// Gets details about a single account user.
    ///
    /// GET /users/{userId}
    pub async fn get_user_by_id(&self, user_id: i32) -> Result<AccountUser> {
        self.client.get(&format!("/users/{}", user_id)).await
    }

    /// Update a user
    /// Updates an account user's name or role.
    ///
    /// PUT /users/{userId}
    pub async fn update_user(
        &self,
        user_id: i32,
        request: &AccountUserUpdateRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .put(&format!("/users/{}", user_id), request)
            .await
    }
}
