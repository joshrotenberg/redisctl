//! User management models for Redis Cloud

use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

/// Cloud user information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudUser {
    pub id: u32,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub role: String,
    pub status: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub last_login: Option<String>,
    pub two_factor_enabled: Option<bool>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Request to create (invite) a new user
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CreateUserRequest {
    pub email: String,
    pub role: String,
    #[builder(default, setter(strip_option))]
    pub first_name: Option<String>,
    #[builder(default, setter(strip_option))]
    pub last_name: Option<String>,
    #[builder(default, setter(strip_option))]
    pub send_invitation_email: Option<bool>,
}

/// Request to update user information
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UpdateUserRequest {
    #[builder(default, setter(strip_option))]
    pub first_name: Option<String>,
    #[builder(default, setter(strip_option))]
    pub last_name: Option<String>,
    #[builder(default, setter(strip_option))]
    pub role: Option<String>,
    #[builder(default, setter(strip_option))]
    pub status: Option<String>,
}

/// User role options
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum UserRole {
    Owner,
    Admin,
    Editor,
    Viewer,
    BillingAdmin,
    CloudAccountsManager,
}

/// User status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UserStatus {
    Active,
    Pending,
    Suspended,
    Deleted,
}
