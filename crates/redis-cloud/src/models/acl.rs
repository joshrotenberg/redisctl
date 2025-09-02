//! ACL and RBAC models for Redis Cloud

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
    pub email: Option<String>,
    #[builder(default, setter(strip_option))]
    pub password: Option<String>,
    #[builder(default, setter(strip_option))]
    pub roles: Option<Vec<u32>>,
}

/// Request to update an ACL user
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UpdateAclUserRequest {
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
    #[builder(default, setter(strip_option))]
    pub email: Option<String>,
    #[builder(default, setter(strip_option))]
    pub password: Option<String>,
    #[builder(default, setter(strip_option))]
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
    pub description: Option<String>,
    #[builder(default, setter(strip_option))]
    pub redis_rules: Option<Vec<u32>>,
    #[builder(default, setter(strip_option))]
    pub databases: Option<Vec<DatabaseAclBinding>>,
}

/// Request to update an ACL role
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UpdateAclRoleRequest {
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
    #[builder(default, setter(strip_option))]
    pub description: Option<String>,
    #[builder(default, setter(strip_option))]
    pub redis_rules: Option<Vec<u32>>,
    #[builder(default, setter(strip_option))]
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
    pub is_active: bool,
}

/// Request to update a Redis rule
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UpdateRedisRuleRequest {
    #[builder(default, setter(strip_option))]
    pub name: Option<String>,
    #[builder(default, setter(strip_option))]
    pub acl_syntax: Option<String>,
    #[builder(default, setter(strip_option))]
    pub description: Option<String>,
    #[builder(default, setter(strip_option))]
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
