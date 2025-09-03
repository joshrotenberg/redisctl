//! API keys endpoint tests for Redis Cloud

use redis_cloud::{CloudApiKeyHandler, CloudClient};
use serde_json::json;
use wiremock::matchers::{header, method, path, query_param};
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

fn create_test_client(base_url: String) -> CloudClient {
    CloudClient::builder()
        .api_key("test-api-key")
        .api_secret("test-secret-key")
        .base_url(base_url)
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_list_api_keys() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api-keys"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "apiKeys": [
                {
                    "id": 1001,
                    "name": "Production API Key",
                    "accountId": 12345,
                    "createdTimestamp": "2023-01-01T00:00:00Z",
                    "lastUsedTimestamp": "2023-12-01T10:30:00Z",
                    "status": "active",
                    "permissions": ["read", "write"]
                },
                {
                    "id": 1002,
                    "name": "Development API Key",
                    "accountId": 12345,
                    "createdTimestamp": "2023-06-01T00:00:00Z",
                    "lastUsedTimestamp": "2023-12-01T09:15:00Z",
                    "status": "active",
                    "permissions": ["read"]
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudApiKeyHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let keys_vec = result.unwrap();
    let response = json!({"apiKeys": keys_vec});
    let api_keys = response["apiKeys"].as_array().unwrap();
    assert_eq!(api_keys.len(), 2);
    assert_eq!(api_keys[0]["name"], "Production API Key");
    assert_eq!(api_keys[1]["name"], "Development API Key");
}

#[tokio::test]
async fn test_list_api_keys_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api-keys"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            401,
            json!({
                "error": {
                    "type": "UNAUTHORIZED",
                    "status": 401,
                    "description": "Invalid API credentials"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudApiKeyHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_api_key() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api-keys/1001"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "apiKey": {
                "id": 1001,
                "name": "Production API Key",
                "accountId": 12345,
                "createdTimestamp": "2023-01-01T00:00:00Z",
                "lastUsedTimestamp": "2023-12-01T10:30:00Z",
                "status": "active",
                "permissions": ["read", "write"],
                "description": "Main production environment key"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudApiKeyHandler::new(client);
    let result = handler.get(1001).await;

    assert!(result.is_ok());
    let api_key_obj = result.unwrap();
    let response = json!({"apiKey": api_key_obj});
    let api_key = &response["apiKey"];
    assert_eq!(api_key["id"], 1001);
    assert_eq!(api_key["name"], "Production API Key");
    assert_eq!(api_key["status"], "active");
}

#[tokio::test]
async fn test_get_api_key_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api-keys/9999"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            404,
            json!({
                "error": {
                    "type": "NOT_FOUND",
                    "status": 404,
                    "description": "API key not found"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudApiKeyHandler::new(client);
    let result = handler.get(9999).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_api_key() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api-keys"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(created_response(json!({
            "apiKey": {
                "id": 1003,
                "name": "Test API Key",
                "accountId": 12345,
                "createdTimestamp": "2023-12-01T12:00:00Z",
                "status": "active",
                "permissions": ["read"],
                "secret": "sk_test_abc123..."
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudApiKeyHandler::new(client);
    let request = json!({
        "name": "Test API Key",
        "permissions": ["read"],
        "description": "Test key for development"
    });
    let result = handler.create(&request).await;

    assert!(result.is_ok());
    let api_key_obj = result.unwrap();
    let response = json!({"apiKey": api_key_obj});
    let api_key = &response["apiKey"];
    assert_eq!(api_key["name"], "Test API Key");
    assert_eq!(api_key["status"], "active");
    assert!(api_key["secret"].as_str().unwrap().starts_with("sk_test_"));
}

#[tokio::test]
async fn test_update_api_key() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/api-keys/1001"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "apiKey": {
                "id": 1001,
                "name": "Updated Production API Key",
                "accountId": 12345,
                "createdTimestamp": "2023-01-01T00:00:00Z",
                "updatedTimestamp": "2023-12-01T12:00:00Z",
                "status": "active",
                "permissions": ["read", "write"],
                "description": "Updated description"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudApiKeyHandler::new(client);
    let request = json!({
        "name": "Updated Production API Key",
        "description": "Updated description"
    });
    let result = handler.update(1001, &request).await;

    assert!(result.is_ok());
    let api_key_obj = result.unwrap();
    let response = json!({"apiKey": api_key_obj});
    let api_key = &response["apiKey"];
    assert_eq!(api_key["name"], "Updated Production API Key");
    assert_eq!(api_key["description"], "Updated description");
}

#[tokio::test]
async fn test_delete_api_key() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/api-keys/1001"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudApiKeyHandler::new(client);
    let result = handler.delete(1001).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["message"], "API key 1001 deleted");
}

#[tokio::test]
async fn test_regenerate_api_key() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api-keys/1001/regenerate"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "apiKey": {
                "id": 1001,
                "name": "Production API Key",
                "accountId": 12345,
                "secret": "sk_new_regenerated_key123...",
                "regeneratedTimestamp": "2023-12-01T12:00:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudApiKeyHandler::new(client);
    let result = handler.regenerate(1001).await;

    assert!(result.is_ok());
    let api_key_obj = result.unwrap();
    let response = json!({"apiKey": api_key_obj});
    let api_key = &response["apiKey"];
    assert_eq!(api_key["id"], 1001);
    assert!(api_key["secret"].as_str().unwrap().starts_with("sk_new_"));
    assert!(api_key["regeneratedTimestamp"].is_string());
}

#[tokio::test]
async fn test_get_permissions() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api-keys/1001/permissions"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "permissions": {
                "resources": [
                    {
                        "type": "subscription",
                        "id": "sub_123",
                        "permissions": ["read", "write"]
                    },
                    {
                        "type": "database",
                        "id": "db_456",
                        "permissions": ["read"]
                    }
                ],
                "globalPermissions": ["account:read"]
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudApiKeyHandler::new(client);
    let result = handler.get_permissions(1001).await;

    assert!(result.is_ok());
    let perms_obj = result.unwrap();
    let response = json!({"permissions": perms_obj});
    let permissions = &response["permissions"];
    let resources = permissions["resources"].as_array().unwrap();
    assert_eq!(resources.len(), 2);
    assert_eq!(resources[0]["type"], "subscription");
    assert_eq!(resources[1]["type"], "database");
}

#[tokio::test]
async fn test_update_permissions() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/api-keys/1001/permissions"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "permissions": {
                "resources": [
                    {
                        "type": "subscription",
                        "id": "sub_123",
                        "permissions": ["read", "write", "delete"]
                    }
                ],
                "globalPermissions": ["account:read", "billing:read"]
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudApiKeyHandler::new(client);
    let request = json!({
        "resources": [
            {
                "type": "subscription",
                "id": "sub_123",
                "permissions": ["read", "write", "delete"]
            }
        ],
        "globalPermissions": ["account:read", "billing:read"]
    });
    let result = handler.update_permissions(1001, &request).await;

    assert!(result.is_ok());
    let perms_obj = result.unwrap();
    let response = json!({"permissions": perms_obj});
    let permissions = &response["permissions"];
    let global_perms = permissions["globalPermissions"].as_array().unwrap();
    assert!(global_perms.contains(&json!("billing:read")));
}

