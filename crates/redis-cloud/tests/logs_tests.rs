//! Logs endpoint tests for Redis Cloud

use redis_cloud::{CloudClient, CloudLogsHandler};
use serde_json::json;
use wiremock::matchers::{header, method, path, query_param};
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

// Helper function to create mock database logs response
fn database_logs_response() -> serde_json::Value {
    json!({
        "logs": [
            {
                "timestamp": "2023-01-01T10:00:00Z",
                "level": "INFO",
                "message": "Database connection established",
                "source": "redis-server",
                "databaseId": 123
            },
            {
                "timestamp": "2023-01-01T10:01:00Z",
                "level": "WARN",
                "message": "Memory usage above 80%",
                "source": "monitoring",
                "databaseId": 123
            },
            {
                "timestamp": "2023-01-01T10:02:00Z",
                "level": "ERROR",
                "message": "Connection timeout",
                "source": "client",
                "databaseId": 123
            }
        ],
        "pagination": {
            "total": 50,
            "limit": 3,
            "offset": 0,
            "hasMore": true
        }
    })
}

// Helper function to create mock system logs response
fn system_logs_response() -> serde_json::Value {
    json!({
        "logs": [
            {
                "timestamp": "2023-01-01T09:00:00Z",
                "level": "INFO",
                "message": "System health check passed",
                "source": "health-monitor",
                "accountId": 12345
            },
            {
                "timestamp": "2023-01-01T09:05:00Z",
                "level": "WARN",
                "message": "High CPU usage detected",
                "source": "resource-monitor",
                "accountId": 12345
            }
        ],
        "pagination": {
            "total": 100,
            "limit": 2,
            "offset": 0,
            "hasMore": true
        }
    })
}

// Helper function to create mock session logs response
fn session_logs_response() -> serde_json::Value {
    json!({
        "sessionLogs": [
            {
                "sessionId": "sess_abc123",
                "timestamp": "2023-01-01T11:00:00Z",
                "userId": 456,
                "action": "LOGIN",
                "userAgent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64)",
                "ipAddress": "192.168.1.100",
                "status": "SUCCESS"
            },
            {
                "sessionId": "sess_def456",
                "timestamp": "2023-01-01T11:15:00Z",
                "userId": 789,
                "action": "API_CALL",
                "endpoint": "/subscriptions",
                "method": "GET",
                "status": "SUCCESS"
            },
            {
                "sessionId": "sess_ghi789",
                "timestamp": "2023-01-01T11:30:00Z",
                "userId": 321,
                "action": "LOGOUT",
                "status": "SUCCESS"
            }
        ],
        "pagination": {
            "total": 25,
            "limit": 3,
            "offset": 0,
            "hasMore": true
        }
    })
}

#[tokio::test]
async fn test_database_logs_basic() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/12345/databases/123/logs"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(database_logs_response()))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudLogsHandler::new(client);

    let result = handler.database(12345, 123, None, None).await;

    assert!(result.is_ok());
    let response = serde_json::to_value(result.unwrap()).unwrap();
    let logs = response["logs"].as_array().unwrap();
    assert_eq!(logs.len(), 3);

    assert_eq!(logs[0]["level"], "INFO");
    assert_eq!(logs[0]["message"], "Database connection established");
    assert_eq!(logs[0]["databaseId"], 123);

    assert_eq!(logs[1]["level"], "WARN");
    assert_eq!(logs[2]["level"], "ERROR");

    let pagination = &response["pagination"];
    assert_eq!(pagination["total"], 50);
    assert_eq!(pagination["hasMore"], true);
}

#[tokio::test]
async fn test_database_logs_with_limit() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/12345/databases/123/logs"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(query_param("limit", "10"))
        .respond_with(success_response(database_logs_response()))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudLogsHandler::new(client);

    let result = handler.database(12345, 123, Some(10), None).await;

    assert!(result.is_ok());
    let response = serde_json::to_value(result.unwrap()).unwrap();
    assert!(response["logs"].is_array());
}

