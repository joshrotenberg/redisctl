//! Peering endpoint tests for Redis Cloud

use redis_cloud::{CloudClient, CloudPeeringHandler, CreatePeeringRequest};
use serde_json::json;
use wiremock::matchers::{body_json, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// Test helper functions
fn success_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(body)
}

fn created_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(201).set_body_json(body)
}

fn no_content_response() -> ResponseTemplate {
    ResponseTemplate::new(204)
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

// Helper function to create mock peerings list response
fn peerings_list_response() -> serde_json::Value {
    json!({
        "peerings": [
            {
                "peering_id": "peer_12345",
                "subscription_id": 67890,
                "status": "active",
                "provider_peering_id": "pcx-0123456789abcdef0",
                "aws_account_id": "123456789012",
                "vpc_id": "vpc-0123456789abcdef0",
                "vpc_cidr": "10.0.0.0/16",
                "region": "us-east-1",
                "provider": "AWS",
                "createdAt": "2023-01-01T10:00:00Z",
                "updatedAt": "2023-01-01T10:30:00Z"
            },
            {
                "peering_id": "peer_67890",
                "subscription_id": 67890,
                "status": "pending",
                "provider_peering_id": null,
                "aws_account_id": "987654321098",
                "vpc_id": "vpc-0987654321fedcba0",
                "vpc_cidr": "172.16.0.0/16",
                "region": "us-west-2",
                "provider": "AWS",
                "createdAt": "2023-01-01T11:00:00Z",
                "updatedAt": "2023-01-01T11:00:00Z"
            },
            {
                "peering_id": "peer_13579",
                "subscription_id": 67890,
                "status": "failed",
                "provider_peering_id": null,
                "aws_account_id": "555666777888",
                "vpc_id": "vpc-0555666777888999a",
                "vpc_cidr": "192.168.0.0/16",
                "region": "eu-west-1",
                "provider": "AWS",
                "createdAt": "2023-01-01T12:00:00Z",
                "updatedAt": "2023-01-01T12:15:00Z",
                "error": "Invalid VPC configuration"
            }
        ]
    })
}

// Helper function to create mock single peering response
fn single_peering_response() -> serde_json::Value {
    json!({
        "peering_id": "peer_12345",
        "subscription_id": 67890,
        "status": "active",
        "provider_peering_id": "pcx-0123456789abcdef0",
        "aws_account_id": "123456789012",
        "vpc_id": "vpc-0123456789abcdef0",
        "vpc_cidr": "10.0.0.0/16",
        "region": "us-east-1",
        "provider": "AWS",
        "createdAt": "2023-01-01T10:00:00Z",
        "updatedAt": "2023-01-01T10:30:00Z",
        "routeTableIds": ["rtb-0123456789abcdef0", "rtb-0987654321fedcba0"],
        "dnsResolution": true,
        "tags": {
            "Environment": "production",
            "Team": "backend"
        }
    })
}

// Helper function to create pending peering response
fn pending_peering_response() -> serde_json::Value {
    json!({
        "peering_id": "peer_pending",
        "subscription_id": 67890,
        "status": "pending-acceptance",
        "provider_peering_id": "pcx-pending123456789",
        "aws_account_id": "123456789012",
        "vpc_id": "vpc-0123456789abcdef0",
        "vpc_cidr": "10.0.0.0/16",
        "region": "us-east-1",
        "provider": "AWS",
        "createdAt": "2023-01-01T13:00:00Z",
        "updatedAt": "2023-01-01T13:00:00Z"
    })
}

// Helper function to create failed peering response
fn failed_peering_response() -> serde_json::Value {
    json!({
        "peering_id": "peer_failed",
        "subscription_id": 67890,
        "status": "failed",
        "provider_peering_id": null,
        "aws_account_id": "123456789012",
        "vpc_id": "vpc-invalid123456789",
        "vpc_cidr": "10.0.0.0/16",
        "region": "us-east-1",
        "provider": "AWS",
        "createdAt": "2023-01-01T14:00:00Z",
        "updatedAt": "2023-01-01T14:05:00Z",
        "error": "VPC not found in the specified region",
        "errorCode": "VPC_NOT_FOUND"
    })
}

#[tokio::test]
async fn test_list_peerings() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/67890/peerings"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(peerings_list_response()))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudPeeringHandler::new(client);

    let result = handler.list(67890).await;

    assert!(result.is_ok());
    let peerings = result.unwrap();
    assert_eq!(peerings.len(), 3);

    // Check first peering (active)
    assert_eq!(peerings[0].peering_id, "peer_12345");
    assert_eq!(peerings[0].subscription_id, 67890);
    assert_eq!(peerings[0].status, "active");
    assert_eq!(
        peerings[0].provider_peering_id,
        Some("pcx-0123456789abcdef0".to_string())
    );
    assert_eq!(peerings[0].aws_account_id, Some("123456789012".to_string()));
    assert_eq!(
        peerings[0].vpc_id,
        Some("vpc-0123456789abcdef0".to_string())
    );
    assert_eq!(peerings[0].vpc_cidr, Some("10.0.0.0/16".to_string()));

    // Check second peering (pending)
    assert_eq!(peerings[1].peering_id, "peer_67890");
    assert_eq!(peerings[1].status, "pending");
    assert_eq!(peerings[1].provider_peering_id, None);
    assert_eq!(peerings[1].vpc_cidr, Some("172.16.0.0/16".to_string()));

    // Check third peering (failed)
    assert_eq!(peerings[2].peering_id, "peer_13579");
    assert_eq!(peerings[2].status, "failed");
    assert_eq!(peerings[2].vpc_cidr, Some("192.168.0.0/16".to_string()));
}

