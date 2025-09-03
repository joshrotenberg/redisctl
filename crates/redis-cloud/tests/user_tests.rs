use redis_cloud::client::CloudClient;
use redis_cloud::users::{CreateUserRequest, UpdateUserRequest, UserHandler};
use wiremock::matchers::{header, method, path, body_json};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_list_users() {
    let mock_server = MockServer::start().await;

    let response = r#"[
        {
            "id": 1,
            "name": "John Doe",
            "email": "john@example.com",
            "role": "admin",
            "status": "active"
        },
        {
            "id": 2,
            "name": "Jane Smith",
            "email": "jane@example.com",
            "role": "viewer",
            "status": "active"
        }
    ]"#;

    Mock::given(method("GET"))
        .and(path("/users"))
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

    let handler = UserHandler::new(client);
    let users = handler.list().await.unwrap();

    assert_eq!(users.len(), 2);
    assert_eq!(users[0].id, 1);
    assert_eq!(users[0].name, "John Doe");
    assert_eq!(users[1].id, 2);
    assert_eq!(users[1].name, "Jane Smith");
}

#[tokio::test]
async fn test_get_user() {
    let mock_server = MockServer::start().await;

    let response = r#"{
        "id": 1,
        "name": "John Doe",
        "email": "john@example.com",
        "role": "admin",
        "status": "active",
        "lastLogin": "2024-01-01T00:00:00Z"
    }"#;

    Mock::given(method("GET"))
        .and(path("/users/1"))
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

    let handler = UserHandler::new(client);
    let user = handler.get(1).await.unwrap();

    assert_eq!(user.id, 1);
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.email, "john@example.com");
    assert_eq!(user.role.as_deref(), Some("admin"));
}

#[tokio::test]
async fn test_create_user() {
    let mock_server = MockServer::start().await;

    let response = r#"{
        "id": 3,
        "name": "New User",
        "email": "newuser@example.com",
        "role": "member",
        "status": "active"
    }"#;

    let expected_body = serde_json::json!({
        "name": "New User",
        "email": "newuser@example.com",
        "role": "member"
    });

    Mock::given(method("POST"))
        .and(path("/users"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .and(body_json(&expected_body))
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

    let request = CreateUserRequest::builder()
        .name("New User")
        .email("newuser@example.com")
        .role("member")
        .build();

    let handler = UserHandler::new(client);
    let user = handler.create(request).await.unwrap();

    assert_eq!(user.id, 3);
    assert_eq!(user.name, "New User");
    assert_eq!(user.email, "newuser@example.com");
}

#[tokio::test]
async fn test_update_user() {
    let mock_server = MockServer::start().await;

    let response = r#"{
        "id": 1,
        "name": "John Doe",
        "email": "newemail@example.com",
        "role": "admin",
        "status": "active"
    }"#;

    let expected_body = serde_json::json!({
        "name": "John Doe Updated"
    });

    Mock::given(method("PUT"))
        .and(path("/users/1"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .and(body_json(&expected_body))
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

    let request = UpdateUserRequest::builder()
        .name("John Doe Updated")
        .build();

    let handler = UserHandler::new(client);
    let user = handler.update(1, request).await.unwrap();

    assert_eq!(user.id, 1);
    assert_eq!(user.email, "newemail@example.com");
}

#[tokio::test]
async fn test_delete_user() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/users/1"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({})))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret_key("test-secret")
        .base_url(mock_server.uri())
        .build();

    let handler = UserHandler::new(client);
    let result = handler.delete(1).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_user_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/users/999"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(
            ResponseTemplate::new(404)
                .set_body_json(serde_json::json!({
                    "error": "User not found"
                })),
        )
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret_key("test-secret")
        .base_url(mock_server.uri())
        .build();

    let handler = UserHandler::new(client);
    let result = handler.get(999).await;

    assert!(result.is_err());
}