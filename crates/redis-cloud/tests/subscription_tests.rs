use redis_cloud::client::CloudClient;
use redis_cloud::subscriptions::{
    CloudProviderConfig, CreateSubscriptionRequest, RegionConfig,
    SubscriptionHandler, UpdateSubscriptionRequest,
};
use redis_cloud::types::{CloudProvider, MemoryStorage};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_list_subscriptions() {
    let mock_server = MockServer::start().await;

    let response = r#"[
        {
            "id": 1234,
            "name": "My Subscription",
            "status": "active",
            "paymentMethodId": 5678,
            "paymentMethodType": "credit-card",
            "memoryStorage": "ram",
            "numberOfDatabases": 2
        }
    ]"#;

    Mock::given(method("GET"))
        .and(path("/subscriptions"))
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

    let handler = SubscriptionHandler::new(client);
    let subscriptions = handler.list().await.unwrap();

    assert_eq!(subscriptions.len(), 1);
    assert_eq!(subscriptions[0].id, 1234);
    assert_eq!(subscriptions[0].name, "My Subscription");
}

#[tokio::test]
async fn test_get_subscription() {
    let mock_server = MockServer::start().await;

    let response = r#"{
        "id": 1234,
        "name": "My Subscription",
        "status": "active",
        "paymentMethodId": 5678,
        "paymentMethodType": "credit-card",
        "memoryStorage": "ram",
        "numberOfDatabases": 2
    }"#;

    Mock::given(method("GET"))
        .and(path("/subscriptions/1234"))
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

    let handler = SubscriptionHandler::new(client);
    let subscription = handler.get(1234).await.unwrap();

    assert_eq!(subscription.id, 1234);
    assert_eq!(subscription.name, "My Subscription");
}

#[tokio::test]
async fn test_create_subscription() {
    let mock_server = MockServer::start().await;

    let response = r#"{
        "id": 1234,
        "name": "New Subscription",
        "status": "pending",
        "paymentMethodId": 5678,
        "paymentMethodType": "credit-card",
        "memoryStorage": "ram",
        "numberOfDatabases": 0
    }"#;

    Mock::given(method("POST"))
        .and(path("/subscriptions"))
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

    let region_config = RegionConfig::builder().region("us-east-1").build();

    let cloud_provider = CloudProviderConfig::builder()
        .provider(CloudProvider::Aws)
        .regions(vec![region_config])
        .build();

    let request = CreateSubscriptionRequest::builder()
        .name("New Subscription")
        .payment_method_id(5678)
        .cloud_providers(vec![cloud_provider])
        .memory_storage(MemoryStorage::Ram)
        .build();

    let handler = SubscriptionHandler::new(client);
    let subscription = handler.create(request).await.unwrap();

    assert_eq!(subscription.id, 1234);
    assert_eq!(subscription.name, "New Subscription");
}

#[tokio::test]
async fn test_update_subscription() {
    let mock_server = MockServer::start().await;

    let response = r#"{
        "id": 1234,
        "name": "Updated Subscription",
        "status": "active",
        "paymentMethodId": 5678,
        "paymentMethodType": "credit-card",
        "memoryStorage": "ram",
        "numberOfDatabases": 2
    }"#;

    Mock::given(method("PUT"))
        .and(path("/subscriptions/1234"))
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

    let request = UpdateSubscriptionRequest::builder()
        .name("Updated Subscription")
        .build();

    let handler = SubscriptionHandler::new(client);
    let subscription = handler.update(1234, request).await.unwrap();

    assert_eq!(subscription.id, 1234);
    assert_eq!(subscription.name, "Updated Subscription");
}

#[tokio::test]
async fn test_delete_subscription() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/subscriptions/1234"))
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

    let handler = SubscriptionHandler::new(client);
    let result = handler.delete(1234).await;

    assert!(result.is_ok());
}
