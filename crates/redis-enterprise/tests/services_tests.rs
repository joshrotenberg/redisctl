//! Services endpoint tests for Redis Enterprise

use redis_enterprise::{EnterpriseClient, ServiceConfigRequest, ServicesHandler};
use serde_json::json;
use wiremock::matchers::{basic_auth, body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// Test helper functions
fn success_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(body)
}

fn error_response(code: u16, message: &str) -> ResponseTemplate {
    ResponseTemplate::new(code).set_body_json(json!({
        "error": message,
        "code": code
    }))
}

fn test_service() -> serde_json::Value {
    json!({
        "service_id": "redis-db",
        "name": "Redis Database Service",
        "service_type": "database",
        "enabled": true,
        "config": {
            "max_connections": 1000,
            "timeout": 30
        },
        "status": "running",
        "node_uids": [1, 2, 3]
    })
}

fn test_service_minimal() -> serde_json::Value {
    json!({
        "service_id": "monitoring",
        "name": "Monitoring Service",
        "service_type": "monitoring",
        "enabled": false
    })
}

fn test_service_status_data() -> serde_json::Value {
    json!({
        "service_id": "redis-db",
        "status": "running",
        "message": "Service is healthy",
        "node_statuses": [
            {
                "node_uid": 1,
                "status": "running",
                "message": "OK"
            },
            {
                "node_uid": 2,
                "status": "running",
                "message": "OK"
            }
        ]
    })
}

fn test_service_status_minimal_data() -> serde_json::Value {
    json!({
        "service_id": "monitoring",
        "status": "stopped"
    })
}

#[tokio::test]
async fn test_services_list() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/services"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            test_service(),
            test_service_minimal()
        ])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ServicesHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let services = result.unwrap();
    assert_eq!(services.len(), 2);
    assert_eq!(services[0].service_id, "redis-db");
    assert_eq!(services[0].name, "Redis Database Service");
    assert!(services[0].enabled);
    assert_eq!(services[1].service_id, "monitoring");
    assert!(!services[1].enabled);
}

#[tokio::test]
async fn test_services_list_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/services"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ServicesHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let services = result.unwrap();
    assert_eq!(services.len(), 0);
}

#[tokio::test]
async fn test_service_get() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/services/redis-db"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_service()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ServicesHandler::new(client);
    let result = handler.get("redis-db").await;

    assert!(result.is_ok());
    let service = result.unwrap();
    assert_eq!(service.service_id, "redis-db");
    assert_eq!(service.name, "Redis Database Service");
    assert_eq!(service.service_type, "database");
    assert!(service.enabled);
    assert!(service.config.is_some());
    assert_eq!(service.status, Some("running".to_string()));
    assert!(service.node_uids.is_some());
}

#[tokio::test]
async fn test_service_get_minimal() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/services/monitoring"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_service_minimal()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ServicesHandler::new(client);
    let result = handler.get("monitoring").await;

    assert!(result.is_ok());
    let service = result.unwrap();
    assert_eq!(service.service_id, "monitoring");
    assert_eq!(service.name, "Monitoring Service");
    assert!(!service.enabled);
    assert!(service.config.is_none());
    assert!(service.status.is_none());
    assert!(service.node_uids.is_none());
}

