//! Private Service Connect endpoint tests for Redis Cloud

use redis_cloud::{CloudClient, CloudConfig, CloudPrivateServiceConnectHandler};
use serde_json::json;
use wiremock::matchers::{header, method, path};
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
    let config = CloudConfig {
        api_key: "test-api-key".to_string(),
        api_secret: "test-secret-key".to_string(),
        base_url,
        timeout: std::time::Duration::from_secs(30),
    };
    CloudClient::new(config).unwrap()
}

#[tokio::test]
async fn test_list_private_service_connect() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/100001/private-service-connect"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "privateServiceConnects": [
                {
                    "id": "psc-123456",
                    "name": "Production PSC",
                    "description": "Private service connect for production environment",
                    "status": "active",
                    "provider": "GCP",
                    "region": "us-central1",
                    "serviceId": "projects/redis-cloud/regions/us-central1/serviceAttachments/redis-psc-prod",
                    "endpoints": [
                        {
                            "id": "endpoint-001",
                            "name": "prod-endpoint-1",
                            "projectId": "customer-project-123",
                            "network": "projects/customer-project-123/global/networks/vpc-main",
                            "subnetwork": "projects/customer-project-123/regions/us-central1/subnetworks/subnet-redis",
                            "status": "connected"
                        }
                    ],
                    "createdTimestamp": "2023-01-01T00:00:00Z",
                    "updatedTimestamp": "2023-12-01T10:00:00Z"
                },
                {
                    "id": "psc-789012",
                    "name": "Development PSC",
                    "description": "Private service connect for dev environment",
                    "status": "pending",
                    "provider": "GCP",
                    "region": "us-west1",
                    "serviceId": "projects/redis-cloud/regions/us-west1/serviceAttachments/redis-psc-dev",
                    "endpoints": [],
                    "createdTimestamp": "2023-11-01T00:00:00Z",
                    "updatedTimestamp": "2023-11-01T00:00:00Z"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudPrivateServiceConnectHandler::new(client);
    let result = handler.list(100001).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let pscs = response["privateServiceConnects"].as_array().unwrap();
    assert_eq!(pscs.len(), 2);
    assert_eq!(pscs[0]["id"], "psc-123456");
    assert_eq!(pscs[0]["name"], "Production PSC");
    assert_eq!(pscs[0]["status"], "active");
    assert_eq!(pscs[0]["provider"], "GCP");
    let endpoints = pscs[0]["endpoints"].as_array().unwrap();
    assert_eq!(endpoints.len(), 1);
    assert_eq!(endpoints[0]["status"], "connected");
}

#[tokio::test]
async fn test_list_private_service_connect_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/100001/private-service-connect"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "privateServiceConnects": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudPrivateServiceConnectHandler::new(client);
    let result = handler.list(100001).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let pscs = response["privateServiceConnects"].as_array().unwrap();
    assert_eq!(pscs.len(), 0);
}

#[tokio::test]
async fn test_get_private_service_connect() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/100001/private-service-connect/psc-123456"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "privateServiceConnect": {
                "id": "psc-123456",
                "name": "Production PSC",
                "description": "Private service connect for production environment",
                "status": "active",
                "provider": "GCP",
                "region": "us-central1",
                "serviceId": "projects/redis-cloud/regions/us-central1/serviceAttachments/redis-psc-prod",
                "configuration": {
                    "acceptAutoConnections": false,
                    "connectionLimit": 50,
                    "consumerRejectLists": []
                },
                "endpoints": [
                    {
                        "id": "endpoint-001",
                        "name": "prod-endpoint-1",
                        "projectId": "customer-project-123",
                        "network": "projects/customer-project-123/global/networks/vpc-main",
                        "subnetwork": "projects/customer-project-123/regions/us-central1/subnetworks/subnet-redis",
                        "ipAddress": "10.0.1.5",
                        "status": "connected",
                        "createdTimestamp": "2023-01-01T01:00:00Z"
                    }
                ],
                "dnsConfiguration": {
                    "enabled": true,
                    "domain": "redis.internal",
                    "zones": [
                        {
                            "name": "redis-prod.internal",
                            "recordType": "A",
                            "ttl": 300
                        }
                    ]
                },
                "createdTimestamp": "2023-01-01T00:00:00Z",
                "updatedTimestamp": "2023-12-01T10:00:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudPrivateServiceConnectHandler::new(client);
    let result = handler.get(100001, "psc-123456").await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let psc = &response["privateServiceConnect"];
    assert_eq!(psc["id"], "psc-123456");
    assert_eq!(psc["name"], "Production PSC");
    assert_eq!(psc["status"], "active");
    let config = &psc["configuration"];
    assert_eq!(config["acceptAutoConnections"], false);
    assert_eq!(config["connectionLimit"], 50);
    let dns_config = &psc["dnsConfiguration"];
    assert_eq!(dns_config["enabled"], true);
    assert_eq!(dns_config["domain"], "redis.internal");
}

