//! ACL and RBAC operations handler

use crate::{Result, client::CloudClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

/// ACL User information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AclUser {
    pub id: u32,
    pub name: String,
    pub email: Option<String>,
    pub status: Option<String>,
    pub roles: Option<Vec<u32>>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Request to create an ACL user
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CreateAclUserRequest {
    pub name: String,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<u32>>,
}

/// Request to update an ACL user
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UpdateAclUserRequest {
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<u32>>,
}

/// ACL Role information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AclRole {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub redis_rules: Option<Vec<u32>>,
    pub databases: Option<Vec<DatabaseAclBinding>>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Database ACL binding information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseAclBinding {
    pub subscription_id: u32,
    pub database_id: u32,
    pub redis_rule_ids: Vec<u32>,
}

/// Request to create an ACL role
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CreateAclRoleRequest {
    pub name: String,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis_rules: Option<Vec<u32>>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub databases: Option<Vec<DatabaseAclBinding>>,
}

/// Request to update an ACL role
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UpdateAclRoleRequest {
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis_rules: Option<Vec<u32>>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub databases: Option<Vec<DatabaseAclBinding>>,
}

/// Redis ACL rule information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisRule {
    pub id: u32,
    pub name: String,
    pub acl_syntax: String,
    pub description: Option<String>,
    pub is_active: Option<bool>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Request to create a Redis rule
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CreateRedisRuleRequest {
    pub name: String,
    pub acl_syntax: String,
    #[builder(default, setter(strip_option))]
    pub description: Option<String>,
    #[builder(default = true)]
    #[serde(default = "default_true")]
    #[serde(skip_serializing_if = "is_true")]
    pub is_active: bool,
}

/// Request to update a Redis rule
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UpdateRedisRuleRequest {
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_syntax: Option<String>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
}

/// Database ACL configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseAcl {
    pub id: u32,
    pub subscription_id: u32,
    pub database_id: u32,
    pub name: String,
    pub redis_rule_id: u32,
    pub is_active: Option<bool>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Request to create a database ACL
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CreateDatabaseAclRequest {
    pub name: String,
    pub redis_rule_id: u32,
    #[builder(default = true)]
    #[serde(default = "default_true")]
    pub is_active: bool,
}

/// Request to update a database ACL
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UpdateDatabaseAclRequest {
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
    #[builder(default, setter(strip_option))]
    pub redis_rule_id: Option<u32>,
    #[builder(default, setter(strip_option))]
    pub is_active: Option<bool>,
}

pub(crate) fn default_true() -> bool {
    true
}
pub(crate) fn is_true(b: &bool) -> bool {
    *b
}

/// Handler for Cloud ACL/RBAC operations
pub struct CloudAclHandler {
    client: CloudClient,
}

impl CloudAclHandler {
    pub fn new(client: CloudClient) -> Self {
        CloudAclHandler { client }
    }

    // Database ACL methods
    pub async fn list(&self, subscription_id: u32, database_id: u32) -> Result<Vec<DatabaseAcl>> {
        self.client
            .get(&format!(
                "/subscriptions/{}/databases/{}/acl",
                subscription_id, database_id
            ))
            .await
    }

    pub async fn get(
        &self,
        subscription_id: u32,
        database_id: u32,
        acl_id: u32,
    ) -> Result<DatabaseAcl> {
        self.client
            .get(&format!(
                "/subscriptions/{}/databases/{}/acl/{}",
                subscription_id, database_id, acl_id
            ))
            .await
    }

    pub async fn create(
        &self,
        subscription_id: u32,
        database_id: u32,
        request: CreateDatabaseAclRequest,
    ) -> Result<DatabaseAcl> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/databases/{}/acl",
                    subscription_id, database_id
                ),
                &request,
            )
            .await
    }

    pub async fn update(
        &self,
        subscription_id: u32,
        database_id: u32,
        acl_id: u32,
        request: UpdateDatabaseAclRequest,
    ) -> Result<DatabaseAcl> {
        self.client
            .put(
                &format!(
                    "/subscriptions/{}/databases/{}/acl/{}",
                    subscription_id, database_id, acl_id
                ),
                &request,
            )
            .await
    }

    pub async fn delete(
        &self,
        subscription_id: u32,
        database_id: u32,
        acl_id: u32,
    ) -> Result<Value> {
        self.client
            .delete(&format!(
                "/subscriptions/{}/databases/{}/acl/{}",
                subscription_id, database_id, acl_id
            ))
            .await?;
        Ok(serde_json::json!({"message": format!("ACL rule {} deleted", acl_id)}))
    }

    /// List all ACL users
    pub async fn list_users(&self) -> Result<Vec<AclUser>> {
        self.client.get("/acl/users").await
    }

    /// Get ACL user by ID
    pub async fn get_user(&self, user_id: u32) -> Result<AclUser> {
        self.client.get(&format!("/acl/users/{}", user_id)).await
    }

    /// Create ACL user
    pub async fn create_user(&self, request: CreateAclUserRequest) -> Result<AclUser> {
        self.client.post("/acl/users", &request).await
    }

    /// Update ACL user
    pub async fn update_user(
        &self,
        user_id: u32,
        request: UpdateAclUserRequest,
    ) -> Result<AclUser> {
        self.client
            .put(&format!("/acl/users/{}", user_id), &request)
            .await
    }

    /// Delete ACL user
    pub async fn delete_user(&self, user_id: u32) -> Result<()> {
        self.client.delete(&format!("/acl/users/{}", user_id)).await
    }

    /// List all ACL roles
    pub async fn list_roles(&self) -> Result<Vec<AclRole>> {
        self.client.get("/acl/roles").await
    }

    /// Get ACL role by ID
    pub async fn get_role(&self, role_id: u32) -> Result<AclRole> {
        self.client.get(&format!("/acl/roles/{}", role_id)).await
    }

    /// Create ACL role
    pub async fn create_role(&self, request: CreateAclRoleRequest) -> Result<AclRole> {
        self.client.post("/acl/roles", &request).await
    }

    /// Update ACL role
    pub async fn update_role(
        &self,
        role_id: u32,
        request: UpdateAclRoleRequest,
    ) -> Result<AclRole> {
        self.client
            .put(&format!("/acl/roles/{}", role_id), &request)
            .await
    }

    /// Delete ACL role
    pub async fn delete_role(&self, role_id: u32) -> Result<()> {
        self.client.delete(&format!("/acl/roles/{}", role_id)).await
    }

    /// List Redis rules
    pub async fn list_redis_rules(&self) -> Result<Vec<RedisRule>> {
        let v: Value = self.client.get("/acl/redisRules").await?;
        if v.is_array() {
            serde_json::from_value(v).map_err(Into::into)
        } else if let Some(arr) = v.get("rules") {
            serde_json::from_value(arr.clone()).map_err(Into::into)
        } else {
            Ok(vec![])
        }
    }

    /// Get Redis rule by ID
    pub async fn get_redis_rule(&self, rule_id: u32) -> Result<RedisRule> {
        self.client
            .get(&format!("/acl/redisRules/{}", rule_id))
            .await
    }

    /// Create Redis rule
    pub async fn create_redis_rule(&self, request: CreateRedisRuleRequest) -> Result<RedisRule> {
        self.client.post("/acl/redisRules", &request).await
    }

    /// Update Redis rule
    pub async fn update_redis_rule(
        &self,
        rule_id: u32,
        request: UpdateRedisRuleRequest,
    ) -> Result<RedisRule> {
        self.client
            .put(&format!("/acl/redisRules/{}", rule_id), &request)
            .await
    }

    /// Delete Redis rule
    pub async fn delete_redis_rule(&self, rule_id: u32) -> Result<()> {
        self.client
            .delete(&format!("/acl/redisRules/{}", rule_id))
            .await
    }
}
