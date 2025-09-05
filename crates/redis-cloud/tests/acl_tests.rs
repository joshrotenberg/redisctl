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
async fn test_update_redis_rule() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/acl/redisRules/123"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-update-123",
            "commandType": "UPDATE_REDIS_RULE",
            "status": "processing",
            "description": "Updating Redis ACL rule",
            "timestamp": "2024-01-01T12:00:00Z"
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
    let request = redis_cloud::acl::AclRedisRuleUpdateRequest {
        redis_rule_id: Some(123),
        name: "updated-rule".to_string(),
        redis_rule: "+get +set +del".to_string(),
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler.update_redis_rule(123, &request).await.unwrap();
    assert_eq!(result.task_id, Some("task-update-123".to_string()));
    assert_eq!(result.command_type, Some("UPDATE_REDIS_RULE".to_string()));
    assert_eq!(result.status, Some("processing".to_string()));
}

#[tokio::test]
async fn test_create_role() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/acl/roles"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-create-role-789",
            "commandType": "CREATE_ROLE",
            "status": "processing",
            "description": "Creating ACL role",
            "timestamp": "2024-01-01T13:00:00Z",
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

    let handler = AclHandler::new(client);
    let request = redis_cloud::acl::AclRoleCreateRequest {
        name: "test-role".to_string(),
        redis_rules: vec![redis_cloud::acl::AclRoleRedisRuleSpec {
            rule_name: "test-rule".to_string(),
            databases: vec![redis_cloud::acl::AclRoleDatabaseSpec {
                subscription_id: 100,
                database_id: 200,
                regions: Some(vec!["us-east-1".to_string()]),
                extra: serde_json::Value::Null,
            }],
            extra: serde_json::Value::Null,
        }],
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler.create_role(&request).await.unwrap();
    assert_eq!(result.task_id, Some("task-create-role-789".to_string()));
    assert_eq!(result.command_type, Some("CREATE_ROLE".to_string()));
    assert_eq!(result.status, Some("processing".to_string()));
}

#[tokio::test]
async fn test_delete_acl_role() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/acl/roles/456"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-delete-role-456",
            "commandType": "DELETE_ROLE",
            "status": "processing",
            "description": "Deleting ACL role"
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
    let result = handler.delete_acl_role(456).await.unwrap();

    assert_eq!(result.task_id, Some("task-delete-role-456".to_string()));
    assert_eq!(result.command_type, Some("DELETE_ROLE".to_string()));
    assert_eq!(result.status, Some("processing".to_string()));
}

#[tokio::test]
async fn test_update_role() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/acl/roles/789"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-update-role-789",
            "commandType": "UPDATE_ROLE",
            "status": "processing",
            "description": "Updating ACL role"
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
    let request = redis_cloud::acl::AclRoleUpdateRequest {
        name: Some("updated-role-name".to_string()),
        redis_rules: Some(vec![redis_cloud::acl::AclRoleRedisRuleSpec {
            rule_name: "updated-rule".to_string(),
            databases: vec![redis_cloud::acl::AclRoleDatabaseSpec {
                subscription_id: 101,
                database_id: 201,
                regions: None,
                extra: serde_json::Value::Null,
            }],
            extra: serde_json::Value::Null,
        }]),
        role_id: Some(789),
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler.update_role(789, &request).await.unwrap();
    assert_eq!(result.task_id, Some("task-update-role-789".to_string()));
    assert_eq!(result.command_type, Some("UPDATE_ROLE".to_string()));
}

#[tokio::test]
async fn test_get_all_users() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/acl/users"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "accountId": 123,
            "links": [
                {
                    "href": "https://api.redislabs.com/v1/acl/users/1",
                    "type": "GET",
                    "rel": "user"
                },
                {
                    "href": "https://api.redislabs.com/v1/acl/users/2",
                    "type": "GET",
                    "rel": "user"
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
    let result = handler.get_all_acl_users().await.unwrap();

    assert_eq!(result.account_id, Some(123));
    assert!(result.links.is_some());
    let links = result.links.unwrap();
    assert_eq!(links.len(), 2);
}

#[tokio::test]
async fn test_delete_user() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/acl/users/789"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-delete-user-789",
            "commandType": "DELETE_USER",
            "status": "processing",
            "description": "Deleting ACL user"
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
    let result = handler.delete_user(789).await.unwrap();

    assert_eq!(result.task_id, Some("task-delete-user-789".to_string()));
    assert_eq!(result.command_type, Some("DELETE_USER".to_string()));
    assert_eq!(result.status, Some("processing".to_string()));
}

#[tokio::test]
async fn test_update_user() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/acl/users/456"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-update-user-456",
            "commandType": "UPDATE_USER",
            "status": "processing",
            "description": "Updating ACL user",
            "timestamp": "2024-01-01T14:00:00Z"
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
    let request = redis_cloud::acl::AclUserUpdateRequest {
        user_id: Some(456),
        role: Some("updated-role".to_string()),
        password: Some("new-password".to_string()),
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler.update_acl_user(456, &request).await.unwrap();
    assert_eq!(result.task_id, Some("task-update-user-456".to_string()));
    assert_eq!(result.command_type, Some("UPDATE_USER".to_string()));
    assert_eq!(result.status, Some("processing".to_string()));
}

