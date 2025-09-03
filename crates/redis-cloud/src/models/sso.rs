//! SSO/SAML models for Redis Cloud

use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsoConfig {
    pub enabled: bool,
    #[serde(rename = "autoProvisioning")]
    pub auto_provision: Option<bool>,
    pub provider: Option<String>,
    #[serde(rename = "entityId")]
    pub entity_id: Option<String>,
    #[serde(rename = "ssoUrl")]
    pub sso_url: Option<String>,
    #[serde(rename = "signOnUrl")]
    pub sign_on_url: Option<String>,
    #[serde(rename = "logoutUrl")]
    pub logout_url: Option<String>,
    #[serde(rename = "defaultRole")]
    pub default_role: Option<String>,
    #[serde(rename = "certificateFingerprint")]
    pub certificate_fingerprint: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
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
    #[serde(rename = "entityId")]
    pub entity_id: Option<String>,
    #[serde(rename = "ssoUrl")]
    pub sso_url: Option<String>,
    pub certificate: Option<String>,
    #[serde(rename = "signRequest")]
    pub sign_request: Option<bool>,
    #[serde(rename = "encryptAssertion")]
    pub encrypt_assertion: Option<bool>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
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
    #[serde(rename = "xml")]
    pub xml: Option<String>,
    #[serde(rename = "downloadUrl")]
    pub download_url: Option<String>,
    #[serde(rename = "entityId")]
    pub entity_id: Option<String>,
    #[serde(rename = "acsUrl")]
    pub acs_url: Option<String>,
    #[serde(rename = "sloUrl")]
    pub slo_url: Option<String>,
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
