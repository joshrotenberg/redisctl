use redis_cloud::{CloudClient, ConnectivityHandler};
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_vpc_peering() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/123/peerings"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "taskId": "task-get-peering",
            "commandType": "GET_VPC_PEERING",
            "status": "completed",
            "description": "Getting VPC peerings"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = ConnectivityHandler::new(client);
    let result = handler.get_vpc_peering(123).await.unwrap();

    assert_eq!(result.task_id, Some("task-get-peering".to_string()));
    assert_eq!(result.command_type, Some("GET_VPC_PEERING".to_string()));
}

#[tokio::test]
async fn test_create_vpc_peering() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/subscriptions/123/peerings"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-create-peering",
            "commandType": "CREATE_VPC_PEERING",
            "status": "processing",
            "description": "Creating VPC peering"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = ConnectivityHandler::new(client);
    let request = redis_cloud::connectivity::VpcPeeringCreateBaseRequest {
        provider: Some("AWS".to_string()),
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler.create_vpc_peering(123, &request).await.unwrap();
    assert_eq!(result.task_id, Some("task-create-peering".to_string()));
    assert_eq!(result.command_type, Some("CREATE_VPC_PEERING".to_string()));
}

#[tokio::test]
async fn test_delete_vpc_peering() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/subscriptions/123/peerings/456"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-delete-peering",
            "commandType": "DELETE_VPC_PEERING",
            "status": "processing",
            "description": "Deleting VPC peering"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = ConnectivityHandler::new(client);
    let result = handler.delete_vpc_peering(123, 456).await.unwrap();

    assert_eq!(result.task_id, Some("task-delete-peering".to_string()));
    assert_eq!(result.command_type, Some("DELETE_VPC_PEERING".to_string()));
}

#[tokio::test]
async fn test_get_psc_service() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/123/private-service-connect"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "taskId": "task-get-psc",
            "commandType": "GET_PSC_SERVICE",
            "status": "completed",
            "description": "Getting PSC service"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = ConnectivityHandler::new(client);
    let result = handler.get_psc_service(123).await.unwrap();

    assert_eq!(result.task_id, Some("task-get-psc".to_string()));
    assert_eq!(result.command_type, Some("GET_PSC_SERVICE".to_string()));
}

#[tokio::test]
async fn test_create_psc_service() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/subscriptions/123/private-service-connect"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-create-psc",
            "commandType": "CREATE_PSC_SERVICE",
            "status": "processing",
            "description": "Creating PSC service"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = ConnectivityHandler::new(client);
    let result = handler.create_psc_service(123).await.unwrap();

    assert_eq!(result.task_id, Some("task-create-psc".to_string()));
    assert_eq!(result.command_type, Some("CREATE_PSC_SERVICE".to_string()));
}

#[tokio::test]
async fn test_get_tgws() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/123/transitGateways"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "taskId": "task-get-tgws",
            "commandType": "GET_TGWS",
            "status": "completed",
            "description": "Getting TGWs"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = ConnectivityHandler::new(client);
    let result = handler.get_tgws(123).await.unwrap();

    assert_eq!(result.task_id, Some("task-get-tgws".to_string()));
    assert_eq!(result.command_type, Some("GET_TGWS".to_string()));
}

#[tokio::test]
async fn test_error_handling_404() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/999/peerings"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "error": "Subscription not found"
        })))
        .mount(&mock_server)
        .await;

    let client = CloudClient::builder()
        .api_key("test-key".to_string())
        .api_secret("test-secret".to_string())
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = ConnectivityHandler::new(client);
    let result = handler.get_vpc_peering(999).await;

    assert!(result.is_err());
    if let Err(redis_cloud::CloudError::NotFound { message }) = result {
        assert!(message.contains("not found") || message.contains("404"));
    } else {
        panic!("Expected NotFound error");
    }
}