#[tokio::test]
async fn test_enable_api_key() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api-keys/1001/enable"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "apiKey": {
                "id": 1001,
                "status": "active",
                "enabledTimestamp": "2023-12-01T12:00:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudApiKeyHandler::new(client);
    let result = handler.enable(1001).await;

    assert!(result.is_ok());
    let api_key_obj = result.unwrap();
    let response = json!({"apiKey": api_key_obj});
    let api_key = &response["apiKey"];
    assert_eq!(api_key["status"], "active");
    assert!(api_key["enabledTimestamp"].is_string());
}

#[tokio::test]
async fn test_disable_api_key() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api-keys/1001/disable"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "apiKey": {
                "id": 1001,
                "status": "disabled",
                "disabledTimestamp": "2023-12-01T12:00:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudApiKeyHandler::new(client);
    let result = handler.disable(1001).await;

    assert!(result.is_ok());
    let api_key_obj = result.unwrap();
    let response = json!({"apiKey": api_key_obj});
    let api_key = &response["apiKey"];
    assert_eq!(api_key["status"], "disabled");
    assert!(api_key["disabledTimestamp"].is_string());
}

#[tokio::test]
async fn test_get_usage() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api-keys/1001/usage"))
        .and(query_param("period", "30d"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "usage": {
                "period": "30d",
                "totalRequests": 15420,
                "successfulRequests": 15350,
                "failedRequests": 70,
                "averageResponseTime": 125.5,
                "peakRequestsPerHour": 150,
                "endpoints": [
                    {
                        "path": "/subscriptions",
                        "method": "GET",
                        "requests": 8500,
                        "averageResponseTime": 95.2
                    },
                    {
                        "path": "/databases",
                        "method": "GET",
                        "requests": 4200,
                        "averageResponseTime": 180.1
                    }
                ]
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudApiKeyHandler::new(client);
    let result = handler.get_usage(1001, "30d").await;

    assert!(result.is_ok());
    let usage_obj = result.unwrap();
    let response = json!({"usage": usage_obj});
    let usage = &response["usage"];
    assert_eq!(usage["period"], "30d");
    assert_eq!(usage["totalRequests"], 15420);
    assert_eq!(usage["successfulRequests"], 15350);
    let endpoints = usage["endpoints"].as_array().unwrap();
    assert_eq!(endpoints.len(), 2);
}

#[tokio::test]
async fn test_get_audit_logs() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api-keys/1001/audit"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "auditLogs": [
                {
                    "id": "log_001",
                    "timestamp": "2023-12-01T12:00:00Z",
                    "action": "KEY_CREATED",
                    "user": "admin@example.com",
                    "details": {
                        "keyName": "Production API Key",
                        "permissions": ["read", "write"]
                    }
                },
                {
                    "id": "log_002",
                    "timestamp": "2023-12-01T11:30:00Z",
                    "action": "PERMISSIONS_UPDATED",
                    "user": "admin@example.com",
                    "details": {
                        "previousPermissions": ["read"],
                        "newPermissions": ["read", "write"]
                    }
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudApiKeyHandler::new(client);
    let result = handler.get_audit_logs(1001).await;

    assert!(result.is_ok());
    let audits_obj = result.unwrap();
    let response = json!({"auditLogs": audits_obj.logs});
    let audit_logs = response["auditLogs"].as_array().unwrap();
    assert_eq!(audit_logs.len(), 2);
    assert_eq!(audit_logs[0]["action"], "KEY_CREATED");
    assert_eq!(audit_logs[1]["action"], "PERMISSIONS_UPDATED");
}

#[tokio::test]
async fn test_get_audit_logs_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api-keys/1001/audit"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            403,
            json!({
                "error": {
                    "type": "FORBIDDEN",
                    "status": 403,
                    "description": "Insufficient permissions to access audit logs"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudApiKeyHandler::new(client);
    let result = handler.get_audit_logs(1001).await;

    assert!(result.is_err());
}
