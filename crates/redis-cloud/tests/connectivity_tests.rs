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

    // Delete methods now return serde_json::Value::Null
    assert_eq!(result, serde_json::Value::Null);
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
async fn test_update_vpc_peering() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/subscriptions/123/peerings/456"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-update-peering",
            "commandType": "UPDATE_VPC_PEERING",
            "status": "processing",
            "description": "Updating VPC peering"
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
    let request = redis_cloud::connectivity::VpcPeeringUpdateAwsRequest {
        subscription_id: None,
        vpc_peering_id: None,
        vpc_cidr: Some("10.0.0.0/16".to_string()),
        vpc_cidrs: Some(vec!["10.0.0.0/16".to_string()]),
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler
        .update_vpc_peering(123, 456, &request)
        .await
        .unwrap();
    assert_eq!(result.task_id, Some("task-update-peering".to_string()));
    assert_eq!(result.command_type, Some("UPDATE_VPC_PEERING".to_string()));
}

#[tokio::test]
async fn test_create_vpc_peering_gcp() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/subscriptions/123/peerings"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-create-gcp-peering",
            "commandType": "CREATE_VPC_PEERING",
            "status": "processing",
            "description": "Creating GCP VPC peering",
            "timestamp": "2024-01-01T10:00:00Z",
            "response": {
                "resourceId": 789,
                "additionalInfo": "GCP peering initiated"
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

    let handler = ConnectivityHandler::new(client);
    let request = redis_cloud::connectivity::VpcPeeringCreateBaseRequest {
        provider: Some("GCP".to_string()),
        command_type: Some("CREATE_VPC_PEERING".to_string()),
        extra: json!({
            "gcp_project_id": "my-gcp-project",
            "gcp_network_name": "default"
        }),
    };

    let result = handler.create_vpc_peering(123, &request).await.unwrap();
    assert_eq!(result.task_id, Some("task-create-gcp-peering".to_string()));
    assert_eq!(result.command_type, Some("CREATE_VPC_PEERING".to_string()));
    assert_eq!(
        result.status,
        Some(redis_cloud::types::TaskStatus::ProcessingInProgress)
    );
    assert!(result.response.is_some());
}

#[tokio::test]
async fn test_create_vpc_peering_azure() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/subscriptions/456/peerings"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-create-azure-peering",
            "commandType": "CREATE_VPC_PEERING",
            "status": "processing",
            "description": "Creating Azure VNet peering"
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
        provider: Some("Azure".to_string()),
        command_type: None,
        extra: json!({
            "azure_subscription_id": "12345678-1234-1234-1234-123456789012",
            "azure_vnet_name": "my-vnet",
            "azure_resource_group": "my-resource-group"
        }),
    };

    let result = handler.create_vpc_peering(456, &request).await.unwrap();
    assert_eq!(
        result.task_id,
        Some("task-create-azure-peering".to_string())
    );
    assert_eq!(result.command_type, Some("CREATE_VPC_PEERING".to_string()));
}

#[tokio::test]
async fn test_update_psc_service() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path(
            "/subscriptions/123/private-service-connect/789/endpoints/1",
        ))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-update-psc",
            "commandType": "UPDATE_PSC_SERVICE",
            "status": "processing",
            "description": "Updating PSC service"
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
    let request = redis_cloud::connectivity::PscEndpointUpdateRequest {
        subscription_id: 123,
        psc_service_id: 789,
        endpoint_id: 1,
        gcp_project_id: Some("project1".to_string()),
        gcp_vpc_name: Some("vpc1".to_string()),
        gcp_vpc_subnet_name: Some("subnet1".to_string()),
        endpoint_connection_name: Some("psc-endpoint".to_string()),
    };

    let result = handler
        .update_psc_service_endpoint(123, 1, &request)
        .await
        .unwrap();
    assert_eq!(result.task_id, Some("task-update-psc".to_string()));
    assert_eq!(result.command_type, Some("UPDATE_PSC_SERVICE".to_string()));
}

#[tokio::test]
async fn test_delete_psc_service() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/subscriptions/123/private-service-connect"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-delete-psc",
            "commandType": "DELETE_PSC_SERVICE",
            "status": "processing",
            "description": "Deleting PSC service"
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
    let result = handler.delete_psc_service(123).await.unwrap();

    // Delete methods now return serde_json::Value::Null
    assert_eq!(result, serde_json::Value::Null);
}

