//! Cloud accounts endpoint tests for Redis Cloud

use redis_cloud::{CloudAccountsHandler, CloudClient};
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

// Helper function to create mock cloud accounts list response
fn cloud_accounts_list_response() -> serde_json::Value {
    json!({
        "cloudAccounts": [
            {
                "id": 101,
                "name": "Production AWS Account",
                "provider": "AWS",
                "providerId": "123456789012",
                "status": "active",
                "region": "us-east-1",
                "createdAt": "2023-01-01T10:00:00Z",
                "updatedAt": "2023-01-01T10:30:00Z",
                "credentials": {
                    "type": "assume_role",
                    "roleArn": "arn:aws:iam::123456789012:role/RedisCloudAccess",
                    "externalId": "redis-cloud-12345"
                },
                "permissions": ["read", "write", "manage"],
                "tags": {
                    "Environment": "production",
                    "Team": "platform"
                }
            },
            {
                "id": 102,
                "name": "Staging GCP Account",
                "provider": "GCP",
                "providerId": "my-gcp-project-staging",
                "status": "active",
                "region": "us-central1",
                "createdAt": "2023-01-01T11:00:00Z",
                "updatedAt": "2023-01-01T11:15:00Z",
                "credentials": {
                    "type": "service_account",
                    "keyId": "gcp-key-123456",
                    "projectId": "my-gcp-project-staging"
                },
                "permissions": ["read", "write"],
                "tags": {
                    "Environment": "staging",
                    "Team": "development"
                }
            },
            {
                "id": 103,
                "name": "Development Azure Account",
                "provider": "Azure",
                "providerId": "11111111-2222-3333-4444-555555555555",
                "status": "inactive",
                "region": "eastus",
                "createdAt": "2023-01-01T12:00:00Z",
                "updatedAt": "2023-01-01T12:05:00Z",
                "credentials": {
                    "type": "service_principal",
                    "clientId": "azure-client-123",
                    "subscriptionId": "azure-sub-456"
                },
                "permissions": ["read"],
                "tags": {
                    "Environment": "development"
                },
                "error": "Authentication failed"
            }
        ],
        "pagination": {
            "total": 3,
            "limit": 10,
            "offset": 0,
            "hasMore": false
        }
    })
}

// Helper function to create mock single cloud account response
fn single_cloud_account_response() -> serde_json::Value {
    json!({
        "id": 101,
        "name": "Production AWS Account",
        "provider": "AWS",
        "providerId": "123456789012",
        "status": "active",
        "region": "us-east-1",
        "createdAt": "2023-01-01T10:00:00Z",
        "updatedAt": "2023-01-01T10:30:00Z",
        "credentials": {
            "type": "assume_role",
            "roleArn": "arn:aws:iam::123456789012:role/RedisCloudAccess",
            "externalId": "redis-cloud-12345"
        },
        "permissions": ["read", "write", "manage"],
        "tags": {
            "Environment": "production",
            "Team": "platform"
        },
        "resources": {
            "vpcs": [
                {
                    "vpcId": "vpc-0123456789abcdef0",
                    "cidr": "10.0.0.0/16",
                    "region": "us-east-1"
                }
            ],
            "subnets": [
                {
                    "subnetId": "subnet-0123456789abcdef0",
                    "vpcId": "vpc-0123456789abcdef0",
                    "cidr": "10.0.1.0/24",
                    "availabilityZone": "us-east-1a"
                }
            ]
        }
    })
}

// Helper function to create new cloud account response
fn new_cloud_account_response() -> serde_json::Value {
    json!({
        "id": 104,
        "name": "New Test Account",
        "provider": "AWS",
        "providerId": "999888777666",
        "status": "pending",
        "region": "us-west-2",
        "createdAt": "2023-01-01T15:00:00Z",
        "updatedAt": "2023-01-01T15:00:00Z",
        "credentials": {
            "type": "assume_role",
            "roleArn": "arn:aws:iam::999888777666:role/RedisCloudAccess",
            "externalId": "redis-cloud-67890"
        },
        "permissions": ["read", "write"],
        "tags": {
            "Environment": "test"
        }
    })
}