#[tokio::test]
async fn test_get_private_service_connect_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(
            "/subscriptions/100001/private-service-connect/psc-nonexistent",
        ))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            404,
            json!({
                "error": {
                    "type": "NOT_FOUND",
                    "status": 404,
                    "description": "Private service connect not found"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudPrivateServiceConnectHandler::new(client);
    let result = handler.get(100001, "psc-nonexistent").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_private_service_connect() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/subscriptions/100001/private-service-connect"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(accepted_response(json!({
            "taskId": "task_psc_create_123",
            "privateServiceConnect": {
                "id": "psc-new-789",
                "name": "Staging PSC",
                "description": "Private service connect for staging",
                "status": "pending",
                "provider": "GCP",
                "region": "us-east1",
                "serviceId": "projects/redis-cloud/regions/us-east1/serviceAttachments/redis-psc-staging",
                "createdTimestamp": "2023-12-01T12:00:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudPrivateServiceConnectHandler::new(client);
    let service_request = json!({
        "name": "Staging PSC",
        "description": "Private service connect for staging",
        "region": "us-east1",
        "configuration": {
            "acceptAutoConnections": true,
            "connectionLimit": 25
        },
        "dnsConfiguration": {
            "enabled": true,
            "domain": "redis.staging"
        }
    });
    let result = handler.create(100001, service_request).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response["taskId"].is_string());
    let psc = &response["privateServiceConnect"];
    assert_eq!(psc["name"], "Staging PSC");
    assert_eq!(psc["status"], "pending");
    assert_eq!(psc["region"], "us-east1");
}

#[tokio::test]
async fn test_update_private_service_connect() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path(
            "/subscriptions/100001/private-service-connect/psc-123456",
        ))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(accepted_response(json!({
            "taskId": "task_psc_update_456",
            "privateServiceConnect": {
                "id": "psc-123456",
                "name": "Updated Production PSC",
                "description": "Updated private service connect for production",
                "status": "updating",
                "updatedTimestamp": "2023-12-01T12:00:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudPrivateServiceConnectHandler::new(client);
    let update_request = json!({
        "name": "Updated Production PSC",
        "description": "Updated private service connect for production",
        "configuration": {
            "connectionLimit": 100
        }
    });
    let result = handler.update(100001, "psc-123456", update_request).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response["taskId"].is_string());
    let psc = &response["privateServiceConnect"];
    assert_eq!(psc["name"], "Updated Production PSC");
    assert_eq!(psc["status"], "updating");
}

#[tokio::test]
async fn test_delete_private_service_connect() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(
            "/subscriptions/100001/private-service-connect/psc-123456",
        ))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task_psc_delete_789"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudPrivateServiceConnectHandler::new(client);
    let result = handler.delete(100001, "psc-123456").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_endpoint() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/100001/private-service-connect/psc-123456/endpoints/endpoint-001"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "endpoint": {
                "id": "endpoint-001",
                "name": "prod-endpoint-1",
                "projectId": "customer-project-123",
                "network": "projects/customer-project-123/global/networks/vpc-main",
                "subnetwork": "projects/customer-project-123/regions/us-central1/subnetworks/subnet-redis",
                "ipAddress": "10.0.1.5",
                "status": "connected",
                "connectionDetails": {
                    "pscConnectionId": "12345678901234567890",
                    "connectionStatus": "ACCEPTED",
                    "consumerForwardingRule": "projects/customer-project-123/regions/us-central1/forwardingRules/redis-psc-endpoint"
                },
                "dnsRecords": [
                    {
                        "name": "redis-prod.internal",
                        "type": "A",
                        "value": "10.0.1.5",
                        "ttl": 300
                    }
                ],
                "createdTimestamp": "2023-01-01T01:00:00Z",
                "updatedTimestamp": "2023-01-01T01:15:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudPrivateServiceConnectHandler::new(client);
    let result = handler
        .get_endpoint(100001, "psc-123456", "endpoint-001")
        .await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let endpoint = &response["endpoint"];
    assert_eq!(endpoint["id"], "endpoint-001");
    assert_eq!(endpoint["name"], "prod-endpoint-1");
    assert_eq!(endpoint["status"], "connected");
    assert_eq!(endpoint["ipAddress"], "10.0.1.5");
    let conn_details = &endpoint["connectionDetails"];
    assert_eq!(conn_details["connectionStatus"], "ACCEPTED");
    let dns_records = endpoint["dnsRecords"].as_array().unwrap();
    assert_eq!(dns_records.len(), 1);
    assert_eq!(dns_records[0]["type"], "A");
}

#[tokio::test]
async fn test_get_creation_scripts() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/100001/private-service-connect/psc-123456/endpoints/endpoint-001/creationScripts"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "scripts": {
                "terraform": {
                    "main": "resource \"google_compute_forwarding_rule\" \"redis_psc_endpoint\" {\n  name   = \"redis-psc-endpoint\"\n  region = \"us-central1\"\n  target = \"projects/redis-cloud/regions/us-central1/serviceAttachments/redis-psc-prod\"\n  network = \"projects/customer-project-123/global/networks/vpc-main\"\n  subnetwork = \"projects/customer-project-123/regions/us-central1/subnetworks/subnet-redis\"\n  ip_address = \"10.0.1.5\"\n}",
                    "variables": "variable \"project_id\" {\n  description = \"GCP Project ID\"\n  type        = string\n  default     = \"customer-project-123\"\n}",
                    "outputs": "output \"endpoint_ip\" {\n  description = \"Private Service Connect endpoint IP address\"\n  value       = google_compute_forwarding_rule.redis_psc_endpoint.ip_address\n}"
                },
                "gcloud": {
                    "commands": [
                        "gcloud compute forwarding-rules create redis-psc-endpoint \\",
                        "  --region=us-central1 \\",
                        "  --network=projects/customer-project-123/global/networks/vpc-main \\",
                        "  --subnet=projects/customer-project-123/regions/us-central1/subnetworks/subnet-redis \\",
                        "  --address=10.0.1.5 \\",
                        "  --target-service-attachment=projects/redis-cloud/regions/us-central1/serviceAttachments/redis-psc-prod"
                    ]
                },
                "documentation": "https://docs.redis.com/latest/rc/cloud-integrations/gcp-private-service-connect/"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudPrivateServiceConnectHandler::new(client);
    let result = handler
        .get_creation_scripts(100001, "psc-123456", "endpoint-001")
        .await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let scripts = &response["scripts"];
    assert!(scripts["terraform"].is_object());
    assert!(scripts["gcloud"].is_object());
    assert!(
        scripts["terraform"]["main"]
            .as_str()
            .unwrap()
            .contains("google_compute_forwarding_rule")
    );
    let gcloud_cmds = scripts["gcloud"]["commands"].as_array().unwrap();
    assert!(!gcloud_cmds.is_empty());
    assert!(
        scripts["documentation"]
            .as_str()
            .unwrap()
            .starts_with("https://")
    );
}

#[tokio::test]
async fn test_get_deletion_scripts() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/100001/private-service-connect/psc-123456/endpoints/endpoint-001/deletionScripts"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "scripts": {
                "terraform": {
                    "main": "# Remove the Private Service Connect endpoint\n# terraform destroy -target=google_compute_forwarding_rule.redis_psc_endpoint",
                    "warning": "This will permanently delete the PSC endpoint and interrupt Redis connectivity"
                },
                "gcloud": {
                    "commands": [
                        "gcloud compute forwarding-rules delete redis-psc-endpoint \\",
                        "  --region=us-central1 \\",
                        "  --quiet"
                    ],
                    "warning": "This will permanently delete the PSC endpoint and interrupt Redis connectivity"
                },
                "preDeleteChecklist": [
                    "Ensure no applications are actively using the Redis connection",
                    "Backup any critical data if needed",
                    "Verify alternative connection methods are available",
                    "Notify team members of the maintenance window"
                ]
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudPrivateServiceConnectHandler::new(client);
    let result = handler
        .get_deletion_scripts(100001, "psc-123456", "endpoint-001")
        .await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let scripts = &response["scripts"];
    assert!(
        scripts["terraform"]["warning"]
            .as_str()
            .unwrap()
            .contains("permanently delete")
    );
    assert!(
        scripts["gcloud"]["warning"]
            .as_str()
            .unwrap()
            .contains("permanently delete")
    );
    let checklist = scripts["preDeleteChecklist"].as_array().unwrap();
    assert_eq!(checklist.len(), 4);
    assert!(checklist[0].as_str().unwrap().contains("applications"));
}

