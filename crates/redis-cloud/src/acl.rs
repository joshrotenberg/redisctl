//! Role-based access control (RBAC) for Redis Cloud
//!
//! ## Overview
//! - Manage ACL users, roles, and Redis rules
//! - Configure fine-grained access control
//! - Associate roles with databases

use crate::client::CloudClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

/// ACL User
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AclUser {
    pub id: u32,
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

/// ACL Role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AclRole {
    pub id: u32,
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub databases: Option<Vec<AclRoleDatabaseSpec>>,

    #[serde(rename = "redisRules", skip_serializing_if = "Option::is_none")]
    pub redis_rules: Option<Vec<AclRoleRedisRuleSpec>>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Database specification for ACL role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AclRoleDatabaseSpec {
    #[serde(rename = "subscriptionId")]
    pub subscription_id: u32,

    #[serde(rename = "databaseId")]
    pub database_id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
}

/// Redis rule specification for ACL role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AclRoleRedisRuleSpec {
    #[serde(rename = "ruleId")]
    pub rule_id: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub databases: Option<Vec<u32>>,
}

/// ACL Redis Rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AclRedisRule {
    pub id: u32,
    pub name: String,

    #[serde(rename = "acl")]
    pub acl_string: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Create ACL user request
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CreateAclUserRequest {
    #[builder(setter(into))]
    pub name: String,

    #[builder(setter(into))]
    pub role: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub password: Option<String>,
}

/// Update ACL user request
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UpdateAclUserRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub role: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub password: Option<String>,
}

/// Create ACL role request
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CreateAclRoleRequest {
    #[builder(setter(into))]
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub databases: Option<Vec<AclRoleDatabaseSpec>>,

    #[serde(rename = "redisRules", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub redis_rules: Option<Vec<AclRoleRedisRuleSpec>>,
}

/// Update ACL role request
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UpdateAclRoleRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub databases: Option<Vec<AclRoleDatabaseSpec>>,

    #[serde(rename = "redisRules", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub redis_rules: Option<Vec<AclRoleRedisRuleSpec>>,
}

/// Create ACL Redis rule request
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CreateAclRedisRuleRequest {
    #[builder(setter(into))]
    pub name: String,

    #[serde(rename = "acl")]
    #[builder(setter(into))]
    pub acl_string: String,
}

/// Update ACL Redis rule request  
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UpdateAclRedisRuleRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub name: Option<String>,

    #[serde(rename = "acl", skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub acl_string: Option<String>,
}

/// ACL handler
pub struct AclHandler {
    client: CloudClient,
}

impl AclHandler {
    pub fn new(client: CloudClient) -> Self {
        AclHandler { client }
    }

    // ACL Users

    /// List all ACL users
    pub async fn list_users(&self) -> Result<Vec<AclUser>> {
        self.client.get("/acl/users").await
    }

    /// Get a specific ACL user
    pub async fn get_user(&self, user_id: u32) -> Result<AclUser> {
        self.client.get(&format!("/acl/users/{}", user_id)).await
    }

    /// Create a new ACL user
    pub async fn create_user(&self, request: CreateAclUserRequest) -> Result<AclUser> {
        self.client.post("/acl/users", &request).await
    }

    /// Update an ACL user
    pub async fn update_user(
        &self,
        user_id: u32,
        request: UpdateAclUserRequest,
    ) -> Result<AclUser> {
        self.client
            .put(&format!("/acl/users/{}", user_id), &request)
            .await
    }

    /// Delete an ACL user
    pub async fn delete_user(&self, user_id: u32) -> Result<Value> {
        self.client.delete(&format!("/acl/users/{}", user_id)).await
    }

    // ACL Roles

    /// List all ACL roles
    pub async fn list_roles(&self) -> Result<Vec<AclRole>> {
        self.client.get("/acl/roles").await
    }

    /// Get a specific ACL role
    pub async fn get_role(&self, role_id: u32) -> Result<AclRole> {
        self.client.get(&format!("/acl/roles/{}", role_id)).await
    }

    /// Create a new ACL role
    pub async fn create_role(&self, request: CreateAclRoleRequest) -> Result<AclRole> {
        self.client.post("/acl/roles", &request).await
    }

    /// Update an ACL role
    pub async fn update_role(
        &self,
        role_id: u32,
        request: UpdateAclRoleRequest,
    ) -> Result<AclRole> {
        self.client
            .put(&format!("/acl/roles/{}", role_id), &request)
            .await
    }

    /// Delete an ACL role
    pub async fn delete_role(&self, role_id: u32) -> Result<Value> {
        self.client.delete(&format!("/acl/roles/{}", role_id)).await
    }

    // ACL Redis Rules

    /// List all ACL Redis rules
    pub async fn list_redis_rules(&self) -> Result<Vec<AclRedisRule>> {
        self.client.get("/acl/redisRules").await
    }

    /// Get a specific ACL Redis rule
    pub async fn get_redis_rule(&self, rule_id: u32) -> Result<AclRedisRule> {
        self.client
            .get(&format!("/acl/redisRules/{}", rule_id))
            .await
    }

    /// Create a new ACL Redis rule
    pub async fn create_redis_rule(
        &self,
        request: CreateAclRedisRuleRequest,
    ) -> Result<AclRedisRule> {
        self.client.post("/acl/redisRules", &request).await
    }

    /// Update an ACL Redis rule
    pub async fn update_redis_rule(
        &self,
        rule_id: u32,
        request: UpdateAclRedisRuleRequest,
    ) -> Result<AclRedisRule> {
        self.client
            .put(&format!("/acl/redisRules/{}", rule_id), &request)
            .await
    }

    /// Delete an ACL Redis rule
    pub async fn delete_redis_rule(&self, rule_id: u32) -> Result<Value> {
        self.client
            .delete(&format!("/acl/redisRules/{}", rule_id))
            .await
    }
}
