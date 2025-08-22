//! Region endpoint tests for Redis Cloud

use redis_cloud::{CloudClient, CloudConfig, CloudRegionHandler};
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
async fn test_regions_list_aws() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/cloud-providers/AWS/regions"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "regions": [
                {
                    "id": "us-east-1",
                    "name": "US East (N. Virginia)",
                    "provider": "AWS",
                    "available": true,
                    "zones": ["us-east-1a", "us-east-1b", "us-east-1c"],
                    "networking": {
                        "vpc": true,
                        "privateServiceConnect": false,
                        "transitGateway": true
                    }
                },
                {
                    "id": "us-west-2",
                    "name": "US West (Oregon)",
                    "provider": "AWS",
                    "available": true,
                    "zones": ["us-west-2a", "us-west-2b", "us-west-2c"],
                    "networking": {
                        "vpc": true,
                        "privateServiceConnect": false,
                        "transitGateway": true
                    }
                },
                {
                    "id": "eu-west-1",
                    "name": "Europe (Ireland)",
                    "provider": "AWS",
                    "available": true,
                    "zones": ["eu-west-1a", "eu-west-1b", "eu-west-1c"],
                    "networking": {
                        "vpc": true,
                        "privateServiceConnect": false,
                        "transitGateway": true
                    }
                }
            ],
            "totalCount": 3
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudRegionHandler::new(client);
    let result = handler.list("AWS").await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let regions = response["regions"].as_array().unwrap();
    assert_eq!(regions.len(), 3);

    // Check first region
    assert_eq!(regions[0]["id"], "us-east-1");
    assert_eq!(regions[0]["name"], "US East (N. Virginia)");
    assert_eq!(regions[0]["provider"], "AWS");
    assert_eq!(regions[0]["available"], true);

    let zones = regions[0]["zones"].as_array().unwrap();
    assert_eq!(zones.len(), 3);
    assert!(zones.contains(&json!("us-east-1a")));
    assert!(zones.contains(&json!("us-east-1b")));
    assert!(zones.contains(&json!("us-east-1c")));

    let networking = &regions[0]["networking"];
    assert_eq!(networking["vpc"], true);
    assert_eq!(networking["privateServiceConnect"], false);
    assert_eq!(networking["transitGateway"], true);

    assert_eq!(response["totalCount"], 3);
}

#[tokio::test]
async fn test_regions_list_gcp() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/cloud-providers/GCP/regions"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "regions": [
                {
                    "id": "us-central1",
                    "name": "Iowa",
                    "provider": "GCP",
                    "available": true,
                    "zones": ["us-central1-a", "us-central1-b", "us-central1-c"],
                    "networking": {
                        "vpc": true,
                        "privateServiceConnect": true,
                        "transitGateway": false
                    }
                },
                {
                    "id": "europe-west1",
                    "name": "Belgium",
                    "provider": "GCP",
                    "available": true,
                    "zones": ["europe-west1-b", "europe-west1-c", "europe-west1-d"],
                    "networking": {
                        "vpc": true,
                        "privateServiceConnect": true,
                        "transitGateway": false
                    }
                }
            ],
            "totalCount": 2
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudRegionHandler::new(client);
    let result = handler.list("GCP").await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let regions = response["regions"].as_array().unwrap();
    assert_eq!(regions.len(), 2);

    assert_eq!(regions[0]["id"], "us-central1");
    assert_eq!(regions[0]["provider"], "GCP");
    assert_eq!(regions[0]["networking"]["privateServiceConnect"], true);
    assert_eq!(regions[0]["networking"]["transitGateway"], false);

    assert_eq!(regions[1]["id"], "europe-west1");
    assert_eq!(response["totalCount"], 2);
}

#[tokio::test]
async fn test_regions_list_azure() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/cloud-providers/Azure/regions"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "regions": [
                {
                    "id": "East US",
                    "name": "East US",
                    "provider": "Azure",
                    "available": true,
                    "zones": ["1", "2", "3"],
                    "networking": {
                        "vpc": true,
                        "privateServiceConnect": false,
                        "transitGateway": false
                    }
                }
            ],
            "totalCount": 1
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudRegionHandler::new(client);
    let result = handler.list("Azure").await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let regions = response["regions"].as_array().unwrap();
    assert_eq!(regions.len(), 1);

    assert_eq!(regions[0]["id"], "East US");
    assert_eq!(regions[0]["provider"], "Azure");
    assert_eq!(response["totalCount"], 1);
}