#[tokio::test]
async fn test_list_regional_private_service_connect() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/100001/regions/us-central1/private-service-connect"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "privateServiceConnects": [
                {
                    "id": "psc-regional-001",
                    "name": "Regional PSC Central",
                    "description": "Regional private service connect",
                    "status": "active",
                    "provider": "GCP",
                    "region": "us-central1",
                    "serviceId": "projects/redis-cloud/regions/us-central1/serviceAttachments/redis-psc-regional",
                    "endpoints": [
                        {
                            "id": "endpoint-regional-001",
                            "name": "regional-endpoint-1",
                            "status": "connected"
                        }
                    ],
                    "createdTimestamp": "2023-06-01T00:00:00Z"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudPrivateServiceConnectHandler::new(client);
    let result = handler.list_regional(100001, "us-central1").await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let pscs = response["privateServiceConnects"].as_array().unwrap();
    assert_eq!(pscs.len(), 1);
    assert_eq!(pscs[0]["id"], "psc-regional-001");
    assert_eq!(pscs[0]["region"], "us-central1");
}

#[tokio::test]
async fn test_get_regional_private_service_connect() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/100001/regions/us-central1/private-service-connect/psc-regional-001"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "privateServiceConnect": {
                "id": "psc-regional-001",
                "name": "Regional PSC Central",
                "description": "Regional private service connect",
                "status": "active",
                "provider": "GCP",
                "region": "us-central1",
                "availabilityZones": [
                    "us-central1-a",
                    "us-central1-b",
                    "us-central1-c"
                ],
                "serviceId": "projects/redis-cloud/regions/us-central1/serviceAttachments/redis-psc-regional",
                "configuration": {
                    "acceptAutoConnections": true,
                    "connectionLimit": 20
                },
                "endpoints": [
                    {
                        "id": "endpoint-regional-001",
                        "name": "regional-endpoint-1",
                        "availabilityZone": "us-central1-a",
                        "status": "connected"
                    }
                ]
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudPrivateServiceConnectHandler::new(client);
    let result = handler
        .get_regional(100001, "us-central1", "psc-regional-001")
        .await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let psc = &response["privateServiceConnect"];
    assert_eq!(psc["id"], "psc-regional-001");
    assert_eq!(psc["region"], "us-central1");
    let azs = psc["availabilityZones"].as_array().unwrap();
    assert_eq!(azs.len(), 3);
}

