//! Active-Active (CRDB) database endpoint tests for Redis Cloud

use redis_cloud::{CloudClient, CloudCrdbHandler};
use serde_json::json;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

// Test helper functions
fn success_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(body)
}

fn accepted_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(202).set_body_json(body)
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

#[tokio::test]
async fn test_list_crdb() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/crdb"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "crdbs": [
                {
                    "crdbId": 1001,
                    "name": "global-cache",
                    "status": "active",
                    "protocol": "redis",
                    "memoryLimitInGb": 2.5,
                    "createdTimestamp": "2023-01-01T00:00:00Z",
                    "regions": [
                        {
                            "regionId": 1,
                            "regionName": "us-east-1",
                            "subscriptionId": 100001
                        },
                        {
                            "regionId": 2,
                            "regionName": "eu-west-1",
                            "subscriptionId": 100002
                        }
                    ]
                },
                {
                    "crdbId": 1002,
                    "name": "user-sessions",
                    "status": "active",
                    "protocol": "redis",
                    "memoryLimitInGb": 1.0,
                    "createdTimestamp": "2023-06-01T00:00:00Z",
                    "regions": [
                        {
                            "regionId": 3,
                            "regionName": "ap-southeast-1",
                            "subscriptionId": 100003
                        }
                    ]
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudCrdbHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let crdbs_vec = result.unwrap();
    let response = json!({"crdbs": crdbs_vec});
    let crdbs = response["crdbs"].as_array().unwrap();
    assert_eq!(crdbs.len(), 2);
    assert_eq!(crdbs[0]["name"], "global-cache");
    assert_eq!(crdbs[1]["name"], "user-sessions");
    let regions = crdbs[0]["regions"].as_array().unwrap();
    assert_eq!(regions.len(), 2);
}

#[tokio::test]
async fn test_list_crdb_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/crdb"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
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

    let client = create_test_client(mock_server.uri());
    let handler = CloudCrdbHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_crdb() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/crdb/1001"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "crdb": {
                "crdbId": 1001,
                "name": "global-cache",
                "status": "active",
                "protocol": "redis",
                "memoryLimitInGb": 2.5,
                "throughputMeasurement": {
                    "by": "operations-per-second",
                    "value": 25000
                },
                "createdTimestamp": "2023-01-01T00:00:00Z",
                "updatedTimestamp": "2023-12-01T10:30:00Z",
                "regions": [
                    {
                        "regionId": 1,
                        "regionName": "us-east-1",
                        "subscriptionId": 100001,
                        "endpoint": "redis-1001-us-east-1.redislabs.com:15001",
                        "status": "active"
                    },
                    {
                        "regionId": 2,
                        "regionName": "eu-west-1",
                        "subscriptionId": 100002,
                        "endpoint": "redis-1001-eu-west-1.redislabs.com:15001",
                        "status": "active"
                    }
                ],
                "modules": [
                    {
                        "name": "RedisJSON",
                        "version": "2.4.7"
                    }
                ]
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudCrdbHandler::new(client);
    let result = handler.get(1001).await;

    assert!(result.is_ok());
    let crdb_obj = result.unwrap();
    let response = json!({"crdb": crdb_obj});
    let crdb = &response["crdb"];
    assert_eq!(crdb["crdbId"], 1001);
    assert_eq!(crdb["name"], "global-cache");
    assert_eq!(crdb["status"], "active");
    assert_eq!(crdb["memoryLimitInGb"], 2.5);
    let regions = crdb["regions"].as_array().unwrap();
    assert_eq!(regions.len(), 2);
    let modules = crdb["modules"].as_array().unwrap();
    assert_eq!(modules.len(), 1);
}

