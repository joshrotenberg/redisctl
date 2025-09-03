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
