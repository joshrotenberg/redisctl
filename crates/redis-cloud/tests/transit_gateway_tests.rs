//! Transit Gateway endpoint tests for Redis Cloud

use redis_cloud::{CloudClient, CloudTransitGatewayHandler};
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
    CloudClient::builder()
        .api_key("test-api-key")
        .api_secret("test-secret-key")
        .base_url(base_url)
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_list_transit_gateways() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/100001/transitGateways"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "transitGateways": [
                {
                    "id": "tgw-12345",
                    "name": "Production TGW",
                    "awsAccountId": "123456789012",
                    "region": "us-east-1",
                    "status": "active",
                    "attachment": {
                        "status": "attached",
                        "attachmentId": "tgw-attach-abcdef123456",
                        "vpcId": "vpc-redis-prod",
                        "subnets": [
                            "subnet-redis-1a",
                            "subnet-redis-1b"
                        ]
                    },
                    "createdTimestamp": "2023-01-01T00:00:00Z",
                    "updatedTimestamp": "2023-12-01T10:00:00Z"
                },
                {
                    "id": "tgw-67890",
                    "name": "Development TGW",
                    "awsAccountId": "123456789012",
                    "region": "us-west-2",
                    "status": "pending",
                    "attachment": {
                        "status": "pending",
                        "attachmentId": null,
                        "vpcId": null,
                        "subnets": []
                    },
                    "createdTimestamp": "2023-11-01T00:00:00Z",
                    "updatedTimestamp": "2023-11-01T00:00:00Z"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudTransitGatewayHandler::new(client);
    let result = handler.list(100001).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let tgws = response["transitGateways"].as_array().unwrap();
    assert_eq!(tgws.len(), 2);
    assert_eq!(tgws[0]["id"], "tgw-12345");
    assert_eq!(tgws[0]["name"], "Production TGW");
    assert_eq!(tgws[0]["status"], "active");
    assert_eq!(tgws[0]["attachment"]["status"], "attached");
    assert_eq!(tgws[1]["id"], "tgw-67890");
    assert_eq!(tgws[1]["status"], "pending");
}

#[tokio::test]
async fn test_list_transit_gateways_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/100001/transitGateways"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "transitGateways": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudTransitGatewayHandler::new(client);
    let result = handler.list(100001).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let tgws = response["transitGateways"].as_array().unwrap();
    assert_eq!(tgws.len(), 0);
}

#[tokio::test]
async fn test_get_attachment() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(
            "/subscriptions/100001/transitGateways/tgw-12345/attachment",
        ))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "attachment": {
                "transitGatewayId": "tgw-12345",
                "attachmentId": "tgw-attach-abcdef123456",
                "status": "attached",
                "state": "available",
                "vpcId": "vpc-redis-prod",
                "subnets": [
                    "subnet-redis-1a",
                    "subnet-redis-1b"
                ],
                "routeTables": [
                    "rtb-main-12345"
                ],
                "cidrBlocks": [
                    "10.0.0.0/16"
                ],
                "createdTimestamp": "2023-01-01T01:00:00Z",
                "updatedTimestamp": "2023-01-01T01:15:00Z",
                "tags": {
                    "Environment": "Production",
                    "Project": "RedisCloud"
                }
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudTransitGatewayHandler::new(client);
    let result = handler.get_attachment(100001, "tgw-12345").await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let attachment = &response["attachment"];
    assert_eq!(attachment["transitGatewayId"], "tgw-12345");
    assert_eq!(attachment["attachmentId"], "tgw-attach-abcdef123456");
    assert_eq!(attachment["status"], "attached");
    assert_eq!(attachment["state"], "available");
    assert_eq!(attachment["vpcId"], "vpc-redis-prod");
    let subnets = attachment["subnets"].as_array().unwrap();
    assert_eq!(subnets.len(), 2);
    let cidrs = attachment["cidrBlocks"].as_array().unwrap();
    assert_eq!(cidrs[0], "10.0.0.0/16");
}

