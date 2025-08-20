//! Tests for the Cloud library

#[cfg(test)]
mod tests {
    use crate::{CloudClient, CloudConfig, CloudError, Result};
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_cloud_config_default() {
        let config = CloudConfig::default();
        assert_eq!(config.base_url, "https://api.redislabs.com/v1");
        assert_eq!(config.timeout, std::time::Duration::from_secs(30));
        assert!(config.api_key.is_empty());
        assert!(config.api_secret.is_empty());
    }

    #[tokio::test]
    async fn test_cloud_client_creation() {
        let config = CloudConfig {
            api_key: "test_key".to_string(),
            api_secret: "test_secret".to_string(),
            base_url: "https://example.com".to_string(),
            timeout: std::time::Duration::from_secs(10),
        };

        let result = CloudClient::new(config.clone());

        // Client should be created successfully
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_cloud_client_get_request() {
        // Start a background HTTP server on a random local port
        let mock_server = MockServer::start().await;

        // Arrange the behaviour of the MockServer adding a Mock
        Mock::given(method("GET"))
            .and(path("/test"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"status": "ok"})),
            )
            .mount(&mock_server)
            .await;

        let config = CloudConfig {
            api_key: "test_key".to_string(),
            api_secret: "test_secret".to_string(),
            base_url: mock_server.uri(),
            timeout: std::time::Duration::from_secs(10),
        };

        let client = CloudClient::new(config).unwrap();
        let result: Result<serde_json::Value> = client.get("/test").await;

        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value["status"], "ok");
    }

    #[tokio::test]
    async fn test_cloud_client_post_request() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/test"))
            .respond_with(
                ResponseTemplate::new(201).set_body_json(serde_json::json!({"created": true})),
            )
            .mount(&mock_server)
            .await;

        let config = CloudConfig {
            api_key: "test_key".to_string(),
            api_secret: "test_secret".to_string(),
            base_url: mock_server.uri(),
            timeout: std::time::Duration::from_secs(10),
        };

        let client = CloudClient::new(config).unwrap();
        let test_data = serde_json::json!({"name": "test"});
        let result: Result<serde_json::Value> = client.post("/test", &test_data).await;

        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value["created"], true);
    }

    #[tokio::test]
    async fn test_cloud_client_error_handling() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/error"))
            .respond_with(
                ResponseTemplate::new(404).set_body_json(serde_json::json!({"error": "Not found"})),
            )
            .mount(&mock_server)
            .await;

        let config = CloudConfig {
            api_key: "test_key".to_string(),
            api_secret: "test_secret".to_string(),
            base_url: mock_server.uri(),
            timeout: std::time::Duration::from_secs(10),
        };

        let client = CloudClient::new(config).unwrap();
        let result: Result<serde_json::Value> = client.get("/error").await;

        assert!(result.is_err());
        match result.unwrap_err() {
            CloudError::ApiError { code, .. } => {
                assert_eq!(code, 404);
            }
            _ => panic!("Expected ApiError"),
        }
    }

    #[test]
    fn test_cloud_error_display() {
        let err = CloudError::AuthenticationFailed;
        assert_eq!(err.to_string(), "Authentication failed");

        let err = CloudError::ApiError {
            code: 400,
            message: "Bad request".to_string(),
        };
        assert_eq!(err.to_string(), "API error (400): Bad request");
    }
}