#[tokio::test]
async fn test_get_crdb_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/crdb/9999"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            404,
            json!({
                "error": {
                    "type": "NOT_FOUND",
                    "status": 404,
                    "description": "Active-Active database not found"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudCrdbHandler::new(client);
    let result = handler.get(9999).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_crdb() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/crdb"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(accepted_response(json!({
            "taskId": "task_123456",
            "crdb": {
                "crdbId": 1003,
                "name": "new-global-db",
                "status": "pending",
                "protocol": "redis",
                "memoryLimitInGb": 1.0,
                "regions": [
                    {
                        "regionId": 10,
                        "regionName": "us-west-1",
                        "subscriptionId": 100010
                    },
                    {
                        "regionId": 20,
                        "regionName": "eu-central-1",
                        "subscriptionId": 100020
                    }
                ]
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudCrdbHandler::new(client);
    let request = json!({
        "name": "new-global-db",
        "protocol": "redis",
        "memoryLimitInGb": 1.0,
        "regions": [
            {
                "regionName": "us-west-1",
                "subscriptionId": 100010
            },
            {
                "regionName": "eu-central-1",
                "subscriptionId": 100020
            }
        ],
        "modules": [
            {
                "name": "RedisJSON"
            }
        ]
    });
    let result = handler.create(request).await;

    assert!(result.is_ok());
    let crdb_obj = result.unwrap();
    let response = json!({"crdb": crdb_obj});
    let crdb = &response["crdb"];
    assert_eq!(crdb["name"], "new-global-db");
    assert_eq!(crdb["status"], "pending");
    let regions = crdb["regions"].as_array().unwrap();
    assert_eq!(regions.len(), 2);
}

#[tokio::test]
async fn test_update_crdb() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/crdb/1001"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(accepted_response(json!({
            "taskId": "task_789012",
            "crdb": {
                "crdbId": 1001,
                "name": "updated-global-cache",
                "status": "updating",
                "memoryLimitInGb": 5.0,
                "updatedTimestamp": "2023-12-01T12:00:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudCrdbHandler::new(client);
    let request = json!({
        "name": "updated-global-cache",
        "memoryLimitInGb": 5.0
    });
    let result = handler.update(1001, request).await;

    assert!(result.is_ok());
    let crdb_obj = result.unwrap();
    let response = json!({"crdb": crdb_obj});
    let crdb = &response["crdb"];
    assert_eq!(crdb["name"], "updated-global-cache");
    assert_eq!(crdb["status"], "updating");
    assert_eq!(crdb["memoryLimitInGb"], 5.0);
}

#[tokio::test]
async fn test_delete_crdb() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/crdb/1001"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task_delete_123"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudCrdbHandler::new(client);
    let result = handler.delete(1001).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_regions() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/crdb/1001/regions"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "regions": [
                {
                    "regionId": 1,
                    "regionName": "us-east-1",
                    "subscriptionId": 100001,
                    "endpoint": "redis-1001-us-east-1.redislabs.com:15001",
                    "status": "active",
                    "memoryUsedInMb": 1024.5,
                    "memoryLimitInMb": 2560,
                    "numberOfShards": 2
                },
                {
                    "regionId": 2,
                    "regionName": "eu-west-1",
                    "subscriptionId": 100002,
                    "endpoint": "redis-1001-eu-west-1.redislabs.com:15001",
                    "status": "active",
                    "memoryUsedInMb": 850.2,
                    "memoryLimitInMb": 2560,
                    "numberOfShards": 2
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudCrdbHandler::new(client);
    let result = handler.get_regions(1001).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let regions = response["regions"].as_array().unwrap();
    assert_eq!(regions.len(), 2);
    assert_eq!(regions[0]["regionName"], "us-east-1");
    assert_eq!(regions[0]["status"], "active");
    assert_eq!(regions[1]["regionName"], "eu-west-1");
}

#[tokio::test]
async fn test_add_region() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/crdb/1001/regions"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(accepted_response(json!({
            "taskId": "task_add_region_456",
            "region": {
                "regionId": 30,
                "regionName": "ap-northeast-1",
                "subscriptionId": 100030,
                "status": "pending"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudCrdbHandler::new(client);
    let request = json!({
        "regionName": "ap-northeast-1",
        "subscriptionId": 100030
    });
    let result = handler.add_region(1001, request).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response["taskId"].is_string());
    let region = &response["region"];
    assert_eq!(region["regionName"], "ap-northeast-1");
    assert_eq!(region["status"], "pending");
}

#[tokio::test]
async fn test_remove_region() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/crdb/1001/regions/2"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task_remove_region_789"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudCrdbHandler::new(client);
    let result = handler.remove_region(1001, 2).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_tasks() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/crdb/1001/tasks"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "tasks": [
                {
                    "taskId": "task_123456",
                    "commandType": "crdbDatabaseCreateRequest",
                    "status": "received",
                    "description": "Create CRDB database",
                    "timestamp": "2023-12-01T12:00:00Z",
                    "response": {}
                },
                {
                    "taskId": "task_789012",
                    "commandType": "crdbDatabaseUpdateRequest",
                    "status": "processing-completed",
                    "description": "Update CRDB database configuration",
                    "timestamp": "2023-12-01T11:30:00Z",
                    "response": {
                        "resourceId": 1001
                    }
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudCrdbHandler::new(client);
    let result = handler.get_tasks(1001).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let tasks = response["tasks"].as_array().unwrap();
    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0]["status"], "received");
    assert_eq!(tasks[1]["status"], "processing-completed");
    assert_eq!(tasks[0]["commandType"], "crdbDatabaseCreateRequest");
}

