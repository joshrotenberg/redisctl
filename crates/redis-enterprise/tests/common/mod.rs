//! Common test utilities for unit tests

use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path, basic_auth, header};
use serde_json::json;

/// Create a mock server with default configuration
pub async fn setup_mock_server() -> MockServer {
    MockServer::start().await
}

/// Create a standard success response
pub fn success_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200)
        .set_body_json(body)
}

/// Create a standard error response
pub fn error_response(code: u16, message: &str) -> ResponseTemplate {
    ResponseTemplate::new(code)
        .set_body_json(json!({
            "error": message,
            "code": code
        }))
}

/// Create a 204 No Content response
pub fn no_content_response() -> ResponseTemplate {
    ResponseTemplate::new(204)
}

/// Create a 201 Created response
pub fn created_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(201)
        .set_body_json(body)
}

/// Mock Enterprise auth header matcher
pub fn enterprise_auth(username: &str, password: &str) -> impl wiremock::Match {
    basic_auth(username, password)
}

/// Mock Cloud auth header matchers
pub fn cloud_auth() -> Vec<Box<dyn wiremock::Match>> {
    vec![
        Box::new(header("x-api-key", "test-key")),
        Box::new(header("x-api-secret-key", "test-secret")),
    ]
}

/// Standard test database response
pub fn test_database() -> serde_json::Value {
    json!({
        "uid": 1,
        "name": "test-db",
        "type": "redis",
        "memory_size": 1073741824,
        "port": 12000,
        "status": "active"
    })
}

/// Standard test cluster response
pub fn test_cluster() -> serde_json::Value {
    json!({
        "name": "test-cluster",
        "nodes": [1, 2, 3],
        "databases": [1, 2],
        "version": "7.2.0",
        "license_expired": false
    })
}

/// Standard test node response
pub fn test_node() -> serde_json::Value {
    json!({
        "uid": 1,
        "address": "192.168.1.10",
        "status": "active",
        "role": "master",
        "total_memory": 16000000000i64,
        "used_memory": 8000000000i64
    })
}

/// Standard test user response
pub fn test_user() -> serde_json::Value {
    json!({
        "uid": 1,
        "username": "testuser",
        "email": "test@example.com",
        "role": "admin",
        "status": "active"
    })
}

/// Standard action response
pub fn action_response(action: &str) -> serde_json::Value {
    json!({
        "action_uid": format!("{}-123", action),
        "status": "completed",
        "progress": 100
    })
}

/// Standard task response
pub fn task_response(task_id: &str) -> serde_json::Value {
    json!({
        "task_id": task_id,
        "status": "completed",
        "progress": 100,
        "description": "Task completed successfully"
    })
}