#[tokio::test]
async fn test_list_cloud_accounts() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/cloud-accounts"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(cloud_accounts_list_response()))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAccountsHandler::new(client);

    let result = handler.list().await;

    assert!(result.is_ok());
    let accounts_vec = result.unwrap();
    let response = json!({"cloudAccounts": accounts_vec});
    let accounts = response["cloudAccounts"].as_array().unwrap();
    assert_eq!(accounts.len(), 3);

    // Check first account (AWS)
    assert_eq!(accounts[0]["id"], 101);
    assert_eq!(accounts[0]["name"], "Production AWS Account");
    assert_eq!(accounts[0]["provider"], "AWS");
    assert_eq!(accounts[0]["providerId"], "123456789012");
    assert_eq!(accounts[0]["status"], "active");
    assert_eq!(accounts[0]["region"], "us-east-1");

    let credentials = &accounts[0]["credentials"];
    assert_eq!(credentials["type"], "assume_role");
    assert_eq!(
        credentials["roleArn"],
        "arn:aws:iam::123456789012:role/RedisCloudAccess"
    );

    let permissions = accounts[0]["permissions"].as_array().unwrap();
    assert_eq!(permissions.len(), 3);
    assert!(permissions.contains(&json!("read")));
    assert!(permissions.contains(&json!("write")));
    assert!(permissions.contains(&json!("manage")));

    // Check second account (GCP)
    assert_eq!(accounts[1]["id"], 102);
    assert_eq!(accounts[1]["provider"], "GCP");
    assert_eq!(accounts[1]["credentials"]["type"], "service_account");

    // Check third account (Azure, inactive)
    assert_eq!(accounts[2]["id"], 103);
    assert_eq!(accounts[2]["provider"], "Azure");
    assert_eq!(accounts[2]["status"], "inactive");
    assert_eq!(accounts[2]["error"], "Authentication failed");

    // Check pagination
    // pagination omitted in typed results
}

#[tokio::test]
async fn test_list_cloud_accounts_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/cloud-accounts"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "cloudAccounts": [],
            "pagination": {
                "total": 0,
                "limit": 10,
                "offset": 0,
                "hasMore": false
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAccountsHandler::new(client);

    let result = handler.list().await;

    assert!(result.is_ok());
    let accounts_vec = result.unwrap();
    let response = json!({"cloudAccounts": accounts_vec});
    let accounts = response["cloudAccounts"].as_array().unwrap();
    assert_eq!(accounts.len(), 0);
}

#[tokio::test]
async fn test_get_cloud_account() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/cloud-accounts/101"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(single_cloud_account_response()))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAccountsHandler::new(client);

    let result = handler.get(101).await;

    assert!(result.is_ok());
    let account = serde_json::to_value(result.unwrap()).unwrap();
    assert_eq!(account["id"], 101);
    assert_eq!(account["name"], "Production AWS Account");
    assert_eq!(account["provider"], "AWS");
    assert_eq!(account["status"], "active");

    // Check detailed resources
    let resources = &account["resources"];
    let vpcs = resources["vpcs"].as_array().unwrap();
    assert_eq!(vpcs.len(), 1);
    assert_eq!(vpcs[0]["vpcId"], "vpc-0123456789abcdef0");
    assert_eq!(vpcs[0]["cidr"], "10.0.0.0/16");

    let subnets = resources["subnets"].as_array().unwrap();
    assert_eq!(subnets.len(), 1);
    assert_eq!(subnets[0]["subnetId"], "subnet-0123456789abcdef0");
    assert_eq!(subnets[0]["availabilityZone"], "us-east-1a");
}

