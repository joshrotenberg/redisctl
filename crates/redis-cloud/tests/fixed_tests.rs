//! Fixed (Essentials) subscription endpoint tests for Redis Cloud

use redis_cloud::{CloudClient, CloudConfig, CloudFixedHandler};
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
    let config = CloudConfig {
        api_key: "test-api-key".to_string(),
        api_secret: "test-secret-key".to_string(),
        base_url,
        timeout: std::time::Duration::from_secs(30),
    };
    CloudClient::new(config).unwrap()
}

#[tokio::test]
async fn test_list_fixed_subscriptions() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/fixed/subscriptions"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "subscriptions": [
                {
                    "id": 1001,
                    "name": "Essential Development",
                    "status": "active",
                    "planId": 101,
                    "planName": "Redis Cloud Essentials 30MB",
                    "provider": "AWS",
                    "region": "us-east-1",
                    "createdTimestamp": "2023-01-01T00:00:00Z",
                    "activatedTimestamp": "2023-01-01T01:00:00Z",
                    "pricing": {
                        "type": "fixed",
                        "currency": "USD",
                        "monthlyPrice": 5.0
                    },
                    "databases": [
                        {
                            "databaseId": 2001,
                            "name": "dev-cache",
                            "status": "active"
                        }
                    ]
                },
                {
                    "id": 1002,
                    "name": "Essential Testing",
                    "status": "active",
                    "planId": 102,
                    "planName": "Redis Cloud Essentials 100MB",
                    "provider": "GCP",
                    "region": "us-central1",
                    "createdTimestamp": "2023-06-01T00:00:00Z",
                    "activatedTimestamp": "2023-06-01T02:30:00Z",
                    "pricing": {
                        "type": "fixed",
                        "currency": "USD",
                        "monthlyPrice": 15.0
                    },
                    "databases": [
                        {
                            "databaseId": 2002,
                            "name": "test-session-store",
                            "status": "active"
                        }
                    ]
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudFixedHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let subscriptions = response["subscriptions"].as_array().unwrap();
    assert_eq!(subscriptions.len(), 2);
    assert_eq!(subscriptions[0]["name"], "Essential Development");
    assert_eq!(subscriptions[0]["planName"], "Redis Cloud Essentials 30MB");
    assert_eq!(subscriptions[0]["pricing"]["monthlyPrice"], 5.0);
    assert_eq!(subscriptions[1]["name"], "Essential Testing");
    assert_eq!(subscriptions[1]["pricing"]["monthlyPrice"], 15.0);
}

#[tokio::test]
async fn test_list_fixed_subscriptions_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/fixed/subscriptions"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "subscriptions": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudFixedHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let subscriptions = response["subscriptions"].as_array().unwrap();
    assert_eq!(subscriptions.len(), 0);
}

#[tokio::test]
async fn test_list_fixed_subscriptions_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/fixed/subscriptions"))
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
    let handler = CloudFixedHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_fixed_subscription() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/fixed/subscriptions/1001"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "subscription": {
                "id": 1001,
                "name": "Essential Development",
                "status": "active",
                "planId": 101,
                "planName": "Redis Cloud Essentials 30MB",
                "provider": "AWS",
                "region": "us-east-1",
                "cloudAccountId": 9001,
                "createdTimestamp": "2023-01-01T00:00:00Z",
                "activatedTimestamp": "2023-01-01T01:00:00Z",
                "updatedTimestamp": "2023-12-01T10:00:00Z",
                "pricing": {
                    "type": "fixed",
                    "currency": "USD",
                    "monthlyPrice": 5.0,
                    "nextBillingDate": "2024-01-01T00:00:00Z"
                },
                "limits": {
                    "memoryLimitInMb": 30,
                    "connectionLimit": 30,
                    "opsPerSecond": 1000,
                    "bandwidthLimitMbps": 1
                },
                "databases": [
                    {
                        "databaseId": 2001,
                        "name": "dev-cache",
                        "status": "active",
                        "publicEndpoint": "redis-12001.c1.us-east-1.cache.redislabs.com:12001",
                        "privateEndpoint": null
                    }
                ]
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudFixedHandler::new(client);
    let result = handler.get(1001).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let subscription = &response["subscription"];
    assert_eq!(subscription["id"], 1001);
    assert_eq!(subscription["name"], "Essential Development");
    assert_eq!(subscription["status"], "active");
    assert_eq!(subscription["pricing"]["monthlyPrice"], 5.0);
    let limits = &subscription["limits"];
    assert_eq!(limits["memoryLimitInMb"], 30);
    assert_eq!(limits["connectionLimit"], 30);
    let databases = subscription["databases"].as_array().unwrap();
    assert_eq!(databases.len(), 1);
    assert_eq!(databases[0]["name"], "dev-cache");
}

