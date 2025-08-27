//! Tasks endpoint tests for Redis Cloud

use redis_cloud::{CloudClient, CloudTasksHandler};
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
    CloudClient::builder()
        .api_key("test-api-key")
        .api_secret("test-secret-key")
        .base_url(base_url)
        .build()
        .unwrap()
}

// Helper function to create mock tasks list response
fn tasks_list_response() -> serde_json::Value {
    json!({
        "tasks": [
            {
                "taskId": "task_12345",
                "type": "database_creation",
                "status": "processing",
                "description": "Creating database test-db",
                "progress": 75,
                "createdAt": "2023-01-01T10:00:00Z",
                "updatedAt": "2023-01-01T10:30:00Z",
                "subscriptionId": 67890,
                "databaseId": null,
                "estimatedTimeRemaining": "00:05:00"
            },
            {
                "taskId": "task_12346",
                "type": "subscription_update",
                "status": "completed",
                "description": "Updating subscription plan",
                "progress": 100,
                "createdAt": "2023-01-01T09:00:00Z",
                "updatedAt": "2023-01-01T09:45:00Z",
                "completedAt": "2023-01-01T09:45:00Z",
                "subscriptionId": 67890,
                "databaseId": null,
                "estimatedTimeRemaining": null
            },
            {
                "taskId": "task_12347",
                "type": "backup_creation",
                "status": "failed",
                "description": "Creating backup for database prod-cache",
                "progress": 0,
                "createdAt": "2023-01-01T08:00:00Z",
                "updatedAt": "2023-01-01T08:15:00Z",
                "failedAt": "2023-01-01T08:15:00Z",
                "subscriptionId": 67890,
                "databaseId": 123,
                "error": "Insufficient storage space",
                "estimatedTimeRemaining": null
            }
        ],
        "pagination": {
            "total": 25,
            "limit": 10,
            "offset": 0,
            "hasMore": true
        }
    })
}

// Helper function to create mock single task response
fn single_task_response() -> serde_json::Value {
    json!({
        "task": {
            "taskId": "task_12345",
            "type": "database_creation",
            "status": "processing",
            "description": "Creating database test-db",
            "progress": 85,
            "createdAt": "2023-01-01T10:00:00Z",
            "updatedAt": "2023-01-01T10:35:00Z",
            "subscriptionId": 67890,
            "databaseId": null,
            "estimatedTimeRemaining": "00:03:00",
            "steps": [
                {
                    "name": "provision_resources",
                    "status": "completed",
                    "completedAt": "2023-01-01T10:10:00Z"
                },
                {
                    "name": "configure_database",
                    "status": "processing",
                    "progress": 70
                },
                {
                    "name": "setup_networking",
                    "status": "pending"
                }
            ]
        }
    })
}

// Helper function to create completed task response
fn completed_task_response() -> serde_json::Value {
    json!({
        "task": {
            "taskId": "task_completed",
            "type": "database_backup",
            "status": "completed",
            "description": "Backup completed successfully",
            "progress": 100,
            "createdAt": "2023-01-01T12:00:00Z",
            "updatedAt": "2023-01-01T12:30:00Z",
            "completedAt": "2023-01-01T12:30:00Z",
            "subscriptionId": 67890,
            "databaseId": 456,
            "estimatedTimeRemaining": null,
            "result": {
                "backupId": "backup_789",
                "size": "1.2GB",
                "location": "s3://redis-backups/backup_789.rdb"
            }
        }
    })
}

// Helper function to create failed task response
fn failed_task_response() -> serde_json::Value {
    json!({
        "task": {
            "taskId": "task_failed",
            "type": "database_restore",
            "status": "failed",
            "description": "Restore database from backup",
            "progress": 45,
            "createdAt": "2023-01-01T11:00:00Z",
            "updatedAt": "2023-01-01T11:20:00Z",
            "failedAt": "2023-01-01T11:20:00Z",
            "subscriptionId": 67890,
            "databaseId": 789,
            "estimatedTimeRemaining": null,
            "error": "Backup file corrupted",
            "errorCode": "BACKUP_CORRUPTION",
            "steps": [
                {
                    "name": "validate_backup",
                    "status": "completed",
                    "completedAt": "2023-01-01T11:05:00Z"
                },
                {
                    "name": "restore_data",
                    "status": "failed",
                    "failedAt": "2023-01-01T11:20:00Z",
                    "error": "Checksum mismatch"
                }
            ]
        }
    })
}

#[tokio::test]
async fn test_list_tasks() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/tasks"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(tasks_list_response()))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudTasksHandler::new(client);

    let result = handler.list().await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let tasks = response["tasks"].as_array().unwrap();
    assert_eq!(tasks.len(), 3);

    // Check first task (processing)
    assert_eq!(tasks[0]["taskId"], "task_12345");
    assert_eq!(tasks[0]["type"], "database_creation");
    assert_eq!(tasks[0]["status"], "processing");
    assert_eq!(tasks[0]["progress"], 75);
    assert_eq!(tasks[0]["subscriptionId"], 67890);

    // Check second task (completed)
    assert_eq!(tasks[1]["taskId"], "task_12346");
    assert_eq!(tasks[1]["status"], "completed");
    assert_eq!(tasks[1]["progress"], 100);
    assert!(tasks[1]["completedAt"].is_string());

    // Check third task (failed)
    assert_eq!(tasks[2]["taskId"], "task_12347");
    assert_eq!(tasks[2]["status"], "failed");
    assert_eq!(tasks[2]["error"], "Insufficient storage space");
    assert_eq!(tasks[2]["databaseId"], 123);

    // Check pagination
    let pagination = &response["pagination"];
    assert_eq!(pagination["total"], 25);
    assert_eq!(pagination["hasMore"], true);
}

