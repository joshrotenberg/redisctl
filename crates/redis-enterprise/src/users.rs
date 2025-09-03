//! Users management for Redis Enterprise
//!
//! ## Overview
//! - List and query resources
//! - Create and update configurations
//! - Monitor status and metrics
//!
//! Example
//! ```no_run
//! ```

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
///     .email("john.doe@example.com")
///     .password("secure-password-123")
///     .role("db_admin")
///     .name("John Doe")
///     .email_alerts(true)
///     .build();
/// ```
#[derive(Debug, Serialize, TypedBuilder)]
pub struct CreateUserRequest {
    /// User's email address (required, used as login)
    #[builder(setter(into))]
    pub email: String,
    /// User's password (required)
    #[builder(setter(into))]
    pub password: String,
    /// User's role (required) - for RBAC-enabled clusters, use role_uids instead
    #[builder(setter(into))]
    pub role: String,
    /// User's full name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub name: Option<String>,
    /// Whether user should receive email alerts
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub email_alerts: Option<bool>,
    /// Database IDs for which the user should receive email alerts
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub bdbs_email_alerts: Option<Vec<String>>,
    /// Role IDs for RBAC-enabled clusters
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub role_uids: Option<Vec<u32>>,
    /// Authentication method (e.g., "regular")
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub auth_method: Option<String>,
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
    /// New password for the user
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub password: Option<String>,
    /// Update user's role
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub role: Option<String>,
    /// Update user's email address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub email: Option<String>,
    /// Update user's full name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub name: Option<String>,
    /// Update email alerts preference
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub email_alerts: Option<bool>,
    /// Update database IDs for email alerts
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub bdbs_email_alerts: Option<Vec<String>>,
    /// Update role IDs for RBAC-enabled clusters
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub role_uids: Option<Vec<u32>>,
    /// Update authentication method
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub auth_method: Option<String>,
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

/// Alias for backwards compatibility and intuitive plural naming
pub type UsersHandler = UserHandler;

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

    /// Get permissions - GET /v1/users/permissions (raw)
    pub async fn permissions(&self) -> Result<Value> {
        self.client.get("/v1/users/permissions").await
    }

    /// Get permission detail - GET /v1/users/permissions/{perm} (raw)
    pub async fn permission_detail(&self, perm: &str) -> Result<Value> {
        self.client
            .get(&format!("/v1/users/permissions/{}", perm))
            .await
    }

    /// Authorize user (login) - POST /v1/users/authorize (raw)
    pub async fn authorize(&self, body: AuthRequest) -> Result<AuthResponse> {
        self.client.post("/v1/users/authorize", &body).await
    }

    /// Set password - POST /v1/users/password (raw)
    pub async fn password_set(&self, body: PasswordSet) -> Result<()> {
        self.client.post_action("/v1/users/password", &body).await
    }

    /// Update password - PUT /v1/users/password (raw)
    pub async fn password_update(&self, body: PasswordUpdate) -> Result<()> {
        self.client.put("/v1/users/password", &body).await
    }

    /// Delete password - DELETE /v1/users/password
    pub async fn password_delete(&self) -> Result<()> {
        self.client.delete("/v1/users/password").await
    }

    /// Refresh JWT - POST /v1/users/refresh_jwt (raw)
    pub async fn refresh_jwt(&self, body: JwtRefreshRequest) -> Result<JwtRefreshResponse> {
        self.client.post("/v1/users/refresh_jwt", &body).await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub jwt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordSet {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_password: Option<String>,
    pub new_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtRefreshRequest {
    pub jwt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtRefreshResponse {
    pub jwt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
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
