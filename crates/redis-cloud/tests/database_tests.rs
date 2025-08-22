//! Database endpoint tests for Redis Cloud

use redis_cloud::{CloudClient, CloudConfig, CloudDatabaseHandler};
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// Test helper functions
fn success_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(body)
}

fn create_test_client(base_url: String) -> CloudClient {
    let config = CloudConfig {
        api_key: "test-api-key".to_string(),
        api_secret: "test-secret-key".to_string(),
        base_url,
        timeout: std::time::Duration::from_secs(30),
    };
    CloudClient::new(config).unwrap()
}

#[tokio::test]
async fn test_database_get() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/12345/databases/67890"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "db_id": 67890,
            "name": "test-db",
            "protocol": "redis",
            "provider": "AWS",
            "region": "us-east-1",
            "status": "active",
            "memory_limit_in_gb": 1.0,
            "data_persistence": "none",
            "replication": false
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudDatabaseHandler::new(client);
    let result = handler.get(12345, 67890).await;

    assert!(result.is_ok());
    let database = result.unwrap();
    assert_eq!(database.db_id, 67890);
    assert_eq!(database.name, "test-db");
}

#[tokio::test]
async fn test_database_delete() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/subscriptions/12345/databases/67890"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "taskId": "delete-db-123"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudDatabaseHandler::new(client);
    let result = handler.delete(12345, 67890).await;

    assert!(result.is_ok());
}
