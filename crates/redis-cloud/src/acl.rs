//! ACL and RBAC operations handler

use crate::models::acl::*;
use crate::{Result, client::CloudClient};
use serde_json::Value;

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
