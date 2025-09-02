//! SSO/SAML models for Redis Cloud

use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsoConfig {
    pub enabled: bool,
    pub auto_provision: Option<bool>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UpdateSsoConfigRequest {
    #[builder(default, setter(strip_option))]
    pub enabled: Option<bool>,
    #[builder(default, setter(strip_option))]
    pub auto_provision: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamlConfig {
    pub entity_id: Option<String>,
    pub sso_url: Option<String>,
    pub certificate: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UpdateSamlConfigRequest {
    #[builder(default, setter(into, strip_option))]
    pub entity_id: Option<String>,
    #[builder(default, setter(into, strip_option))]
    pub sso_url: Option<String>,
    #[builder(default, setter(into, strip_option))]
    pub certificate: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamlMetadata {
    pub metadata_xml: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsoTestResponse {
    pub success: bool,
    pub message: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsoUserMapping {
    pub id: u32,
    pub email: Option<String>,
    pub role: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsoGroupMapping {
    pub id: u32,
    pub name: Option<String>,
    pub role: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}
