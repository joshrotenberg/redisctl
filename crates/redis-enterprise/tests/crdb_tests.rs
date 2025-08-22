//! Active-Active (CRDB) endpoint tests for Redis Enterprise

use redis_enterprise::{CrdbHandler, CreateCrdbInstance, CreateCrdbRequest, EnterpriseClient};
use serde_json::json;
use wiremock::matchers::{basic_auth, body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// Test helper functions
fn success_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(body)
}

fn created_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(201).set_body_json(body)
}

fn error_response(code: u16, message: &str) -> ResponseTemplate {
    ResponseTemplate::new(code).set_body_json(json!({
        "error": message,
        "code": code
    }))
}

fn no_content_response() -> ResponseTemplate {
    ResponseTemplate::new(204)
}

fn test_crdb_full() -> serde_json::Value {
    json!({
        "guid": "12345-abcdef-67890",
        "name": "production-active-active",
        "status": "active",
        "memory_size": 2147483648u64,
        "encryption": true,
        "data_persistence": "aof",
        "replication": true,
        "eviction_policy": "allkeys-lru",
        "instances": [
            {
                "id": 1,
                "cluster": "cluster1.example.com:9443",
                "cluster_name": "prod-cluster-1",
                "status": "active",
                "endpoints": ["10.0.1.100:12000", "10.0.1.101:12000"]
            },
            {
                "id": 2,
                "cluster": "cluster2.example.com:9443",
                "cluster_name": "prod-cluster-2",
                "status": "active",
                "endpoints": ["10.0.2.100:12000", "10.0.2.101:12000"]
            }
        ]
    })
}

fn test_crdb_simple() -> serde_json::Value {
    json!({
        "guid": "simple-guid-123",
        "name": "simple-crdb",
        "status": "active",
        "memory_size": 1073741824u64,
        "instances": [
            {
                "id": 1,
                "cluster": "cluster1.local",
                "status": "active"
            }
        ]
    })
}

fn test_crdb_tasks_data() -> serde_json::Value {
    json!([
        {
            "task_id": "task-123",
            "type": "cluster_connect",
            "status": "completed",
            "start_time": "2023-01-01T12:00:00Z",
            "end_time": "2023-01-01T12:01:00Z"
        },
        {
            "task_id": "task-456",
            "type": "sync_status",
            "status": "running",
            "start_time": "2023-01-01T12:02:00Z"
        }
    ])
}

#[tokio::test]
async fn test_crdbs_list() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/crdbs"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            test_crdb_full(),
            test_crdb_simple()
        ])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = CrdbHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let crdbs = result.unwrap();
    assert_eq!(crdbs.len(), 2);
    assert_eq!(crdbs[0].guid, "12345-abcdef-67890");
    assert_eq!(crdbs[0].name, "production-active-active");
    assert_eq!(crdbs[0].instances.len(), 2);
    assert_eq!(crdbs[1].guid, "simple-guid-123");
    assert_eq!(crdbs[1].instances.len(), 1);
}

#[tokio::test]
async fn test_crdbs_list_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/crdbs"))
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

    let handler = CrdbHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let crdbs = result.unwrap();
    assert_eq!(crdbs.len(), 0);
}

#[tokio::test]
async fn test_crdb_get() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/crdbs/12345-abcdef-67890"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_crdb_full()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = CrdbHandler::new(client);
    let result = handler.get("12345-abcdef-67890").await;

    assert!(result.is_ok());
    let crdb = result.unwrap();
    assert_eq!(crdb.guid, "12345-abcdef-67890");
    assert_eq!(crdb.name, "production-active-active");
    assert_eq!(crdb.status, "active");
    assert_eq!(crdb.memory_size, 2147483648u64);
    assert!(crdb.encryption.unwrap());
    assert_eq!(crdb.data_persistence.unwrap(), "aof");
    assert!(crdb.replication.unwrap());
    assert_eq!(crdb.eviction_policy.unwrap(), "allkeys-lru");
    assert_eq!(crdb.instances.len(), 2);
    assert_eq!(crdb.instances[0].id, 1);
    assert_eq!(crdb.instances[0].cluster, "cluster1.example.com:9443");
    assert_eq!(
        crdb.instances[0].cluster_name.as_ref().unwrap(),
        "prod-cluster-1"
    );
}