#[tokio::test]
async fn test_database_logs_with_limit_and_offset() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/12345/databases/123/logs"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(query_param("limit", "10"))
        .and(query_param("offset", "20"))
        .respond_with(success_response(database_logs_response()))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudLogsHandler::new(client);

    let result = handler.database(12345, 123, Some(10), Some(20)).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_database_logs_with_offset_only() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/12345/databases/123/logs"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(query_param("offset", "5"))
        .respond_with(success_response(database_logs_response()))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudLogsHandler::new(client);

    let result = handler.database(12345, 123, None, Some(5)).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_database_logs_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/12345/databases/999/logs"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            404,
            json!({
                "error": {
                    "type": "RESOURCE_NOT_FOUND",
                    "status": 404,
                    "description": "Database not found"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudLogsHandler::new(client);

    let result = handler.database(12345, 999, None, None).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_system_logs_basic() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/logs"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(system_logs_response()))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudLogsHandler::new(client);

    let result = handler.system(None, None).await;

    assert!(result.is_ok());
    let resp = result.unwrap();
    let response = json!({
        "logs": resp.logs,
        "pagination": {"total": resp.total, "limit": resp.limit, "offset": resp.offset}
    });
    let logs = response["logs"].as_array().unwrap();
    assert_eq!(logs.len(), 2);

    assert_eq!(logs[0]["level"], "INFO");
    assert_eq!(logs[0]["message"], "System health check passed");
    assert_eq!(logs[0]["source"], "health-monitor");

    assert_eq!(logs[1]["level"], "WARN");
    assert_eq!(logs[1]["message"], "High CPU usage detected");

    let pagination = &response["pagination"];
    assert_eq!(pagination["total"], 100);
}

#[tokio::test]
async fn test_system_logs_with_pagination() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/logs"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(query_param("limit", "50"))
        .and(query_param("offset", "10"))
        .respond_with(success_response(system_logs_response()))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudLogsHandler::new(client);

    let result = handler.system(Some(50), Some(10)).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_system_logs_unauthorized() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/logs"))
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
    let handler = CloudLogsHandler::new(client);

    let result = handler.system(None, None).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_session_logs_basic() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/session-logs"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(session_logs_response()))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudLogsHandler::new(client);

    let result = handler.session(None, None).await;

    assert!(result.is_ok());
    let resp = result.unwrap();
    let response = serde_json::to_value(resp).unwrap();
    let session_logs = response["sessionLogs"].as_array().unwrap();
    assert_eq!(session_logs.len(), 3);

    assert_eq!(session_logs[0]["sessionId"], "sess_abc123");
    assert_eq!(session_logs[0]["action"], "LOGIN");
    assert_eq!(session_logs[0]["status"], "SUCCESS");

    assert_eq!(session_logs[1]["action"], "API_CALL");
    assert_eq!(session_logs[1]["endpoint"], "/subscriptions");

    assert_eq!(session_logs[2]["action"], "LOGOUT");

    let pagination = &response["pagination"];
    assert_eq!(pagination["total"], 25);
}

#[tokio::test]
async fn test_session_logs_with_pagination() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/session-logs"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(query_param("limit", "25"))
        .and(query_param("offset", "5"))
        .respond_with(success_response(session_logs_response()))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudLogsHandler::new(client);

    let result = handler.session(Some(25), Some(5)).await;

    assert!(result.is_ok());
    let response = serde_json::to_value(result.unwrap()).unwrap();
    assert!(response["sessionLogs"].is_array());
}

#[tokio::test]
async fn test_session_logs_forbidden() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/session-logs"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            403,
            json!({
                "error": {
                    "type": "FORBIDDEN",
                    "status": 403,
                    "description": "Insufficient permissions to access session logs"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudLogsHandler::new(client);

    let result = handler.session(None, None).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_session_logs_empty_response() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/session-logs"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "sessionLogs": [],
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
    let handler = CloudLogsHandler::new(client);

    let result = handler.session(None, None).await;

    assert!(result.is_ok());
    let resp = result.unwrap();
    let response = serde_json::to_value(resp).unwrap();
    let session_logs = response["sessionLogs"].as_array().unwrap();
    assert_eq!(session_logs.len(), 0);
    assert_eq!(response["pagination"]["total"], 0);
}
