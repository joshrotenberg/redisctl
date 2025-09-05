//! Role-based Access Control (RBAC) operations and models
//!
//! This module provides comprehensive access control management for Redis Cloud,
//! including ACL management for users, roles, Redis rules, and database-level
//! access controls.
//!
//! # Overview
//!
//! The ACL module implements Redis Cloud's role-based access control system, allowing
//! fine-grained control over who can access what resources and perform which operations.
//! It supports both user-level and database-level access controls.
//!
//! # Key Features
//!
//! - **User ACLs**: Manage user access control lists and permissions
//! - **Role Management**: Create and manage roles with specific permissions
//! - **Redis Rules**: Define Redis command-level access rules
//! - **Database ACLs**: Control access at the database level
//! - **Rule Association**: Link users and roles to specific databases
//!
//! # Example Usage
//!
//! ```no_run
//! use redis_cloud::{CloudClient, AclHandler};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = CloudClient::builder()
//!     .api_key("your-api-key")
//!     .api_secret("your-api-secret")
//!     .build()?;
//!
//! let handler = AclHandler::new(client);
//!
//! // List all ACL users
//! let users = handler.get_all_acl_users().await?;
//!
//! // Get all Redis rules
//! let rules = handler.get_all_redis_rules().await?;
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

/// ACL role create request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AclRoleCreateRequest {
    /// Database access role name.
    pub name: String,

    /// A list of Redis ACL rules to assign to this database access role.
    pub redis_rules: Vec<AclRoleRedisRuleSpec>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

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