#[tokio::test]
async fn test_crdb_get_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/crdbs/nonexistent-guid"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(404, "CRDB not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = CrdbHandler::new(client);
    let result = handler.get("nonexistent-guid").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_crdb_create_simple() {
    let mock_server = MockServer::start().await;

    let request = CreateCrdbRequest {
        name: "test-crdb".to_string(),
        memory_size: 1073741824,
        instances: vec![CreateCrdbInstance {
            cluster: "cluster1.local:9443".to_string(),
            cluster_url: Some("https://cluster1.local:9443".to_string()),
            username: Some("admin".to_string()),
            password: Some("secret".to_string()),
        }],
        encryption: Some(false),
        data_persistence: Some("rdb".to_string()),
        eviction_policy: Some("noeviction".to_string()),
    };

    Mock::given(method("POST"))
        .and(path("/v1/crdbs"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&request))
        .respond_with(created_response(json!({
            "guid": "new-crdb-guid-123",
            "name": "test-crdb",
            "status": "active",
            "memory_size": 1073741824,
            "encryption": false,
            "data_persistence": "rdb",
            "eviction_policy": "noeviction",
            "instances": [
                {
                    "id": 1,
                    "cluster": "cluster1.local:9443",
                    "status": "active"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = CrdbHandler::new(client);
    let result = handler.create(request).await;

    assert!(result.is_ok());
    let crdb = result.unwrap();
    assert_eq!(crdb.guid, "new-crdb-guid-123");
    assert_eq!(crdb.name, "test-crdb");
    assert_eq!(crdb.memory_size, 1073741824);
    assert!(!crdb.encryption.unwrap());
    assert_eq!(crdb.data_persistence.unwrap(), "rdb");
    assert_eq!(crdb.eviction_policy.unwrap(), "noeviction");
}

#[tokio::test]
async fn test_crdb_create_multi_instance() {
    let mock_server = MockServer::start().await;

    let request = CreateCrdbRequest {
        name: "multi-region-crdb".to_string(),
        memory_size: 4294967296u64,
        instances: vec![
            CreateCrdbInstance {
                cluster: "us-east.example.com:9443".to_string(),
                cluster_url: None,
                username: Some("admin".to_string()),
                password: Some("secret1".to_string()),
            },
            CreateCrdbInstance {
                cluster: "eu-west.example.com:9443".to_string(),
                cluster_url: None,
                username: Some("admin".to_string()),
                password: Some("secret2".to_string()),
            },
        ],
        encryption: Some(true),
        data_persistence: Some("aof".to_string()),
        eviction_policy: Some("allkeys-lru".to_string()),
    };

    Mock::given(method("POST"))
        .and(path("/v1/crdbs"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&request))
        .respond_with(created_response(json!({
            "guid": "multi-guid-456",
            "name": "multi-region-crdb",
            "status": "active",
            "memory_size": 4294967296u64,
            "encryption": true,
            "data_persistence": "aof",
            "eviction_policy": "allkeys-lru",
            "instances": [
                {
                    "id": 1,
                    "cluster": "us-east.example.com:9443",
                    "status": "active"
                },
                {
                    "id": 2,
                    "cluster": "eu-west.example.com:9443",
                    "status": "active"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = CrdbHandler::new(client);
    let result = handler.create(request).await;

    assert!(result.is_ok());
    let crdb = result.unwrap();
    assert_eq!(crdb.guid, "multi-guid-456");
    assert_eq!(crdb.name, "multi-region-crdb");
    assert_eq!(crdb.instances.len(), 2);
}

#[tokio::test]
async fn test_crdb_create_invalid() {
    let mock_server = MockServer::start().await;

    let request = CreateCrdbRequest {
        name: "".to_string(), // Invalid empty name
        memory_size: 0,       // Invalid memory size
        instances: vec![],
        encryption: None,
        data_persistence: None,
        eviction_policy: None,
    };

    Mock::given(method("POST"))
        .and(path("/v1/crdbs"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&request))
        .respond_with(error_response(400, "Invalid CRDB configuration"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = CrdbHandler::new(client);
    let result = handler.create(request).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_crdb_update() {
    let mock_server = MockServer::start().await;

    let updates = json!({
        "memory_size": 8589934592u64,
        "eviction_policy": "volatile-lru"
    });

    Mock::given(method("PUT"))
        .and(path("/v1/crdbs/12345-abcdef-67890"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&updates))
        .respond_with(success_response(json!({
            "guid": "12345-abcdef-67890",
            "name": "production-active-active",
            "status": "active",
            "memory_size": 8589934592u64,
            "encryption": true,
            "data_persistence": "aof",
            "replication": true,
            "eviction_policy": "volatile-lru",
            "instances": []
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = CrdbHandler::new(client);
    let result = handler.update("12345-abcdef-67890", updates).await;

    assert!(result.is_ok());
    let crdb = result.unwrap();
    assert_eq!(crdb.guid, "12345-abcdef-67890");
    assert_eq!(crdb.memory_size, 8589934592u64);
    assert_eq!(crdb.eviction_policy.unwrap(), "volatile-lru");
}

#[tokio::test]
async fn test_crdb_update_nonexistent() {
    let mock_server = MockServer::start().await;

    let updates = json!({
        "memory_size": 2147483648u64
    });

    Mock::given(method("PUT"))
        .and(path("/v1/crdbs/nonexistent-guid"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&updates))
        .respond_with(error_response(404, "CRDB not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = CrdbHandler::new(client);
    let result = handler.update("nonexistent-guid", updates).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_crdb_delete() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/crdbs/12345-abcdef-67890"))
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

    let handler = CrdbHandler::new(client);
    let result = handler.delete("12345-abcdef-67890").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_crdb_delete_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/crdbs/nonexistent-guid"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(404, "CRDB not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = CrdbHandler::new(client);
    let result = handler.delete("nonexistent-guid").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_crdb_tasks() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/crdbs/12345-abcdef-67890/tasks"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_crdb_tasks_data()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = CrdbHandler::new(client);
    let result = handler.tasks("12345-abcdef-67890").await;

    assert!(result.is_ok());
    let tasks = result.unwrap();
    assert!(tasks.is_array());
    let tasks_array = tasks.as_array().unwrap();
    assert_eq!(tasks_array.len(), 2);
    assert_eq!(tasks_array[0]["task_id"], "task-123");
    assert_eq!(tasks_array[0]["status"], "completed");
    assert_eq!(tasks_array[1]["task_id"], "task-456");
    assert_eq!(tasks_array[1]["status"], "running");
}

#[tokio::test]
async fn test_crdb_tasks_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/crdbs/simple-guid-123/tasks"))
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

    let handler = CrdbHandler::new(client);
    let result = handler.tasks("simple-guid-123").await;

    assert!(result.is_ok());
    let tasks = result.unwrap();
    assert!(tasks.is_array());
    let tasks_array = tasks.as_array().unwrap();
    assert_eq!(tasks_array.len(), 0);
}

#[tokio::test]
async fn test_crdb_tasks_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/crdbs/nonexistent-guid/tasks"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(404, "CRDB not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = CrdbHandler::new(client);
    let result = handler.tasks("nonexistent-guid").await;

    assert!(result.is_err());
}
