//! User and role management for Redis Enterprise

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

/// User information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub uid: u32,
    pub username: String,
    pub email: Option<String>,
    pub role: String,
    pub status: Option<String>,
    pub password_issue_date: Option<String>,
    pub email_alerts: Option<bool>,
    pub bdbs: Option<Vec<u32>>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Create user request
///
/// # Examples
///
/// ```rust,no_run
/// use redis_enterprise::CreateUserRequest;
///
/// let request = CreateUserRequest::builder()
///     .username("john.doe")
///     .password("secure-password-123")
///     .role("db_admin")
///     .email("john.doe@example.com")
///     .email_alerts(true)
///     .build();
/// ```
#[derive(Debug, Serialize, TypedBuilder)]
pub struct CreateUserRequest {
    #[builder(setter(into))]
    pub username: String,
    #[builder(setter(into))]
    pub password: String,
    #[builder(setter(into))]
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub email_alerts: Option<bool>,
}

/// Update user request
///
/// # Examples
///
/// ```rust,no_run
/// use redis_enterprise::UpdateUserRequest;
///
/// let request = UpdateUserRequest::builder()
///     .password("new-secure-password")
///     .email_alerts(false)
///     .build();
/// ```
#[derive(Debug, Serialize, TypedBuilder)]
pub struct UpdateUserRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub email_alerts: Option<bool>,
}

/// Role information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub uid: u32,
    pub name: String,
    pub management: Option<String>,
    pub data_access: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

/// User handler for managing users
pub struct UserHandler {
    client: RestClient,
}

impl UserHandler {
    pub fn new(client: RestClient) -> Self {
        UserHandler { client }
    }

    /// List all users
    pub async fn list(&self) -> Result<Vec<User>> {
        self.client.get("/v1/users").await
    }

    /// Get specific user
    pub async fn get(&self, uid: u32) -> Result<User> {
        self.client.get(&format!("/v1/users/{}", uid)).await
    }

    /// Create new user
    pub async fn create(&self, request: CreateUserRequest) -> Result<User> {
        self.client.post("/v1/users", &request).await
    }

    /// Update user
    pub async fn update(&self, uid: u32, request: UpdateUserRequest) -> Result<User> {
        self.client
            .put(&format!("/v1/users/{}", uid), &request)
            .await
    }

    /// Delete user
    pub async fn delete(&self, uid: u32) -> Result<()> {
        self.client.delete(&format!("/v1/users/{}", uid)).await
    }
}

/// Role handler for managing roles
pub struct RoleHandler {
    client: RestClient,
}

impl RoleHandler {
    pub fn new(client: RestClient) -> Self {
        RoleHandler { client }
    }

    /// List all roles
    pub async fn list(&self) -> Result<Vec<Role>> {
        self.client.get("/v1/roles").await
    }

    /// Get specific role
    pub async fn get(&self, uid: u32) -> Result<Role> {
        self.client.get(&format!("/v1/roles/{}", uid)).await
    }
}