#[tokio::test]
async fn test_create_tgw() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/subscriptions/123/transitGateways/456/attachment"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-create-tgw",
            "commandType": "CREATE_TGW_ATTACHMENT",
            "status": "processing",
            "description": "Creating Transit Gateway attachment",
            "timestamp": "2024-01-02T14:00:00Z"
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
    // TGW attachment creation uses tgw_id parameter, not a request body

    let result = handler.create_tgw_attachment(123, "456").await.unwrap();
    assert_eq!(result.task_id, Some("task-create-tgw".to_string()));
    assert_eq!(
        result.command_type,
        Some("CREATE_TGW_ATTACHMENT".to_string())
    );
    assert_eq!(
        result.status,
        Some(redis_cloud::types::TaskStatus::ProcessingInProgress)
    );
}

#[tokio::test]
async fn test_update_tgw() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/subscriptions/123/transitGateways/456/attachment"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-update-tgw",
            "commandType": "UPDATE_TGW_ATTACHMENT_CIDRS",
            "status": "processing",
            "description": "Updating Transit Gateway attachment"
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
    let request = redis_cloud::connectivity::TgwUpdateCidrsRequest {
        cidrs: Some(vec![
            redis_cloud::connectivity::Cidr {
                cidr_address: Some("10.0.0.0/16".to_string()),
                extra: serde_json::Value::Null,
            },
            redis_cloud::connectivity::Cidr {
                cidr_address: Some("192.168.0.0/16".to_string()),
                extra: serde_json::Value::Null,
            },
        ]),
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler
        .update_tgw_attachment_cidrs(123, "456", &request)
        .await
        .unwrap();
    assert_eq!(result.task_id, Some("task-update-tgw".to_string()));
    assert_eq!(
        result.command_type,
        Some("UPDATE_TGW_ATTACHMENT_CIDRS".to_string())
    );
}

#[tokio::test]
async fn test_delete_tgw() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/subscriptions/123/transitGateways/456/attachment"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-delete-tgw",
            "commandType": "DELETE_TGW_ATTACHMENT",
            "status": "processing",
            "description": "Deleting Transit Gateway attachment"
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
    let result = handler.delete_tgw_attachment(123, 456).await.unwrap();

    // Delete methods now return serde_json::Value::Null
    assert_eq!(result, serde_json::Value::Null);
}

#[tokio::test]
async fn test_task_response_with_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/subscriptions/123/peerings"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-failed-peering",
            "commandType": "CREATE_VPC_PEERING",
            "status": "failed",
            "description": "Failed to create VPC peering",
            "response": {
                "error": "INVALID_CIDR_RANGE"
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

    let handler = ConnectivityHandler::new(client);
    let request = redis_cloud::connectivity::VpcPeeringCreateBaseRequest {
        provider: Some("AWS".to_string()),
        command_type: None,
        extra: serde_json::Value::Null,
    };

    let result = handler.create_vpc_peering(123, &request).await.unwrap();
    assert_eq!(result.task_id, Some("task-failed-peering".to_string()));
    // Status field would be an enum, not a string - check if it exists
    assert!(result.status.is_some());
    assert_eq!(
        result.description,
        Some("Failed to create VPC peering".to_string())
    );

    assert!(result.response.is_some());
    let response = result.response.unwrap();
    // Check that error field exists and is the expected enum variant
    assert!(response.error.is_some());
}

// Error handling tests

#[tokio::test]
async fn test_error_handling_401() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/subscriptions/123/transitGateways"))
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

    let handler = ConnectivityHandler::new(client);
    let result = handler.get_tgws(123).await;

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
        .and(path("/subscriptions/123/peerings/456"))
        .and(header("x-api-key", "test-key"))
        .and(header("x-api-secret-key", "test-secret"))
        .respond_with(ResponseTemplate::new(403).set_body_json(json!({
            "error": "Forbidden: Insufficient permissions"
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
    let result = handler.delete_vpc_peering(123, 456).await;

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

#[tokio::test]
async fn test_error_handling_500() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/subscriptions/123/private-service-connect"))
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

    let handler = ConnectivityHandler::new(client);
    let result = handler.create_psc_service(123).await;

    assert!(result.is_err());
    match result {
        Err(redis_cloud::CloudError::InternalServerError { .. }) => {}
        _ => panic!("Expected InternalServerError error"),
    }
}
