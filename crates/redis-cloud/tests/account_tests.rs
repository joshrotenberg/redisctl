//! Account endpoint tests for Redis Cloud

use redis_cloud::{CloudAccountHandler, CloudClient, CloudConfig};
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// Test helper functions
fn success_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(body)
}

fn error_response(status: u16, body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(status).set_body_json(body)
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
async fn test_account_info() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "account": {
                "id": 12345,
                "name": "Test Account",
                "createdTimestamp": "2023-01-01T00:00:00Z",
                "updatedTimestamp": "2023-01-02T00:00:00Z",
                "key": {
                    "name": "default",
                    "accountId": 12345,
                    "createdTimestamp": "2023-01-01T00:00:00Z",
                    "updatedTimestamp": "2023-01-01T00:00:00Z"
                }
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAccountHandler::new(client);
    let result = handler.info().await;

    assert!(result.is_ok());
    let account = result.unwrap();
    assert_eq!(account.id, 12345);
    assert_eq!(account.name, "Test Account");
    assert!(account.key.is_some());
    let key = account.key.unwrap();
    assert_eq!(key.name, "default");
    assert_eq!(key.account_id, Some(12345));
}

#[tokio::test]
async fn test_account_info_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            401,
            json!({
                "error": {
                    "type": "UNAUTHORIZED",
                    "status": 401,
                    "description": "Invalid API key"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAccountHandler::new(client);
    let result = handler.info().await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_account_owner() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users/owners"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "users": [{
                "id": 1,
                "name": "Account Owner",
                "email": "owner@example.com",
                "role": "owner",
                "status": "active"
            }]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAccountHandler::new(client);
    let result = handler.owner().await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let users = response["users"].as_array().unwrap();
    assert_eq!(users.len(), 1);
    assert_eq!(users[0]["email"], "owner@example.com");
    assert_eq!(users[0]["role"], "owner");
}

#[tokio::test]
async fn test_account_users() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "users": [
                {
                    "id": 1,
                    "name": "Account Owner",
                    "email": "owner@example.com",
                    "role": "owner",
                    "status": "active"
                },
                {
                    "id": 2,
                    "name": "User One",
                    "email": "user1@example.com",
                    "role": "admin",
                    "status": "active"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAccountHandler::new(client);
    let result = handler.users().await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let users = response["users"].as_array().unwrap();
    assert_eq!(users.len(), 2);
    assert_eq!(users[0]["role"], "owner");
    assert_eq!(users[1]["role"], "admin");
}

#[tokio::test]
async fn test_get_account_alias() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "account": {
                "id": 12345,
                "name": "Test Account"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAccountHandler::new(client);
    let result = handler.get_account().await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["account"]["id"], 12345);
}

#[tokio::test]
async fn test_get_users_alias() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "users": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAccountHandler::new(client);
    let result = handler.get_users().await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_owner_alias() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users/owners"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "users": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAccountHandler::new(client);
    let result = handler.get_owner().await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_payment_methods() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/payment-methods"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "paymentMethods": [
                {
                    "id": "pm_12345",
                    "type": "card",
                    "cardType": "visa",
                    "last4": "4242",
                    "expiryMonth": 12,
                    "expiryYear": 2025,
                    "isDefault": true
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAccountHandler::new(client);
    let result = handler.get_payment_methods().await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let payment_methods = response["paymentMethods"].as_array().unwrap();
    assert_eq!(payment_methods.len(), 1);
    assert_eq!(payment_methods[0]["type"], "card");
    assert_eq!(payment_methods[0]["last4"], "4242");
}

#[tokio::test]
async fn test_payment_methods_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/payment-methods"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            403,
            json!({
                "error": {
                    "type": "FORBIDDEN",
                    "status": 403,
                    "description": "Insufficient permissions to access payment methods"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAccountHandler::new(client);
    let result = handler.get_payment_methods().await;

    assert!(result.is_err());
}