#[tokio::test]
async fn test_service_get_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/services/nonexistent"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(404, "Service not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ServicesHandler::new(client);
    let result = handler.get("nonexistent").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_service_update() {
    let mock_server = MockServer::start().await;

    let request = ServiceConfigRequest {
        enabled: true,
        config: Some(json!({
            "max_connections": 2000,
            "timeout": 60
        })),
        node_uids: Some(vec![1, 2]),
    };

    Mock::given(method("PUT"))
        .and(path("/v1/services/redis-db"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&request))
        .respond_with(success_response(json!({
            "service_id": "redis-db",
            "name": "Redis Database Service",
            "service_type": "database",
            "enabled": true,
            "config": {
                "max_connections": 2000,
                "timeout": 60
            },
            "status": "running",
            "node_uids": [1, 2]
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ServicesHandler::new(client);
    let result = handler.update("redis-db", request).await;

    assert!(result.is_ok());
    let service = result.unwrap();
    assert_eq!(service.service_id, "redis-db");
    assert!(service.enabled);

    // Check updated config
    let config = service.config.unwrap();
    assert_eq!(config["max_connections"], 2000);
    assert_eq!(config["timeout"], 60);
}

#[tokio::test]
async fn test_service_update_minimal() {
    let mock_server = MockServer::start().await;

    let request = ServiceConfigRequest {
        enabled: false,
        config: None,
        node_uids: None,
    };

    Mock::given(method("PUT"))
        .and(path("/v1/services/monitoring"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&request))
        .respond_with(success_response(json!({
            "service_id": "monitoring",
            "name": "Monitoring Service",
            "service_type": "monitoring",
            "enabled": false
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ServicesHandler::new(client);
    let result = handler.update("monitoring", request).await;

    assert!(result.is_ok());
    let service = result.unwrap();
    assert_eq!(service.service_id, "monitoring");
    assert!(!service.enabled);
}

#[tokio::test]
async fn test_service_update_nonexistent() {
    let mock_server = MockServer::start().await;

    let request = ServiceConfigRequest {
        enabled: true,
        config: None,
        node_uids: None,
    };

    Mock::given(method("PUT"))
        .and(path("/v1/services/nonexistent"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&request))
        .respond_with(error_response(404, "Service not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ServicesHandler::new(client);
    let result = handler.update("nonexistent", request).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_service_status() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/services/redis-db/status"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_service_status_data()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ServicesHandler::new(client);
    let result = handler.status("redis-db").await;

    assert!(result.is_ok());
    let status = result.unwrap();
    assert_eq!(status.service_id, "redis-db");
    assert_eq!(status.status, "running");
    assert_eq!(status.message, Some("Service is healthy".to_string()));

    let node_statuses = status.node_statuses.unwrap();
    assert_eq!(node_statuses.len(), 2);
    assert_eq!(node_statuses[0].node_uid, 1);
    assert_eq!(node_statuses[0].status, "running");
}

#[tokio::test]
async fn test_service_status_minimal() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/services/monitoring/status"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_service_status_minimal_data()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ServicesHandler::new(client);
    let result = handler.status("monitoring").await;

    assert!(result.is_ok());
    let status = result.unwrap();
    assert_eq!(status.service_id, "monitoring");
    assert_eq!(status.status, "stopped");
    assert!(status.message.is_none());
    assert!(status.node_statuses.is_none());
}

#[tokio::test]
async fn test_service_status_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/services/nonexistent/status"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(404, "Service not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ServicesHandler::new(client);
    let result = handler.status("nonexistent").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_service_restart() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/services/redis-db/restart"))
        .and(basic_auth("admin", "password"))
        .and(body_json(json!(null)))
        .respond_with(success_response(json!({
            "service_id": "redis-db",
            "status": "restarting",
            "message": "Service restart initiated"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ServicesHandler::new(client);
    let result = handler.restart("redis-db").await;

    assert!(result.is_ok());
    let status = result.unwrap();
    assert_eq!(status.service_id, "redis-db");
    assert_eq!(status.status, "restarting");
    assert_eq!(
        status.message,
        Some("Service restart initiated".to_string())
    );
}

#[tokio::test]
async fn test_service_restart_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/services/nonexistent/restart"))
        .and(basic_auth("admin", "password"))
        .and(body_json(json!(null)))
        .respond_with(error_response(404, "Service not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ServicesHandler::new(client);
    let result = handler.restart("nonexistent").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_service_stop() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/services/redis-db/stop"))
        .and(basic_auth("admin", "password"))
        .and(body_json(json!(null)))
        .respond_with(success_response(json!({
            "service_id": "redis-db",
            "status": "stopping",
            "message": "Service stop initiated"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ServicesHandler::new(client);
    let result = handler.stop("redis-db").await;

    assert!(result.is_ok());
    let status = result.unwrap();
    assert_eq!(status.service_id, "redis-db");
    assert_eq!(status.status, "stopping");
    assert_eq!(status.message, Some("Service stop initiated".to_string()));
}

#[tokio::test]
async fn test_service_stop_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/services/nonexistent/stop"))
        .and(basic_auth("admin", "password"))
        .and(body_json(json!(null)))
        .respond_with(error_response(404, "Service not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ServicesHandler::new(client);
    let result = handler.stop("nonexistent").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_service_start() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/services/monitoring/start"))
        .and(basic_auth("admin", "password"))
        .and(body_json(json!(null)))
        .respond_with(success_response(json!({
            "service_id": "monitoring",
            "status": "starting",
            "message": "Service start initiated"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ServicesHandler::new(client);
    let result = handler.start("monitoring").await;

    assert!(result.is_ok());
    let status = result.unwrap();
    assert_eq!(status.service_id, "monitoring");
    assert_eq!(status.status, "starting");
    assert_eq!(status.message, Some("Service start initiated".to_string()));
}

#[tokio::test]
async fn test_service_start_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/services/nonexistent/start"))
        .and(basic_auth("admin", "password"))
        .and(body_json(json!(null)))
        .respond_with(error_response(404, "Service not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = ServicesHandler::new(client);
    let result = handler.start("nonexistent").await;

    assert!(result.is_err());
}
