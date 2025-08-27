//! Redis Cloud API client core implementation
//!
//! This module contains the core HTTP client for interacting with the Redis Cloud REST API.
//! It provides authentication handling, request/response processing, and error management.
//!
//! The client is designed around a builder pattern for flexible configuration and supports
//! both typed and untyped API interactions.

use crate::{CloudError as RestError, Result};
use reqwest::Client;
use serde::Serialize;
use std::sync::Arc;

/// Builder for constructing a CloudClient with custom configuration
///
/// Provides a fluent interface for configuring API credentials, base URL, timeouts,
/// and other client settings before creating the final CloudClient instance.
///
/// # Examples
///
/// ```rust,no_run
/// use redis_cloud::CloudClient;
///
/// // Basic configuration
/// let client = CloudClient::builder()
///     .api_key("your-api-key")
///     .api_secret("your-api-secret")
///     .build()?;
///
/// // Advanced configuration
/// let client = CloudClient::builder()
///     .api_key("your-api-key")
///     .api_secret("your-api-secret")
///     .base_url("https://api.redislabs.com/v1".to_string())
///     .timeout(std::time::Duration::from_secs(120))
///     .build()?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Clone)]
pub struct CloudClientBuilder {
    api_key: Option<String>,
    api_secret: Option<String>,
    base_url: String,
    timeout: std::time::Duration,
}

impl Default for CloudClientBuilder {
    fn default() -> Self {
        Self {
            api_key: None,
            api_secret: None,
            base_url: "https://api.redislabs.com/v1".to_string(),
            timeout: std::time::Duration::from_secs(30),
        }
    }
}

impl CloudClientBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the API key
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self
    }

    /// Set the API secret
    pub fn api_secret(mut self, secret: impl Into<String>) -> Self {
        self.api_secret = Some(secret.into());
        self
    }

    /// Set the base URL
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Set the timeout
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Build the client
    pub fn build(self) -> Result<CloudClient> {
        let api_key = self
            .api_key
            .ok_or_else(|| RestError::ConnectionError("API key is required".to_string()))?;
        let api_secret = self
            .api_secret
            .ok_or_else(|| RestError::ConnectionError("API secret is required".to_string()))?;

        let client = Client::builder()
            .timeout(self.timeout)
            .build()
            .map_err(|e| RestError::ConnectionError(e.to_string()))?;

        Ok(CloudClient {
            api_key,
            api_secret,
            base_url: self.base_url,
            timeout: self.timeout,
            client: Arc::new(client),
        })
    }
}

/// Redis Cloud API client
#[derive(Clone)]
pub struct CloudClient {
    pub(crate) api_key: String,
    pub(crate) api_secret: String,
    pub(crate) base_url: String,
    #[allow(dead_code)]
    pub(crate) timeout: std::time::Duration,
    pub(crate) client: Arc<Client>,
}

impl CloudClient {
    /// Create a new builder for the client
    pub fn builder() -> CloudClientBuilder {
        CloudClientBuilder::new()
    }

    /// Make a GET request with API key authentication
    pub async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);

        // Redis Cloud API uses these headers for authentication
        let response = self
            .client
            .get(&url)
            .header("x-api-key", &self.api_key)
            .header("x-api-secret-key", &self.api_secret)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Make a POST request
    pub async fn post<B: Serialize, T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);

        // Same backwards header naming as GET
        let response = self
            .client
            .post(&url)
            .header("x-api-key", &self.api_key)
            .header("x-api-secret-key", &self.api_secret)
            .json(body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Make a PUT request
    pub async fn put<B: Serialize, T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);

        // Same backwards header naming as GET
        let response = self
            .client
            .put(&url)
            .header("x-api-key", &self.api_key)
            .header("x-api-secret-key", &self.api_secret)
            .json(body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Make a DELETE request
    pub async fn delete(&self, path: &str) -> Result<()> {
        let url = format!("{}{}", self.base_url, path);

        // Same backwards header naming as GET
        let response = self
            .client
            .delete(&url)
            .header("x-api-key", &self.api_key)
            .header("x-api-secret-key", &self.api_secret)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            Err(RestError::ApiError {
                code: status.as_u16(),
                message: text,
            })
        }
    }

    /// Execute raw GET request returning JSON Value
    pub async fn get_raw(&self, path: &str) -> Result<serde_json::Value> {
        self.get(path).await
    }

    /// Execute raw POST request with JSON body
    pub async fn post_raw(&self, path: &str, body: serde_json::Value) -> Result<serde_json::Value> {
        self.post(path, &body).await
    }

    /// Execute raw PUT request with JSON body
    pub async fn put_raw(&self, path: &str, body: serde_json::Value) -> Result<serde_json::Value> {
        self.put(path, &body).await
    }

    /// Execute raw PATCH request with JSON body
    pub async fn patch_raw(
        &self,
        path: &str,
        body: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.base_url, path);

        // Use backwards header names for compatibility
        let response = self
            .client
            .patch(&url)
            .header("x-api-key", &self.api_key)
            .header("x-api-secret-key", &self.api_secret)
            .json(&body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Execute raw DELETE request returning any response body
    pub async fn delete_raw(&self, path: &str) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.base_url, path);

        // Use backwards header names for compatibility
        let response = self
            .client
            .delete(&url)
            .header("x-api-key", &self.api_key)
            .header("x-api-secret-key", &self.api_secret)
            .send()
            .await?;

        if response.status().is_success() {
            if response.content_length() == Some(0) {
                Ok(serde_json::json!({"status": "deleted"}))
            } else {
                response.json().await.map_err(Into::into)
            }
        } else {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            Err(RestError::ApiError {
                code: status.as_u16(),
                message: text,
            })
        }
    }

    /// Handle HTTP response
    async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T> {
        let status = response.status();

        if status.is_success() {
            // Try to get the response text first for debugging
            let text = response.text().await.map_err(|e| {
                RestError::ConnectionError(format!("Failed to read response: {}", e))
            })?;

            // Try to parse as JSON
            serde_json::from_str::<T>(&text).map_err(|e| {
                // If parsing fails, include the actual response for debugging
                RestError::JsonError(e)
            })
        } else if status == 401 {
            // Get the error message from the response
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "No error message".to_string());
            Err(RestError::ApiError {
                code: 401,
                message: format!("Authentication failed: {}", text),
            })
        } else {
            let text = response.text().await.unwrap_or_default();
            Err(RestError::ApiError {
                code: status.as_u16(),
                message: text,
            })
        }
    }
}