/// ACL user update request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AclUserUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i32>,

    /// Optional. Changes the ACL role assigned to the user. Use GET '/acl/roles' to get a list of database access roles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    /// Optional. Changes the user's database password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Redis list of ACL users in current account
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountACLUsers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<i32>,

    /// HATEOAS links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<HashMap<String, Value>>>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Redis list of ACL redis rules in current account
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountACLRedisRules {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<i32>,

    /// HATEOAS links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<HashMap<String, Value>>>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// ACL redis rule create request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AclRedisRuleCreateRequest {
    /// Redis ACL rule name.
    pub name: String,

    /// Redis ACL rule pattern. See [ACL syntax](https://redis.io/docs/latest/operate/rc/security/access-control/data-access-control/configure-acls/#define-permissions-with-acl-syntax) to learn how to define rules.
    pub redis_rule: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Redis list of ACL roles in current account
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountACLRoles {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<i32>,

    /// HATEOAS links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<HashMap<String, Value>>>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// ACL redis rule update request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AclRedisRuleUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis_rule_id: Option<i32>,

    /// Optional. Changes the Redis ACL rule name.
    pub name: String,

    /// Optional. Changes the Redis ACL rule pattern. See [ACL syntax](https://redis.io/docs/latest/operate/rc/security/access-control/data-access-control/configure-acls/#define-permissions-with-acl-syntax) to learn how to define rules.
    pub redis_rule: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// A list of databases where the specified rule applies for this role.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AclRoleDatabaseSpec {
    /// Subscription ID for the database's subscription. Use 'GET /subscriptions' or 'GET /fixed/subscriptions' to get a list of available subscriptions and their IDs.
    pub subscription_id: i32,

    /// The database's ID. Use 'GET /subscriptions/{subscriptionId}/databases' or 'GET /fixed/subscriptions/{subscriptionId}/databases' to get a list of databases in a subscription and their IDs.
    pub database_id: i32,

    /// (Active-Active databases only) Optional. A list of regions where this rule applies for this role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// ACL user create request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AclUserCreateRequest {
    /// Access control user name.
    pub name: String,

    /// Name of the database access role to assign to this user. Use GET '/acl/roles' to get a list of database access roles.
    pub role: String,

    /// The database password for this user.
    pub password: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Redis ACL user information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACLUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// HATEOAS links
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<HashMap<String, Value>>>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// ACL role update request
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AclRoleUpdateRequest {
    /// Optional. Changes the database access role name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Optional. Changes the Redis ACL rules to assign to this database access role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis_rules: Option<Vec<AclRoleRedisRuleSpec>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_type: Option<String>,

    /// Additional fields from the API
    #[serde(flatten)]
    pub extra: Value,
}

/// Optional. Changes the Redis ACL rules to assign to this database access role.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AclRoleRedisRuleSpec {
    /// The name of a Redis ACL rule to assign to the role. Use 'GET /acl/redisRules' to get a list of available rules for your account.
    pub rule_name: String,

    /// A list of databases where the specified rule applies for this role.
    pub databases: Vec<AclRoleDatabaseSpec>,

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

/// Handler for Role-based Access Control (RBAC) operations
///
/// Manages ACLs for users, roles, Redis rules, and database-level access controls.
/// Provides fine-grained permission management for Redis Cloud resources.
pub struct AclHandler {
    client: CloudClient,
}

impl AclHandler {
    /// Create a new handler
    pub fn new(client: CloudClient) -> Self {
        Self { client }
    }

    /// Get Redis ACL rules
    /// Gets a list of all Redis ACL rules for this account.
    ///
    /// GET /acl/redisRules
    pub async fn get_all_redis_rules(&self) -> Result<AccountACLRedisRules> {
        self.client.get("/acl/redisRules").await
    }

    /// Create Redis ACL rule
    /// Creates a new Redis ACL rule.
    ///
    /// POST /acl/redisRules
    pub async fn create_redis_rule(
        &self,
        request: &AclRedisRuleCreateRequest,
    ) -> Result<TaskStateUpdate> {
        self.client.post("/acl/redisRules", request).await
    }

    /// Delete Redis ACL rule
    /// Deletes a Redis ACL rule.
    ///
    /// DELETE /acl/redisRules/{aclRedisRuleId}
    pub async fn delete_redis_rule(&self, acl_redis_rule_id: i32) -> Result<TaskStateUpdate> {
        let response = self
            .client
            .delete_raw(&format!("/acl/redisRules/{}", acl_redis_rule_id))
            .await?;
        serde_json::from_value(response).map_err(Into::into)
    }

    /// Update Redis ACL rule
    /// Updates a Redis ACL rule.
    ///
    /// PUT /acl/redisRules/{aclRedisRuleId}
    pub async fn update_redis_rule(
        &self,
        acl_redis_rule_id: i32,
        request: &AclRedisRuleUpdateRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .put(&format!("/acl/redisRules/{}", acl_redis_rule_id), request)
            .await
    }

    /// Get database access roles
    /// Gets a list of all database access roles for this account.
    ///
    /// GET /acl/roles
    pub async fn get_roles(&self) -> Result<AccountACLRoles> {
        self.client.get("/acl/roles").await
    }

    /// Create database access role
    /// Creates a new database access role with the assigned permissions and associates it with the provided databases.
    ///
    /// POST /acl/roles
    pub async fn create_role(&self, request: &AclRoleCreateRequest) -> Result<TaskStateUpdate> {
        self.client.post("/acl/roles", request).await
    }

    /// Delete database access role
    /// Deletes a database access role.
    ///
    /// DELETE /acl/roles/{aclRoleId}
    pub async fn delete_acl_role(&self, acl_role_id: i32) -> Result<TaskStateUpdate> {
        let response = self
            .client
            .delete_raw(&format!("/acl/roles/{}", acl_role_id))
            .await?;
        serde_json::from_value(response).map_err(Into::into)
    }

    /// Update database access role
    /// Updates a database access role with new assigned permissions or associated databases.
    ///
    /// PUT /acl/roles/{aclRoleId}
    pub async fn update_role(
        &self,
        acl_role_id: i32,
        request: &AclRoleUpdateRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .put(&format!("/acl/roles/{}", acl_role_id), request)
            .await
    }

    /// Get access control users
    /// Gets a list of all access control users for this account.
    ///
    /// GET /acl/users
    pub async fn get_all_acl_users(&self) -> Result<AccountACLUsers> {
        self.client.get("/acl/users").await
    }

    /// Create access control user
    /// Creates a new access control user with the assigned database access role.
    ///
    /// POST /acl/users
    pub async fn create_user(&self, request: &AclUserCreateRequest) -> Result<TaskStateUpdate> {
        self.client.post("/acl/users", request).await
    }

    /// Delete access control user
    /// Deletes a access control user.
    ///
    /// DELETE /acl/users/{aclUserId}
    pub async fn delete_user(&self, acl_user_id: i32) -> Result<TaskStateUpdate> {
        let response = self
            .client
            .delete_raw(&format!("/acl/users/{}", acl_user_id))
            .await?;
        serde_json::from_value(response).map_err(Into::into)
    }

    /// Get a single access control user
    /// Gets details and settings for single access control user.
    ///
    /// GET /acl/users/{aclUserId}
    pub async fn get_user_by_id(&self, acl_user_id: i32) -> Result<ACLUser> {
        self.client
            .get(&format!("/acl/users/{}", acl_user_id))
            .await
    }

    /// Update access control user
    /// Updates a access control user with a different role or database password.
    ///
    /// PUT /acl/users/{aclUserId}
    pub async fn update_acl_user(
        &self,
        acl_user_id: i32,
        request: &AclUserUpdateRequest,
    ) -> Result<TaskStateUpdate> {
        self.client
            .put(&format!("/acl/users/{}", acl_user_id), request)
            .await
    }
}
