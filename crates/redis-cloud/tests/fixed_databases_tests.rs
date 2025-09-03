use redis_cloud::{CloudClient, FixedDatabaseHandler};
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_fixed_subscription_databases() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/fixed/subscriptions/123/databases"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "subscription": {
                "subscriptionId": 123,
                "numberOfDatabases": 3,
                "planType": "fixed"
            },
            "links": [
                {
                    "href": "https://api.redislabs.com/v1/fixed/subscriptions/123/databases/1",
                    "rel": "database",
                    "type": "GET"
                }
            ],
            "accountId": 456
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = FixedDatabaseHandler::new(client);
    let result = handler
        .get_fixed_subscription_databases(123, None, None)
        .await
        .unwrap();

    assert_eq!(result.account_id, Some(456));
    assert!(result.links.is_some());
}

#[tokio::test]
async fn test_create_fixed_database() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/fixed/subscriptions/123/databases"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-create-fixed-db",
            "commandType": "CREATE_FIXED_DATABASE",
            "status": "processing",
            "description": "Creating fixed database",
            "timestamp": "2024-01-01T00:00:00Z",
            "response": {
                "resourceId": 789
            }
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = FixedDatabaseHandler::new(client);
    let request = redis_cloud::fixed_databases::FixedDatabaseCreateRequest {
        name: "fixed-test-database".to_string(),
        subscription_id: None,
        protocol: None,
        memory_limit_in_gb: Some(1.0),
        dataset_size_in_gb: None,
        support_oss_cluster_api: None,
        redis_version: None,
        resp_version: None,
        use_external_endpoint_for_oss_cluster_api: None,
        enable_database_clustering: None,
        number_of_shards: None,
        data_persistence: None,
        data_eviction_policy: Some("noeviction".to_string()),
        replication: Some(true),
        periodic_backup_path: None,
        source_ips: None,
        regex_rules: None,
        replica_of: None,
        replica: None,
        client_ssl_certificate: None,
        client_tls_certificates: None,
        enable_tls: None,
        password: None,
        alerts: None,
        modules: None,
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler.create_fixed_database(123, &request).await.unwrap();
    assert_eq!(result.task_id, Some("task-create-fixed-db".to_string()));
    assert_eq!(
        result.command_type,
        Some("CREATE_FIXED_DATABASE".to_string())
    );
}

#[tokio::test]
async fn test_fixed_database_update() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/fixed/subscriptions/123/databases/456"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-update-fixed-db",
            "commandType": "UPDATE_FIXED_DATABASE",
            "status": "processing",
            "description": "Updating fixed database configuration"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = FixedDatabaseHandler::new(client);
    let request = redis_cloud::fixed_databases::FixedDatabaseUpdateRequest {
        subscription_id: None,
        database_id: None,
        name: Some("updated-fixed-database".to_string()),
        memory_limit_in_gb: Some(2.0),
        dataset_size_in_gb: None,
        data_persistence: None,
        data_eviction_policy: Some("allkeys-lru".to_string()),
        replication: None,
        resp_version: None,
        support_oss_cluster_api: None,
        use_external_endpoint_for_oss_cluster_api: None,
        password: None,
        enable_database_clustering: None,
        number_of_shards: None,
        periodic_backup_path: None,
        source_ips: None,
        regex_rules: None,
        replica_of: None,
        replica: None,
        client_ssl_certificate: None,
        client_tls_certificates: None,
        enable_tls: None,
        enable_default_user: None,
        alerts: None,
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler
        .fixed_database_by_id(123, 456, &request)
        .await
        .unwrap();
    assert_eq!(result.task_id, Some("task-update-fixed-db".to_string()));
}

#[tokio::test]
async fn test_delete_fixed_database() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/fixed/subscriptions/123/databases/456"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-delete-fixed-db",
            "commandType": "DELETE_FIXED_DATABASE",
            "status": "processing",
            "description": "Deleting fixed database"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = FixedDatabaseHandler::new(client);
    let result = handler
        .delete_fixed_database_by_id_1(123, 456)
        .await
        .unwrap();

    assert_eq!(result.task_id, Some("task-delete-fixed-db".to_string()));
    assert_eq!(
        result.command_type,
        Some("DELETE_FIXED_DATABASE".to_string())
    );
}

