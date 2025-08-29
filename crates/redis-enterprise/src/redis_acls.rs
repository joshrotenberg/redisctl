//! Redis Access Control List (ACL) management for Redis Enterprise

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

/// Redis ACL information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisAcl {
    pub uid: u32,
    pub name: String,
    pub acl: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Create or update Redis ACL request
#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
pub struct CreateRedisAclRequest {
    #[builder(setter(into))]
    pub name: String,
    #[builder(setter(into))]
    pub acl: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub description: Option<String>,
}

/// Redis ACL handler for managing ACLs
pub struct RedisAclHandler {
    client: RestClient,
}

/// Alias for backwards compatibility and intuitive plural naming
pub type RedisAclsHandler = RedisAclHandler;

impl RedisAclHandler {
    pub fn new(client: RestClient) -> Self {
        RedisAclHandler { client }
    }

    /// List all Redis ACLs
    pub async fn list(&self) -> Result<Vec<RedisAcl>> {
        self.client.get("/v1/redis_acls").await
    }

    /// Get specific Redis ACL
    pub async fn get(&self, uid: u32) -> Result<RedisAcl> {
        self.client.get(&format!("/v1/redis_acls/{}", uid)).await
    }

    /// Create a new Redis ACL
    pub async fn create(&self, request: CreateRedisAclRequest) -> Result<RedisAcl> {
        self.client.post("/v1/redis_acls", &request).await
    }

    /// Update an existing Redis ACL
    pub async fn update(&self, uid: u32, request: CreateRedisAclRequest) -> Result<RedisAcl> {
        self.client
            .put(&format!("/v1/redis_acls/{}", uid), &request)
            .await
    }

    /// Delete a Redis ACL
    pub async fn delete(&self, uid: u32) -> Result<()> {
        self.client.delete(&format!("/v1/redis_acls/{}", uid)).await
    }
}