#[tokio::test]
async fn test_get_task() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/crdb/1001/tasks/task_123456"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "task": {
                "taskId": "task_123456",
                "commandType": "crdbDatabaseCreateRequest",
                "status": "processing-completed",
                "description": "Create CRDB database",
                "timestamp": "2023-12-01T12:00:00Z",
                "response": {
                    "resourceId": 1003,
                    "error": null
                },
                "progress": {
                    "percentage": 100,
                    "currentStep": "Database created successfully"
                }
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudCrdbHandler::new(client);
    let result = handler.get_task(1001, "task_123456").await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let task = &response["task"];
    assert_eq!(task["taskId"], "task_123456");
    assert_eq!(task["status"], "processing-completed");
    let progress = &task["progress"];
    assert_eq!(progress["percentage"], 100);
}

#[tokio::test]
async fn test_get_metrics() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/crdb/1001/metrics"))
        .and(query_param("metrics", "memory,ops"))
        .and(query_param("period", "1h"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "metrics": {
                "crdbId": 1001,
                "period": "1h",
                "sampleRate": "1m",
                "data": {
                    "memory": [
                        {
                            "timestamp": "2023-12-01T12:00:00Z",
                            "value": 1024.5
                        },
                        {
                            "timestamp": "2023-12-01T12:01:00Z",
                            "value": 1028.2
                        }
                    ],
                    "ops": [
                        {
                            "timestamp": "2023-12-01T12:00:00Z",
                            "value": 15420
                        },
                        {
                            "timestamp": "2023-12-01T12:01:00Z",
                            "value": 15650
                        }
                    ]
                },
                "regionMetrics": [
                    {
                        "regionId": 1,
                        "regionName": "us-east-1",
                        "memory": 512.3,
                        "ops": 7800
                    },
                    {
                        "regionId": 2,
                        "regionName": "eu-west-1",
                        "memory": 512.2,
                        "ops": 7850
                    }
                ]
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudCrdbHandler::new(client);
    let result = handler.get_metrics(1001, "memory,ops", "1h").await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let metrics = &response["metrics"];
    assert_eq!(metrics["crdbId"], 1001);
    assert_eq!(metrics["period"], "1h");
    let data = &metrics["data"];
    assert!(data["memory"].is_array());
    assert!(data["ops"].is_array());
    let region_metrics = metrics["regionMetrics"].as_array().unwrap();
    assert_eq!(region_metrics.len(), 2);
}

#[tokio::test]
async fn test_backup() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/crdb/1001/backup"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(accepted_response(json!({
            "taskId": "task_backup_456789",
            "backup": {
                "backupId": "backup_global_001",
                "status": "initiated",
                "initiatedTimestamp": "2023-12-01T12:00:00Z",
                "estimatedCompletionTime": "2023-12-01T12:30:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudCrdbHandler::new(client);
    let result = handler.backup(1001).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response["taskId"].is_string());
    let backup = &response["backup"];
    assert_eq!(backup["status"], "initiated");
    assert!(backup["estimatedCompletionTime"].is_string());
}

#[tokio::test]
async fn test_import() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/crdb/1001/import"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(accepted_response(json!({
            "taskId": "task_import_987654",
            "import": {
                "importId": "import_001",
                "status": "initiated",
                "sourceType": "redis-rdb",
                "sourceLocation": "s3://backup-bucket/dump.rdb",
                "initiatedTimestamp": "2023-12-01T12:00:00Z",
                "estimatedCompletionTime": "2023-12-01T13:00:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudCrdbHandler::new(client);
    let request = json!({
        "sourceType": "redis-rdb",
        "sourceLocation": "s3://backup-bucket/dump.rdb",
        "importOptions": {
            "flushBeforeImport": false,
            "includeExpiry": true
        }
    });
    let result = handler.import(1001, request).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response["taskId"].is_string());
    let import = &response["import"];
    assert_eq!(import["status"], "initiated");
    assert_eq!(import["sourceType"], "redis-rdb");
    assert_eq!(import["sourceLocation"], "s3://backup-bucket/dump.rdb");
}

#[tokio::test]
async fn test_import_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/crdb/1001/import"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            400,
            json!({
                "error": {
                    "type": "INVALID_REQUEST",
                    "status": 400,
                    "description": "Invalid source location format"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudCrdbHandler::new(client);
    let request = json!({
        "sourceType": "redis-rdb",
        "sourceLocation": "invalid-location"
    });
    let result = handler.import(1001, request).await;

    assert!(result.is_err());
}