#[tokio::test]
async fn test_create_regional_private_service_connect() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(
            "/subscriptions/100001/regions/us-west1/private-service-connect",
        ))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(accepted_response(json!({
            "taskId": "task_psc_regional_create_999",
            "privateServiceConnect": {
                "id": "psc-regional-west-001",
                "name": "Regional PSC West",
                "region": "us-west1",
                "status": "pending",
                "createdTimestamp": "2023-12-01T12:00:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudPrivateServiceConnectHandler::new(client);
    let service_request = json!({
        "name": "Regional PSC West",
        "description": "Regional PSC for west region",
        "availabilityZones": ["us-west1-a", "us-west1-b"]
    });
    let result = handler
        .create_regional(100001, "us-west1", service_request)
        .await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response["taskId"].is_string());
    let psc = &response["privateServiceConnect"];
    assert_eq!(psc["name"], "Regional PSC West");
    assert_eq!(psc["region"], "us-west1");
    assert_eq!(psc["status"], "pending");
}

#[tokio::test]
async fn test_update_regional_private_service_connect() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path(
            "/subscriptions/100001/regions/us-central1/private-service-connect/psc-regional-001",
        ))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(accepted_response(json!({
            "taskId": "task_psc_regional_update_888",
            "privateServiceConnect": {
                "id": "psc-regional-001",
                "name": "Updated Regional PSC Central",
                "region": "us-central1",
                "status": "updating"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudPrivateServiceConnectHandler::new(client);
    let update_request = json!({
        "name": "Updated Regional PSC Central",
        "configuration": {
            "connectionLimit": 30
        }
    });
    let result = handler
        .update_regional(100001, "us-central1", "psc-regional-001", update_request)
        .await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response["taskId"].is_string());
    let psc = &response["privateServiceConnect"];
    assert_eq!(psc["name"], "Updated Regional PSC Central");
    assert_eq!(psc["status"], "updating");
}

#[tokio::test]
async fn test_delete_regional_private_service_connect() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(
            "/subscriptions/100001/regions/us-central1/private-service-connect/psc-regional-001",
        ))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task_psc_regional_delete_777"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudPrivateServiceConnectHandler::new(client);
    let result = handler
        .delete_regional(100001, "us-central1", "psc-regional-001")
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_regional_endpoint() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/100001/regions/us-central1/private-service-connect/psc-regional-001/endpoints/endpoint-regional-001"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "endpoint": {
                "id": "endpoint-regional-001",
                "name": "regional-endpoint-1",
                "projectId": "customer-project-456",
                "network": "projects/customer-project-456/global/networks/vpc-regional",
                "subnetwork": "projects/customer-project-456/regions/us-central1/subnetworks/subnet-redis-regional",
                "availabilityZone": "us-central1-a",
                "ipAddress": "10.1.2.10",
                "status": "connected",
                "connectionDetails": {
                    "pscConnectionId": "98765432109876543210",
                    "connectionStatus": "ACCEPTED"
                }
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudPrivateServiceConnectHandler::new(client);
    let result = handler
        .get_regional_endpoint(
            100001,
            "us-central1",
            "psc-regional-001",
            "endpoint-regional-001",
        )
        .await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let endpoint = &response["endpoint"];
    assert_eq!(endpoint["id"], "endpoint-regional-001");
    assert_eq!(endpoint["availabilityZone"], "us-central1-a");
    assert_eq!(endpoint["status"], "connected");
}

#[tokio::test]
async fn test_get_regional_creation_scripts() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/100001/regions/us-central1/private-service-connect/psc-regional-001/endpoints/endpoint-regional-001/creationScripts"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "scripts": {
                "terraform": {
                    "main": "resource \"google_compute_forwarding_rule\" \"redis_psc_regional_endpoint\" {\n  name   = \"redis-psc-regional-endpoint\"\n  region = \"us-central1\"\n  target = \"projects/redis-cloud/regions/us-central1/serviceAttachments/redis-psc-regional\"\n}"
                },
                "gcloud": {
                    "commands": [
                        "gcloud compute forwarding-rules create redis-psc-regional-endpoint \\",
                        "  --region=us-central1 \\",
                        "  --target-service-attachment=projects/redis-cloud/regions/us-central1/serviceAttachments/redis-psc-regional"
                    ]
                }
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudPrivateServiceConnectHandler::new(client);
    let result = handler
        .get_regional_creation_scripts(
            100001,
            "us-central1",
            "psc-regional-001",
            "endpoint-regional-001",
        )
        .await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let scripts = &response["scripts"];
    assert!(
        scripts["terraform"]["main"]
            .as_str()
            .unwrap()
            .contains("redis_psc_regional_endpoint")
    );
}

#[tokio::test]
async fn test_get_regional_deletion_scripts() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/100001/regions/us-central1/private-service-connect/psc-regional-001/endpoints/endpoint-regional-001/deletionScripts"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "scripts": {
                "terraform": {
                    "main": "# terraform destroy -target=google_compute_forwarding_rule.redis_psc_regional_endpoint"
                },
                "gcloud": {
                    "commands": [
                        "gcloud compute forwarding-rules delete redis-psc-regional-endpoint \\",
                        "  --region=us-central1 \\",
                        "  --quiet"
                    ]
                }
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudPrivateServiceConnectHandler::new(client);
    let result = handler
        .get_regional_deletion_scripts(
            100001,
            "us-central1",
            "psc-regional-001",
            "endpoint-regional-001",
        )
        .await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let scripts = &response["scripts"];
    assert!(
        scripts["terraform"]["main"]
            .as_str()
            .unwrap()
            .contains("terraform destroy")
    );
}

#[tokio::test]
async fn test_private_service_connect_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/100001/private-service-connect"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            403,
            json!({
                "error": {
                    "type": "FORBIDDEN",
                    "status": 403,
                    "description": "Private Service Connect is not available for this subscription type"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudPrivateServiceConnectHandler::new(client);
    let result = handler.list(100001).await;

    assert!(result.is_err());
}