#[tokio::test]
async fn test_get_cloud_account_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/cloud-accounts/999"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            404,
            json!({
                "error": {
                    "type": "RESOURCE_NOT_FOUND",
                    "status": 404,
                    "description": "Cloud account not found"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAccountsHandler::new(client);

    let result = handler.get(999).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_cloud_account() {
    let mock_server = MockServer::start().await;

    let create_request = json!({
        "name": "New Test Account",
        "provider": "AWS",
        "providerId": "999888777666",
        "region": "us-west-2",
        "credentials": {
            "type": "assume_role",
            "roleArn": "arn:aws:iam::999888777666:role/RedisCloudAccess",
            "externalId": "redis-cloud-67890"
        },
        "permissions": ["read", "write"],
        "tags": {
            "Environment": "test"
        }
    });

    Mock::given(method("POST"))
        .and(path("/cloud-accounts"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(body_json(&create_request))
        .respond_with(created_response(new_cloud_account_response()))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAccountsHandler::new(client);

    let result = handler.create(create_request).await;

    assert!(result.is_ok());
    let account = serde_json::to_value(result.unwrap()).unwrap();
    assert_eq!(account["id"], 104);
    assert_eq!(account["name"], "New Test Account");
    assert_eq!(account["provider"], "AWS");
    assert_eq!(account["status"], "pending");
    assert_eq!(account["region"], "us-west-2");
}

#[tokio::test]
async fn test_create_cloud_account_invalid_credentials() {
    let mock_server = MockServer::start().await;

    let create_request = json!({
        "name": "Invalid Account",
        "provider": "AWS",
        "providerId": "invalid-id",
        "region": "us-west-2",
        "credentials": {
            "type": "assume_role",
            "roleArn": "invalid-arn"
        }
    });

    Mock::given(method("POST"))
        .and(path("/cloud-accounts"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(body_json(&create_request))
        .respond_with(error_response(
            400,
            json!({
                "error": {
                    "type": "INVALID_REQUEST",
                    "status": 400,
                    "description": "Invalid role ARN format"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAccountsHandler::new(client);

    let result = handler.create(create_request).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_cloud_account() {
    let mock_server = MockServer::start().await;

    let update_request = json!({
        "name": "Updated Production AWS Account",
        "permissions": ["read", "write", "manage", "admin"],
        "tags": {
            "Environment": "production",
            "Team": "platform",
            "Updated": "true"
        }
    });

    let updated_response = json!({
        "id": 101,
        "name": "Updated Production AWS Account",
        "provider": "AWS",
        "providerId": "123456789012",
        "status": "active",
        "region": "us-east-1",
        "updatedAt": "2023-01-01T16:00:00Z",
        "permissions": ["read", "write", "manage", "admin"],
        "tags": {
            "Environment": "production",
            "Team": "platform",
            "Updated": "true"
        }
    });

    Mock::given(method("PUT"))
        .and(path("/cloud-accounts/101"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(body_json(&update_request))
        .respond_with(success_response(updated_response))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAccountsHandler::new(client);

    let result = handler.update(101, update_request).await;

    assert!(result.is_ok());
    let account = serde_json::to_value(result.unwrap()).unwrap();
    assert_eq!(account["id"], 101);
    assert_eq!(account["name"], "Updated Production AWS Account");

    let permissions = account["permissions"].as_array().unwrap();
    assert_eq!(permissions.len(), 4);
    assert!(permissions.contains(&json!("admin")));

    assert_eq!(account["tags"]["Updated"], "true");
}

#[tokio::test]
async fn test_update_cloud_account_not_found() {
    let mock_server = MockServer::start().await;

    let update_request = json!({
        "name": "Updated Account"
    });

    Mock::given(method("PUT"))
        .and(path("/cloud-accounts/999"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(body_json(&update_request))
        .respond_with(error_response(
            404,
            json!({
                "error": {
                    "type": "RESOURCE_NOT_FOUND",
                    "status": 404,
                    "description": "Cloud account not found"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAccountsHandler::new(client);

    let result = handler.update(999, update_request).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_cloud_account() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/cloud-accounts/101"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(no_content_response())
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAccountsHandler::new(client);

    let result = handler.delete(101).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["message"], "Cloud account 101 deleted");
}

#[tokio::test]
async fn test_delete_cloud_account_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/cloud-accounts/999"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            404,
            json!({
                "error": {
                    "type": "RESOURCE_NOT_FOUND",
                    "status": 404,
                    "description": "Cloud account not found"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAccountsHandler::new(client);

    let result = handler.delete(999).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_cloud_account_with_dependencies() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/cloud-accounts/101"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            409,
            json!({
                "error": {
                    "type": "CONFLICT",
                    "status": 409,
                    "description": "Cannot delete cloud account with active subscriptions"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAccountsHandler::new(client);

    let result = handler.delete(101).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_cloud_accounts_forbidden() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/cloud-accounts"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            403,
            json!({
                "error": {
                    "type": "FORBIDDEN",
                    "status": 403,
                    "description": "Insufficient permissions to access cloud accounts"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAccountsHandler::new(client);

    let result = handler.list().await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_cloud_account_gcp() {
    let mock_server = MockServer::start().await;

    let create_request = json!({
        "name": "New GCP Account",
        "provider": "GCP",
        "providerId": "my-gcp-project-new",
        "region": "us-central1",
        "credentials": {
            "type": "service_account",
            "keyId": "gcp-key-new",
            "projectId": "my-gcp-project-new"
        },
        "permissions": ["read", "write"]
    });

    let gcp_response = json!({
        "id": 105,
        "name": "New GCP Account",
        "provider": "GCP",
        "providerId": "my-gcp-project-new",
        "status": "pending",
        "region": "us-central1",
        "credentials": {
            "type": "service_account",
            "keyId": "gcp-key-new",
            "projectId": "my-gcp-project-new"
        },
        "permissions": ["read", "write"]
    });

    Mock::given(method("POST"))
        .and(path("/cloud-accounts"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .and(body_json(&create_request))
        .respond_with(created_response(gcp_response))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudAccountsHandler::new(client);

    let result = handler.create(create_request).await;

    assert!(result.is_ok());
    let account = serde_json::to_value(result.unwrap()).unwrap();
    assert_eq!(account["id"], 105);
    assert_eq!(account["provider"], "GCP");
    assert_eq!(account["credentials"]["type"], "service_account");
}
