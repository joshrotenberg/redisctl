//! Backup endpoint tests for Redis Cloud

use redis_cloud::{CloudBackupHandler, CloudClient, CloudConfig};
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
    let config = CloudConfig {
        api_key: "test-api-key".to_string(),
        api_secret: "test-secret-key".to_string(),
        base_url,
        timeout: std::time::Duration::from_secs(30),
    };
    CloudClient::new(config).unwrap()
}

#[tokio::test]
async fn test_backup_list() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/12345/databases/67890/backups"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "backups": [
                {
                    "backup_id": "backup-123",
                    "database_id": 67890,
                    "status": "completed",
                    "created_at": "2023-01-01T00:00:00Z",
                    "size_bytes": 1024000,
                    "download_url": "https://example.com/download/backup-123"
                },
                {
                    "backup_id": "backup-456",
                    "database_id": 67890,
                    "status": "in_progress",
                    "created_at": "2023-01-02T00:00:00Z",
                    "size_bytes": null,
                    "download_url": null
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudBackupHandler::new(client);
    let result = handler.list(12345, 67890).await;

    assert!(result.is_ok());
    let backups = result.unwrap();
    assert_eq!(backups.len(), 2);

    assert_eq!(backups[0].backup_id, "backup-123");
    assert_eq!(backups[0].database_id, 67890);
    assert_eq!(backups[0].status, "completed");
    assert_eq!(backups[0].size_bytes, Some(1024000));
    assert!(backups[0].download_url.is_some());

    assert_eq!(backups[1].backup_id, "backup-456");
    assert_eq!(backups[1].status, "in_progress");
    assert_eq!(backups[1].size_bytes, None);
    assert!(backups[1].download_url.is_none());
}

#[tokio::test]
async fn test_backup_list_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/12345/databases/67890/backups"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "backups": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudBackupHandler::new(client);
    let result = handler.list(12345, 67890).await;

    assert!(result.is_ok());
    let backups = result.unwrap();
    assert_eq!(backups.len(), 0);
}

#[tokio::test]
async fn test_backup_list_no_backups_field() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/12345/databases/67890/backups"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({})))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudBackupHandler::new(client);
    let result = handler.list(12345, 67890).await;

    assert!(result.is_ok());
    let backups = result.unwrap();
    assert_eq!(backups.len(), 0);
}