#[tokio::test]
async fn test_region_get_aws_details() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/cloud-providers/AWS/regions/us-east-1"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "id": "us-east-1",
            "name": "US East (N. Virginia)",
            "provider": "AWS",
            "available": true,
            "zones": ["us-east-1a", "us-east-1b", "us-east-1c", "us-east-1d", "us-east-1f"],
            "networking": {
                "vpc": true,
                "privateServiceConnect": false,
                "transitGateway": true,
                "supportedFeatures": ["peering", "nat-gateway", "internet-gateway"]
            },
            "pricing": {
                "currency": "USD",
                "dataTransfer": {
                    "inbound": "free",
                    "outbound": 0.09
                }
            },
            "compliance": ["SOC2", "PCI-DSS", "HIPAA", "FedRAMP"],
            "maxInstances": 100,
            "diskTypes": ["gp2", "gp3", "io1", "io2"]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudRegionHandler::new(client);
    let result = handler.get("AWS", "us-east-1").await;

    assert!(result.is_ok());
    let region = result.unwrap();
    assert_eq!(region["id"], "us-east-1");
    assert_eq!(region["name"], "US East (N. Virginia)");
    assert_eq!(region["provider"], "AWS");
    assert_eq!(region["available"], true);

    let zones = region["zones"].as_array().unwrap();
    assert_eq!(zones.len(), 5);
    assert!(zones.contains(&json!("us-east-1a")));
    assert!(zones.contains(&json!("us-east-1f")));

    let networking = &region["networking"];
    assert_eq!(networking["vpc"], true);
    assert_eq!(networking["transitGateway"], true);
    let features = networking["supportedFeatures"].as_array().unwrap();
    assert!(features.contains(&json!("peering")));
    assert!(features.contains(&json!("nat-gateway")));

    let pricing = &region["pricing"];
    assert_eq!(pricing["currency"], "USD");
    assert_eq!(pricing["dataTransfer"]["inbound"], "free");
    assert_eq!(pricing["dataTransfer"]["outbound"], 0.09);

    let compliance = region["compliance"].as_array().unwrap();
    assert!(compliance.contains(&json!("SOC2")));
    assert!(compliance.contains(&json!("HIPAA")));

    assert_eq!(region["maxInstances"], 100);

    let disk_types = region["diskTypes"].as_array().unwrap();
    assert!(disk_types.contains(&json!("gp3")));
    assert!(disk_types.contains(&json!("io2")));
}

#[tokio::test]
async fn test_region_get_gcp_details() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/cloud-providers/GCP/regions/us-central1"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "id": "us-central1",
            "name": "Iowa",
            "provider": "GCP",
            "available": true,
            "zones": ["us-central1-a", "us-central1-b", "us-central1-c", "us-central1-f"],
            "networking": {
                "vpc": true,
                "privateServiceConnect": true,
                "transitGateway": false,
                "supportedFeatures": ["peering", "private-service-connect"]
            },
            "pricing": {
                "currency": "USD",
                "dataTransfer": {
                    "inbound": "free",
                    "outbound": 0.12
                }
            },
            "compliance": ["SOC2", "PCI-DSS", "HIPAA", "ISO27001"],
            "maxInstances": 50,
            "diskTypes": ["pd-standard", "pd-ssd", "pd-balanced"]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudRegionHandler::new(client);
    let result = handler.get("GCP", "us-central1").await;

    assert!(result.is_ok());
    let region = result.unwrap();
    assert_eq!(region["id"], "us-central1");
    assert_eq!(region["provider"], "GCP");
    assert_eq!(region["networking"]["privateServiceConnect"], true);
    assert_eq!(region["networking"]["transitGateway"], false);

    let features = region["networking"]["supportedFeatures"]
        .as_array()
        .unwrap();
    assert!(features.contains(&json!("private-service-connect")));

    let disk_types = region["diskTypes"].as_array().unwrap();
    assert!(disk_types.contains(&json!("pd-balanced")));
}

#[tokio::test]
async fn test_region_list_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/cloud-providers/Custom/regions"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "regions": [],
            "totalCount": 0
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudRegionHandler::new(client);
    let result = handler.list("Custom").await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let regions = response["regions"].as_array().unwrap();
    assert_eq!(regions.len(), 0);
    assert_eq!(response["totalCount"], 0);
}

// Error handling tests
#[tokio::test]
async fn test_region_list_invalid_provider() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/cloud-providers/INVALID/regions"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            400,
            json!({
                "error": {
                    "type": "INVALID_PROVIDER",
                    "status": 400,
                    "description": "Invalid cloud provider. Supported providers: AWS, GCP, Azure"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudRegionHandler::new(client);
    let result = handler.list("INVALID").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_region_get_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/cloud-providers/AWS/regions/nonexistent-region"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            404,
            json!({
                "error": {
                    "type": "NOT_FOUND",
                    "status": 404,
                    "description": "Region 'nonexistent-region' not found for provider AWS"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudRegionHandler::new(client);
    let result = handler.get("AWS", "nonexistent-region").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_region_list_unauthorized() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/cloud-providers/AWS/regions"))
        .and(header("x-api-key", "invalid-key"))
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

    let config = CloudConfig {
        api_key: "invalid-key".to_string(),
        api_secret: "test-secret-key".to_string(),
        base_url: mock_server.uri(),
        timeout: std::time::Duration::from_secs(30),
    };
    let client = CloudClient::new(config).unwrap();
    let handler = CloudRegionHandler::new(client);
    let result = handler.list("AWS").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_region_get_unavailable() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/cloud-providers/AWS/regions/us-gov-east-1"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "id": "us-gov-east-1",
            "name": "AWS GovCloud (US-East)",
            "provider": "AWS",
            "available": false,
            "zones": [],
            "networking": {
                "vpc": false,
                "privateServiceConnect": false,
                "transitGateway": false
            },
            "reason": "Government cloud region requires special access permissions"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudRegionHandler::new(client);
    let result = handler.get("AWS", "us-gov-east-1").await;

    assert!(result.is_ok());
    let region = result.unwrap();
    assert_eq!(region["id"], "us-gov-east-1");
    assert_eq!(region["available"], false);
    assert!(region["reason"].is_string());

    let zones = region["zones"].as_array().unwrap();
    assert_eq!(zones.len(), 0);
}
