use redis_cloud::{CloudAccountHandler, CloudClient};
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_cloud_accounts() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/cloud-accounts"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "accountId": 123,
            "links": [
                {
                    "href": "https://api.redislabs.com/v1/cloud-accounts/1",
                    "type": "GET",
                    "rel": "cloud-account"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = CloudAccountHandler::new(client);
    let result = handler.get_cloud_accounts().await.unwrap();

    assert_eq!(result.account_id, Some(123));
    assert!(result.links.is_some());
}

#[tokio::test]
async fn test_get_cloud_account_by_id() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/cloud-accounts/456"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 456,
            "name": "Test Cloud Account",
            "status": "active",
            "accessKeyId": "AKIAIOSFODNN7EXAMPLE",
            "signInLoginUrl": "https://console.aws.amazon.com",
            "provider": "AWS",
            "links": [
                {
                    "href": "https://api.redislabs.com/v1/cloud-accounts/456",
                    "type": "GET",
                    "rel": "self"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = CloudAccountHandler::new(client);
    let result = handler.get_cloud_account_by_id(456).await.unwrap();

    assert_eq!(result.id, Some(456));
    assert_eq!(result.name, Some("Test Cloud Account".to_string()));
    assert_eq!(result.status, Some("active".to_string()));
    assert_eq!(result.provider, Some("AWS".to_string()));
}

#[tokio::test]
async fn test_create_cloud_account() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/cloud-accounts"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-create-123",
            "commandType": "CREATE_CLOUD_ACCOUNT",
            "status": "processing",
            "description": "Creating cloud account",
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

    let handler = CloudAccountHandler::new(client);
    let request = redis_cloud::cloud_accounts::CloudAccountCreateRequest {
        name: "New Cloud Account".to_string(),
        provider: Some("AWS".to_string()),
        access_key_id: "AKIAIOSFODNN7EXAMPLE".to_string(),
        access_secret_key: "secret-key".to_string(),
        console_username: "admin".to_string(),
        console_password: "password".to_string(),
        sign_in_login_url: "https://console.aws.amazon.com".to_string(),
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler.create_cloud_account(&request).await.unwrap();
    assert_eq!(result.task_id, Some("task-create-123".to_string()));
    assert_eq!(
        result.command_type,
        Some("CREATE_CLOUD_ACCOUNT".to_string())
    );
    assert_eq!(result.status, Some("processing".to_string()));
}

#[tokio::test]
async fn test_update_cloud_account() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/cloud-accounts/456"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-update-456",
            "commandType": "UPDATE_CLOUD_ACCOUNT",
            "status": "processing",
            "description": "Updating cloud account"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = CloudAccountHandler::new(client);
    let request = redis_cloud::cloud_accounts::CloudAccountUpdateRequest {
        name: Some("Updated Cloud Account".to_string()),
        cloud_account_id: None,
        access_key_id: "AKIAIOSFODNN7UPDATED".to_string(),
        access_secret_key: "updated-secret".to_string(),
        console_username: "admin-updated".to_string(),
        console_password: "password-updated".to_string(),
        sign_in_login_url: Some("https://console.aws.amazon.com/updated".to_string()),
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler.update_cloud_account(456, &request).await.unwrap();
    assert_eq!(result.task_id, Some("task-update-456".to_string()));
    assert_eq!(
        result.command_type,
        Some("UPDATE_CLOUD_ACCOUNT".to_string())
    );
}

#[tokio::test]
async fn test_delete_cloud_account() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/cloud-accounts/456"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-delete-456",
            "commandType": "DELETE_CLOUD_ACCOUNT",
            "status": "processing",
            "description": "Deleting cloud account"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = CloudAccountHandler::new(client);
    let result = handler.delete_cloud_account(456).await.unwrap();

    assert_eq!(result.task_id, Some("task-delete-456".to_string()));
    assert_eq!(
        result.command_type,
        Some("DELETE_CLOUD_ACCOUNT".to_string())
    );
}

#[tokio::test]
async fn test_create_cloud_account_without_provider() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/cloud-accounts"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-default-provider",
            "commandType": "CREATE_CLOUD_ACCOUNT",
            "status": "processing",
            "description": "Creating cloud account with default provider (AWS)"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = CloudAccountHandler::new(client);
    let request = redis_cloud::cloud_accounts::CloudAccountCreateRequest {
        name: "Default Provider Account".to_string(),
        provider: None, // Should default to AWS
        access_key_id: "AKIAIOSFODNN7EXAMPLE".to_string(),
        access_secret_key: "secret-key".to_string(),
        console_username: "admin".to_string(),
        console_password: "password".to_string(),
        sign_in_login_url: "https://console.aws.amazon.com".to_string(),
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler.create_cloud_account(&request).await.unwrap();
    assert_eq!(result.task_id, Some("task-default-provider".to_string()));
    assert_eq!(
        result.description,
        Some("Creating cloud account with default provider (AWS)".to_string())
    );
}

#[tokio::test]
async fn test_create_gcp_cloud_account() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/cloud-accounts"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-gcp-123",
            "commandType": "CREATE_CLOUD_ACCOUNT",
            "status": "processing",
            "description": "Creating GCP cloud account",
            "timestamp": "2024-01-02T10:00:00Z",
            "response": {
                "resourceId": 890,
                "additionalInfo": "GCP account validation in progress"
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

    let handler = CloudAccountHandler::new(client);
    let request = redis_cloud::cloud_accounts::CloudAccountCreateRequest {
        name: "GCP Cloud Account".to_string(),
        provider: Some("GCP".to_string()),
        access_key_id: "gcp-key-id".to_string(),
        access_secret_key: "gcp-secret-key".to_string(),
        console_username: "gcp-admin".to_string(),
        console_password: "gcp-password".to_string(),
        sign_in_login_url: "https://console.cloud.google.com".to_string(),
        command_type: Some("CREATE_CLOUD_ACCOUNT".to_string()),
        extra: json!({
            "project_id": "my-gcp-project"
        }),
    };

    let result = handler.create_cloud_account(&request).await.unwrap();
    assert_eq!(result.task_id, Some("task-gcp-123".to_string()));
    assert_eq!(
        result.command_type,
        Some("CREATE_CLOUD_ACCOUNT".to_string())
    );
    assert_eq!(result.status, Some("processing".to_string()));
    assert!(result.response.is_some());

    let response = result.response.unwrap();
    assert_eq!(response.resource_id, Some(890));
    assert_eq!(
        response.additional_info,
        Some("GCP account validation in progress".to_string())
    );
}

#[tokio::test]
async fn test_create_azure_cloud_account() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/cloud-accounts"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-azure-456",
            "commandType": "CREATE_CLOUD_ACCOUNT",
            "status": "processing",
            "description": "Creating Azure cloud account"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = CloudAccountHandler::new(client);
    let request = redis_cloud::cloud_accounts::CloudAccountCreateRequest {
        name: "Azure Cloud Account".to_string(),
        provider: Some("Azure".to_string()),
        access_key_id: "azure-client-id".to_string(),
        access_secret_key: "azure-client-secret".to_string(),
        console_username: "azure-admin".to_string(),
        console_password: "azure-password".to_string(),
        sign_in_login_url: "https://portal.azure.com".to_string(),
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler.create_cloud_account(&request).await.unwrap();
    assert_eq!(result.task_id, Some("task-azure-456".to_string()));
    assert_eq!(
        result.command_type,
        Some("CREATE_CLOUD_ACCOUNT".to_string())
    );
}

#[tokio::test]
async fn test_get_cloud_accounts_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/cloud-accounts"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "accountId": 123,
            "links": []
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = CloudAccountHandler::new(client);
    let result = handler.get_cloud_accounts().await.unwrap();

    assert_eq!(result.account_id, Some(123));
    assert!(result.links.is_some());
    let links = result.links.unwrap();
    assert_eq!(links.len(), 0);
}

