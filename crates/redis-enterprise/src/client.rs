//! REST API client implementation

use crate::error::{RestError, Result};
use reqwest::{Client, Response};
use serde::{de::DeserializeOwned, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, trace};

/// Enterprise API configuration (deprecated - use builder pattern)
#[derive(Debug, Clone)]
pub struct EnterpriseConfig {
    pub base_url: String,
    pub username: String,
    pub password: String,
    pub timeout: Duration,
    pub insecure: bool,
}

impl Default for EnterpriseConfig {
    fn default() -> Self {
        EnterpriseConfig {
            base_url: "https://localhost:9443".to_string(),
            username: String::new(),
            password: String::new(),
            timeout: Duration::from_secs(30),
            insecure: true,
        }
    }
}

// Alias for backwards compatibility
pub type RestConfig = EnterpriseConfig;

/// Builder for EnterpriseClient
#[derive(Debug, Clone)]
pub struct EnterpriseClientBuilder {
    base_url: String,
    username: Option<String>,
    password: Option<String>,
    timeout: Duration,
    insecure: bool,
}

impl Default for EnterpriseClientBuilder {
    fn default() -> Self {
        Self {
            base_url: "https://localhost:9443".to_string(),
            username: None,
            password: None,
            timeout: Duration::from_secs(30),
            insecure: false,
        }
    }
}

impl EnterpriseClientBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the base URL
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Set the username
    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    /// Set the password
    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    /// Set the timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Allow insecure TLS connections (self-signed certificates)
    pub fn insecure(mut self, insecure: bool) -> Self {
        self.insecure = insecure;
        self
    }

    /// Build the client
    pub fn build(self) -> Result<EnterpriseClient> {
        let username = self.username.unwrap_or_default();
        let password = self.password.unwrap_or_default();

        let client_builder = Client::builder()
            .timeout(self.timeout)
            .danger_accept_invalid_certs(self.insecure);

        let client = client_builder
            .build()
            .map_err(|e| RestError::ConnectionError(e.to_string()))?;

        Ok(EnterpriseClient {
            base_url: self.base_url,
            username,
            password,
            timeout: self.timeout,
            client: Arc::new(client),
        })
    }
}

/// REST API client for Redis Enterprise
#[derive(Clone)]
pub struct EnterpriseClient {
    base_url: String,
    username: String,
    password: String,
    timeout: Duration,
    client: Arc<Client>,
}

// Alias for backwards compatibility
pub type RestClient = EnterpriseClient;

impl EnterpriseClient {
    /// Create a new builder for the client
    pub fn builder() -> EnterpriseClientBuilder {
        EnterpriseClientBuilder::new()
    }

    /// Create a new REST API client (deprecated - use builder())
    pub fn new(config: EnterpriseConfig) -> Result<Self> {
        Self::builder()
            .base_url(config.base_url)
            .username(config.username)
            .password(config.password)
            .timeout(config.timeout)
            .insecure(config.insecure)
            .build()
    }

    /// Make a GET request
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        debug!("GET {}", url);

        let response = self
            .client
            .get(&url)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await
            .map_err(|e| self.map_reqwest_error(e, &url))?;