#[tokio::test]
async fn test_get_user_by_id_with_links() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/acl/users/456"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 456,
            "name": "test-user",
            "role": "test-role",
            "status": "active",
            "links": [
                {
                    "href": "https://api.redislabs.com/v1/acl/users/456",
                    "type": "GET",
                    "rel": "self"
                },
                {
                    "href": "https://api.redislabs.com/v1/acl/users/456",
                    "type": "PUT",
                    "rel": "update"
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
    let result = handler.get_user_by_id(456).await.unwrap();

    assert_eq!(result.id, Some(456));
    assert_eq!(result.name, Some("test-user".to_string()));
    assert_eq!(result.role, Some("test-role".to_string()));
    assert_eq!(result.status, Some("active".to_string()));
    assert!(result.links.is_some());

    let links = result.links.unwrap();
    assert_eq!(links.len(), 2);
}

#[tokio::test]
async fn test_create_role_without_regions() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/acl/roles"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-create-role-simple",
            "commandType": "CREATE_ROLE",
            "status": "processing",
            "description": "Creating ACL role without regions"
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
    let request = redis_cloud::acl::AclRoleCreateRequest {
        name: "simple-role".to_string(),
        redis_rules: vec![redis_cloud::acl::AclRoleRedisRuleSpec {
            rule_name: "basic-rule".to_string(),
            databases: vec![redis_cloud::acl::AclRoleDatabaseSpec {
                subscription_id: 300,
                database_id: 400,
                regions: None,
                extra: serde_json::Value::Null,
            }],
            extra: serde_json::Value::Null,
        }],
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler.create_role(&request).await.unwrap();
    assert_eq!(result.task_id, Some("task-create-role-simple".to_string()));
    assert_eq!(
        result.description,
        Some("Creating ACL role without regions".to_string())
    );
}

#[tokio::test]
async fn test_get_all_redis_rules_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/acl/redisRules"))
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

    let handler = AclHandler::new(client);
    let result = handler.get_all_redis_rules().await.unwrap();

    assert_eq!(result.account_id, Some(123));
    assert!(result.links.is_some());
    let links = result.links.unwrap();
    assert_eq!(links.len(), 0);
}

#[tokio::test]
async fn test_get_roles_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/acl/roles"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "accountId": 456,
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

    let handler = AclHandler::new(client);
    let result = handler.get_roles().await.unwrap();

    assert_eq!(result.account_id, Some(456));
    assert!(result.links.is_some());
    let links = result.links.unwrap();
    assert_eq!(links.len(), 0);
}

#[tokio::test]
async fn test_task_state_update_with_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/acl/redisRules"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-error-123",
            "commandType": "CREATE_REDIS_RULE",
            "status": "failed",
            "description": "Failed to create Redis ACL rule",
            "timestamp": "2024-01-01T15:00:00Z",
            "response": {
                "error": "Invalid rule pattern",
                "additionalInfo": "Rule pattern '+invalid' is not valid"
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
        name: "invalid-rule".to_string(),
        redis_rule: "+invalid".to_string(),
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler.create_redis_rule(&request).await.unwrap();
    assert_eq!(result.task_id, Some("task-error-123".to_string()));
    assert_eq!(result.status, Some("failed".to_string()));
    assert_eq!(
        result.description,
        Some("Failed to create Redis ACL rule".to_string())
    );
    assert!(result.response.is_some());

    let response = result.response.unwrap();
    assert_eq!(response.error, Some("Invalid rule pattern".to_string()));
    assert_eq!(
        response.additional_info,
        Some("Rule pattern '+invalid' is not valid".to_string())
    );
}

// Error handling tests

#[tokio::test]
async fn test_error_handling_401() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/acl/roles"))
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

    let handler = AclHandler::new(client);
    let result = handler.get_roles().await;

    assert!(result.is_err());
    match result {
        Err(redis_cloud::CloudError::AuthenticationFailed { .. }) => {}
        _ => panic!("Expected AuthenticationFailed error"),
    }
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

#[tokio::test]
async fn test_error_handling_500() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/acl/users"))
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

    let handler = AclHandler::new(client);
    let request = redis_cloud::acl::AclUserCreateRequest {
        name: "test-user".to_string(),
        role: "test-role".to_string(),
        password: "test-password".to_string(),
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler.create_user(&request).await;

    assert!(result.is_err());
    match result {
        Err(redis_cloud::CloudError::InternalServerError { .. }) => {}
        _ => panic!("Expected InternalServerError error"),
    }
}

#[tokio::test]
async fn test_create_user_with_extra_fields() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/acl/users"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-789",
            "commandType": "CREATE_USER",
            "status": "processing",
            "description": "Creating ACL user",
            "timestamp": "2024-01-01T16:00:00Z",
            "response": {
                "resourceId": 999,
                "additionalResourceId": 888
            },
            "customField": "custom value",
            "links": [
                {
                    "href": "https://api.redislabs.com/v1/tasks/task-789",
                    "type": "GET",
                    "rel": "task"
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
    let request = redis_cloud::acl::AclUserCreateRequest {
        name: "test-user".to_string(),
        role: "test-role".to_string(),
        password: "test-password".to_string(),
        command_type: Some("CREATE_USER".to_string()),
        extra: json!({
            "metadata": "user metadata",
            "customFlag": true
        }),
    };

    let result = handler.create_user(&request).await.unwrap();
    assert_eq!(result.task_id, Some("task-789".to_string()));
    assert_eq!(result.command_type, Some("CREATE_USER".to_string()));
    assert_eq!(result.status, Some("processing".to_string()));
    assert_eq!(result.timestamp, Some("2024-01-01T16:00:00Z".to_string()));

    // Test that extra fields are captured
    assert!(result.extra.get("customField").is_some());
    assert!(result.links.is_some());

    let response = result.response.unwrap();
    assert_eq!(response.resource_id, Some(999));
    assert_eq!(response.additional_resource_id, Some(888));
}