#[tokio::test]
async fn test_get_attachment_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(
            "/subscriptions/100001/transitGateways/tgw-nonexistent/attachment",
        ))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            404,
            json!({
                "error": {
                    "type": "NOT_FOUND",
                    "status": 404,
                    "description": "Transit gateway attachment not found"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudTransitGatewayHandler::new(client);
    let result = handler.get_attachment(100001, "tgw-nonexistent").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_attachment() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(
            "/subscriptions/100001/transitGateways/tgw-12345/attachment",
        ))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(accepted_response(json!({
            "taskId": "task_tgw_attach_123",
            "attachment": {
                "transitGatewayId": "tgw-12345",
                "status": "pending",
                "vpcId": "vpc-redis-new",
                "subnets": [
                    "subnet-redis-2a",
                    "subnet-redis-2b"
                ],
                "cidrBlocks": [
                    "10.1.0.0/16"
                ],
                "requestedTimestamp": "2023-12-01T12:00:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudTransitGatewayHandler::new(client);
    let attachment_request = json!({
        "vpcId": "vpc-redis-new",
        "subnets": [
            "subnet-redis-2a",
            "subnet-redis-2b"
        ],
        "cidrBlocks": [
            "10.1.0.0/16"
        ],
        "tags": {
            "Environment": "Staging",
            "Project": "RedisCloud"
        }
    });
    let result = handler
        .create_attachment(100001, "tgw-12345", attachment_request)
        .await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response["taskId"].is_string());
    let attachment = &response["attachment"];
    assert_eq!(attachment["transitGatewayId"], "tgw-12345");
    assert_eq!(attachment["status"], "pending");
    assert_eq!(attachment["vpcId"], "vpc-redis-new");
    let subnets = attachment["subnets"].as_array().unwrap();
    assert_eq!(subnets.len(), 2);
}

#[tokio::test]
async fn test_delete_attachment() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(
            "/subscriptions/100001/transitGateways/tgw-12345/attachment",
        ))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task_tgw_detach_456"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudTransitGatewayHandler::new(client);
    let result = handler.delete_attachment(100001, "tgw-12345").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_invitations() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/100001/transitGateways/invitations"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "invitations": [
                {
                    "id": "invite-tgw-001",
                    "transitGatewayId": "tgw-abcdef123456",
                    "transitGatewayOwnerAccountId": "123456789012",
                    "region": "us-east-1",
                    "status": "pending",
                    "invitedTimestamp": "2023-11-15T10:00:00Z",
                    "expiresAt": "2023-12-15T10:00:00Z",
                    "invitationMessage": "Please join our transit gateway for VPC connectivity",
                    "tags": {
                        "Project": "SharedInfrastructure"
                    }
                },
                {
                    "id": "invite-tgw-002",
                    "transitGatewayId": "tgw-xyz789012345",
                    "transitGatewayOwnerAccountId": "123456789013",
                    "region": "us-west-2",
                    "status": "expired",
                    "invitedTimestamp": "2023-10-01T10:00:00Z",
                    "expiresAt": "2023-11-01T10:00:00Z",
                    "invitationMessage": "Join development environment TGW",
                    "tags": {}
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudTransitGatewayHandler::new(client);
    let result = handler.list_invitations(100001).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let invitations = response["invitations"].as_array().unwrap();
    assert_eq!(invitations.len(), 2);
    assert_eq!(invitations[0]["id"], "invite-tgw-001");
    assert_eq!(invitations[0]["status"], "pending");
    assert_eq!(invitations[1]["id"], "invite-tgw-002");
    assert_eq!(invitations[1]["status"], "expired");
}

#[tokio::test]
async fn test_accept_invitation() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(
            "/subscriptions/100001/transitGateways/invitations/invite-tgw-001/accept",
        ))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(accepted_response(json!({
            "taskId": "task_accept_invite_789",
            "invitation": {
                "id": "invite-tgw-001",
                "status": "accepted",
                "acceptedTimestamp": "2023-12-01T12:00:00Z",
                "transitGateway": {
                    "id": "tgw-abcdef123456",
                    "name": "Shared Production TGW",
                    "ownerAccountId": "123456789012",
                    "region": "us-east-1"
                }
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudTransitGatewayHandler::new(client);
    let result = handler.accept_invitation(100001, "invite-tgw-001").await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response["taskId"].is_string());
    let invitation = &response["invitation"];
    assert_eq!(invitation["id"], "invite-tgw-001");
    assert_eq!(invitation["status"], "accepted");
    assert!(invitation["acceptedTimestamp"].is_string());
}

#[tokio::test]
async fn test_reject_invitation() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(
            "/subscriptions/100001/transitGateways/invitations/invite-tgw-001/reject",
        ))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "invitation": {
                "id": "invite-tgw-001",
                "status": "rejected",
                "rejectedTimestamp": "2023-12-01T12:00:00Z",
                "reason": "Not needed for current infrastructure"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudTransitGatewayHandler::new(client);
    let result = handler.reject_invitation(100001, "invite-tgw-001").await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let invitation = &response["invitation"];
    assert_eq!(invitation["status"], "rejected");
    assert!(invitation["rejectedTimestamp"].is_string());
}

#[tokio::test]
async fn test_list_regional_transit_gateways() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(
            "/subscriptions/100001/regions/us-east-1/transitGateways",
        ))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "transitGateways": [
                {
                    "id": "tgw-region-12345",
                    "name": "Regional Production TGW",
                    "awsAccountId": "123456789012",
                    "region": "us-east-1",
                    "status": "active",
                    "availabilityZones": [
                        "us-east-1a",
                        "us-east-1b",
                        "us-east-1c"
                    ],
                    "attachment": {
                        "status": "attached",
                        "attachmentId": "tgw-attach-regional-123",
                        "vpcId": "vpc-redis-regional",
                        "subnets": [
                            "subnet-redis-1a-regional",
                            "subnet-redis-1b-regional"
                        ]
                    },
                    "createdTimestamp": "2023-01-01T00:00:00Z"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudTransitGatewayHandler::new(client);
    let result = handler.list_regional(100001, "us-east-1").await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let tgws = response["transitGateways"].as_array().unwrap();
    assert_eq!(tgws.len(), 1);
    assert_eq!(tgws[0]["id"], "tgw-region-12345");
    assert_eq!(tgws[0]["region"], "us-east-1");
    let azs = tgws[0]["availabilityZones"].as_array().unwrap();
    assert_eq!(azs.len(), 3);
}

#[tokio::test]
async fn test_get_regional_attachment() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(
            "/subscriptions/100001/regions/us-east-1/transitGateways/tgw-region-12345/attachment",
        ))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "attachment": {
                "transitGatewayId": "tgw-region-12345",
                "attachmentId": "tgw-attach-regional-123",
                "region": "us-east-1",
                "status": "attached",
                "state": "available",
                "vpcId": "vpc-redis-regional",
                "subnets": [
                    "subnet-redis-1a-regional",
                    "subnet-redis-1b-regional"
                ],
                "availabilityZones": [
                    "us-east-1a",
                    "us-east-1b"
                ],
                "routeTables": [
                    "rtb-regional-12345"
                ],
                "cidrBlocks": [
                    "10.2.0.0/16"
                ]
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudTransitGatewayHandler::new(client);
    let result = handler
        .get_regional_attachment(100001, "us-east-1", "tgw-region-12345")
        .await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let attachment = &response["attachment"];
    assert_eq!(attachment["transitGatewayId"], "tgw-region-12345");
    assert_eq!(attachment["region"], "us-east-1");
    assert_eq!(attachment["status"], "attached");
    let azs = attachment["availabilityZones"].as_array().unwrap();
    assert_eq!(azs.len(), 2);
}

#[tokio::test]
async fn test_create_regional_attachment() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(
            "/subscriptions/100001/regions/us-west-2/transitGateways/tgw-region-67890/attachment",
        ))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(accepted_response(json!({
            "taskId": "task_regional_attach_999",
            "attachment": {
                "transitGatewayId": "tgw-region-67890",
                "region": "us-west-2",
                "status": "pending",
                "vpcId": "vpc-redis-west",
                "subnets": [
                    "subnet-redis-2a-west",
                    "subnet-redis-2b-west"
                ],
                "availabilityZones": [
                    "us-west-2a",
                    "us-west-2b"
                ],
                "cidrBlocks": [
                    "10.3.0.0/16"
                ]
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudTransitGatewayHandler::new(client);
    let attachment_request = json!({
        "vpcId": "vpc-redis-west",
        "subnets": [
            "subnet-redis-2a-west",
            "subnet-redis-2b-west"
        ],
        "availabilityZones": [
            "us-west-2a",
            "us-west-2b"
        ],
        "cidrBlocks": [
            "10.3.0.0/16"
        ]
    });
    let result = handler
        .create_regional_attachment(100001, "us-west-2", "tgw-region-67890", attachment_request)
        .await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response["taskId"].is_string());
    let attachment = &response["attachment"];
    assert_eq!(attachment["region"], "us-west-2");
    assert_eq!(attachment["status"], "pending");
}

#[tokio::test]
async fn test_delete_regional_attachment() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path(
            "/subscriptions/100001/regions/us-east-1/transitGateways/tgw-region-12345/attachment",
        ))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task_regional_detach_888"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudTransitGatewayHandler::new(client);
    let result = handler
        .delete_regional_attachment(100001, "us-east-1", "tgw-region-12345")
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_regional_invitations() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path(
            "/subscriptions/100001/regions/us-east-1/transitGateways/invitations",
        ))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "invitations": [
                {
                    "id": "invite-regional-001",
                    "transitGatewayId": "tgw-regional-abc123",
                    "region": "us-east-1",
                    "transitGatewayOwnerAccountId": "123456789014",
                    "status": "pending",
                    "invitedTimestamp": "2023-12-01T09:00:00Z",
                    "expiresAt": "2024-01-01T09:00:00Z",
                    "invitationMessage": "Join regional infrastructure TGW"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudTransitGatewayHandler::new(client);
    let result = handler.list_regional_invitations(100001, "us-east-1").await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let invitations = response["invitations"].as_array().unwrap();
    assert_eq!(invitations.len(), 1);
    assert_eq!(invitations[0]["id"], "invite-regional-001");
    assert_eq!(invitations[0]["region"], "us-east-1");
    assert_eq!(invitations[0]["status"], "pending");
}

#[tokio::test]
async fn test_accept_regional_invitation() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/subscriptions/100001/regions/us-east-1/transitGateways/invitations/invite-regional-001/accept"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(accepted_response(json!({
            "taskId": "task_accept_regional_777",
            "invitation": {
                "id": "invite-regional-001",
                "region": "us-east-1",
                "status": "accepted",
                "acceptedTimestamp": "2023-12-01T12:00:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudTransitGatewayHandler::new(client);
    let result = handler
        .accept_regional_invitation(100001, "us-east-1", "invite-regional-001")
        .await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response["taskId"].is_string());
    let invitation = &response["invitation"];
    assert_eq!(invitation["status"], "accepted");
    assert_eq!(invitation["region"], "us-east-1");
}

#[tokio::test]
async fn test_reject_regional_invitation() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/subscriptions/100001/regions/us-east-1/transitGateways/invitations/invite-regional-001/reject"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "invitation": {
                "id": "invite-regional-001",
                "region": "us-east-1",
                "status": "rejected",
                "rejectedTimestamp": "2023-12-01T12:00:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudTransitGatewayHandler::new(client);
    let result = handler
        .reject_regional_invitation(100001, "us-east-1", "invite-regional-001")
        .await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let invitation = &response["invitation"];
    assert_eq!(invitation["status"], "rejected");
    assert_eq!(invitation["region"], "us-east-1");
}

#[tokio::test]
async fn test_transit_gateway_error_unauthorized() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/100001/transitGateways"))
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
    let handler = CloudTransitGatewayHandler::new(client);
    let result = handler.list(100001).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_subscription_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/999999/transitGateways"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            404,
            json!({
                "error": {
                    "type": "NOT_FOUND",
                    "status": 404,
                    "description": "Subscription not found"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudTransitGatewayHandler::new(client);
    let result = handler.list(999999).await;

    assert!(result.is_err());
}
