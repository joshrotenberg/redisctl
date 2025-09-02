//! Users endpoint tests for Redis Cloud

use redis_cloud::{CloudClient, CloudUserHandler};
use serde_json::json;
use wiremock::matchers::{body_json, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// Test helper functions
fn success_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(body)
}

fn created_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(201).set_body_json(body)
}

fn error_response(status: u16, body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(status).set_body_json(body)
}

fn no_content_response() -> ResponseTemplate {
    ResponseTemplate::new(204)
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
async fn test_users_list() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "users": [
                {
                    "id": 1,
                    "name": "John Doe",
                    "email": "john.doe@example.com",
                    "role": "owner",
                    "status": "active",
                    "createdAt": "2023-01-01T00:00:00Z",
                    "lastLogin": "2023-01-15T10:30:00Z"
                },
                {
                    "id": 2,
                    "name": "Jane Smith",
                    "email": "jane.smith@example.com",
                    "role": "admin",
                    "status": "active",
                    "createdAt": "2023-01-02T00:00:00Z",
                    "lastLogin": "2023-01-14T14:20:00Z"
                },
                {
                    "id": 3,
                    "name": "Bob Wilson",
                    "email": "bob.wilson@example.com",
                    "role": "user",
                    "status": "invited",
                    "createdAt": "2023-01-10T00:00:00Z",
                    "lastLogin": null
                }
            ],
            "totalCount": 3
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudUserHandler::new(client);
    let result = handler.list_raw().await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let users = response["users"].as_array().unwrap();
    assert_eq!(users.len(), 3);

    assert_eq!(users[0]["id"], 1);
    assert_eq!(users[0]["name"], "John Doe");
    assert_eq!(users[0]["email"], "john.doe@example.com");
    assert_eq!(users[0]["role"], "owner");
    assert_eq!(users[0]["status"], "active");

    assert_eq!(users[1]["role"], "admin");
    assert_eq!(users[2]["role"], "user");
    assert_eq!(users[2]["status"], "invited");
    assert!(users[2]["lastLogin"].is_null());

    assert_eq!(response["totalCount"], 3);
}

#[tokio::test]
async fn test_users_list_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "users": [],
            "totalCount": 0
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudUserHandler::new(client);
    let result = handler.list_raw().await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let users = response["users"].as_array().unwrap();
    assert_eq!(users.len(), 0);
    assert_eq!(response["totalCount"], 0);
}

#[tokio::test]
async fn test_user_get() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users/1"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "id": 1,
            "name": "John Doe",
            "email": "john.doe@example.com",
            "role": "owner",
            "status": "active",
            "createdAt": "2023-01-01T00:00:00Z",
            "updatedAt": "2023-01-15T00:00:00Z",
            "lastLogin": "2023-01-15T10:30:00Z",
            "permissions": ["read", "write", "admin", "billing"],
            "mfaEnabled": true
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudUserHandler::new(client);
    let result = handler.get_raw(1).await;

    assert!(result.is_ok());
    let user = result.unwrap();
    assert_eq!(user["id"], 1);
    assert_eq!(user["name"], "John Doe");
    assert_eq!(user["email"], "john.doe@example.com");
    assert_eq!(user["role"], "owner");
    assert_eq!(user["status"], "active");
    assert_eq!(user["mfaEnabled"], true);

    let permissions = user["permissions"].as_array().unwrap();
    assert!(permissions.contains(&json!("read")));
    assert!(permissions.contains(&json!("write")));
    assert!(permissions.contains(&json!("admin")));
    assert!(permissions.contains(&json!("billing")));
}

