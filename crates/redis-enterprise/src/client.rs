//! REST API client implementation

use crate::error::{RestError, Result};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest::{Client, Response};
use serde::{Serialize, de::DeserializeOwned};
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, trace};

/// Default user agent for the Redis Enterprise client
const DEFAULT_USER_AGENT: &str = concat!("redis-enterprise/", env!("CARGO_PKG_VERSION"));

// Legacy alias for backwards compatibility during migration
pub type RestConfig = EnterpriseClientBuilder;

/// Builder for EnterpriseClient
#[derive(Debug, Clone)]
pub struct EnterpriseClientBuilder {
    base_url: String,
    username: Option<String>,
    password: Option<String>,
    timeout: Duration,
    insecure: bool,
    user_agent: String,
}

impl Default for EnterpriseClientBuilder {
    fn default() -> Self {
        Self {
            base_url: "https://localhost:9443".to_string(),
            username: None,
            password: None,
            timeout: Duration::from_secs(30),
            insecure: false,
            user_agent: DEFAULT_USER_AGENT.to_string(),
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

    /// Set the user agent string for HTTP requests
    ///
    /// The default user agent is `redis-enterprise/{version}`.
    /// This can be overridden to identify specific clients, for example:
    /// `redisctl/1.2.3` or `my-app/1.0.0`.
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    /// Build the client
    pub fn build(self) -> Result<EnterpriseClient> {
        let username = self.username.unwrap_or_default();
        let password = self.password.unwrap_or_default();

        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            USER_AGENT,
            HeaderValue::from_str(&self.user_agent)
                .map_err(|e| RestError::ConnectionError(format!("Invalid user agent: {}", e)))?,
        );

        let client_builder = Client::builder()
            .timeout(self.timeout)
            .danger_accept_invalid_certs(self.insecure)
            .default_headers(default_headers);

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

    /// Get a reference to the underlying client (for use with handlers)
    pub fn client(&self) -> Arc<Client> {
        self.client.clone()
    }

    /// Normalize URL path concatenation to avoid double slashes
    fn normalize_url(&self, path: &str) -> String {
        let base = self.base_url.trim_end_matches('/');
        let path = path.trim_start_matches('/');
        format!("{}/{}", base, path)
    }

    /// Create a client from environment variables
    ///
    /// Reads configuration from:
    /// - `REDIS_ENTERPRISE_URL`: Base URL for the cluster (default: "https://localhost:9443")
    /// - `REDIS_ENTERPRISE_USER`: Username for authentication (default: "admin@redis.local")
    /// - `REDIS_ENTERPRISE_PASSWORD`: Password for authentication (required)
    /// - `REDIS_ENTERPRISE_INSECURE`: Set to "true" to skip SSL verification (default: "false")
    pub fn from_env() -> Result<Self> {
        use std::env;

        let base_url = env::var("REDIS_ENTERPRISE_URL")
            .unwrap_or_else(|_| "https://localhost:9443".to_string());
        let username =
            env::var("REDIS_ENTERPRISE_USER").unwrap_or_else(|_| "admin@redis.local".to_string());
        let password =
            env::var("REDIS_ENTERPRISE_PASSWORD").map_err(|_| RestError::AuthenticationFailed)?;
        let insecure = env::var("REDIS_ENTERPRISE_INSECURE")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false);

        Self::builder()
            .base_url(base_url)
            .username(username)
            .password(password)
            .insecure(insecure)
            .build()
    }

    /// Make a GET request
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = self.normalize_url(path);
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
        let url = self.normalize_url(path);
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
            let text = response.text().await?;
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

    /// Make a GET request for binary content (e.g., tar.gz files)
    pub async fn get_binary(&self, path: &str) -> Result<Vec<u8>> {
        let url = self.normalize_url(path);
        debug!("GET {} (binary)", url);

        let response = self
            .client
            .get(&url)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await
            .map_err(|e| self.map_reqwest_error(e, &url))?;

        trace!("Response status: {}", response.status());
        trace!(
            "Response content-type: {:?}",
            response.headers().get("content-type")
        );

        if response.status().is_success() {
            let bytes = response.bytes().await?;
            Ok(bytes.to_vec())
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
        let url = self.normalize_url(path);
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
        let url = self.normalize_url(path);
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
        let url = self.normalize_url(path);
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
        let url = self.normalize_url(path);
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

    /// POST request with multipart/form-data for file uploads
    pub async fn post_multipart<T: DeserializeOwned>(
        &self,
        path: &str,
        file_data: Vec<u8>,
        field_name: &str,
        file_name: &str,
    ) -> Result<T> {
        let url = self.normalize_url(path);
        debug!("POST {} (multipart)", url);

        let part = reqwest::multipart::Part::bytes(file_data).file_name(file_name.to_string());

        let form = reqwest::multipart::Form::new().part(field_name.to_string(), part);

        let response = self
            .client
            .post(&url)
            .basic_auth(&self.username, Some(&self.password))
            .multipart(form)
            .send()
            .await
            .map_err(|e| self.map_reqwest_error(e, &url))?;

        trace!("Response status: {}", response.status());
        self.handle_response(response).await
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
        let url = self.normalize_url(path);

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
        let url = self.normalize_url(path);
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
        let url = self.normalize_url(path);
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
            RestError::RequestFailed(error.to_string())
        }
    }

    /// Handle HTTP response
    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
        if response.status().is_success() {
            // Get the response bytes for better error reporting
            let bytes = response.bytes().await.map_err(Into::<RestError>::into)?;

            // Use serde_path_to_error for better deserialization error messages
            let deserializer = &mut serde_json::Deserializer::from_slice(&bytes);
            serde_path_to_error::deserialize(deserializer).map_err(|err| {
                let path = err.path().to_string();
                RestError::ParseError(format!(
                    "Failed to deserialize field '{}': {}",
                    path,
                    err.inner()
                ))
            })
        } else {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();

            match status.as_u16() {
                401 => Err(RestError::Unauthorized),
                404 => Err(RestError::NotFound),
                500..=599 => Err(RestError::ServerError(text)),
                _ => Err(RestError::ApiError {
                    code: status.as_u16(),
                    message: text,
                }),
            }
        }
    }

    /// Execute a Redis command on a specific database (internal use only)
    /// This uses the /v1/bdbs/{uid}/command endpoint which may not be publicly documented
    pub async fn execute_command(&self, db_uid: u32, command: &str) -> Result<serde_json::Value> {
        let url = self.normalize_url(&format!("/v1/bdbs/{}/command", db_uid));
        let body = serde_json::json!({
            "command": command
        });

        debug!("Executing command on database {}: {}", db_uid, command);

        let response = self
            .client
            .post(&url)
            .basic_auth(&self.username, Some(&self.password))
            .json(&body)
            .send()
            .await
            .map_err(|e| self.map_reqwest_error(e, &url))?;

        self.handle_response(response).await
    }
}

/// Tower Service integration for EnterpriseClient
///
/// This module provides Tower Service implementations for EnterpriseClient, enabling
/// middleware composition with patterns like circuit breakers, retry, and rate limiting.
///
/// # Feature Flag
///
/// This module is only available when the `tower-integration` feature is enabled.
///
/// # Examples
///
/// ```rust,ignore
/// use redis_enterprise::EnterpriseClient;
/// use redis_enterprise::tower_support::ApiRequest;
/// use tower::ServiceExt;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = EnterpriseClient::builder()
///     .base_url("https://localhost:9443")
///     .username("admin")
///     .password("password")
///     .insecure(true)
///     .build()?;
///
/// // Convert to a Tower service
/// let mut service = client.into_service();
///
/// // Use the service
/// let response = service.oneshot(ApiRequest::get("/v1/cluster")).await?;
/// println!("Status: {}", response.status);
/// # Ok(())
/// # }
/// ```
#[cfg(feature = "tower-integration")]
pub mod tower_support {
    use super::*;
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use tower::Service;

