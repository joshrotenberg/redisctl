//! Subscription endpoint tests for Redis Cloud

use redis_cloud::{CloudClient, CloudSubscriptionHandler};
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// Test helper functions
fn success_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(body)
}

fn create_test_client(base_url: String) -> CloudClient {
    CloudClient::builder()
        .api_key("test-api-key")
        .api_secret("test-secret-key")
        .base_url(base_url)
        .build()
        .unwrap()
}

#[tokio::test]
#[ignore = "Endpoint needs verification against actual API"]
async fn test_subscription_list() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "subscriptions": [
                {
                    "id": 12345,
                    "name": "test-subscription",
                    "status": "active"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudSubscriptionHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let subscriptions = result.unwrap();
    assert!(!subscriptions.is_empty());
}

#[tokio::test]
#[ignore = "Endpoint needs verification against actual API"]
async fn test_subscription_get() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/12345"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "id": 12345,
            "name": "test-subscription",
            "status": "active"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudSubscriptionHandler::new(client);
    let result = handler.get(12345).await;

    assert!(result.is_ok());
    let subscription = result.unwrap();
    assert_eq!(subscription.id, 12345);
}

#[tokio::test]
#[ignore = "Endpoint needs verification against actual API"]
async fn test_subscription_delete() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/subscriptions/12345"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "taskId": "delete-123"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudSubscriptionHandler::new(client);
    let result = handler.delete(12345).await;

    assert!(result.is_ok());
}