#[tokio::test]
async fn test_backup_create_with_description() {
    let mock_server = MockServer::start().await;
    let expected_request = json!({
        "database_id": 67890,
        "description": "Manual backup before upgrade"
    });

    Mock::given(method("POST"))
        .and(path("/subscriptions/12345/databases/67890/backup"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(body_json(&expected_request))
        .respond_with(created_response(json!({
            "backup_id": "backup-789",
            "database_id": 67890,
            "status": "in_progress",
            "created_at": "2023-01-01T00:00:00Z",
            "size_bytes": null,
            "download_url": null,
            "description": "Manual backup before upgrade"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudBackupHandler::new(client);
    let result = handler
        .create(
            12345,
            67890,
            Some("Manual backup before upgrade".to_string()),
        )
        .await;

    assert!(result.is_ok());
    let backup = result.unwrap();
    assert_eq!(backup.backup_id, "backup-789");
    assert_eq!(backup.database_id, 67890);
    assert_eq!(backup.status, "in_progress");
}

#[tokio::test]
async fn test_backup_create_without_description() {
    let mock_server = MockServer::start().await;
    let expected_request = json!({
        "database_id": 67890
    });

    Mock::given(method("POST"))
        .and(path("/subscriptions/12345/databases/67890/backup"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(body_json(&expected_request))
        .respond_with(created_response(json!({
            "backup_id": "backup-auto",
            "database_id": 67890,
            "status": "in_progress",
            "created_at": "2023-01-01T00:00:00Z",
            "size_bytes": null,
            "download_url": null
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudBackupHandler::new(client);
    let result = handler.create(12345, 67890, None).await;

    assert!(result.is_ok());
    let backup = result.unwrap();
    assert_eq!(backup.backup_id, "backup-auto");
    assert_eq!(backup.database_id, 67890);
}

#[tokio::test]
async fn test_backup_get() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(
            "/subscriptions/12345/databases/67890/backups/backup-123",
        ))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "backup_id": "backup-123",
            "database_id": 67890,
            "status": "completed",
            "created_at": "2023-01-01T00:00:00Z",
            "size_bytes": 1024000,
            "download_url": "https://example.com/download/backup-123",
            "description": "Daily backup",
            "retention_days": 30
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudBackupHandler::new(client);
    let result = handler.get(12345, 67890, "backup-123").await;

    assert!(result.is_ok());
    let backup = result.unwrap();
    assert_eq!(backup.backup_id, "backup-123");
    assert_eq!(backup.database_id, 67890);
    assert_eq!(backup.status, "completed");
    assert_eq!(backup.size_bytes, Some(1024000));
    assert!(backup.download_url.is_some());
    assert_eq!(backup.extra["description"], "Daily backup");
    assert_eq!(backup.extra["retention_days"], 30);
}

#[tokio::test]
async fn test_backup_restore() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(
            "/subscriptions/12345/databases/67890/backups/backup-123/restore",
        ))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(body_json(json!({})))
        .respond_with(success_response(json!({
            "taskId": "restore-task-456",
            "status": "in_progress",
            "message": "Backup restore initiated"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudBackupHandler::new(client);
    let result = handler.restore(12345, 67890, "backup-123").await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["taskId"], "restore-task-456");
    assert_eq!(response["status"], "in_progress");
}

#[tokio::test]
async fn test_backup_delete() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(
            "/subscriptions/12345/databases/67890/backups/backup-123",
        ))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(no_content_response())
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudBackupHandler::new(client);
    let result = handler.delete(12345, 67890, "backup-123").await;

    assert!(result.is_ok());
}

// Error handling tests
#[tokio::test]
async fn test_backup_list_database_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/12345/databases/99999/backups"))
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
    let handler = CloudBackupHandler::new(client);
    let result = handler.list(12345, 99999).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_backup_create_insufficient_storage() {
    let mock_server = MockServer::start().await;
    let request_body = json!({
        "database_id": 67890,
        "description": "Test backup"
    });

    Mock::given(method("POST"))
        .and(path("/subscriptions/12345/databases/67890/backup"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(body_json(&request_body))
        .respond_with(error_response(
            402,
            json!({
                "error": {
                    "type": "PAYMENT_REQUIRED",
                    "status": 402,
                    "description": "Insufficient storage quota for backup"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudBackupHandler::new(client);
    let result = handler
        .create(12345, 67890, Some("Test backup".to_string()))
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_backup_get_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(
            "/subscriptions/12345/databases/67890/backups/nonexistent",
        ))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            404,
            json!({
                "error": {
                    "type": "NOT_FOUND",
                    "status": 404,
                    "description": "Backup not found"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudBackupHandler::new(client);
    let result = handler.get(12345, 67890, "nonexistent").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_backup_restore_in_progress() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(
            "/subscriptions/12345/databases/67890/backups/backup-123/restore",
        ))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            409,
            json!({
                "error": {
                    "type": "CONFLICT",
                    "status": 409,
                    "description": "Another restore operation is already in progress"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudBackupHandler::new(client);
    let result = handler.restore(12345, 67890, "backup-123").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_backup_delete_protected() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(
            "/subscriptions/12345/databases/67890/backups/protected-backup",
        ))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            403,
            json!({
                "error": {
                    "type": "FORBIDDEN",
                    "status": 403,
                    "description": "Cannot delete protected backup"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudBackupHandler::new(client);
    let result = handler.delete(12345, 67890, "protected-backup").await;

    assert!(result.is_err());
}
