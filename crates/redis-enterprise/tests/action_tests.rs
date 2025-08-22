//! Action endpoint tests for Redis Enterprise

use redis_enterprise::{ActionHandler, EnterpriseClient};
use serde_json::json;
use wiremock::matchers::{basic_auth, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// Test helper functions
fn success_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(body)
}

fn no_content_response() -> ResponseTemplate {
    ResponseTemplate::new(204)
}

fn test_action() -> serde_json::Value {
    json!({
        "action_uid": "action-123-abc",
        "name": "database_backup",
        "status": "running",
        "progress": 45.5,
        "start_time": "2023-01-01T12:00:00Z",
        "end_time": null,
        "description": "Backing up database test-db",
        "error": null
    })
}

fn completed_action() -> serde_json::Value {
    json!({
        "action_uid": "action-456-def",
        "name": "database_restore",
        "status": "completed",
        "progress": 100.0,
        "start_time": "2023-01-01T11:00:00Z",
        "end_time": "2023-01-01T11:30:00Z",
        "description": "Restored database from backup",
        "error": null
    })
}

fn failed_action() -> serde_json::Value {
    json!({
        "action_uid": "action-789-ghi",
        "name": "node_add",
        "status": "failed",
        "progress": 25.0,
        "start_time": "2023-01-01T10:00:00Z",
        "end_time": "2023-01-01T10:15:00Z",
        "description": "Adding new node to cluster",
        "error": "Connection timeout to new node"
    })
}

#[tokio::test]
async fn test_action_list() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/actions"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            test_action(),
            completed_action(),
            failed_action()
        ])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ActionHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let actions = result.unwrap();
    assert_eq!(actions.len(), 3);

    // Verify first action details
    let running_action = &actions[0];
    assert_eq!(running_action.action_uid, "action-123-abc");
    assert_eq!(running_action.name, "database_backup");
    assert_eq!(running_action.status, "running");
    assert_eq!(running_action.progress, Some(45.5));
    assert!(running_action.end_time.is_none());
    assert!(running_action.error.is_none());
}

#[tokio::test]
async fn test_action_list_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/actions"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ActionHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let actions = result.unwrap();
    assert_eq!(actions.len(), 0);
}

#[tokio::test]
async fn test_action_get_running() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/actions/action-123-abc"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_action()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ActionHandler::new(client);
    let result = handler.get("action-123-abc").await;

    assert!(result.is_ok());
    let action = result.unwrap();
    assert_eq!(action.action_uid, "action-123-abc");
    assert_eq!(action.name, "database_backup");
    assert_eq!(action.status, "running");
    assert_eq!(action.progress, Some(45.5));
    assert_eq!(
        action.description,
        Some("Backing up database test-db".to_string())
    );
    assert!(action.end_time.is_none());
    assert!(action.error.is_none());
}

#[tokio::test]
async fn test_action_get_completed() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/actions/action-456-def"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(completed_action()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ActionHandler::new(client);
    let result = handler.get("action-456-def").await;

    assert!(result.is_ok());
    let action = result.unwrap();
    assert_eq!(action.action_uid, "action-456-def");
    assert_eq!(action.status, "completed");
    assert_eq!(action.progress, Some(100.0));
    assert!(action.end_time.is_some());
    assert!(action.error.is_none());
}

#[tokio::test]
async fn test_action_get_failed() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/actions/action-789-ghi"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(failed_action()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ActionHandler::new(client);
    let result = handler.get("action-789-ghi").await;

    assert!(result.is_ok());
    let action = result.unwrap();
    assert_eq!(action.action_uid, "action-789-ghi");
    assert_eq!(action.status, "failed");
    assert_eq!(action.progress, Some(25.0));
    assert_eq!(
        action.error,
        Some("Connection timeout to new node".to_string())
    );
    assert!(action.end_time.is_some());
}

#[tokio::test]
async fn test_action_cancel() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/actions/action-123-abc"))
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

    let handler = ActionHandler::new(client);
    let result = handler.cancel("action-123-abc").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_action_cancel_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/actions/nonexistent-action"))
        .and(basic_auth("admin", "password"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "error": "Action not found"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ActionHandler::new(client);
    let result = handler.cancel("nonexistent-action").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_action_get_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/actions/nonexistent-action"))
        .and(basic_auth("admin", "password"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "error": "Action not found"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ActionHandler::new(client);
    let result = handler.get("nonexistent-action").await;

    assert!(result.is_err());
}
