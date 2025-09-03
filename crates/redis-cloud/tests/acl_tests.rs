use redis_cloud::{AclHandler, CloudClient};
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_all_redis_rules() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/acl/redisRules"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "accountId": 123,
            "links": [
                {
                    "href": "https://api.redislabs.com/v1/acl/redisRules/1",
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

    let handler = AclHandler::new(client);
    let result = handler.get_all_redis_rules().await.unwrap();

    assert_eq!(result.account_id, Some(123));
    assert!(result.links.is_some());
}

#[tokio::test]
async fn test_create_redis_rule() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/acl/redisRules"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-123",
            "commandType": "CREATE_REDIS_RULE",
            "status": "processing",
            "description": "Creating Redis ACL rule",
            "timestamp": "2024-01-01T00:00:00Z",
            "response": {
                "resourceId": 456
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

    let handler = AclHandler::new(client);
    let request = redis_cloud::acl::AclRedisRuleCreateRequest {
        name: "test-rule".to_string(),
        redis_rule: "+get +set".to_string(),
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler.create_redis_rule(&request).await.unwrap();
    assert_eq!(result.task_id, Some("task-123".to_string()));
    assert_eq!(result.status, Some("processing".to_string()));
}

#[tokio::test]
async fn test_delete_redis_rule() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/acl/redisRules/123"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-456",
            "commandType": "DELETE_REDIS_RULE",
            "status": "processing",
            "description": "Deleting Redis ACL rule"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = AclHandler::new(client);
    let result = handler.delete_redis_rule(123).await.unwrap();

    assert_eq!(result.task_id, Some("task-456".to_string()));
    assert_eq!(result.command_type, Some("DELETE_REDIS_RULE".to_string()));
}

#[tokio::test]
async fn test_get_roles() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/acl/roles"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "accountId": 123,
            "links": [
                {
                    "href": "https://api.redislabs.com/v1/acl/roles/1",
                    "type": "GET",
                    "rel": "role"
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

    let handler = AclHandler::new(client);
    let result = handler.get_roles().await.unwrap();

    assert_eq!(result.account_id, Some(123));
}

#[tokio::test]
async fn test_create_user() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/acl/users"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-789",
            "commandType": "CREATE_USER",
            "status": "processing",
            "description": "Creating ACL user"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = AclHandler::new(client);
    let request = redis_cloud::acl::AclUserCreateRequest {
        name: "test-user".to_string(),
        role: "test-role".to_string(),
        password: "test-password".to_string(),
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler.create_user(&request).await.unwrap();
    assert_eq!(result.task_id, Some("task-789".to_string()));
}

#[tokio::test]
async fn test_get_user_by_id() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/acl/users/456"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 456,
            "name": "test-user",
            "role": "test-role",
            "status": "active"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = AclHandler::new(client);
    let result = handler.get_user_by_id(456).await.unwrap();

    assert_eq!(result.id, Some(456));
    assert_eq!(result.name, Some("test-user".to_string()));
    assert_eq!(result.role, Some("test-role".to_string()));
}

#[tokio::test]
async fn test_error_handling_404() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/acl/users/999"))
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

    let handler = AclHandler::new(client);
    let result = handler.get_user_by_id(999).await;

    assert!(result.is_err());
    if let Err(redis_cloud::CloudError::NotFound { message }) = result {
        assert!(message.contains("not found") || message.contains("404"));
    } else {
        panic!("Expected NotFound error");
    }
}