#[tokio::test]
async fn test_get_cloud_accounts_multiple() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/cloud-accounts"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "accountId": 123,
            "links": [
                {
                    "href": "https://api.redislabs.com/v1/cloud-accounts/1",
                    "type": "GET",
                    "rel": "cloud-account",
                    "title": "AWS Production"
                },
                {
                    "href": "https://api.redislabs.com/v1/cloud-accounts/2",
                    "type": "GET",
                    "rel": "cloud-account",
                    "title": "GCP Development"
                },
                {
                    "href": "https://api.redislabs.com/v1/cloud-accounts/3",
                    "type": "GET",
                    "rel": "cloud-account",
                    "title": "Azure Testing"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = CloudAccountHandler::new(client);
    let result = handler.get_cloud_accounts().await.unwrap();

    assert_eq!(result.account_id, Some(123));
    assert!(result.links.is_some());
    let links = result.links.unwrap();
    assert_eq!(links.len(), 3);
}

#[tokio::test]
async fn test_get_cloud_account_by_id_gcp() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/cloud-accounts/789"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 789,
            "name": "GCP Development Account",
            "status": "active",
            "accessKeyId": "gcp-service-account-email",
            "signInLoginUrl": "https://console.cloud.google.com",
            "provider": "GCP",
            "links": [
                {
                    "href": "https://api.redislabs.com/v1/cloud-accounts/789",
                    "type": "GET",
                    "rel": "self"
                },
                {
                    "href": "https://api.redislabs.com/v1/cloud-accounts/789",
                    "type": "PUT",
                    "rel": "update"
                },
                {
                    "href": "https://api.redislabs.com/v1/cloud-accounts/789",
                    "type": "DELETE",
                    "rel": "delete"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = CloudAccountHandler::new(client);
    let result = handler.get_cloud_account_by_id(789).await.unwrap();

    assert_eq!(result.id, Some(789));
    assert_eq!(result.name, Some("GCP Development Account".to_string()));
    assert_eq!(result.status, Some("active".to_string()));
    assert_eq!(result.provider, Some("GCP".to_string()));
    assert_eq!(
        result.access_key_id,
        Some("gcp-service-account-email".to_string())
    );
    assert_eq!(
        result.sign_in_login_url,
        Some("https://console.cloud.google.com".to_string())
    );
    assert!(result.links.is_some());

    let links = result.links.unwrap();
    assert_eq!(links.len(), 3);
}

#[tokio::test]
async fn test_get_cloud_account_by_id_with_extra_fields() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/cloud-accounts/456"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 456,
            "name": "Test Cloud Account",
            "status": "active",
            "accessKeyId": "AKIAIOSFODNN7EXAMPLE",
            "signInLoginUrl": "https://console.aws.amazon.com",
            "provider": "AWS",
            "region": "us-west-2",
            "accountNumber": "123456789012",
            "roleName": "RedisLabsRole",
            "externalId": "external-123",
            "customMetadata": {
                "environment": "production",
                "team": "platform"
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

    let handler = CloudAccountHandler::new(client);
    let result = handler.get_cloud_account_by_id(456).await.unwrap();

    assert_eq!(result.id, Some(456));
    assert_eq!(result.name, Some("Test Cloud Account".to_string()));
    assert_eq!(result.status, Some("active".to_string()));
    assert_eq!(result.provider, Some("AWS".to_string()));

    // Test that extra fields are captured
    assert!(result.extra.get("region").is_some());
    assert!(result.extra.get("accountNumber").is_some());
    assert!(result.extra.get("roleName").is_some());
    assert!(result.extra.get("customMetadata").is_some());

    if let Some(region) = result.extra.get("region") {
        assert_eq!(region.as_str().unwrap(), "us-west-2");
    }

    if let Some(account_number) = result.extra.get("accountNumber") {
        assert_eq!(account_number.as_str().unwrap(), "123456789012");
    }
}

#[tokio::test]
async fn test_update_cloud_account_minimal() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/cloud-accounts/456"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-minimal-update",
            "commandType": "UPDATE_CLOUD_ACCOUNT",
            "status": "processing",
            "description": "Updating cloud account credentials only"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = CloudAccountHandler::new(client);
    let request = redis_cloud::cloud_accounts::CloudAccountUpdateRequest {
        name: None, // Not updating name
        cloud_account_id: Some(456),
        access_key_id: "NEW-ACCESS-KEY".to_string(),
        access_secret_key: "new-secret-key".to_string(),
        console_username: "updated-admin".to_string(),
        console_password: "updated-password".to_string(),
        sign_in_login_url: None, // Not updating URL
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler.update_cloud_account(456, &request).await.unwrap();
    assert_eq!(result.task_id, Some("task-minimal-update".to_string()));
    assert_eq!(
        result.command_type,
        Some("UPDATE_CLOUD_ACCOUNT".to_string())
    );
    assert_eq!(
        result.description,
        Some("Updating cloud account credentials only".to_string())
    );
}

#[tokio::test]
async fn test_task_state_update_with_failed_status() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/cloud-accounts"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-failed-123",
            "commandType": "CREATE_CLOUD_ACCOUNT",
            "status": "failed",
            "description": "Failed to create cloud account",
            "timestamp": "2024-01-01T15:00:00Z",
            "response": {
                "error": "Invalid credentials",
                "additionalInfo": "Access key validation failed"
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

    let handler = CloudAccountHandler::new(client);
    let request = redis_cloud::cloud_accounts::CloudAccountCreateRequest {
        name: "Failed Account".to_string(),
        provider: Some("AWS".to_string()),
        access_key_id: "INVALID-KEY".to_string(),
        access_secret_key: "invalid-secret".to_string(),
        console_username: "admin".to_string(),
        console_password: "password".to_string(),
        sign_in_login_url: "https://console.aws.amazon.com".to_string(),
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler.create_cloud_account(&request).await.unwrap();
    assert_eq!(result.task_id, Some("task-failed-123".to_string()));
    assert_eq!(result.status, Some("failed".to_string()));
    assert_eq!(
        result.description,
        Some("Failed to create cloud account".to_string())
    );
    assert!(result.response.is_some());

    let response = result.response.unwrap();
    assert_eq!(response.error, Some("Invalid credentials".to_string()));
    assert_eq!(
        response.additional_info,
        Some("Access key validation failed".to_string())
    );
}

// Error handling tests

#[tokio::test]
async fn test_error_handling_401() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/cloud-accounts"))
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

    let handler = CloudAccountHandler::new(client);
    let result = handler.get_cloud_accounts().await;

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
        .and(path("/cloud-accounts/456"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(403).set_body_json(json!({
            "error": "Forbidden: Cannot delete cloud account with active subscriptions"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = CloudAccountHandler::new(client);
    let result = handler.delete_cloud_account(456).await;

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
        .and(path("/cloud-accounts/999"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "error": "Cloud account not found"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = CloudAccountHandler::new(client);
    let result = handler.get_cloud_account_by_id(999).await;

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

    Mock::given(method("PUT"))
        .and(path("/cloud-accounts/456"))
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

    let handler = CloudAccountHandler::new(client);
    let request = redis_cloud::cloud_accounts::CloudAccountUpdateRequest {
        name: Some("Updated Account".to_string()),
        cloud_account_id: None,
        access_key_id: "ACCESS-KEY".to_string(),
        access_secret_key: "secret-key".to_string(),
        console_username: "admin".to_string(),
        console_password: "password".to_string(),
        sign_in_login_url: None,
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler.update_cloud_account(456, &request).await;

    assert!(result.is_err());
    match result {
        Err(redis_cloud::CloudError::InternalServerError { .. }) => {}
        _ => panic!("Expected InternalServerError error"),
    }
}
