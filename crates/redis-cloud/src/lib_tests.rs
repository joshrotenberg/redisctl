//! Tests for the Cloud library

#[cfg(test)]
mod tests {
    use crate::{CloudClient, CloudError, Result};
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_cloud_client_creation() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/test"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"status": "ok"})),
            )
            .mount(&mock_server)
            .await;

        let client = CloudClient::builder()
            .api_key("test_key")
            .api_secret("test_secret")
            .base_url(mock_server.uri())
            .build()
            .unwrap();
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

        let client = CloudClient::builder()
            .api_key("test_key")
            .api_secret("test_secret")
            .base_url(mock_server.uri())
            .build()
            .unwrap();
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

        let client = CloudClient::builder()
            .api_key("test_key")
            .api_secret("test_secret")
            .base_url(mock_server.uri())
            .build()
            .unwrap();
        let result: Result<serde_json::Value> = client.get("/error").await;

        assert!(result.is_err());
        match result.unwrap_err() {
            CloudError::NotFound { .. } => {
                // Expected 404 Not Found error
            }
            err => panic!("Expected NotFound error, got: {:?}", err),
        }
    }

    #[test]
    fn test_cloud_error_display() {
        let err = CloudError::AuthenticationFailed {
            message: "Invalid credentials".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "Authentication failed (401): Invalid credentials"
        );

        let err = CloudError::ApiError {
            code: 400,
            message: "Bad request".to_string(),
        };
        assert_eq!(err.to_string(), "API error (400): Bad request");
    }

    #[test]
    fn test_cloud_error_is_retryable() {
        // Retryable errors
        assert!(
            CloudError::RateLimited {
                message: "Too many requests".to_string()
            }
            .is_retryable()
        );
        assert!(
            CloudError::ServiceUnavailable {
                message: "Service down".to_string()
            }
            .is_retryable()
        );
        assert!(CloudError::Request("Connection reset".to_string()).is_retryable());
        assert!(CloudError::ConnectionError("DNS failed".to_string()).is_retryable());

        // Non-retryable errors
        assert!(
            !CloudError::BadRequest {
                message: "Invalid input".to_string()
            }
            .is_retryable()
        );
        assert!(
            !CloudError::AuthenticationFailed {
                message: "Bad creds".to_string()
            }
            .is_retryable()
        );
        assert!(
            !CloudError::Forbidden {
                message: "No access".to_string()
            }
            .is_retryable()
        );
        assert!(
            !CloudError::NotFound {
                message: "Not found".to_string()
            }
            .is_retryable()
        );
        assert!(
            !CloudError::ApiError {
                code: 400,
                message: "Error".to_string()
            }
            .is_retryable()
        );
    }

    #[tokio::test]
    async fn test_url_normalization() {
        // Test various combinations of base URLs and paths to ensure no double slashes
        let test_cases = vec![
            (
                "https://api.redislabs.com/v1",
                "/subscriptions",
                "https://api.redislabs.com/v1/subscriptions",
            ),
            (
                "https://api.redislabs.com/v1/",
                "/subscriptions",
                "https://api.redislabs.com/v1/subscriptions",
            ),
            (
                "https://api.redislabs.com/v1",
                "subscriptions",
                "https://api.redislabs.com/v1/subscriptions",
            ),
            (
                "https://api.redislabs.com/v1/",
                "subscriptions",
                "https://api.redislabs.com/v1/subscriptions",
            ),
            (
                "https://api.redislabs.com/v1",
                "/subscriptions/123/databases",
                "https://api.redislabs.com/v1/subscriptions/123/databases",
            ),
            (
                "https://api.redislabs.com/v1/",
                "/subscriptions/123/databases",
                "https://api.redislabs.com/v1/subscriptions/123/databases",
            ),
        ];

        for (base_url, test_path, _expected) in test_cases {
            let mock_server = MockServer::start().await;

            // Mock will fail if the URL has double slashes
            Mock::given(method("GET"))
                .and(path(test_path.trim_start_matches('/')))
                .respond_with(
                    ResponseTemplate::new(200).set_body_json(serde_json::json!({"ok": true})),
                )
                .mount(&mock_server)
                .await;

            let client = CloudClient::builder()
                .base_url(base_url.replace("https://api.redislabs.com/v1", &mock_server.uri()))
                .api_key("test_key")
                .api_secret("test_secret")
                .build()
                .unwrap();

            let result: Result<serde_json::Value> = client.get(test_path).await;
            assert!(
                result.is_ok(),
                "Failed for base_url: {}, path: {}",
                base_url,
                test_path
            );
        }
    }
}
