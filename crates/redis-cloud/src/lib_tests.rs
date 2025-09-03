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
}
