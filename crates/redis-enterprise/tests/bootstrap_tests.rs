//! Bootstrap endpoint tests for Redis Enterprise

use redis_enterprise::{
    BootstrapConfig, BootstrapHandler, ClusterBootstrap, CredentialsBootstrap, EnterpriseClient,
    NodeBootstrap, NodePaths,
};
use serde_json::json;
use wiremock::matchers::{basic_auth, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// Test helper functions
fn success_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(body)
}

fn created_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(201).set_body_json(body)
}

fn no_content_response() -> ResponseTemplate {
    ResponseTemplate::new(204)
}

fn bootstrap_status_response(
    status: &str,
    progress: Option<f32>,
    message: Option<&str>,
) -> serde_json::Value {
    json!({
        "status": status,
        "progress": progress,
        "message": message
    })
}

fn cluster_bootstrap_config() -> BootstrapConfig {
    BootstrapConfig {
        action: "cluster_create".to_string(),
        cluster: Some(ClusterBootstrap {
            name: "test-cluster".to_string(),
            dns_suffixes: Some(vec!["cluster.local".to_string()]),
            rack_aware: Some(false),
        }),
        node: Some(NodeBootstrap {
            paths: Some(NodePaths {
                persistent_path: Some("/opt/redislabs/persist".to_string()),
                ephemeral_path: Some("/opt/redislabs/tmp".to_string()),
            }),
        }),
        credentials: Some(CredentialsBootstrap {
            username: "admin".to_string(),
            password: "secure123".to_string(),
        }),
        extra: json!({}),
    }
}

fn join_node_config() -> BootstrapConfig {
    BootstrapConfig {
        action: "join_cluster".to_string(),
        cluster: None,
        node: Some(NodeBootstrap {
            paths: Some(NodePaths {
                persistent_path: Some("/opt/redislabs/persist".to_string()),
                ephemeral_path: Some("/opt/redislabs/tmp".to_string()),
            }),
        }),
        credentials: Some(CredentialsBootstrap {
            username: "admin".to_string(),
            password: "secure123".to_string(),
        }),
        extra: json!({
            "cluster_host": "10.0.0.1",
            "cluster_port": 9443
        }),
    }
}

#[tokio::test]
async fn test_bootstrap_create_cluster() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/bootstrap"))
        .and(basic_auth("admin", "password"))
        .respond_with(created_response(bootstrap_status_response(
            "in_progress",
            Some(10.0),
            Some("Initializing cluster"),
        )))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = BootstrapHandler::new(client);
    let config = cluster_bootstrap_config();
    let result = handler.create(config).await;

    assert!(result.is_ok());
    let status = result.unwrap();
    assert_eq!(status.status, "in_progress");
    assert_eq!(status.progress, Some(10.0));
    assert_eq!(status.message, Some("Initializing cluster".to_string()));
}

#[tokio::test]
async fn test_bootstrap_status_in_progress() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/bootstrap"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(bootstrap_status_response(
            "in_progress",
            Some(75.5),
            Some("Configuring cluster nodes"),
        )))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = BootstrapHandler::new(client);
    let result = handler.status().await;

    assert!(result.is_ok());
    let status = result.unwrap();
    assert_eq!(status.status, "in_progress");
    assert_eq!(status.progress, Some(75.5));
    assert_eq!(
        status.message,
        Some("Configuring cluster nodes".to_string())
    );
}

#[tokio::test]
async fn test_bootstrap_status_completed() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/bootstrap"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(bootstrap_status_response(
            "completed",
            Some(100.0),
            Some("Cluster initialization completed successfully"),
        )))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = BootstrapHandler::new(client);
    let result = handler.status().await;

    assert!(result.is_ok());
    let status = result.unwrap();
    assert_eq!(status.status, "completed");
    assert_eq!(status.progress, Some(100.0));
}

#[tokio::test]
async fn test_bootstrap_status_failed() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/bootstrap"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(bootstrap_status_response(
            "failed",
            Some(45.0),
            Some("Failed to connect to cluster node"),
        )))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = BootstrapHandler::new(client);
    let result = handler.status().await;

    assert!(result.is_ok());
    let status = result.unwrap();
    assert_eq!(status.status, "failed");
    assert_eq!(status.progress, Some(45.0));
    assert_eq!(
        status.message,
        Some("Failed to connect to cluster node".to_string())
    );
}

#[tokio::test]
async fn test_bootstrap_status_not_started() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/bootstrap"))
        .and(basic_auth("admin", "password"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "error": "Bootstrap not initiated"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = BootstrapHandler::new(client);
    let result = handler.status().await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_bootstrap_join_node() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/bootstrap/join"))
        .and(basic_auth("admin", "password"))
        .respond_with(created_response(bootstrap_status_response(
            "in_progress",
            Some(5.0),
            Some("Joining node to cluster"),
        )))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = BootstrapHandler::new(client);
    let config = join_node_config();
    let result = handler.join(config).await;

    assert!(result.is_ok());
    let status = result.unwrap();
    assert_eq!(status.status, "in_progress");
    assert_eq!(status.progress, Some(5.0));
    assert_eq!(status.message, Some("Joining node to cluster".to_string()));
}

#[tokio::test]
async fn test_bootstrap_reset() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/bootstrap"))
        .and(basic_auth("admin", "password"))
        .respond_with(no_content_response())
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = BootstrapHandler::new(client);
    let result = handler.reset().await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_bootstrap_create_minimal_config() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v1/bootstrap"))
        .and(basic_auth("admin", "password"))
        .respond_with(created_response(bootstrap_status_response(
            "in_progress",
            Some(0.0),
            Some("Starting bootstrap process"),
        )))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = BootstrapHandler::new(client);

    // Minimal config - just action and credentials
    let config = BootstrapConfig {
        action: "minimal_cluster".to_string(),
        cluster: None,
        node: None,
        credentials: Some(CredentialsBootstrap {
            username: "admin".to_string(),
            password: "minimal123".to_string(),
        }),
        extra: json!({}),
    };

    let result = handler.create(config).await;

    assert!(result.is_ok());
    let status = result.unwrap();
    assert_eq!(status.status, "in_progress");
    assert_eq!(status.progress, Some(0.0));
}
