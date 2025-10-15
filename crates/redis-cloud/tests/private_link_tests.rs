use redis_cloud::{CloudClient, PrivateLinkHandler};
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_private_link() {
    let mock_server = MockServer::start().await;

    let response_body = json!({
        "resourceId": 123456,
        "status": "active",
        "shareName": "my-redis-share",
        "principals": [
            {
                "principal": "123456789012",
                "type": "aws_account",
                "alias": "Production Account"
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/subscriptions/123/private-link"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret("test-secret")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = PrivateLinkHandler::new(client);
    let result = handler.get(123).await.unwrap();

    assert_eq!(result["resourceId"], 123456);
    assert_eq!(result["status"], "active");
    assert_eq!(result["shareName"], "my-redis-share");
}

#[tokio::test]
async fn test_create_private_link() {
    let mock_server = MockServer::start().await;

    let response_body = json!({
        "resourceId": 123456,
        "status": "pending",
        "taskId": "task-789"
    });

    Mock::given(method("POST"))
        .and(path("/subscriptions/123/private-link"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret("test-secret")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = PrivateLinkHandler::new(client);

    let request = json!({
        "shareName": "my-redis-share",
        "principal": "123456789012",
        "type": "aws_account",
        "alias": "Production Account"
    });

    let result = handler.create(123, request).await.unwrap();

    assert_eq!(result["resourceId"], 123456);
    assert_eq!(result["status"], "pending");
    assert_eq!(result["taskId"], "task-789");
}

#[tokio::test]
async fn test_add_principals() {
    let mock_server = MockServer::start().await;

    let response_body = json!({
        "resourceId": 123456,
        "principals": [
            {
                "principal": "987654321098",
                "type": "iam_role",
                "alias": "Dev Role"
            }
        ]
    });

    Mock::given(method("POST"))
        .and(path("/subscriptions/123/private-link/principals"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret("test-secret")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = PrivateLinkHandler::new(client);

    let request = json!({
        "principal": "987654321098",
        "type": "iam_role",
        "alias": "Dev Role"
    });

    let result = handler.add_principals(123, request).await.unwrap();

    assert_eq!(result["resourceId"], 123456);
    assert!(result["principals"].is_array());
}

#[tokio::test]
async fn test_remove_principals() {
    let mock_server = MockServer::start().await;

    let response_body = json!({
        "status": "deleted"
    });

    Mock::given(method("DELETE"))
        .and(path("/subscriptions/123/private-link/principals"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret("test-secret")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = PrivateLinkHandler::new(client);

    let request = json!({
        "principal": "987654321098",
        "type": "iam_role",
        "alias": "Dev Role"
    });

    let result = handler.remove_principals(123, request).await.unwrap();

    assert_eq!(result["status"], "deleted");
}

#[tokio::test]
async fn test_get_endpoint_script() {
    let mock_server = MockServer::start().await;

    let response_body = json!({
        "script": "aws ec2 create-vpc-endpoint --vpc-id vpc-123 --service-name com.amazonaws.vpce.us-east-1.vpce-svc-abc123"
    });

    Mock::given(method("GET"))
        .and(method("GET"))
        .and(path("/subscriptions/123/private-link/endpoint-script"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret("test-secret")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = PrivateLinkHandler::new(client);
    let result = handler.get_endpoint_script(123).await.unwrap();

    assert!(result["script"].is_string());
    assert!(result["script"].as_str().unwrap().contains("aws ec2"));
}

#[tokio::test]
async fn test_get_active_active_private_link() {
    let mock_server = MockServer::start().await;

    let response_body = json!({
        "resourceId": 123456,
        "regionId": 1,
        "status": "active",
        "shareName": "my-crdb-share"
    });

    Mock::given(method("GET"))
        .and(method("GET"))
        .and(path("/subscriptions/123/regions/1/private-link"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret("test-secret")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = PrivateLinkHandler::new(client);
    let result = handler.get_active_active(123, 1).await.unwrap();

    assert_eq!(result["resourceId"], 123456);
    assert_eq!(result["regionId"], 1);
    assert_eq!(result["shareName"], "my-crdb-share");
}

#[tokio::test]
async fn test_create_active_active_private_link() {
    let mock_server = MockServer::start().await;

    let response_body = json!({
        "resourceId": 123456,
        "regionId": 1,
        "status": "pending",
        "taskId": "task-999"
    });

    Mock::given(method("POST"))
        .and(path("/subscriptions/123/regions/1/private-link"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret("test-secret")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = PrivateLinkHandler::new(client);

    let request = json!({
        "shareName": "my-crdb-share",
        "principal": "111222333444",
        "type": "aws_account"
    });

    let result = handler.create_active_active(123, 1, request).await.unwrap();

    assert_eq!(result["resourceId"], 123456);
    assert_eq!(result["regionId"], 1);
    assert_eq!(result["taskId"], "task-999");
}

#[tokio::test]
async fn test_add_principals_active_active() {
    let mock_server = MockServer::start().await;

    let response_body = json!({
        "resourceId": 123456,
        "regionId": 1,
        "principals": [
            {
                "principal": "555666777888",
                "type": "aws_account"
            }
        ]
    });

    Mock::given(method("POST"))
        .and(path("/subscriptions/123/regions/1/private-link/principals"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret("test-secret")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = PrivateLinkHandler::new(client);

    let request = json!({
        "principal": "555666777888",
        "type": "aws_account"
    });

    let result = handler
        .add_principals_active_active(123, 1, request)
        .await
        .unwrap();

    assert_eq!(result["resourceId"], 123456);
    assert_eq!(result["regionId"], 1);
}

#[tokio::test]
async fn test_remove_principals_active_active() {
    let mock_server = MockServer::start().await;

    let response_body = json!({
        "status": "deleted"
    });

    Mock::given(method("DELETE"))
        .and(path("/subscriptions/123/regions/1/private-link/principals"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret("test-secret")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = PrivateLinkHandler::new(client);

    let request = json!({
        "principal": "555666777888",
        "type": "aws_account"
    });

    let result = handler
        .remove_principals_active_active(123, 1, request)
        .await
        .unwrap();

    assert_eq!(result["status"], "deleted");
}

#[tokio::test]
async fn test_get_endpoint_script_active_active() {
    let mock_server = MockServer::start().await;

    let response_body = json!({
        "script": "aws ec2 create-vpc-endpoint --vpc-id vpc-456 --service-name com.amazonaws.vpce.us-west-2.vpce-svc-xyz789"
    });

    Mock::given(method("GET"))
        .and(method("GET"))
        .and(path(
            "/subscriptions/123/regions/1/private-link/endpoint-script",
        ))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret("test-secret")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = PrivateLinkHandler::new(client);
    let result = handler
        .get_endpoint_script_active_active(123, 1)
        .await
        .unwrap();

    assert!(result["script"].is_string());
    assert!(result["script"].as_str().unwrap().contains("aws ec2"));
}

#[tokio::test]
async fn test_error_handling_401() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(method("GET"))
        .and(path("/subscriptions/123/private-link"))
        .respond_with(ResponseTemplate::new(401).set_body_string("Unauthorized"))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("invalid-key")
        .api_secret("invalid-secret")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = PrivateLinkHandler::new(client);
    let result = handler.get(123).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_error_handling_404() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(method("GET"))
        .and(path("/subscriptions/999/private-link"))
        .respond_with(ResponseTemplate::new(404).set_body_string("Not Found"))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret("test-secret")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = PrivateLinkHandler::new(client);
    let result = handler.get(999).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_error_handling_500() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/subscriptions/123/private-link"))
        .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key")
        .api_secret("test-secret")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = PrivateLinkHandler::new(client);
    let request = json!({
        "shareName": "test",
        "principal": "123",
        "type": "aws_account"
    });
    let result = handler.create(123, request).await;

    assert!(result.is_err());
}
