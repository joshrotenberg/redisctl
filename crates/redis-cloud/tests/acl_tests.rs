//! ACL endpoint tests for Redis Cloud

use redis_cloud::{CloudAclHandler, CloudClient};
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

// Database ACL tests
#[tokio::test]
async fn test_database_acl_list() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/12345/databases/67890/acl"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "acls": [
                {
                    "id": 1,
                    "rule": "~* +@all",
                    "name": "full-access",
                    "description": "Full access rule"
                },
                {
                    "id": 2,
                    "rule": "~key:* +@read",
                    "name": "read-only",
                    "description": "Read only access"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAclHandler::new(client);
    let result = handler.list(12345, 67890).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let acls = response["acls"].as_array().unwrap();
    assert_eq!(acls.len(), 2);
    assert_eq!(acls[0]["name"], "full-access");
    assert_eq!(acls[1]["name"], "read-only");
}

#[tokio::test]
async fn test_database_acl_get() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/12345/databases/67890/acl/1"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "id": 1,
            "rule": "~* +@all",
            "name": "full-access",
            "description": "Full access rule",
            "createdAt": "2023-01-01T00:00:00Z"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAclHandler::new(client);
    let result = handler.get(12345, 67890, 1).await;

    assert!(result.is_ok());
    let acl = result.unwrap();
    assert_eq!(acl["id"], 1);
    assert_eq!(acl["name"], "full-access");
    assert_eq!(acl["rule"], "~* +@all");
}

