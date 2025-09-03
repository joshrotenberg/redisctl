use redis_cloud::{CloudClient, UserHandler};
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_all_users() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "account": 123
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = UserHandler::new(client);
    let result = handler.get_all_users().await.unwrap();

    assert_eq!(result.account, Some(123));
}

#[tokio::test]
async fn test_get_user_by_id() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users/456"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 456,
            "name": "John Doe",
            "email": "john@example.com",
            "role": "admin",
            "signUp": "2023-01-01T00:00:00Z",
            "userType": "regular",
            "hasApiKey": true,
            "options": {
                "billing": true,
                "emailAlerts": true,
                "operationalEmails": false,
                "mfaEnabled": true
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

    let handler = UserHandler::new(client);
    let result = handler.get_user_by_id(456).await.unwrap();

    assert_eq!(result.id, Some(456));
    assert_eq!(result.name, Some("John Doe".to_string()));
    assert_eq!(result.email, Some("john@example.com".to_string()));
    assert_eq!(result.role, Some("admin".to_string()));
    assert!(result.options.is_some());
}

#[tokio::test]
async fn test_update_user() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/users/456"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "update-user-task",
            "commandType": "UPDATE_USER",
            "status": "processing",
            "description": "Updating user"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = UserHandler::new(client);
    let request = redis_cloud::users::AccountUserUpdateRequest {
        user_id: None,
        name: "Jane Doe".to_string(),
        role: Some("admin".to_string()),
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler.update_user(456, &request).await.unwrap();
    assert_eq!(result.task_id, Some("update-user-task".to_string()));
    assert_eq!(result.command_type, Some("UPDATE_USER".to_string()));
}

#[tokio::test]
async fn test_delete_user() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/users/456"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "delete-user-task",
            "commandType": "DELETE_USER",
            "status": "processing",
            "description": "Deleting user"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = UserHandler::new(client);
    let result = handler.delete_user_by_id(456).await.unwrap();

    assert_eq!(result.task_id, Some("delete-user-task".to_string()));
    assert_eq!(result.command_type, Some("DELETE_USER".to_string()));
}

#[tokio::test]
async fn test_error_handling_404() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users/999"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "error": "User not found"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = UserHandler::new(client);
    let result = handler.get_user_by_id(999).await;

    assert!(result.is_err());
    if let Err(redis_cloud::CloudError::NotFound { message }) = result {
        assert!(message.contains("not found") || message.contains("404"));
    } else {
        panic!("Expected NotFound error");
    }
}
