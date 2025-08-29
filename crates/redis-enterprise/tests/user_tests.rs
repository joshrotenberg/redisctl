//! User endpoint tests for Redis Enterprise

use redis_enterprise::{CreateUserRequest, EnterpriseClient, UpdateUserRequest, UserHandler};
use serde_json::json;
use wiremock::matchers::{basic_auth, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// Test helper functions
fn success_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(body)
}

fn created_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(201).set_body_json(body)
}

fn no_content_response() -> ResponseTemplate {
    ResponseTemplate::new(204)
}

fn test_user() -> serde_json::Value {
    json!({
        "uid": 1,
        "username": "test-user",
        "email": "test@example.com",
        "role": "admin",
        "status": "active"
    })
}

#[tokio::test]
async fn test_user_list() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/users"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            test_user(),
            {
                "uid": 2,
                "username": "user-2",
                "email": "user2@example.com",
                "role": "viewer",
                "status": "active"
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = UserHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let users = result.unwrap();
    assert_eq!(users.len(), 2);
}

#[tokio::test]
async fn test_user_get() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/users/1"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_user()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = UserHandler::new(client);
    let result = handler.get(1).await;

    assert!(result.is_ok());
    let user = result.unwrap();
    assert_eq!(user.uid, 1);
    assert_eq!(user.username, "test-user");
}

#[tokio::test]
async fn test_user_create() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/users"))
        .and(basic_auth("admin", "password"))
        .respond_with(created_response(test_user()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = UserHandler::new(client);
    let user_data = CreateUserRequest::builder()
        .email("test@example.com")
        .password("secret123")
        .role("admin")
        .build();
    let result = handler.create(user_data).await;

    assert!(result.is_ok());
    let user = result.unwrap();
    assert_eq!(user.uid, 1);
    assert_eq!(user.username, "test-user");
}

#[tokio::test]
async fn test_user_update() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/v1/users/1"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_user()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = UserHandler::new(client);
    let user_data = UpdateUserRequest::builder()
        .email("updated@example.com")
        .build();
    let result = handler.update(1, user_data).await;

    assert!(result.is_ok());
    let user = result.unwrap();
    assert_eq!(user.uid, 1);
}

#[tokio::test]
async fn test_user_delete() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/users/1"))
        .and(basic_auth("admin", "password"))
        .respond_with(no_content_response())
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = UserHandler::new(client);
    let result = handler.delete(1).await;

    assert!(result.is_ok());
}
