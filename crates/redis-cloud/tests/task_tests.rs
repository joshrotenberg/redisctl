use redis_cloud::client::CloudClient;
use redis_cloud::tasks::TaskHandler;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_list_tasks() {
    let mock_server = MockServer::start().await;

    let response = r#"[
        {
            "taskId": "task-123",
            "commandType": "createDatabase",
            "status": "completed",
            "description": "Create database operation",
            "startTime": "2024-01-01T00:00:00Z",
            "endTime": "2024-01-01T00:01:00Z"
        },
        {
            "taskId": "task-456",
            "commandType": "updateDatabase",
            "status": "in-progress",
            "description": "Update database operation",
            "startTime": "2024-01-01T00:02:00Z"
        }
    ]"#;

    Mock::given(method("GET"))
        .and(path("/tasks"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::from_str::<serde_json::Value>(response).unwrap()),
        )
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret_key("test-secret")
        .base_url(mock_server.uri())
        .build();

    let handler = TaskHandler::new(client);
    let tasks = handler.list().await.unwrap();

    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].task_id, "task-123");
    assert_eq!(tasks[1].task_id, "task-456");
}

#[tokio::test]
async fn test_get_task() {
    let mock_server = MockServer::start().await;

    let response = r#"{
        "taskId": "task-123",
        "commandType": "createDatabase",
        "status": "completed",
        "description": "Create database operation",
        "startTime": "2024-01-01T00:00:00Z",
        "endTime": "2024-01-01T00:01:00Z",
        "result": {
            "databaseId": 51234567,
            "name": "my-database"
        }
    }"#;

    Mock::given(method("GET"))
        .and(path("/tasks/task-123"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::from_str::<serde_json::Value>(response).unwrap()),
        )
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret_key("test-secret")
        .base_url(mock_server.uri())
        .build();

    let handler = TaskHandler::new(client);
    let task = handler.get("task-123").await.unwrap();

    assert_eq!(task.task_id, "task-123");
    assert_eq!(task.description.as_deref(), Some("Create database operation"));
}

#[tokio::test]
async fn test_task_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/tasks/invalid-task"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(
            ResponseTemplate::new(404)
                .set_body_json(serde_json::json!({
                    "error": "Task not found"
                })),
        )
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret_key("test-secret")
        .base_url(mock_server.uri())
        .build();

    let handler = TaskHandler::new(client);
    let result = handler.get("invalid-task").await;

    assert!(result.is_err());
}