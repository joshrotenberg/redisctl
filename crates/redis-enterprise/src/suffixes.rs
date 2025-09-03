//! Database name suffix management
//!
//! ## Overview
//! - Configure database suffixes
//! - Manage suffix rules
//! - Query suffix usage

use crate::client::RestClient;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

/// DNS suffix configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suffix {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_internal_addr: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_external_addr: Option<bool>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Create suffix request
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CreateSuffixRequest {
    #[builder(setter(into))]
    pub name: String,
    #[builder(setter(into))]
    pub dns_suffix: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub use_internal_addr: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub use_external_addr: Option<bool>,
}

/// Suffixes handler
pub struct SuffixesHandler {
    client: RestClient,
}

impl SuffixesHandler {
    pub fn new(client: RestClient) -> Self {
        SuffixesHandler { client }
    }

    /// List all DNS suffixes
    pub async fn list(&self) -> Result<Vec<Suffix>> {
        self.client.get("/v1/suffixes").await
    }

    /// Get specific suffix
    pub async fn get(&self, name: &str) -> Result<Suffix> {
        self.client.get(&format!("/v1/suffix/{}", name)).await
    }

    /// Create a new suffix
    pub async fn create(&self, request: CreateSuffixRequest) -> Result<Suffix> {
        self.client.post("/v1/suffix", &request).await
    }

    /// Update a suffix
    pub async fn update(&self, name: &str, request: CreateSuffixRequest) -> Result<Suffix> {
        self.client
            .put(&format!("/v1/suffix/{}", name), &request)
            .await
    }

    /// Delete a suffix
    pub async fn delete(&self, name: &str) -> Result<()> {
        self.client.delete(&format!("/v1/suffix/{}", name)).await
    }

    /// Get cluster DNS suffixes configuration
    pub async fn cluster_suffixes(&self) -> Result<Vec<Suffix>> {
        self.client.get("/v1/cluster/suffixes").await
    }
}
