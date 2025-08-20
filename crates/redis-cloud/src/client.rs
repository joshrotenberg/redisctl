//! Redis Cloud API client core implementation

use crate::{CloudError as RestError, Result};
use reqwest::Client;
use serde::Serialize;
use std::sync::Arc;

/// Redis Cloud API configuration
#[derive(Debug, Clone)]
pub struct CloudConfig {
    pub api_key: String,
    pub api_secret: String,
    pub base_url: String,
    pub timeout: std::time::Duration,
}

impl Default for CloudConfig {
    fn default() -> Self {
        CloudConfig {
            api_key: String::new(),
            api_secret: String::new(),
            base_url: "https://api.redislabs.com/v1".to_string(),
            timeout: std::time::Duration::from_secs(30),
        }
    }
}

/// Redis Cloud API client
#[derive(Clone)]
pub struct CloudClient {
    pub(crate) config: CloudConfig,
    pub(crate) client: Arc<Client>,
}

impl CloudClient {
    /// Create a new Cloud API client
    pub fn new(config: CloudConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(config.timeout)
            .build()
            .map_err(|e| RestError::ConnectionError(e.to_string()))?;

        Ok(CloudClient {
            config,
            client: Arc::new(client),
        })
    }

    /// Make a GET request with API key authentication
    pub async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}{}", self.config.base_url, path);

        // Redis Cloud API uses these headers for authentication
        let response = self
            .client
            .get(&url)
            .header("x-api-key", &self.config.api_key)
            .header("x-api-secret-key", &self.config.api_secret)
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
        let url = format!("{}{}", self.config.base_url, path);

        // Same backwards header naming as GET
        let response = self
            .client
            .post(&url)
            .header("x-api-key", &self.config.api_key)
            .header("x-api-secret-key", &self.config.api_secret)
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
        let url = format!("{}{}", self.config.base_url, path);

        // Same backwards header naming as GET
        let response = self
            .client
            .put(&url)
            .header("x-api-key", &self.config.api_key)
            .header("x-api-secret-key", &self.config.api_secret)
            .json(body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Make a DELETE request
    pub async fn delete(&self, path: &str) -> Result<()> {
        let url = format!("{}{}", self.config.base_url, path);

        // Same backwards header naming as GET
        let response = self
            .client
            .delete(&url)
            .header("x-api-key", &self.config.api_key)
            .header("x-api-secret-key", &self.config.api_secret)
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
        let url = format!("{}{}", self.config.base_url, path);

        // Use backwards header names for compatibility
        let response = self
            .client
            .patch(&url)
            .header("x-api-key", &self.config.api_key)
            .header("x-api-secret-key", &self.config.api_secret)
            .json(&body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Execute raw DELETE request returning any response body
    pub async fn delete_raw(&self, path: &str) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.config.base_url, path);

        // Use backwards header names for compatibility
        let response = self
            .client
            .delete(&url)
            .header("x-api-key", &self.config.api_key)
            .header("x-api-secret-key", &self.config.api_secret)
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