#[tokio::test]
async fn test_user_create_invite() {
    let mock_server = MockServer::start().await;
    let request_body = json!({
        "name": "New User",
        "email": "newuser@example.com",
        "role": "user",
        "permissions": ["read", "write"]
    });

    Mock::given(method("POST"))
        .and(path("/users"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(body_json(&request_body))
        .respond_with(created_response(json!({
            "id": 4,
            "name": "New User",
            "email": "newuser@example.com",
            "role": "user",
            "status": "invited",
            "createdAt": "2023-01-16T00:00:00Z",
            "permissions": ["read", "write"],
            "inviteToken": "invite-token-123",
            "inviteExpiresAt": "2023-01-23T00:00:00Z"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudUserHandler::new(client);
    let result = handler.create_raw(request_body).await;

    assert!(result.is_ok());
    let user = result.unwrap();
    assert_eq!(user["id"], 4);
    assert_eq!(user["name"], "New User");
    assert_eq!(user["email"], "newuser@example.com");
    assert_eq!(user["role"], "user");
    assert_eq!(user["status"], "invited");
    assert!(user["inviteToken"].is_string());
    assert!(user["inviteExpiresAt"].is_string());
}

#[tokio::test]
async fn test_user_create_admin() {
    let mock_server = MockServer::start().await;
    let request_body = json!({
        "name": "Admin User",
        "email": "admin@example.com",
        "role": "admin",
        "permissions": ["read", "write", "admin"]
    });

    Mock::given(method("POST"))
        .and(path("/users"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(body_json(&request_body))
        .respond_with(created_response(json!({
            "id": 5,
            "name": "Admin User",
            "email": "admin@example.com",
            "role": "admin",
            "status": "invited",
            "createdAt": "2023-01-16T00:00:00Z",
            "permissions": ["read", "write", "admin"]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudUserHandler::new(client);
    let result = handler.create_raw(request_body).await;

    assert!(result.is_ok());
    let user = result.unwrap();
    assert_eq!(user["role"], "admin");
    let permissions = user["permissions"].as_array().unwrap();
    assert!(permissions.contains(&json!("admin")));
}

#[tokio::test]
async fn test_user_update() {
    let mock_server = MockServer::start().await;
    let request_body = json!({
        "name": "John Doe Updated",
        "role": "admin",
        "permissions": ["read", "write", "admin"]
    });

    Mock::given(method("PUT"))
        .and(path("/users/1"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(body_json(&request_body))
        .respond_with(success_response(json!({
            "id": 1,
            "name": "John Doe Updated",
            "email": "john.doe@example.com",
            "role": "admin",
            "status": "active",
            "createdAt": "2023-01-01T00:00:00Z",
            "updatedAt": "2023-01-16T00:00:00Z",
            "permissions": ["read", "write", "admin"]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudUserHandler::new(client);
    let result = handler.update_raw(1, request_body).await;

    assert!(result.is_ok());
    let user = result.unwrap();
    assert_eq!(user["id"], 1);
    assert_eq!(user["name"], "John Doe Updated");
    assert_eq!(user["role"], "admin");

    let permissions = user["permissions"].as_array().unwrap();
    assert!(permissions.contains(&json!("admin")));
}

#[tokio::test]
async fn test_user_delete() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/users/2"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(no_content_response())
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudUserHandler::new(client);
    let result = handler.delete(2).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["message"], "User 2 deleted");
}

// Error handling tests
#[tokio::test]
async fn test_users_list_unauthorized() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users"))
        .and(header("x-api-key", "invalid-key"))
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

    let client = CloudClient::builder()
        .api_key("invalid-key")
        .api_secret("test-secret-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();
    let handler = CloudUserHandler::new(client);
    let result = handler.list_raw().await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_user_get_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users/999"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            404,
            json!({
                "error": {
                    "type": "NOT_FOUND",
                    "status": 404,
                    "description": "User not found"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudUserHandler::new(client);
    let result = handler.get_raw(999).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_user_create_invalid_email() {
    let mock_server = MockServer::start().await;
    let request_body = json!({
        "name": "Invalid User",
        "email": "invalid-email",
        "role": "user"
    });

    Mock::given(method("POST"))
        .and(path("/users"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(body_json(&request_body))
        .respond_with(error_response(
            400,
            json!({
                "error": {
                    "type": "VALIDATION_ERROR",
                    "status": 400,
                    "description": "Invalid email format",
                    "details": {
                        "email": ["Must be a valid email address"]
                    }
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudUserHandler::new(client);
    let result = handler.create_raw(request_body).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_user_create_duplicate_email() {
    let mock_server = MockServer::start().await;
    let request_body = json!({
        "name": "Duplicate User",
        "email": "existing@example.com",
        "role": "user"
    });

    Mock::given(method("POST"))
        .and(path("/users"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(body_json(&request_body))
        .respond_with(error_response(
            409,
            json!({
                "error": {
                    "type": "CONFLICT",
                    "status": 409,
                    "description": "User with this email already exists"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudUserHandler::new(client);
    let result = handler.create_raw(request_body).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_user_update_insufficient_permissions() {
    let mock_server = MockServer::start().await;
    let request_body = json!({
        "role": "owner"
    });

    Mock::given(method("PUT"))
        .and(path("/users/1"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(body_json(&request_body))
        .respond_with(error_response(
            403,
            json!({
                "error": {
                    "type": "FORBIDDEN",
                    "status": 403,
                    "description": "Insufficient permissions to assign owner role"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudUserHandler::new(client);
    let result = handler.update_raw(1, request_body).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_user_delete_last_owner() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/users/1"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            409,
            json!({
                "error": {
                    "type": "CONFLICT",
                    "status": 409,
                    "description": "Cannot delete the last owner of the account"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudUserHandler::new(client);
    let result = handler.delete(1).await;

    assert!(result.is_err());
}
