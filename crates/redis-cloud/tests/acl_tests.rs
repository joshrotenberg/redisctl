use redis_cloud::acl::{
    AclHandler, CreateAclRedisRuleRequest, CreateAclRoleRequest, CreateAclUserRequest,
};
use redis_cloud::client::CloudClient;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_list_acl_users() {
    let mock_server = MockServer::start().await;

    let response = r#"[
        {
            "id": 1,
            "name": "test-user",
            "role": "admin",
            "status": "active"
        }
    ]"#;

    Mock::given(method("GET"))
        .and(path("/acl/users"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::from_str::<serde_json::Value>(response).unwrap()),
        )
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret_key("test-secret")
        .base_url(mock_server.uri())
        .build();

    let handler = AclHandler::new(client);
    let users = handler.list_users().await.unwrap();

    assert_eq!(users.len(), 1);
    assert_eq!(users[0].id, 1);
    assert_eq!(users[0].name, "test-user");
}

#[tokio::test]
async fn test_create_acl_user() {
    let mock_server = MockServer::start().await;

    let response = r#"{
        "id": 2,
        "name": "new-user",
        "role": "viewer",
        "status": "active"
    }"#;

    Mock::given(method("POST"))
        .and(path("/acl/users"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::from_str::<serde_json::Value>(response).unwrap()),
        )
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret_key("test-secret")
        .base_url(mock_server.uri())
        .build();

    let request = CreateAclUserRequest::builder()
        .name("new-user")
        .role("viewer")
        .password("secret123")
        .build();

    let handler = AclHandler::new(client);
    let user = handler.create_user(request).await.unwrap();

    assert_eq!(user.id, 2);
    assert_eq!(user.name, "new-user");
}

#[tokio::test]
async fn test_list_acl_roles() {
    let mock_server = MockServer::start().await;

    let response = r#"[
        {
            "id": 1,
            "name": "admin-role",
            "databases": [],
            "redisRules": []
        }
    ]"#;

    Mock::given(method("GET"))
        .and(path("/acl/roles"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::from_str::<serde_json::Value>(response).unwrap()),
        )
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret_key("test-secret")
        .base_url(mock_server.uri())
        .build();

    let handler = AclHandler::new(client);
    let roles = handler.list_roles().await.unwrap();

    assert_eq!(roles.len(), 1);
    assert_eq!(roles[0].id, 1);
    assert_eq!(roles[0].name, "admin-role");
}

#[tokio::test]
async fn test_create_acl_role() {
    let mock_server = MockServer::start().await;

    let response = r#"{
        "id": 2,
        "name": "custom-role",
        "databases": [],
        "redisRules": []
    }"#;

    Mock::given(method("POST"))
        .and(path("/acl/roles"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::from_str::<serde_json::Value>(response).unwrap()),
        )
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret_key("test-secret")
        .base_url(mock_server.uri())
        .build();

    let request = CreateAclRoleRequest::builder().name("custom-role").build();

    let handler = AclHandler::new(client);
    let role = handler.create_role(request).await.unwrap();

    assert_eq!(role.id, 2);
    assert_eq!(role.name, "custom-role");
}

#[tokio::test]
async fn test_list_acl_redis_rules() {
    let mock_server = MockServer::start().await;

    let response = r#"[
        {
            "id": 1,
            "name": "read-only",
            "acl": "+@read",
            "status": "active"
        }
    ]"#;

    Mock::given(method("GET"))
        .and(path("/acl/redisRules"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::from_str::<serde_json::Value>(response).unwrap()),
        )
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret_key("test-secret")
        .base_url(mock_server.uri())
        .build();

    let handler = AclHandler::new(client);
    let rules = handler.list_redis_rules().await.unwrap();

    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0].id, 1);
    assert_eq!(rules[0].name, "read-only");
    assert_eq!(rules[0].acl_string, "+@read");
}

#[tokio::test]
async fn test_create_acl_redis_rule() {
    let mock_server = MockServer::start().await;

    let response = r#"{
        "id": 2,
        "name": "write-only",
        "acl": "+@write",
        "status": "active"
    }"#;

    Mock::given(method("POST"))
        .and(path("/acl/redisRules"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::from_str::<serde_json::Value>(response).unwrap()),
        )
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret_key("test-secret")
        .base_url(mock_server.uri())
        .build();

    let request = CreateAclRedisRuleRequest::builder()
        .name("write-only")
        .acl_string("+@write")
        .build();

    let handler = AclHandler::new(client);
    let rule = handler.create_redis_rule(request).await.unwrap();

    assert_eq!(rule.id, 2);
    assert_eq!(rule.name, "write-only");
    assert_eq!(rule.acl_string, "+@write");
}