#[tokio::test]
async fn test_database_acl_create() {
    let mock_server = MockServer::start().await;
    let request_body = json!({
        "rule": "~key:* +@read",
        "name": "test-rule",
        "description": "Test ACL rule"
    });

    Mock::given(method("POST"))
        .and(path("/subscriptions/12345/databases/67890/acl"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(body_json(&request_body))
        .respond_with(created_response(json!({
            "id": 3,
            "rule": "~key:* +@read",
            "name": "test-rule",
            "description": "Test ACL rule",
            "createdAt": "2023-01-01T00:00:00Z"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAclHandler::new(client);
    let result = handler.create(12345, 67890, request_body).await;

    assert!(result.is_ok());
    let acl = result.unwrap();
    assert_eq!(acl["id"], 3);
    assert_eq!(acl["name"], "test-rule");
}

#[tokio::test]
async fn test_database_acl_update() {
    let mock_server = MockServer::start().await;
    let request_body = json!({
        "rule": "~key:* +@read +@write",
        "name": "updated-rule",
        "description": "Updated ACL rule"
    });

    Mock::given(method("PUT"))
        .and(path("/subscriptions/12345/databases/67890/acl/1"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(body_json(&request_body))
        .respond_with(success_response(json!({
            "id": 1,
            "rule": "~key:* +@read +@write",
            "name": "updated-rule",
            "description": "Updated ACL rule",
            "updatedAt": "2023-01-02T00:00:00Z"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAclHandler::new(client);
    let result = handler.update(12345, 67890, 1, request_body).await;

    assert!(result.is_ok());
    let acl = result.unwrap();
    assert_eq!(acl["name"], "updated-rule");
}

#[tokio::test]
async fn test_database_acl_delete() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/subscriptions/12345/databases/67890/acl/1"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(no_content_response())
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAclHandler::new(client);
    let result = handler.delete(12345, 67890, 1).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["message"], "ACL rule 1 deleted");
}

// ACL Users tests
#[tokio::test]
async fn test_acl_users_list() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/acl/users"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "users": [
                {
                    "id": 1,
                    "name": "test-user",
                    "password": "***",
                    "role": "admin",
                    "rules": ["~* +@all"]
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAclHandler::new(client);
    let result = handler.list_users().await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let users = response["users"].as_array().unwrap();
    assert_eq!(users.len(), 1);
    assert_eq!(users[0]["name"], "test-user");
}

#[tokio::test]
async fn test_acl_user_get() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/acl/users/1"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "id": 1,
            "name": "test-user",
            "password": "***",
            "role": "admin",
            "rules": ["~* +@all"],
            "createdAt": "2023-01-01T00:00:00Z"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAclHandler::new(client);
    let result = handler.get_user(1).await;

    assert!(result.is_ok());
    let user = result.unwrap();
    assert_eq!(user["id"], 1);
    assert_eq!(user["name"], "test-user");
}

#[tokio::test]
async fn test_acl_user_create() {
    let mock_server = MockServer::start().await;
    let request_body = json!({
        "name": "new-user",
        "password": "secure-password",
        "role": "user",
        "rules": ["~key:* +@read"]
    });

    Mock::given(method("POST"))
        .and(path("/acl/users"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(body_json(&request_body))
        .respond_with(created_response(json!({
            "id": 2,
            "name": "new-user",
            "password": "***",
            "role": "user",
            "rules": ["~key:* +@read"],
            "createdAt": "2023-01-01T00:00:00Z"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAclHandler::new(client);
    let result = handler.create_user(request_body).await;

    assert!(result.is_ok());
    let user = result.unwrap();
    assert_eq!(user["id"], 2);
    assert_eq!(user["name"], "new-user");
}

#[tokio::test]
async fn test_acl_user_delete() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/acl/users/1"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(no_content_response())
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAclHandler::new(client);
    let result = handler.delete_user(1).await;

    assert!(result.is_ok());
}

// ACL Roles tests
#[tokio::test]
async fn test_acl_roles_list() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/acl/roles"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "roles": [
                {
                    "id": 1,
                    "name": "admin",
                    "description": "Administrator role",
                    "permissions": ["read", "write", "admin"]
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAclHandler::new(client);
    let result = handler.list_roles().await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let roles = response["roles"].as_array().unwrap();
    assert_eq!(roles.len(), 1);
    assert_eq!(roles[0]["name"], "admin");
}

#[tokio::test]
async fn test_acl_role_create() {
    let mock_server = MockServer::start().await;
    let request_body = json!({
        "name": "custom-role",
        "description": "Custom role for testing",
        "permissions": ["read", "write"]
    });

    Mock::given(method("POST"))
        .and(path("/acl/roles"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(body_json(&request_body))
        .respond_with(created_response(json!({
            "id": 2,
            "name": "custom-role",
            "description": "Custom role for testing",
            "permissions": ["read", "write"],
            "createdAt": "2023-01-01T00:00:00Z"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAclHandler::new(client);
    let result = handler.create_role(request_body).await;

    assert!(result.is_ok());
    let role = result.unwrap();
    assert_eq!(role["id"], 2);
    assert_eq!(role["name"], "custom-role");
}

// Redis Rules tests
#[tokio::test]
async fn test_redis_rules_list() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/acl/redisRules"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "rules": [
                {
                    "id": 1,
                    "name": "read-only-rule",
                    "rule": "~key:* +@read",
                    "description": "Read only access to keys"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAclHandler::new(client);
    let result = handler.list_redis_rules().await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let rules = response["rules"].as_array().unwrap();
    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0]["name"], "read-only-rule");
}

#[tokio::test]
async fn test_redis_rule_create() {
    let mock_server = MockServer::start().await;
    let request_body = json!({
        "name": "write-rule",
        "rule": "~key:* +@write",
        "description": "Write access to keys"
    });

    Mock::given(method("POST"))
        .and(path("/acl/redisRules"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(body_json(&request_body))
        .respond_with(created_response(json!({
            "id": 2,
            "name": "write-rule",
            "rule": "~key:* +@write",
            "description": "Write access to keys",
            "createdAt": "2023-01-01T00:00:00Z"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAclHandler::new(client);
    let result = handler.create_redis_rule(request_body).await;

    assert!(result.is_ok());
    let rule = result.unwrap();
    assert_eq!(rule["id"], 2);
    assert_eq!(rule["name"], "write-rule");
}

// Error handling tests
#[tokio::test]
async fn test_acl_list_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/12345/databases/67890/acl"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            404,
            json!({
                "error": {
                    "type": "NOT_FOUND",
                    "status": 404,
                    "description": "Database not found"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAclHandler::new(client);
    let result = handler.list(12345, 67890).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_acl_create_validation_error() {
    let mock_server = MockServer::start().await;
    let request_body = json!({
        "rule": "",
        "name": ""
    });

    Mock::given(method("POST"))
        .and(path("/subscriptions/12345/databases/67890/acl"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(body_json(&request_body))
        .respond_with(error_response(
            400,
            json!({
                "error": {
                    "type": "VALIDATION_ERROR",
                    "status": 400,
                    "description": "Rule and name are required fields"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAclHandler::new(client);
    let result = handler.create(12345, 67890, request_body).await;

    assert!(result.is_err());
}