        trace!("Response status: {}", response.status());
        self.handle_response(response).await
    }

    /// Make a GET request for text content
    pub async fn get_text(&self, path: &str) -> Result<String> {
        let url = format!("{}{}", self.base_url, path);
        debug!("GET {} (text)", url);

        let response = self
            .client
            .get(&url)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await
            .map_err(|e| self.map_reqwest_error(e, &url))?;

        trace!("Response status: {}", response.status());

        if response.status().is_success() {
            let text = response
                .text()
                .await
                .map_err(crate::error::RestError::RequestFailed)?;
            Ok(text)
        } else {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(crate::error::RestError::ApiError {
                code: status.as_u16(),
                message: error_text,
            })
        }
    }

    /// Make a POST request
    pub async fn post<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        debug!("POST {}", url);
        trace!("Request body: {:?}", serde_json::to_value(body).ok());

        let response = self
            .client
            .post(&url)
            .basic_auth(&self.username, Some(&self.password))
            .json(body)
            .send()
            .await
            .map_err(|e| self.map_reqwest_error(e, &url))?;

        trace!("Response status: {}", response.status());
        self.handle_response(response).await
    }

    /// Make a PUT request
    pub async fn put<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        debug!("PUT {}", url);
        trace!("Request body: {:?}", serde_json::to_value(body).ok());

        let response = self
            .client
            .put(&url)
            .basic_auth(&self.username, Some(&self.password))
            .json(body)
            .send()
            .await
            .map_err(|e| self.map_reqwest_error(e, &url))?;

        trace!("Response status: {}", response.status());
        self.handle_response(response).await
    }

    /// Make a DELETE request
    pub async fn delete(&self, path: &str) -> Result<()> {
        let url = format!("{}{}", self.base_url, path);
        debug!("DELETE {}", url);

        let response = self
            .client
            .delete(&url)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await
            .map_err(|e| self.map_reqwest_error(e, &url))?;

        trace!("Response status: {}", response.status());
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

    /// POST request for actions that return no content
    pub async fn post_action<B: Serialize>(&self, path: &str, body: &B) -> Result<()> {
        let url = format!("{}{}", self.base_url, path);
        debug!("POST {}", url);
        trace!("Request body: {:?}", serde_json::to_value(body).ok());

        let response = self
            .client
            .post(&url)
            .basic_auth(&self.username, Some(&self.password))
            .json(body)
            .send()
            .await
            .map_err(|e| self.map_reqwest_error(e, &url))?;

        trace!("Response status: {}", response.status());
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

    /// Get a reference to self for handler construction
    pub fn rest_client(&self) -> Self {
        self.clone()
    }

    /// POST request for bootstrap - handles empty response
    pub async fn post_bootstrap<B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.base_url, path);

        let response = self
            .client
            .post(&url)
            .basic_auth(&self.username, Some(&self.password))
            .json(body)
            .send()
            .await
            .map_err(|e| self.map_reqwest_error(e, &url))?;

        let status = response.status();
        if status.is_success() {
            // Try to parse JSON, but if empty/invalid, return success
            let text = response.text().await.unwrap_or_default();
            if text.is_empty() || text.trim().is_empty() {
                Ok(serde_json::json!({"status": "success"}))
            } else {
                Ok(serde_json::from_str(&text)
                    .unwrap_or_else(|_| serde_json::json!({"status": "success", "response": text})))
            }
        } else {
            let text = response.text().await.unwrap_or_default();
            Err(RestError::ApiError {
                code: status.as_u16(),
                message: text,
            })
        }
    }

    /// Execute raw PATCH request with JSON body
    pub async fn patch_raw(
        &self,
        path: &str,
        body: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .client
            .patch(&url)
            .basic_auth(&self.username, Some(&self.password))
            .json(&body)
            .send()
            .await
            .map_err(|e| self.map_reqwest_error(e, &url))?;

        if response.status().is_success() {
            response
                .json()
                .await
                .map_err(|e| RestError::ParseError(e.to_string()))
        } else {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            Err(RestError::ApiError {
                code: status.as_u16(),
                message: text,
            })
        }
    }

    /// Execute raw DELETE request returning any response body
    pub async fn delete_raw(&self, path: &str) -> Result<serde_json::Value> {
        let url = format!("{}{}", self.base_url, path);
        let response = self
            .client
            .delete(&url)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await
            .map_err(|e| self.map_reqwest_error(e, &url))?;

        if response.status().is_success() {
            if response.content_length() == Some(0) {
                Ok(serde_json::json!({"status": "deleted"}))
            } else {
                response
                    .json()
                    .await
                    .map_err(|e| RestError::ParseError(e.to_string()))
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

    /// Map reqwest errors to more specific error messages
    fn map_reqwest_error(&self, error: reqwest::Error, url: &str) -> RestError {
        if error.is_connect() {
            RestError::ConnectionError(format!(
                "Failed to connect to {}: Connection refused or host unreachable. Check if the Redis Enterprise server is running and accessible.",
                url
            ))
        } else if error.is_timeout() {
            RestError::ConnectionError(format!(
                "Request to {} timed out after {:?}. Check network connectivity or increase timeout.",
                url, self.timeout
            ))
        } else if error.is_decode() {
            RestError::ConnectionError(format!(
                "Failed to decode JSON response from {}: {}. Server may have returned invalid JSON or HTML error page.",
                url, error
            ))
        } else if let Some(status) = error.status() {
            RestError::ApiError {
                code: status.as_u16(),
                message: format!("HTTP {} from {}: {}", status.as_u16(), url, error),
            }
        } else if error.is_request() {
            RestError::ConnectionError(format!(
                "Request to {} failed: {}. Check URL format and network settings.",
                url, error
            ))
        } else {
            RestError::RequestFailed(error)
        }
    }

    /// Handle HTTP response
    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
        if response.status().is_success() {
            response.json::<T>().await.map_err(Into::into)
        } else if response.status() == 401 {
            Err(RestError::AuthenticationFailed)
        } else {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            Err(RestError::ApiError {
                code: status.as_u16(),
                message: text,
            })
        }
    }
}
