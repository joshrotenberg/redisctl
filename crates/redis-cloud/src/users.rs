//! User management for Redis Cloud accounts
//!
//! ## Overview
//! - Manage account users
//! - Configure user roles and permissions
//! - Control user access

use crate::client::CloudClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

/// Account user information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alerts: Option<bool>,

    #[serde(rename = "createdTimestamp", skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,

    #[serde(rename = "lastLoginTimestamp", skip_serializing_if = "Option::is_none")]
    pub last_login_timestamp: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Create user request
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CreateUserRequest {
    #[builder(setter(into))]
    pub name: String,

    #[builder(setter(into))]
    pub email: String,

    #[builder(setter(into))]
    pub role: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub alerts: Option<bool>,
}

/// Update user request
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UpdateUserRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub role: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub alerts: Option<bool>,
}

/// User handler
pub struct UserHandler {
    client: CloudClient,
}

impl UserHandler {
    pub fn new(client: CloudClient) -> Self {
        UserHandler { client }
    }

    /// List all users
    pub async fn list(&self) -> Result<Vec<User>> {
        self.client.get("/users").await
    }

    /// Get a specific user
    pub async fn get(&self, user_id: u32) -> Result<User> {
        self.client.get(&format!("/users/{}", user_id)).await
    }

    /// Create a new user
    pub async fn create(&self, request: CreateUserRequest) -> Result<User> {
        self.client.post("/users", &request).await
    }

    /// Update a user
    pub async fn update(&self, user_id: u32, request: UpdateUserRequest) -> Result<User> {
        self.client
            .put(&format!("/users/{}", user_id), &request)
            .await
    }

    /// Delete a user
    pub async fn delete(&self, user_id: u32) -> Result<Value> {
        self.client.delete(&format!("/users/{}", user_id)).await
    }
}