    /// HTTP method for API requests
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Method {
        /// GET request
        Get,
        /// POST request
        Post,
        /// PUT request
        Put,
        /// PATCH request
        Patch,
        /// DELETE request
        Delete,
    }

    /// Tower-compatible request type for Redis Enterprise API
    ///
    /// This wraps the essential components of an API request in a format
    /// suitable for Tower middleware composition.
    #[derive(Debug, Clone)]
    pub struct ApiRequest {
        /// HTTP method
        pub method: Method,
        /// API endpoint path (e.g., "/v1/cluster")
        pub path: String,
        /// Optional JSON body for POST/PUT/PATCH requests
        pub body: Option<serde_json::Value>,
    }

    impl ApiRequest {
        /// Create a GET request
        pub fn get(path: impl Into<String>) -> Self {
            Self {
                method: Method::Get,
                path: path.into(),
                body: None,
            }
        }

        /// Create a POST request with a JSON body
        pub fn post(path: impl Into<String>, body: serde_json::Value) -> Self {
            Self {
                method: Method::Post,
                path: path.into(),
                body: Some(body),
            }
        }

        /// Create a PUT request with a JSON body
        pub fn put(path: impl Into<String>, body: serde_json::Value) -> Self {
            Self {
                method: Method::Put,
                path: path.into(),
                body: Some(body),
            }
        }

