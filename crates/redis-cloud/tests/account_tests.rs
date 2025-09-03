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
async fn test_get_supported_search_scaling_factors() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/query-performance-factors"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "queryPerformanceFactors": [
                "low",
                "medium",
                "high"
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
        .get_supported_search_scaling_factors()
        .await
        .unwrap();

    assert!(result.query_performance_factors.is_some());
    let factors = result.query_performance_factors.unwrap();
    assert_eq!(factors.len(), 3);
    assert_eq!(factors[0], "low");
    assert_eq!(factors[1], "medium");
    assert_eq!(factors[2], "high");
}

#[tokio::test]
async fn test_get_account_session_logs() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/session-logs"))
        .and(query_param("limit", "10"))
        .and(query_param("offset", "5"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "entries": [
                {
                    "id": "session-123",
                    "time": "2024-01-01T12:00:00Z",
                    "user": "admin@example.com",
                    "userAgent": "Mozilla/5.0",
                    "ipAddress": "192.168.1.1",
                    "userRole": "Admin",
                    "type": "login",
                    "action": "successful_login"
                },
                {
                    "id": "session-124",
                    "time": "2024-01-01T12:05:00Z",
                    "user": "user@example.com",
                    "userAgent": "curl/7.68.0",
                    "ipAddress": "10.0.0.1",
                    "userRole": "Member",
                    "type": "logout",
                    "action": "session_terminated"
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
        .get_account_session_logs(Some(5), Some(10))
        .await
        .unwrap();

    assert!(result.entries.is_some());
    let entries = result.entries.unwrap();
    assert_eq!(entries.len(), 2);

    let first_entry = &entries[0];
    assert_eq!(first_entry.id.as_ref().unwrap(), "session-123");
    assert_eq!(first_entry.user.as_ref().unwrap(), "admin@example.com");
    assert_eq!(first_entry.user_role.as_ref().unwrap(), "Admin");
    assert_eq!(first_entry.r#type.as_ref().unwrap(), "login");

    let second_entry = &entries[1];
    assert_eq!(second_entry.id.as_ref().unwrap(), "session-124");
    assert_eq!(second_entry.user.as_ref().unwrap(), "user@example.com");
    assert_eq!(second_entry.user_role.as_ref().unwrap(), "Member");
    assert_eq!(second_entry.r#type.as_ref().unwrap(), "logout");
}

#[tokio::test]
async fn test_get_account_session_logs_no_params() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/session-logs"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "entries": []
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
    let result = handler.get_account_session_logs(None, None).await.unwrap();

    assert!(result.entries.is_some());
    let entries = result.entries.unwrap();
    assert_eq!(entries.len(), 0);
}

#[tokio::test]
async fn test_get_supported_regions_no_provider() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/regions"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "regions": [
                {
                    "name": "us-east-1",
                    "provider": "AWS"
                },
                {
                    "name": "us-central1",
                    "provider": "GCP"
                },
                {
                    "name": "East US",
                    "provider": "Azure"
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
    let result = handler.get_supported_regions(None).await.unwrap();

    assert!(result.regions.is_some());
    let regions = result.regions.unwrap();
    assert_eq!(regions.len(), 3);

    let providers: Vec<String> = regions
        .iter()
        .map(|r| r.provider.clone().unwrap_or_default())
        .collect();
    assert!(providers.contains(&"AWS".to_string()));
    assert!(providers.contains(&"GCP".to_string()));
    assert!(providers.contains(&"Azure".to_string()));
}

#[tokio::test]
async fn test_get_account_system_logs_no_params() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/logs"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "entries": [
                {
                    "id": 100,
                    "time": "2024-01-01T10:00:00Z",
                    "originator": "API",
                    "apiKeyName": "production-api-key",
                    "resource": "subscription/123",
                    "type": "create",
                    "description": "Subscription created successfully"
                },
                {
                    "id": 101,
                    "time": "2024-01-01T10:05:00Z",
                    "originator": "User",
                    "resource": "database/456",
                    "type": "delete",
                    "description": "Database deleted"
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
    let result = handler.get_account_system_logs(None, None).await.unwrap();

    assert!(result.entries.is_some());
    let entries = result.entries.unwrap();
    assert_eq!(entries.len(), 2);

    let first_entry = &entries[0];
    assert_eq!(first_entry.id.unwrap(), 100);
    assert_eq!(first_entry.originator.as_ref().unwrap(), "API");
    assert_eq!(
        first_entry.api_key_name.as_ref().unwrap(),
        "production-api-key"
    );
    assert_eq!(first_entry.resource.as_ref().unwrap(), "subscription/123");
    assert_eq!(first_entry.r#type.as_ref().unwrap(), "create");

    let second_entry = &entries[1];
    assert_eq!(second_entry.id.unwrap(), 101);
    assert_eq!(second_entry.originator.as_ref().unwrap(), "User");
    assert_eq!(second_entry.resource.as_ref().unwrap(), "database/456");
    assert_eq!(second_entry.r#type.as_ref().unwrap(), "delete");
}

#[tokio::test]
async fn test_get_supported_database_modules_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/database-modules"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "modules": []
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
    let modules = result.modules.unwrap();
    assert_eq!(modules.len(), 0);
}

#[tokio::test]
async fn test_get_data_persistence_options_with_links() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/data-persistence"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "dataPersistence": [
                {
                    "name": "aof-every-1-sec",
                    "description": "Append only file (AOF) - fsync every 1 second"
                }
            ],
            "links": [
                {
                    "rel": "self",
                    "href": "https://api.redislabs.com/v1/data-persistence",
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
    let result = handler.get_data_persistence_options().await.unwrap();

    assert!(result.data_persistence.is_some());
    assert!(result.links.is_some());
    let persistence_options = result.data_persistence.unwrap();
    assert_eq!(persistence_options.len(), 1);
    assert_eq!(
        persistence_options[0].name.as_ref().unwrap(),
        "aof-every-1-sec"
    );

    let links = result.links.unwrap();
    assert_eq!(links.len(), 1);
}

// Error handling tests

#[tokio::test]
async fn test_error_handling_401() {
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

#[tokio::test]
async fn test_error_handling_404() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/regions"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "error": "Resource not found"
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
    let result = handler.get_supported_regions(None).await;

    assert!(result.is_err());
    match result {
        Err(redis_cloud::CloudError::NotFound { .. }) => {}
        _ => panic!("Expected NotFound error"),
    }
}

#[tokio::test]
async fn test_error_handling_500() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/payment-methods"))
        .respond_with(ResponseTemplate::new(500).set_body_json(json!({
            "error": "Internal server error"
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
    let result = handler.get_account_payment_methods().await;

    assert!(result.is_err());
    match result {
        Err(redis_cloud::CloudError::InternalServerError { .. }) => {}
        _ => panic!("Expected InternalServerError error"),
    }
}

#[tokio::test]
async fn test_current_account_with_extra_fields() {
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
            ],
            "accountId": 12345,
            "accountName": "Test Account",
            "customField": "custom value"
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
    // Test that extra fields are captured in the flattened extra field
    assert!(result.extra.get("accountId").is_some());
    assert!(result.extra.get("accountName").is_some());
    assert!(result.extra.get("customField").is_some());

    if let Some(account_id) = result.extra.get("accountId") {
        assert_eq!(account_id.as_i64().unwrap(), 12345);
    }

    if let Some(account_name) = result.extra.get("accountName") {
        assert_eq!(account_name.as_str().unwrap(), "Test Account");
    }
}