#[tokio::test]
async fn test_list_peerings_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/67890/peerings"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "peerings": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudPeeringHandler::new(client);

    let result = handler.list(67890).await;

    assert!(result.is_ok());
    let peerings = result.unwrap();
    assert_eq!(peerings.len(), 0);
}

#[tokio::test]
async fn test_list_peerings_no_peerings_field() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/67890/peerings"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({})))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudPeeringHandler::new(client);

    let result = handler.list(67890).await;

    assert!(result.is_ok());
    let peerings = result.unwrap();
    assert_eq!(peerings.len(), 0);
}

#[tokio::test]
async fn test_create_peering() {
    let mock_server = MockServer::start().await;

    let create_request = CreatePeeringRequest {
        subscription_id: 67890,
        provider: "AWS".to_string(),
        aws_account_id: Some("123456789012".to_string()),
        vpc_id: "vpc-0123456789abcdef0".to_string(),
        vpc_cidr: "10.0.0.0/16".to_string(),
        region: "us-east-1".to_string(),
    };

    Mock::given(method("POST"))
        .and(path("/subscriptions/67890/peerings"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(body_json(&create_request))
        .respond_with(created_response(pending_peering_response()))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudPeeringHandler::new(client);

    let result = handler.create(create_request).await;

    assert!(result.is_ok());
    let peering = result.unwrap();
    assert_eq!(peering.peering_id, "peer_pending");
    assert_eq!(peering.subscription_id, 67890);
    assert_eq!(peering.status, "pending-acceptance");
    assert_eq!(peering.vpc_id, Some("vpc-0123456789abcdef0".to_string()));
    assert_eq!(peering.vpc_cidr, Some("10.0.0.0/16".to_string()));
}

#[tokio::test]
async fn test_create_peering_without_aws_account() {
    let mock_server = MockServer::start().await;

    let create_request = CreatePeeringRequest {
        subscription_id: 67890,
        provider: "GCP".to_string(),
        aws_account_id: None,
        vpc_id: "vpc-gcp-123456789".to_string(),
        vpc_cidr: "10.1.0.0/16".to_string(),
        region: "us-central1".to_string(),
    };

    Mock::given(method("POST"))
        .and(path("/subscriptions/67890/peerings"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(body_json(&create_request))
        .respond_with(created_response(json!({
            "peering_id": "peer_gcp_123",
            "subscription_id": 67890,
            "status": "pending",
            "provider_peering_id": null,
            "aws_account_id": null,
            "vpc_id": "vpc-gcp-123456789",
            "vpc_cidr": "10.1.0.0/16",
            "region": "us-central1",
            "provider": "GCP"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudPeeringHandler::new(client);

    let result = handler.create(create_request).await;

    assert!(result.is_ok());
    let peering = result.unwrap();
    assert_eq!(peering.peering_id, "peer_gcp_123");
    assert_eq!(peering.aws_account_id, None);
    assert_eq!(peering.vpc_id, Some("vpc-gcp-123456789".to_string()));
}

#[tokio::test]
async fn test_create_peering_invalid_cidr() {
    let mock_server = MockServer::start().await;

    let create_request = CreatePeeringRequest {
        subscription_id: 67890,
        provider: "AWS".to_string(),
        aws_account_id: Some("123456789012".to_string()),
        vpc_id: "vpc-0123456789abcdef0".to_string(),
        vpc_cidr: "invalid-cidr".to_string(),
        region: "us-east-1".to_string(),
    };

    Mock::given(method("POST"))
        .and(path("/subscriptions/67890/peerings"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(body_json(&create_request))
        .respond_with(error_response(
            400,
            json!({
                "error": {
                    "type": "INVALID_REQUEST",
                    "status": 400,
                    "description": "Invalid VPC CIDR format"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudPeeringHandler::new(client);

    let result = handler.create(create_request).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_peering() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/67890/peerings/peer_12345"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(single_peering_response()))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudPeeringHandler::new(client);

    let result = handler.get(67890, "peer_12345").await;

    assert!(result.is_ok());
    let peering = result.unwrap();
    assert_eq!(peering.peering_id, "peer_12345");
    assert_eq!(peering.subscription_id, 67890);
    assert_eq!(peering.status, "active");
    assert_eq!(
        peering.provider_peering_id,
        Some("pcx-0123456789abcdef0".to_string())
    );
    assert_eq!(peering.aws_account_id, Some("123456789012".to_string()));
    assert_eq!(peering.vpc_id, Some("vpc-0123456789abcdef0".to_string()));
    assert_eq!(peering.vpc_cidr, Some("10.0.0.0/16".to_string()));

    // Check extra fields
    assert_eq!(peering.extra["region"], "us-east-1");
    assert_eq!(peering.extra["provider"], "AWS");
    assert_eq!(peering.extra["dnsResolution"], true);

    let route_table_ids = peering.extra["routeTableIds"].as_array().unwrap();
    assert_eq!(route_table_ids.len(), 2);
    assert_eq!(route_table_ids[0], "rtb-0123456789abcdef0");
    assert_eq!(route_table_ids[1], "rtb-0987654321fedcba0");
}

#[tokio::test]
async fn test_get_peering_failed() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/67890/peerings/peer_failed"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(failed_peering_response()))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudPeeringHandler::new(client);

    let result = handler.get(67890, "peer_failed").await;

    assert!(result.is_ok());
    let peering = result.unwrap();
    assert_eq!(peering.peering_id, "peer_failed");
    assert_eq!(peering.status, "failed");
    assert_eq!(peering.provider_peering_id, None);
    assert_eq!(
        peering.extra["error"],
        "VPC not found in the specified region"
    );
    assert_eq!(peering.extra["errorCode"], "VPC_NOT_FOUND");
}

#[tokio::test]
async fn test_get_peering_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/67890/peerings/nonexistent"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            404,
            json!({
                "error": {
                    "type": "RESOURCE_NOT_FOUND",
                    "status": 404,
                    "description": "Peering not found"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudPeeringHandler::new(client);

    let result = handler.get(67890, "nonexistent").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_peering() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/subscriptions/67890/peerings/peer_12345"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(no_content_response())
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudPeeringHandler::new(client);

    let result = handler.delete(67890, "peer_12345").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_peering_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/subscriptions/67890/peerings/nonexistent"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            404,
            json!({
                "error": {
                    "type": "RESOURCE_NOT_FOUND",
                    "status": 404,
                    "description": "Peering not found"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudPeeringHandler::new(client);

    let result = handler.delete(67890, "nonexistent").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_peering_conflict() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/subscriptions/67890/peerings/peer_active"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            409,
            json!({
                "error": {
                    "type": "CONFLICT",
                    "status": 409,
                    "description": "Cannot delete active peering connection"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudPeeringHandler::new(client);

    let result = handler.delete(67890, "peer_active").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_peerings_unauthorized() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/67890/peerings"))
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
    let handler = CloudPeeringHandler::new(client);

    let result = handler.list(67890).await;

    assert!(result.is_err());
}