        /// Create a PATCH request with a JSON body
        pub fn patch(path: impl Into<String>, body: serde_json::Value) -> Self {
            Self {
                method: Method::Patch,
                path: path.into(),
                body: Some(body),
            }
        }

        /// Create a DELETE request
        pub fn delete(path: impl Into<String>) -> Self {
            Self {
                method: Method::Delete,
                path: path.into(),
                body: None,
            }
        }
    }

    /// Tower-compatible response type
    ///
    /// Contains the HTTP status code and response body as JSON.
    #[derive(Debug, Clone)]
    pub struct ApiResponse {
        /// HTTP status code
        pub status: u16,
        /// Response body as JSON
        pub body: serde_json::Value,
    }

    impl EnterpriseClient {
        /// Convert this client into a Tower service
        ///
        /// This consumes the client and returns it wrapped in a Tower service
        /// implementation, enabling middleware composition.
        ///
        /// # Examples
        ///
        /// ```rust,ignore
        /// use redis_enterprise::EnterpriseClient;
        /// use tower::ServiceExt;
        /// use redis_enterprise::tower_support::ApiRequest;
        ///
        /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
        /// let client = EnterpriseClient::builder()
        ///     .base_url("https://localhost:9443")
        ///     .username("admin")
        ///     .password("password")
        ///     .insecure(true)
        ///     .build()?;
        ///
        /// let mut service = client.into_service();
        /// let response = service.oneshot(ApiRequest::get("/v1/cluster")).await?;
        /// # Ok(())
        /// # }
        /// ```
        pub fn into_service(self) -> Self {
            self
        }
    }

    impl Service<ApiRequest> for EnterpriseClient {
        type Response = ApiResponse;
        type Error = RestError;
        type Future = Pin<Box<dyn Future<Output = Result<Self::Response>> + Send>>;

        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            // EnterpriseClient is always ready since it uses an internal connection pool
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, req: ApiRequest) -> Self::Future {
            let client = self.clone();
            Box::pin(async move {
                let response: serde_json::Value = match req.method {
                    Method::Get => client.get(&req.path).await?,
                    Method::Post => {
                        let body = req.body.ok_or_else(|| {
                            RestError::ValidationError("POST request requires a body".to_string())
                        })?;
                        client.post(&req.path, &body).await?
                    }
                    Method::Put => {
                        let body = req.body.ok_or_else(|| {
                            RestError::ValidationError("PUT request requires a body".to_string())
                        })?;
                        client.put(&req.path, &body).await?
                    }
                    Method::Patch => {
                        let body = req.body.ok_or_else(|| {
                            RestError::ValidationError("PATCH request requires a body".to_string())
                        })?;
                        client.patch_raw(&req.path, body).await?
                    }
                    Method::Delete => {
                        client.delete(&req.path).await?;
                        serde_json::json!({"status": "deleted"})
                    }
                };

                Ok(ApiResponse {
                    status: 200,
                    body: response,
                })
            })
        }
    }
}
