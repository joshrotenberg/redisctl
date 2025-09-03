//! Metrics endpoint tests for Redis Cloud

use redis_cloud::{CloudClient, CloudMetricsHandler};
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

// Helper function to create mock database metrics response
fn database_metrics_response() -> serde_json::Value {
    json!({
        "database_id": 123,
        "measurements": [
            {
                "name": "used_memory",
                "values": [
                    {
                        "timestamp": "2023-01-01T00:00:00Z",
                        "value": 1048576.0
                    },
                    {
                        "timestamp": "2023-01-01T01:00:00Z",
                        "value": 1073741824.0
                    }
                ]
            },
            {
                "name": "total_requests",
                "values": [
                    {
                        "timestamp": "2023-01-01T00:00:00Z",
                        "value": 1000.0
                    },
                    {
                        "timestamp": "2023-01-01T01:00:00Z",
                        "value": 1500.0
                    }
                ]
            }
        ]
    })
}

// Helper function to create mock subscription metrics response
fn subscription_metrics_response() -> serde_json::Value {
    json!({
        "subscriptionMetrics": {
            "subscriptionId": 12345,
            "totalMemoryUsage": 2097152.0,
            "totalRequests": 5000.0,
            "databases": [
                {
                    "databaseId": 123,
                    "memoryUsage": 1048576.0,
                    "requests": 2500.0
                },
                {
                    "databaseId": 124,
                    "memoryUsage": 1048576.0,
                    "requests": 2500.0
                }
            ]
        }
    })
}

#[tokio::test]
async fn test_database_metrics_basic() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/12345/databases/123/metrics"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(query_param("metricSpecs", "used_memory"))
        .respond_with(success_response(database_metrics_response()))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudMetricsHandler::new(client);

    let result = handler
        .database(12345, 123, vec!["used_memory".to_string()], None, None)
        .await;

    assert!(result.is_ok());
    let metrics = result.unwrap();
    assert_eq!(metrics.database_id, 123);
    assert_eq!(metrics.measurements.len(), 2);
    assert_eq!(metrics.measurements[0].name, "used_memory");
    assert_eq!(metrics.measurements[0].values.len(), 2);
    assert_eq!(metrics.measurements[0].values[0].value, 1048576.0);
}

#[tokio::test]
async fn test_database_metrics_multiple_specs() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/12345/databases/123/metrics"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(query_param("metricSpecs", "used_memory"))
        .and(query_param("metricSpecs", "total_requests"))
        .respond_with(success_response(database_metrics_response()))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudMetricsHandler::new(client);

    let result = handler
        .database(
            12345,
            123,
            vec!["used_memory".to_string(), "total_requests".to_string()],
            None,
            None,
        )
        .await;

    assert!(result.is_ok());
    let metrics = result.unwrap();
    assert_eq!(metrics.measurements.len(), 2);

    let memory_metric = &metrics.measurements[0];
    assert_eq!(memory_metric.name, "used_memory");
    assert_eq!(memory_metric.values.len(), 2);

    let requests_metric = &metrics.measurements[1];
    assert_eq!(requests_metric.name, "total_requests");
    assert_eq!(requests_metric.values.len(), 2);
}

#[tokio::test]
async fn test_database_metrics_with_time_range() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/12345/databases/123/metrics"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(query_param("metricSpecs", "used_memory"))
        .and(query_param("from", "2023-01-01T00:00:00Z"))
        .and(query_param("to", "2023-01-01T23:59:59Z"))
        .respond_with(success_response(database_metrics_response()))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudMetricsHandler::new(client);

    let result = handler
        .database(
            12345,
            123,
            vec!["used_memory".to_string()],
            Some("2023-01-01T00:00:00Z".to_string()),
            Some("2023-01-01T23:59:59Z".to_string()),
        )
        .await;

    assert!(result.is_ok());
    let metrics = result.unwrap();
    assert_eq!(metrics.measurements.len(), 2);
}

#[tokio::test]
async fn test_database_metrics_with_from_only() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/12345/databases/123/metrics"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(query_param("metricSpecs", "used_memory"))
        .and(query_param("from", "2023-01-01T00:00:00Z"))
        .respond_with(success_response(database_metrics_response()))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudMetricsHandler::new(client);

    let result = handler
        .database(
            12345,
            123,
            vec!["used_memory".to_string()],
            Some("2023-01-01T00:00:00Z".to_string()),
            None,
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_database_metrics_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/12345/databases/999/metrics"))
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
    let handler = CloudMetricsHandler::new(client);

    let result = handler
        .database(12345, 999, vec!["used_memory".to_string()], None, None)
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_subscription_metrics_basic() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/12345/metrics"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(subscription_metrics_response()))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudMetricsHandler::new(client);

    let result = handler.subscription(12345, None, None).await;

    assert!(result.is_ok());
    let metrics = result.unwrap();
    let metrics = json!({"subscriptionMetrics": metrics});
    assert_eq!(metrics["subscriptionMetrics"]["subscriptionId"], 12345);
    assert_eq!(
        metrics["subscriptionMetrics"]["totalMemoryUsage"],
        2097152.0
    );
    assert_eq!(metrics["subscriptionMetrics"]["totalRequests"], 5000.0);

    let databases = metrics["subscriptionMetrics"]["databases"]
        .as_array()
        .unwrap();
    assert_eq!(databases.len(), 2);
    assert_eq!(databases[0]["databaseId"], 123);
    assert_eq!(databases[1]["databaseId"], 124);
}

#[tokio::test]
async fn test_subscription_metrics_with_time_range() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/12345/metrics"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(query_param("from", "2023-01-01T00:00:00Z"))
        .and(query_param("to", "2023-01-01T23:59:59Z"))
        .respond_with(success_response(subscription_metrics_response()))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudMetricsHandler::new(client);

    let result = handler
        .subscription(
            12345,
            Some("2023-01-01T00:00:00Z".to_string()),
            Some("2023-01-01T23:59:59Z".to_string()),
        )
        .await;

    assert!(result.is_ok());
    let metrics = result.unwrap();
    let metrics = json!({"subscriptionMetrics": metrics});
    assert_eq!(metrics["subscriptionMetrics"]["subscriptionId"], 12345);
}

#[tokio::test]
async fn test_subscription_metrics_with_from_only() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/12345/metrics"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(query_param("from", "2023-01-01T00:00:00Z"))
        .respond_with(success_response(subscription_metrics_response()))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudMetricsHandler::new(client);

    let result = handler
        .subscription(12345, Some("2023-01-01T00:00:00Z".to_string()), None)
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_subscription_metrics_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/99999/metrics"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            404,
            json!({
                "error": {
                    "type": "RESOURCE_NOT_FOUND",
                    "status": 404,
                    "description": "Subscription not found"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudMetricsHandler::new(client);

    let result = handler.subscription(99999, None, None).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_subscription_metrics_unauthorized() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/12345/metrics"))
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
    let handler = CloudMetricsHandler::new(client);

    let result = handler.subscription(12345, None, None).await;

    assert!(result.is_err());
}
