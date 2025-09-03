//! Redis Cloud API client implementation

use crate::error::{CloudError, Result};
use reqwest::{Client, Response, StatusCode};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::env;

/// Redis Cloud API client
#[derive(Clone, Debug)]
pub struct CloudClient {
    client: Client,
    base_url: String,
    api_key: String,
    api_secret_key: String,
}

/// Builder for CloudClient
#[derive(Default)]
pub struct CloudClientBuilder {
    api_key: Option<String>,
    api_secret_key: Option<String>,
    base_url: Option<String>,
    client: Option<Client>,
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

    /// Set the API secret key
    pub fn api_secret_key(mut self, key: impl Into<String>) -> Self {
        self.api_secret_key = Some(key.into());
        self
    }

    /// Set the base URL
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Set the HTTP client
    pub fn client(mut self, client: Client) -> Self {
        self.client = Some(client);
        self
    }

    /// Build the CloudClient
    pub fn build(self) -> CloudClient {
        let base_url = self
            .base_url
            .unwrap_or_else(|| "https://api.redislabs.com/v1".to_string());

        CloudClient {
            client: self.client.unwrap_or_default(),
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key: self.api_key.expect("API key is required"),
            api_secret_key: self.api_secret_key.expect("API secret key is required"),
        }
    }
}

impl CloudClient {
    /// Create a new CloudClient builder
    pub fn builder() -> CloudClientBuilder {
        CloudClientBuilder::new()
    }

    /// Create a new CloudClient with provided credentials
    pub fn new(api_key: impl Into<String>, api_secret_key: impl Into<String>) -> Self {
        CloudClient {
            client: Client::new(),
            base_url: "https://api.redislabs.com/v1".to_string(),
            api_key: api_key.into(),
            api_secret_key: api_secret_key.into(),
        }
    }

    /// Create a CloudClient from environment variables
    pub fn from_env() -> Result<Self> {
        let api_key = env::var("REDIS_CLOUD_API_KEY")?;
        let api_secret_key = env::var("REDIS_CLOUD_API_SECRET_KEY")?;
        let base_url = env::var("REDIS_CLOUD_URL")
            .unwrap_or_else(|_| "https://api.redislabs.com/v1".to_string());

        Ok(CloudClient {
            client: Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
            api_key,
            api_secret_key,
        })
    }

    /// Make a GET request
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let response = self
            .request(reqwest::Method::GET, path, None::<&()>)
            .await?;
        self.handle_response(response).await
    }

    /// Make a GET request returning raw JSON
    pub async fn get_raw(&self, path: &str) -> Result<Value> {
        let response = self
            .request(reqwest::Method::GET, path, None::<&()>)
            .await?;
        self.handle_response(response).await
    }

    /// Make a POST request
    pub async fn post<T, R>(&self, path: &str, body: &T) -> Result<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let response = self
            .request(reqwest::Method::POST, path, Some(body))
            .await?;
        self.handle_response(response).await
    }

    /// Make a POST request returning raw JSON
    pub async fn post_raw(&self, path: &str, body: &Value) -> Result<Value> {
        let response = self
            .request(reqwest::Method::POST, path, Some(body))
            .await?;
        self.handle_response(response).await
    }

    /// Make a PUT request
    pub async fn put<T, R>(&self, path: &str, body: &T) -> Result<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let response = self.request(reqwest::Method::PUT, path, Some(body)).await?;
        self.handle_response(response).await
    }

    /// Make a PUT request returning raw JSON
    pub async fn put_raw(&self, path: &str, body: &Value) -> Result<Value> {
        let response = self.request(reqwest::Method::PUT, path, Some(body)).await?;
        self.handle_response(response).await
    }

    /// Make a DELETE request
    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let response = self
            .request(reqwest::Method::DELETE, path, None::<&()>)
            .await?;
        self.handle_response(response).await
    }

    /// Make a DELETE request returning raw JSON
    pub async fn delete_raw(&self, path: &str) -> Result<Value> {
        let response = self
            .request(reqwest::Method::DELETE, path, None::<&()>)
            .await?;
        self.handle_response(response).await
    }

    /// Make a PATCH request
    pub async fn patch<T, R>(&self, path: &str, body: &T) -> Result<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let response = self
            .request(reqwest::Method::PATCH, path, Some(body))
            .await?;
        self.handle_response(response).await
    }

    /// Make a PATCH request returning raw JSON
    pub async fn patch_raw(&self, path: &str, body: &Value) -> Result<Value> {
        let response = self
            .request(reqwest::Method::PATCH, path, Some(body))
            .await?;
        self.handle_response(response).await
    }

    /// Internal request method
    async fn request<T: Serialize>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<&T>,
    ) -> Result<Response> {
        let url = format!("{}{}", self.base_url, path);

        let mut request = self
            .client
            .request(method, &url)
            .header("x-api-key", &self.api_key)
            .header("x-api-secret-key", &self.api_secret_key)
            .header("Content-Type", "application/json");

        if let Some(body) = body {
            request = request.json(body);
        }

        Ok(request.send().await?)
    }

    /// Handle the response
    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
        let status = response.status();

        if status.is_success() {
            Ok(response.json().await?)
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            match status {
                StatusCode::UNAUTHORIZED => Err(CloudError::AuthenticationFailed(error_text)),
                StatusCode::NOT_FOUND => Err(CloudError::NotFound(error_text)),
                StatusCode::TOO_MANY_REQUESTS => Err(CloudError::RateLimitExceeded),
                _ => Err(CloudError::ApiError {
                    status: status.as_u16(),
                    message: error_text,
                }),
            }
        }
    }
}
