use redis_cloud::client::CloudClient;
use redis_cloud::databases::{
    CreateDatabaseRequest, DatabaseHandler, UpdateDatabaseRequest,
};
use redis_cloud::types::{DataPersistence, EvictionPolicy, Protocol};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_list_databases() {
    let mock_server = MockServer::start().await;

    let response = r#"[
        {
            "databaseId": 51423456,
            "name": "my-database",
            "subscriptionId": 1234,
            "status": "active",
            "protocol": "redis",
            "memoryLimitInGb": 1.0,
            "memoryUsedInMb": 10.5,
            "memoryStorage": "ram",
            "supportOSSClusterApi": false,
            "dataPersistence": "none",
            "dataEvictionPolicy": "allkeys-lru",
            "replication": true,
            "publicEndpoint": "redis-12345.c123.us-east-1-2.ec2.cloud.redislabs.com:12345",
            "privateEndpoint": "redis-12345.internal.c123.us-east-1-2.ec2.cloud.redislabs.com:12345"
        }
    ]"#;

    Mock::given(method("GET"))
        .and(path("/subscriptions/1234/databases"))
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

    let handler = DatabaseHandler::new(client);
    let databases = handler.list(1234).await.unwrap();

    assert_eq!(databases.len(), 1);
    assert_eq!(databases[0].database_id, 51423456);
    assert_eq!(databases[0].name, "my-database");
    assert_eq!(databases[0].subscription_id, 1234);
}

#[tokio::test]
async fn test_get_database() {
    let mock_server = MockServer::start().await;

    let response = r#"{
        "databaseId": 51423456,
        "name": "my-database",
        "subscriptionId": 1234,
        "status": "active",
        "protocol": "redis",
        "memoryLimitInGb": 1.0,
        "memoryUsedInMb": 10.5,
        "memoryStorage": "ram",
        "supportOSSClusterApi": false,
        "dataPersistence": "none",
        "dataEvictionPolicy": "allkeys-lru",
        "replication": true,
        "publicEndpoint": "redis-12345.c123.us-east-1-2.ec2.cloud.redislabs.com:12345",
        "privateEndpoint": "redis-12345.internal.c123.us-east-1-2.ec2.cloud.redislabs.com:12345"
    }"#;

    Mock::given(method("GET"))
        .and(path("/subscriptions/1234/databases/51423456"))
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

    let handler = DatabaseHandler::new(client);
    let database = handler.get(1234, 51423456).await.unwrap();

    assert_eq!(database.database_id, 51423456);
    assert_eq!(database.name, "my-database");
    assert_eq!(database.subscription_id, 1234);
}

#[tokio::test]
async fn test_create_database() {
    let mock_server = MockServer::start().await;

    let response = r#"{
        "databaseId": 51423456,
        "name": "new-database",
        "subscriptionId": 1234,
        "status": "pending",
        "protocol": "redis",
        "memoryLimitInGb": 2.0,
        "memoryStorage": "ram",
        "supportOSSClusterApi": false,
        "dataPersistence": "aof-every-second",
        "dataEvictionPolicy": "no-eviction",
        "replication": true
    }"#;

    Mock::given(method("POST"))
        .and(path("/subscriptions/1234/databases"))
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

    let request = CreateDatabaseRequest::builder()
        .name("new-database")
        .memory_limit_in_gb(2.0)
        .protocol(Protocol::Redis)
        .data_persistence(DataPersistence::AofEverySecond)
        .data_eviction_policy(EvictionPolicy::NoEviction)
        .replication(true)
        .build();

    let handler = DatabaseHandler::new(client);
    let database = handler.create(1234, request).await.unwrap();

    assert_eq!(database.database_id, 51423456);
    assert_eq!(database.name, "new-database");
}

#[tokio::test]
async fn test_update_database() {
    let mock_server = MockServer::start().await;

    let response = r#"{
        "databaseId": 51423456,
        "name": "updated-database",
        "subscriptionId": 1234,
        "status": "active",
        "protocol": "redis",
        "memoryLimitInGb": 4.0,
        "memoryStorage": "ram",
        "supportOSSClusterApi": false,
        "dataPersistence": "aof-every-second",
        "dataEvictionPolicy": "allkeys-lru",
        "replication": true
    }"#;

    Mock::given(method("PUT"))
        .and(path("/subscriptions/1234/databases/51423456"))
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

    let request = UpdateDatabaseRequest::builder()
        .name("updated-database")
        .memory_limit_in_gb(4.0)
        .data_eviction_policy(EvictionPolicy::AllkeysLru)
        .build();

    let handler = DatabaseHandler::new(client);
    let database = handler.update(1234, 51423456, request).await.unwrap();

    assert_eq!(database.database_id, 51423456);
    assert_eq!(database.name, "updated-database");
    assert_eq!(database.memory_limit_in_gb, Some(4.0));
}

#[tokio::test]
async fn test_delete_database() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/subscriptions/1234/databases/51423456"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({})))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret_key("test-secret")
        .base_url(mock_server.uri())
        .build();

    let handler = DatabaseHandler::new(client);
    let result = handler.delete(1234, 51423456).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_flush_database() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/subscriptions/1234/databases/51423456/flush"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({})))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret_key("test-secret")
        .base_url(mock_server.uri())
        .build();

    let handler = DatabaseHandler::new(client);
    let result = handler.flush(1234, 51423456).await;

    assert!(result.is_ok());
}
