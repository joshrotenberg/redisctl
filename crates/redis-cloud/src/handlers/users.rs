//! User management operations handler

use crate::models::users::*;
use crate::{Result, client::CloudClient};
use serde_json::Value;

/// Handler for Cloud user operations
pub struct CloudUserHandler {
    client: CloudClient,
}

impl CloudUserHandler {
    pub fn new(client: CloudClient) -> Self {
        CloudUserHandler { client }
    }

    /// List all users
    pub async fn list(&self) -> Result<Vec<CloudUser>> {
        self.client.get("/users").await
    }

    /// List all users - raw version
    pub async fn list_raw(&self) -> Result<Value> {
        self.client.get("/users").await
    }

    /// Get user by ID
    pub async fn get(&self, user_id: u32) -> Result<CloudUser> {
        self.client.get(&format!("/users/{}", user_id)).await
    }

    /// Get user by ID - raw version
    pub async fn get_raw(&self, user_id: u32) -> Result<Value> {
        self.client.get(&format!("/users/{}", user_id)).await
    }

    /// Create a new user (invite)
    pub async fn create(&self, request: CreateUserRequest) -> Result<CloudUser> {
        self.client.post("/users", &request).await
    }

    /// Create a new user (invite) - raw version
    pub async fn create_raw(&self, request: Value) -> Result<Value> {
        self.client.post("/users", &request).await
    }

    /// Update user
    pub async fn update(&self, user_id: u32, request: UpdateUserRequest) -> Result<CloudUser> {
        self.client
            .put(&format!("/users/{}", user_id), &request)
            .await
    }

    /// Update user - raw version
    pub async fn update_raw(&self, user_id: u32, request: Value) -> Result<Value> {
        self.client
            .put(&format!("/users/{}", user_id), &request)
            .await
    }

    /// Delete user
    pub async fn delete(&self, user_id: u32) -> Result<Value> {
        self.client.delete(&format!("/users/{}", user_id)).await?;
        Ok(serde_json::json!({"message": format!("User {} deleted", user_id)}))
    }
}
