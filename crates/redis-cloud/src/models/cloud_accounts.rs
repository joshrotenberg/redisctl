//! Cloud Provider Account models

use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudProviderAccount {
    pub id: u32,
    pub provider: String,
    pub name: Option<String>,
    pub account_id: Option<String>,
    pub status: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CreateCloudProviderAccountRequest {
    pub provider: String,
    #[builder(default, setter(into, strip_option))]
    pub name: Option<String>,
    #[builder(default, setter(into, strip_option))]
    pub account_id: Option<String>,
    #[builder(default, setter(strip_option))]
    pub credentials: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UpdateCloudProviderAccountRequest {
    #[builder(default, setter(into, strip_option))]
    pub name: Option<String>,
    #[builder(default, setter(strip_option))]
    pub credentials: Option<Value>,
}
