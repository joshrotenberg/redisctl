//! JSON Schema definitions for Redis Enterprise API

use crate::client::RestClient;
use crate::error::Result;
use serde_json::Value;

/// JSON Schema handler for API schema definitions
pub struct JsonSchemaHandler {
    client: RestClient,
}

impl JsonSchemaHandler {
    pub fn new(client: RestClient) -> Self {
        JsonSchemaHandler { client }
    }

    /// Get all available schemas
    pub async fn list(&self) -> Result<Vec<String>> {
        self.client.get("/v1/jsonschema").await
    }

    /// Get schema for a specific object type
    pub async fn get(&self, schema_name: &str) -> Result<Value> {
        self.client
            .get(&format!("/v1/jsonschema/{}", schema_name))
            .await
    }

    /// Get schema for database object
    pub async fn database_schema(&self) -> Result<Value> {
        self.client.get("/v1/jsonschema/bdb").await
    }

    /// Get schema for cluster object
    pub async fn cluster_schema(&self) -> Result<Value> {
        self.client.get("/v1/jsonschema/cluster").await
    }

    /// Get schema for node object
    pub async fn node_schema(&self) -> Result<Value> {
        self.client.get("/v1/jsonschema/node").await
    }

    /// Get schema for user object
    pub async fn user_schema(&self) -> Result<Value> {
        self.client.get("/v1/jsonschema/user").await
    }

    /// Get schema for CRDB object
    pub async fn crdb_schema(&self) -> Result<Value> {
        self.client.get("/v1/jsonschema/crdb").await
    }

    /// Validate an object against its schema
    pub async fn validate(&self, schema_name: &str, object: &Value) -> Result<Value> {
        self.client
            .post(&format!("/v1/jsonschema/{}/validate", schema_name), object)
            .await
    }
}

