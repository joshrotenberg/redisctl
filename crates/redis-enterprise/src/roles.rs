//! Role management for Redis Enterprise

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

/// Role information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleInfo {
    pub uid: u32,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub management: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bdb_roles: Option<Vec<BdbRole>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_roles: Option<Vec<String>>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Database-specific role permissions
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct BdbRole {
    pub bdb_uid: u32,
    #[builder(setter(into))]
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub redis_acl_uid: Option<u32>,
}

/// Create role request
///
/// # Examples
///
/// ```rust,no_run
/// use redis_enterprise::{CreateRoleRequest, BdbRole};
///
/// let request = CreateRoleRequest::builder()
///     .name("database-admin")
///     .management("admin")
///     .bdb_roles(vec![
///         BdbRole::builder()
///             .bdb_uid(1)
///             .role("admin")
///             .build()
///     ])
///     .build();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CreateRoleRequest {
    #[builder(setter(into))]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub management: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub data_access: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub bdb_roles: Option<Vec<BdbRole>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub cluster_roles: Option<Vec<String>>,
}

/// Roles handler
pub struct RolesHandler {
    client: RestClient,
}

impl RolesHandler {
    pub fn new(client: RestClient) -> Self {
        RolesHandler { client }
    }

    /// List all roles
    pub async fn list(&self) -> Result<Vec<RoleInfo>> {
        self.client.get("/v1/roles").await
    }

    /// Get specific role
    pub async fn get(&self, uid: u32) -> Result<RoleInfo> {
        self.client.get(&format!("/v1/roles/{}", uid)).await
    }

    /// Create a new role
    pub async fn create(&self, request: CreateRoleRequest) -> Result<RoleInfo> {
        self.client.post("/v1/roles", &request).await
    }

    /// Update an existing role
    pub async fn update(&self, uid: u32, request: CreateRoleRequest) -> Result<RoleInfo> {
        self.client
            .put(&format!("/v1/roles/{}", uid), &request)
            .await
    }

    /// Delete a role
    pub async fn delete(&self, uid: u32) -> Result<()> {
        self.client.delete(&format!("/v1/roles/{}", uid)).await
    }

    /// Get built-in roles
    pub async fn built_in(&self) -> Result<Vec<RoleInfo>> {
        self.client.get("/v1/roles/builtin").await
    }

    /// Get users assigned to a role
    pub async fn users(&self, uid: u32) -> Result<Vec<u32>> {
        self.client.get(&format!("/v1/roles/{}/users", uid)).await
    }
}
