//! User management operations handler

use crate::{Result, client::CloudClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudUser {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub role: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

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
        let v: serde_json::Value = self.client.get("/users").await?;
        if let Some(arr) = v.get("users") {
            serde_json::from_value(arr.clone()).map_err(Into::into)
        } else if v.is_array() {
            serde_json::from_value(v).map_err(Into::into)
        } else {
            Ok(vec![])
        }
    }

    /// Get user by ID
    pub async fn get(&self, user_id: u32) -> Result<CloudUser> {
        self.client.get(&format!("/users/{}", user_id)).await
    }

    /// Create a new user (invite)
    pub async fn create(&self, request: serde_json::Value) -> Result<CloudUser> {
        self.client.post("/users", &request).await
    }

    /// Update user
    pub async fn update(&self, user_id: u32, request: serde_json::Value) -> Result<CloudUser> {
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