#[tokio::test]
async fn test_backup_fixed_database() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/fixed/subscriptions/123/databases/456/backup"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-backup-fixed-db",
            "commandType": "BACKUP_FIXED_DATABASE",
            "status": "processing"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = FixedDatabaseHandler::new(client);
    let request = redis_cloud::fixed_databases::FixedDatabaseBackupRequest {
        subscription_id: None,
        database_id: None,
        adhoc_backup_path: None,
        command_type: None,
        extra: serde_json::Value::Null,
    };
    let result = handler.backup_database_1(123, 456, &request).await.unwrap();

    assert_eq!(result.task_id, Some("task-backup-fixed-db".to_string()));
}

#[tokio::test]
async fn test_import_fixed_database() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/fixed/subscriptions/123/databases/456/import"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-import-fixed-db",
            "commandType": "IMPORT_FIXED_DATABASE",
            "status": "processing",
            "description": "Importing data into fixed database"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = FixedDatabaseHandler::new(client);
    let request = redis_cloud::fixed_databases::FixedDatabaseImportRequest {
        source_type: "s3".to_string(),
        import_from_uri: vec!["s3://my-bucket/backup.rdb".to_string()],
        subscription_id: None,
        database_id: None,
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler.import_database_1(123, 456, &request).await.unwrap();
    assert_eq!(result.task_id, Some("task-import-fixed-db".to_string()));
}

#[tokio::test]
async fn test_tag_operations() {
    let mock_server = MockServer::start().await;

    // Test create tag
    Mock::given(method("POST"))
        .and(path("/fixed/subscriptions/123/databases/456/tags"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "key": "environment",
            "value": "production"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = FixedDatabaseHandler::new(client);
    let request = redis_cloud::fixed_databases::DatabaseTagCreateRequest {
        key: "environment".to_string(),
        value: "production".to_string(),
        subscription_id: None,
        database_id: None,
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler.create_tag_1(123, 456, &request).await.unwrap();
    assert!(result.key.is_some());
}

#[tokio::test]
async fn test_error_handling_401() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/fixed/subscriptions/123/databases"))
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

    let handler = FixedDatabaseHandler::new(client);
    let result = handler
        .get_fixed_subscription_databases(123, None, None)
        .await;

    assert!(result.is_err());
    match result {
        Err(redis_cloud::CloudError::AuthenticationFailed { .. }) => {}
        _ => panic!("Expected AuthenticationFailed error"),
    }
}

#[tokio::test]
async fn test_error_handling_403() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/fixed/subscriptions/123/databases/456"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(403).set_body_json(json!({
            "error": "Forbidden: Insufficient permissions"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = FixedDatabaseHandler::new(client);
    let result = handler.delete_fixed_database_by_id_1(123, 456).await;

    assert!(result.is_err());
    match result {
        Err(redis_cloud::CloudError::Forbidden { .. }) => {}
        _ => panic!("Expected Forbidden error"),
    }
}

#[tokio::test]
async fn test_error_handling_404() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/fixed/subscriptions/999/databases/999"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "error": "Database not found"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = FixedDatabaseHandler::new(client);
    let result = handler.get_subscription_database_by_id_1(999, 999).await;

    assert!(result.is_err());
    if let Err(redis_cloud::CloudError::NotFound { message }) = result {
        assert!(message.contains("not found") || message.contains("404"));
    } else {
        panic!("Expected NotFound error");
    }
}

#[tokio::test]
async fn test_error_handling_500() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/fixed/subscriptions/123/databases"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
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

    let handler = FixedDatabaseHandler::new(client);
    let request = redis_cloud::fixed_databases::FixedDatabaseCreateRequest {
        name: "test-database".to_string(),
        subscription_id: None,
        protocol: None,
        memory_limit_in_gb: None,
        dataset_size_in_gb: None,
        support_oss_cluster_api: None,
        redis_version: None,
        resp_version: None,
        use_external_endpoint_for_oss_cluster_api: None,
        enable_database_clustering: None,
        number_of_shards: None,
        data_persistence: None,
        data_eviction_policy: None,
        replication: None,
        periodic_backup_path: None,
        source_ips: None,
        regex_rules: None,
        replica_of: None,
        replica: None,
        client_ssl_certificate: None,
        client_tls_certificates: None,
        enable_tls: None,
        password: None,
        alerts: None,
        modules: None,
        command_type: None,
        extra: serde_json::Value::Null,
    };
    let result = handler.create_fixed_database(123, &request).await;

    assert!(result.is_err());
    match result {
        Err(redis_cloud::CloudError::InternalServerError { .. }) => {}
        _ => panic!("Expected InternalServerError error"),
    }
}