#[tokio::test]
async fn test_list_tasks_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/tasks"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "tasks": [],
            "pagination": {
                "total": 0,
                "limit": 10,
                "offset": 0,
                "hasMore": false
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudTasksHandler::new(client);

    let result = handler.list().await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let tasks = response["tasks"].as_array().unwrap();
    assert_eq!(tasks.len(), 0);
    assert_eq!(response["pagination"]["total"], 0);
}

#[tokio::test]
async fn test_list_tasks_unauthorized() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/tasks"))
        .and(header("x-api-key", "invalid-key"))
        .and(header("x-api-secret-key", "invalid-secret"))
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

    let client = CloudClient::builder()
        .api_key("invalid-key")
        .api_secret("invalid-secret")
        .base_url(mock_server.uri())
        .build()
        .unwrap();
    let handler = CloudTasksHandler::new(client);

    let result = handler.list().await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_task_processing() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/tasks/task_12345"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(single_task_response()))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudTasksHandler::new(client);

    let result = handler.get("task_12345").await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let task = &response["task"];

    assert_eq!(task["taskId"], "task_12345");
    assert_eq!(task["type"], "database_creation");
    assert_eq!(task["status"], "processing");
    assert_eq!(task["progress"], 85);
    assert_eq!(task["subscriptionId"], 67890);
    assert_eq!(task["estimatedTimeRemaining"], "00:03:00");

    // Check steps
    let steps = task["steps"].as_array().unwrap();
    assert_eq!(steps.len(), 3);

    assert_eq!(steps[0]["name"], "provision_resources");
    assert_eq!(steps[0]["status"], "completed");
    assert!(steps[0]["completedAt"].is_string());

    assert_eq!(steps[1]["name"], "configure_database");
    assert_eq!(steps[1]["status"], "processing");
    assert_eq!(steps[1]["progress"], 70);

    assert_eq!(steps[2]["name"], "setup_networking");
    assert_eq!(steps[2]["status"], "pending");
}

#[tokio::test]
async fn test_get_task_completed() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/tasks/task_completed"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(completed_task_response()))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudTasksHandler::new(client);

    let result = handler.get("task_completed").await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let task = &response["task"];

    assert_eq!(task["taskId"], "task_completed");
    assert_eq!(task["status"], "completed");
    assert_eq!(task["progress"], 100);
    assert!(task["completedAt"].is_string());
    assert!(task["estimatedTimeRemaining"].is_null());

    // Check result
    let result_obj = &task["result"];
    assert_eq!(result_obj["backupId"], "backup_789");
    assert_eq!(result_obj["size"], "1.2GB");
    assert_eq!(result_obj["location"], "s3://redis-backups/backup_789.rdb");
}

#[tokio::test]
async fn test_get_task_failed() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/tasks/task_failed"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(failed_task_response()))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudTasksHandler::new(client);

    let result = handler.get("task_failed").await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let task = &response["task"];

    assert_eq!(task["taskId"], "task_failed");
    assert_eq!(task["status"], "failed");
    assert_eq!(task["progress"], 45);
    assert!(task["failedAt"].is_string());
    assert_eq!(task["error"], "Backup file corrupted");
    assert_eq!(task["errorCode"], "BACKUP_CORRUPTION");

    // Check failed steps
    let steps = task["steps"].as_array().unwrap();
    assert_eq!(steps.len(), 2);

    assert_eq!(steps[0]["status"], "completed");
    assert_eq!(steps[1]["status"], "failed");
    assert_eq!(steps[1]["error"], "Checksum mismatch");
}

#[tokio::test]
async fn test_get_task_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/tasks/nonexistent_task"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            404,
            json!({
                "error": {
                    "type": "RESOURCE_NOT_FOUND",
                    "status": 404,
                    "description": "Task not found"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudTasksHandler::new(client);

    let result = handler.get("nonexistent_task").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_task_invalid_id_format() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/tasks/invalid-task-id"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            400,
            json!({
                "error": {
                    "type": "INVALID_REQUEST",
                    "status": 400,
                    "description": "Invalid task ID format"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudTasksHandler::new(client);

    let result = handler.get("invalid-task-id").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_task_with_special_characters() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/tasks/task_abc-123_xyz"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(single_task_response()))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudTasksHandler::new(client);

    let result = handler.get("task_abc-123_xyz").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_task_forbidden() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/tasks/task_restricted"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            403,
            json!({
                "error": {
                    "type": "FORBIDDEN",
                    "status": 403,
                    "description": "Insufficient permissions to access this task"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudTasksHandler::new(client);

    let result = handler.get("task_restricted").await;

    assert!(result.is_err());
}