#[tokio::test]
async fn test_get_fixed_subscription_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/fixed/subscriptions/9999"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            404,
            json!({
                "error": {
                    "type": "NOT_FOUND",
                    "status": 404,
                    "description": "Fixed subscription not found"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudFixedHandler::new(client);
    let result = handler.get(9999).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_fixed_subscription_databases() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/fixed/subscriptions/1001/databases"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "databases": [
                {
                    "databaseId": 2001,
                    "name": "dev-cache",
                    "protocol": "redis",
                    "status": "active",
                    "publicEndpoint": "redis-12001.c1.us-east-1.cache.redislabs.com:12001",
                    "privateEndpoint": null,
                    "password": "mypassword123",
                    "memoryLimitInMb": 30,
                    "memoryUsageInMb": 12.5,
                    "supportOSSClusterApi": false,
                    "replication": false,
                    "dataPersistence": "none",
                    "backup": false,
                    "alerts": {
                        "enabled": true,
                        "thresholds": {
                            "memoryUsage": 80,
                            "connections": 90
                        }
                    },
                    "security": {
                        "sslClientAuthentication": false,
                        "sourceIps": []
                    },
                    "modules": [],
                    "createdTimestamp": "2023-01-01T01:00:00Z",
                    "lastModifiedTimestamp": "2023-12-01T10:00:00Z"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudFixedHandler::new(client);
    let result = handler.databases(1001).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let databases = response["databases"].as_array().unwrap();
    assert_eq!(databases.len(), 1);
    let database = &databases[0];
    assert_eq!(database["databaseId"], 2001);
    assert_eq!(database["name"], "dev-cache");
    assert_eq!(database["status"], "active");
    assert_eq!(database["memoryLimitInMb"], 30);
    assert_eq!(database["memoryUsageInMb"], 12.5);
    assert_eq!(database["replication"], false);
    assert_eq!(database["dataPersistence"], "none");
    let alerts = &database["alerts"];
    assert_eq!(alerts["enabled"], true);
    let security = &database["security"];
    assert_eq!(security["sslClientAuthentication"], false);
}

#[tokio::test]
async fn test_get_fixed_subscription_database() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/fixed/subscriptions/1001/databases/2001"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "database": {
                "databaseId": 2001,
                "name": "dev-cache",
                "protocol": "redis",
                "status": "active",
                "publicEndpoint": "redis-12001.c1.us-east-1.cache.redislabs.com:12001",
                "privateEndpoint": null,
                "password": "mypassword123",
                "memoryLimitInMb": 30,
                "memoryUsageInMb": 12.5,
                "supportOSSClusterApi": false,
                "replication": false,
                "dataPersistence": "none",
                "backup": false,
                "alerts": {
                    "enabled": true,
                    "thresholds": {
                        "memoryUsage": 80,
                        "connections": 90
                    }
                },
                "security": {
                    "sslClientAuthentication": false,
                    "sourceIps": [],
                    "clientSslCertificate": null
                },
                "modules": [],
                "createdTimestamp": "2023-01-01T01:00:00Z",
                "lastModifiedTimestamp": "2023-12-01T10:00:00Z",
                "activatedTimestamp": "2023-01-01T01:15:00Z",
                "metrics": {
                    "opsPerSecond": 245,
                    "hitRatio": 85.2,
                    "keyCount": 15420,
                    "connectedClients": 3
                }
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudFixedHandler::new(client);
    let result = handler.database(1001, 2001).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let database = &response["database"];
    assert_eq!(database["databaseId"], 2001);
    assert_eq!(database["name"], "dev-cache");
    assert_eq!(database["status"], "active");
    assert_eq!(database["memoryLimitInMb"], 30);
    assert_eq!(database["memoryUsageInMb"], 12.5);
    assert_eq!(database["backup"], false);
    let metrics = &database["metrics"];
    assert_eq!(metrics["opsPerSecond"], 245);
    assert_eq!(metrics["hitRatio"], 85.2);
    assert_eq!(metrics["keyCount"], 15420);
    assert_eq!(metrics["connectedClients"], 3);
}

#[tokio::test]
async fn test_get_fixed_subscription_database_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/fixed/subscriptions/1001/databases/9999"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            404,
            json!({
                "error": {
                    "type": "NOT_FOUND",
                    "status": 404,
                    "description": "Database not found in fixed subscription"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudFixedHandler::new(client);
    let result = handler.database(1001, 9999).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_fixed_plans() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/fixed/plans"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "plans": [
                {
                    "id": 101,
                    "name": "Redis Cloud Essentials 30MB",
                    "displayName": "Essential 30MB",
                    "description": "Perfect for development and testing",
                    "provider": "AWS",
                    "region": "us-east-1",
                    "pricing": {
                        "type": "fixed",
                        "currency": "USD",
                        "monthlyPrice": 5.0,
                        "hourlyPrice": null
                    },
                    "specifications": {
                        "memoryLimitMb": 30,
                        "connectionLimit": 30,
                        "opsPerSecond": 1000,
                        "bandwidthMbps": 1,
                        "replication": false,
                        "clustering": false,
                        "persistence": false,
                        "backup": false
                    },
                    "features": [
                        "Redis OSS 7.2",
                        "Standard support",
                        "TLS encryption"
                    ],
                    "available": true
                },
                {
                    "id": 102,
                    "name": "Redis Cloud Essentials 100MB",
                    "displayName": "Essential 100MB",
                    "description": "Ideal for small production workloads",
                    "provider": "GCP",
                    "region": "us-central1",
                    "pricing": {
                        "type": "fixed",
                        "currency": "USD",
                        "monthlyPrice": 15.0,
                        "hourlyPrice": null
                    },
                    "specifications": {
                        "memoryLimitMb": 100,
                        "connectionLimit": 100,
                        "opsPerSecond": 10000,
                        "bandwidthMbps": 5,
                        "replication": false,
                        "clustering": false,
                        "persistence": true,
                        "backup": true
                    },
                    "features": [
                        "Redis OSS 7.2",
                        "Standard support",
                        "TLS encryption",
                        "Data persistence",
                        "Daily backups"
                    ],
                    "available": true
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudFixedHandler::new(client);
    let result = handler.plans().await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let plans = response["plans"].as_array().unwrap();
    assert_eq!(plans.len(), 2);

    let plan1 = &plans[0];
    assert_eq!(plan1["id"], 101);
    assert_eq!(plan1["name"], "Redis Cloud Essentials 30MB");
    assert_eq!(plan1["pricing"]["monthlyPrice"], 5.0);
    assert_eq!(plan1["specifications"]["memoryLimitMb"], 30);
    assert_eq!(plan1["specifications"]["replication"], false);
    assert_eq!(plan1["specifications"]["backup"], false);

    let plan2 = &plans[1];
    assert_eq!(plan2["id"], 102);
    assert_eq!(plan2["pricing"]["monthlyPrice"], 15.0);
    assert_eq!(plan2["specifications"]["memoryLimitMb"], 100);
    assert_eq!(plan2["specifications"]["persistence"], true);
    assert_eq!(plan2["specifications"]["backup"], true);

    let features1 = plan1["features"].as_array().unwrap();
    assert_eq!(features1.len(), 3);
    assert!(features1.contains(&json!("Redis OSS 7.2")));

    let features2 = plan2["features"].as_array().unwrap();
    assert_eq!(features2.len(), 5);
    assert!(features2.contains(&json!("Daily backups")));
}

#[tokio::test]
async fn test_get_fixed_plan() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/fixed/plans/101"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "plan": {
                "id": 101,
                "name": "Redis Cloud Essentials 30MB",
                "displayName": "Essential 30MB",
                "description": "Perfect for development and testing with Redis OSS compatibility",
                "provider": "AWS",
                "region": "us-east-1",
                "pricing": {
                    "type": "fixed",
                    "currency": "USD",
                    "monthlyPrice": 5.0,
                    "yearlyPrice": 50.0,
                    "yearlyDiscount": 16.67
                },
                "specifications": {
                    "memoryLimitMb": 30,
                    "connectionLimit": 30,
                    "opsPerSecond": 1000,
                    "bandwidthMbps": 1,
                    "replication": false,
                    "clustering": false,
                    "persistence": false,
                    "backup": false,
                    "evictionPolicy": "allkeys-lru"
                },
                "features": [
                    "Redis OSS 7.2",
                    "Standard support",
                    "TLS encryption",
                    "24/7 monitoring",
                    "Multi-AZ deployment"
                ],
                "modules": [],
                "limitations": {
                    "maxDatabases": 1,
                    "maxConnections": 30,
                    "maxMemoryMb": 30,
                    "commandsNotSupported": ["FLUSHALL", "FLUSHDB", "CONFIG"]
                },
                "sla": {
                    "availability": "99.5%",
                    "supportLevel": "standard"
                },
                "available": true,
                "deprecated": false
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudFixedHandler::new(client);
    let result = handler.plan(101).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let plan = &response["plan"];
    assert_eq!(plan["id"], 101);
    assert_eq!(plan["name"], "Redis Cloud Essentials 30MB");
    assert_eq!(plan["displayName"], "Essential 30MB");
    assert_eq!(plan["provider"], "AWS");
    assert_eq!(plan["available"], true);
    assert_eq!(plan["deprecated"], false);

    let pricing = &plan["pricing"];
    assert_eq!(pricing["monthlyPrice"], 5.0);
    assert_eq!(pricing["yearlyPrice"], 50.0);
    assert_eq!(pricing["yearlyDiscount"], 16.67);

    let specs = &plan["specifications"];
    assert_eq!(specs["memoryLimitMb"], 30);
    assert_eq!(specs["connectionLimit"], 30);
    assert_eq!(specs["evictionPolicy"], "allkeys-lru");

    let features = plan["features"].as_array().unwrap();
    assert_eq!(features.len(), 5);
    assert!(features.contains(&json!("Multi-AZ deployment")));

    let limitations = &plan["limitations"];
    assert_eq!(limitations["maxDatabases"], 1);
    let unsupported_cmds = limitations["commandsNotSupported"].as_array().unwrap();
    assert_eq!(unsupported_cmds.len(), 3);

    let sla = &plan["sla"];
    assert_eq!(sla["availability"], "99.5%");
    assert_eq!(sla["supportLevel"], "standard");
}

#[tokio::test]
async fn test_get_fixed_plan_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/fixed/plans/9999"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            404,
            json!({
                "error": {
                    "type": "NOT_FOUND",
                    "status": 404,
                    "description": "Fixed plan not found"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudFixedHandler::new(client);
    let result = handler.plan(9999).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_fixed_plan_unavailable() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/fixed/plans/103"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "plan": {
                "id": 103,
                "name": "Redis Cloud Essentials 500MB",
                "displayName": "Essential 500MB (Deprecated)",
                "description": "Legacy plan no longer available for new subscriptions",
                "provider": "AWS",
                "region": "us-east-1",
                "pricing": {
                    "type": "fixed",
                    "currency": "USD",
                    "monthlyPrice": 75.0
                },
                "specifications": {
                    "memoryLimitMb": 500,
                    "connectionLimit": 500,
                    "opsPerSecond": 50000
                },
                "available": false,
                "deprecated": true
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudFixedHandler::new(client);
    let result = handler.plan(103).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let plan = &response["plan"];
    assert_eq!(plan["available"], false);
    assert_eq!(plan["deprecated"], true);
    assert_eq!(plan["displayName"], "Essential 500MB (Deprecated)");
}
