use redis_cloud::{AccountHandler, CloudClient};
use serde_json::json;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_current_account() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "links": [
                {
                    "rel": "self",
                    "href": "https://api.redislabs.com/v1/",
                    "type": "GET"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = AccountHandler::new(client);
    let result = handler.get_current_account().await.unwrap();

    assert!(result.links.is_some());
}

#[tokio::test]
async fn test_get_data_persistence_options() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/data-persistence"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "dataPersistence": [
                {
                    "name": "none",
                    "description": "None"
                },
                {
                    "name": "aof-every-1-sec",
                    "description": "Append only file (AOF) - fsync every 1 second"
                },
                {
                    "name": "aof-every-write",
                    "description": "Append only file (AOF) - fsync every write"
                },
                {
                    "name": "snapshot-every-1-hour",
                    "description": "Snapshot every 1 hour"
                },
                {
                    "name": "snapshot-every-6-hours",
                    "description": "Snapshot every 6 hours"
                },
                {
                    "name": "snapshot-every-12-hours",
                    "description": "Snapshot every 12 hours"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = AccountHandler::new(client);
    let result = handler.get_data_persistence_options().await.unwrap();

    assert!(result.data_persistence.is_some());
}

#[tokio::test]
async fn test_get_supported_database_modules() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/database-modules"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "modules": [
                {
                    "module": "RediSearch",
                    "moduleName": "RediSearch",
                    "displayName": "Search and Query",
                    "description": "Full-text search and secondary indexing",
                    "parameters": []
                },
                {
                    "module": "RedisGraph",
                    "moduleName": "RedisGraph",
                    "displayName": "Graph",
                    "description": "Graph database",
                    "parameters": []
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = AccountHandler::new(client);
    let result = handler.get_supported_database_modules().await.unwrap();

    assert!(result.modules.is_some());
}

#[tokio::test]
async fn test_get_supported_regions() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/regions"))
        .and(query_param("provider", "AWS"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "regions": [
                {
                    "name": "us-east-1",
                    "provider": "AWS"
                },
                {
                    "name": "us-west-2",
                    "provider": "AWS"
                },
                {
                    "name": "eu-west-1",
                    "provider": "AWS"
                },
                {
                    "name": "ap-southeast-1",
                    "provider": "AWS"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = AccountHandler::new(client);
    let result = handler
        .get_supported_regions(Some("AWS".to_string()))
        .await
        .unwrap();

    assert!(result.regions.is_some());
}

#[tokio::test]
async fn test_get_account_payment_methods() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/payment-methods"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "accountId": 123
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = AccountHandler::new(client);
    let result = handler.get_account_payment_methods().await.unwrap();

    assert!(result.account_id.is_some());
}

#[tokio::test]
async fn test_get_account_system_logs() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/logs"))
        .and(query_param("limit", "20"))
        .and(query_param("offset", "0"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "entries": [
                {
                    "id": 1,
                    "time": "2024-01-01T00:00:00Z",
                    "originator": "System",
                    "type": "info",
                    "description": "Test log entry"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = AccountHandler::new(client);
    let result = handler
        .get_account_system_logs(Some(0), Some(20))
        .await
        .unwrap();

    assert!(result.entries.is_some());
    let entries = result.entries.unwrap();
    assert_eq!(entries.len(), 1);
}

#[tokio::test]
async fn test_error_handling() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/"))
        .respond_with(ResponseTemplate::new(401).set_body_json(json!({
            "error": "Invalid API credentials"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("wrong-key".to_string())
        .api_secret("wrong-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = AccountHandler::new(client);
    let result = handler.get_current_account().await;

    assert!(result.is_err());
    match result {
        Err(redis_cloud::CloudError::AuthenticationFailed { .. }) => {}
        _ => panic!("Expected AuthenticationFailed error"),
    }
}
