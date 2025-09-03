//! LDAP integration and role mapping
//!
//! ## Overview
//! - Configure LDAP mappings
//! - Map LDAP groups to roles
//! - Query mapping status

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

/// LDAP mapping information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LdapMapping {
    pub uid: u32,
    pub name: String,
    pub dn: String,
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_uids: Option<Vec<u32>>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Create or update LDAP mapping request
#[derive(Debug, Serialize, Deserialize, TypedBuilder)]
pub struct CreateLdapMappingRequest {
    #[builder(setter(into))]
    pub name: String,
    #[builder(setter(into))]
    pub dn: String,
    #[builder(setter(into))]
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(into, strip_option))]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub role_uids: Option<Vec<u32>>,
}

/// LDAP configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LdapConfig {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<LdapServer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_refresh_interval: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_query_suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_query_suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_dn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_pass: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

/// LDAP server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LdapServer {
    pub host: String,
    pub port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_tls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starttls: Option<bool>,
}

/// LDAP mapping handler
pub struct LdapMappingHandler {
    client: RestClient,
}

impl LdapMappingHandler {
    pub fn new(client: RestClient) -> Self {
        LdapMappingHandler { client }
    }

    /// List all LDAP mappings
    pub async fn list(&self) -> Result<Vec<LdapMapping>> {
        self.client.get("/v1/ldap_mappings").await
    }

    /// Get specific LDAP mapping
    pub async fn get(&self, uid: u32) -> Result<LdapMapping> {
        self.client.get(&format!("/v1/ldap_mappings/{}", uid)).await
    }

    /// Create a new LDAP mapping
    pub async fn create(&self, request: CreateLdapMappingRequest) -> Result<LdapMapping> {
        self.client.post("/v1/ldap_mappings", &request).await
    }

    /// Update an existing LDAP mapping
    pub async fn update(&self, uid: u32, request: CreateLdapMappingRequest) -> Result<LdapMapping> {
        self.client
            .put(&format!("/v1/ldap_mappings/{}", uid), &request)
            .await
    }

    /// Delete an LDAP mapping
    pub async fn delete(&self, uid: u32) -> Result<()> {
        self.client
            .delete(&format!("/v1/ldap_mappings/{}", uid))
            .await
    }

    /// Get LDAP configuration
    pub async fn get_config(&self) -> Result<LdapConfig> {
        self.client.get("/v1/cluster/ldap").await
    }

    /// Update LDAP configuration
    pub async fn update_config(&self, config: LdapConfig) -> Result<LdapConfig> {
        self.client.put("/v1/cluster/ldap", &config).await
    }
}
