use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::json;
use tempfile::TempDir;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Helper to create a test command with isolated config
fn test_cmd(temp_dir: &TempDir) -> Command {
    let mut cmd = Command::cargo_bin("redisctl").unwrap();
    let config_file = temp_dir.path().join("config.toml");
    cmd.arg("--config-file").arg(config_file);

    // Unset environment variables that might override config
    cmd.env_remove("REDIS_CLOUD_API_KEY");
    cmd.env_remove("REDIS_CLOUD_SECRET_KEY");
    cmd.env_remove("REDIS_CLOUD_API_URL");
    cmd.env_remove("REDIS_ENTERPRISE_URL");
    cmd.env_remove("REDIS_ENTERPRISE_USER");
    cmd.env_remove("REDIS_ENTERPRISE_PASSWORD");
    cmd.env_remove("REDIS_ENTERPRISE_INSECURE");
    cmd.env_remove("REDISCTL_PROFILE");

    cmd
}

/// Create a config file with Cloud profile pointing to mock server
fn create_cloud_profile(temp_dir: &TempDir, api_url: &str) -> std::io::Result<()> {
    use std::fs;
    let config_path = temp_dir.path().join("config.toml");
    let config_content = format!(
        r#"
[profiles.test]
deployment_type = "cloud"
api_key = "test-api-key"
api_secret = "test-api-secret"
api_url = "{}"

default_cloud = "test"
"#,
        api_url
    );
    fs::write(config_path, config_content)
}

/// Create a config file with Enterprise profile pointing to mock server
fn create_enterprise_profile(temp_dir: &TempDir, url: &str) -> std::io::Result<()> {
    use std::fs;
    let config_path = temp_dir.path().join("config.toml");
    let config_content = format!(
        r#"
[profiles.test]
deployment_type = "enterprise"
url = "{}"
username = "admin@redis.local"
password = "test-password"
insecure = true

default_enterprise = "test"
"#,
        url
    );
    fs::write(config_path, config_content)
}

#[tokio::test]
async fn test_api_cloud_get_with_auth_headers() {
    let temp_dir = TempDir::new().unwrap();
    let mock_server = MockServer::start().await;

    // Create profile pointing to mock server
    create_cloud_profile(&temp_dir, &mock_server.uri()).unwrap();

    // Mock a simple GET endpoint and verify auth headers
    Mock::given(method("GET"))
        .and(path("/test"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-api-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "message": "authenticated"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    // Run CLI command using raw API access
    test_cmd(&temp_dir)
        .arg("api")
        .arg("cloud")
        .arg("get")
        .arg("/test")
        .assert()
        .success()
        .stdout(predicate::str::contains("authenticated"));
}

#[tokio::test]
async fn test_api_cloud_post_with_json_body() {
    let temp_dir = TempDir::new().unwrap();
    let mock_server = MockServer::start().await;

    create_cloud_profile(&temp_dir, &mock_server.uri()).unwrap();

    // Mock a POST endpoint
    Mock::given(method("POST"))
        .and(path("/create"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-api-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 123,
            "status": "created"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    // Run CLI command with JSON body
    test_cmd(&temp_dir)
        .arg("api")
        .arg("cloud")
        .arg("post")
        .arg("/create")
        .arg("--data")
        .arg(r#"{"name":"test"}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("created"));
}

#[tokio::test]
async fn test_api_enterprise_get_with_basic_auth() {
    let temp_dir = TempDir::new().unwrap();
    let mock_server = MockServer::start().await;

    create_enterprise_profile(&temp_dir, &mock_server.uri()).unwrap();

    // Mock Enterprise API endpoint
    Mock::given(method("GET"))
        .and(path("/v1/cluster"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "name": "Test Cluster",
            "version": "7.4.2"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    // Run CLI command
    test_cmd(&temp_dir)
        .arg("api")
        .arg("enterprise")
        .arg("get")
        .arg("/v1/cluster")
        .assert()
        .success()
        .stdout(predicate::str::contains("Test Cluster"));
}

#[tokio::test]
async fn test_api_error_response_401() {
    let temp_dir = TempDir::new().unwrap();
    let mock_server = MockServer::start().await;

    create_cloud_profile(&temp_dir, &mock_server.uri()).unwrap();

    // Mock 401 authentication failure
    Mock::given(method("GET"))
        .and(path("/test"))
        .respond_with(ResponseTemplate::new(401).set_body_json(json!({
            "error": {
                "type": "UNAUTHORIZED",
                "status": 401,
                "description": "Invalid API credentials"
            }
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    // Run CLI command - should fail with 401
    test_cmd(&temp_dir)
        .arg("api")
        .arg("cloud")
        .arg("get")
        .arg("/test")
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("401")
                .or(predicate::str::contains("Unauthorized"))
                .or(predicate::str::contains("Invalid API credentials")),
        );
}

#[tokio::test]
async fn test_api_json_output_format() {
    let temp_dir = TempDir::new().unwrap();
    let mock_server = MockServer::start().await;

    create_cloud_profile(&temp_dir, &mock_server.uri()).unwrap();

    // Mock endpoint
    Mock::given(method("GET"))
        .and(path("/test"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "items": [
                {"id": 1, "name": "item1"},
                {"id": 2, "name": "item2"}
            ]
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    // Run CLI command with JSON output
    test_cmd(&temp_dir)
        .arg("api")
        .arg("cloud")
        .arg("get")
        .arg("/test")
        .arg("-o")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::contains(r#""id": 1"#).or(predicate::str::contains("item1")));
}

#[tokio::test]
async fn test_cloud_subscription_get_by_id() {
    let temp_dir = TempDir::new().unwrap();
    let mock_server = MockServer::start().await;

    create_cloud_profile(&temp_dir, &mock_server.uri()).unwrap();

    // Mock subscription endpoint
    Mock::given(method("GET"))
        .and(path("/subscriptions/12345"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-api-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "subscriptionId": 12345,
            "name": "Test Subscription",
            "status": "active",
            "cloudProvider": {
                "provider": "AWS",
                "region": "us-east-1"
            }
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_cmd(&temp_dir)
        .arg("api")
        .arg("cloud")
        .arg("get")
        .arg("/subscriptions/12345")
        .assert()
        .success()
        .stdout(predicate::str::contains("Test Subscription"))
        .stdout(predicate::str::contains("active"));
}

#[tokio::test]
async fn test_enterprise_database_list() {
    let temp_dir = TempDir::new().unwrap();
    let mock_server = MockServer::start().await;

    create_enterprise_profile(&temp_dir, &mock_server.uri()).unwrap();

    // Mock databases list endpoint
    Mock::given(method("GET"))
        .and(path("/v1/bdbs"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {
                "uid": 1,
                "name": "db1",
                "type": "redis",
                "status": "active"
            },
            {
                "uid": 2,
                "name": "db2",
                "type": "redis",
                "status": "active"
            }
        ])))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_cmd(&temp_dir)
        .arg("api")
        .arg("enterprise")
        .arg("get")
        .arg("/v1/bdbs")
        .assert()
        .success()
        .stdout(predicate::str::contains("db1"))
        .stdout(predicate::str::contains("db2"));
}

#[tokio::test]
async fn test_api_delete_request() {
    let temp_dir = TempDir::new().unwrap();
    let mock_server = MockServer::start().await;

    create_cloud_profile(&temp_dir, &mock_server.uri()).unwrap();

    // Mock DELETE endpoint
    Mock::given(method("DELETE"))
        .and(path("/resources/999"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-api-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "taskId": "abc-123",
            "status": "processing"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_cmd(&temp_dir)
        .arg("api")
        .arg("cloud")
        .arg("delete")
        .arg("/resources/999")
        .assert()
        .success()
        .stdout(predicate::str::contains("taskId").or(predicate::str::contains("abc-123")));
}

#[tokio::test]
async fn test_api_put_request() {
    let temp_dir = TempDir::new().unwrap();
    let mock_server = MockServer::start().await;

    create_cloud_profile(&temp_dir, &mock_server.uri()).unwrap();

    // Mock PUT endpoint
    Mock::given(method("PUT"))
        .and(path("/resources/555"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-api-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 555,
            "name": "Updated Resource",
            "updated": true
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_cmd(&temp_dir)
        .arg("api")
        .arg("cloud")
        .arg("put")
        .arg("/resources/555")
        .arg("--data")
        .arg(r#"{"name":"Updated Resource"}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("Updated Resource"));
}

#[tokio::test]
async fn test_api_error_404_not_found() {
    let temp_dir = TempDir::new().unwrap();
    let mock_server = MockServer::start().await;

    create_cloud_profile(&temp_dir, &mock_server.uri()).unwrap();

    // Mock 404 not found
    Mock::given(method("GET"))
        .and(path("/nonexistent"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "error": {
                "type": "NOT_FOUND",
                "status": 404,
                "description": "Resource not found"
            }
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_cmd(&temp_dir)
        .arg("api")
        .arg("cloud")
        .arg("get")
        .arg("/nonexistent")
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("404")
                .or(predicate::str::contains("not found"))
                .or(predicate::str::contains("Not Found")),
        );
}

#[tokio::test]
async fn test_api_error_500_server_error() {
    let temp_dir = TempDir::new().unwrap();
    let mock_server = MockServer::start().await;

    create_cloud_profile(&temp_dir, &mock_server.uri()).unwrap();

    // Mock 500 internal server error
    Mock::given(method("GET"))
        .and(path("/error"))
        .respond_with(ResponseTemplate::new(500).set_body_json(json!({
            "error": {
                "type": "INTERNAL_ERROR",
                "status": 500,
                "description": "Internal server error"
            }
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_cmd(&temp_dir)
        .arg("api")
        .arg("cloud")
        .arg("get")
        .arg("/error")
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("500")
                .or(predicate::str::contains("Internal"))
                .or(predicate::str::contains("server error")),
        );
}

#[tokio::test]
async fn test_api_jmespath_query() {
    let temp_dir = TempDir::new().unwrap();
    let mock_server = MockServer::start().await;

    create_cloud_profile(&temp_dir, &mock_server.uri()).unwrap();

    // Mock endpoint with nested data
    Mock::given(method("GET"))
        .and(path("/data"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "subscriptions": [
                {"id": 1, "name": "sub1", "status": "active"},
                {"id": 2, "name": "sub2", "status": "inactive"}
            ]
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    // Run with JMESPath query to filter active subscriptions
    test_cmd(&temp_dir)
        .arg("api")
        .arg("cloud")
        .arg("get")
        .arg("/data")
        .arg("-q")
        .arg("subscriptions[?status=='active'].name")
        .assert()
        .success()
        .stdout(predicate::str::contains("sub1"));
}

#[tokio::test]
async fn test_enterprise_cluster_with_verbose_logging() {
    let temp_dir = TempDir::new().unwrap();
    let mock_server = MockServer::start().await;

    create_enterprise_profile(&temp_dir, &mock_server.uri()).unwrap();

    // Mock cluster endpoint
    Mock::given(method("GET"))
        .and(path("/v1/cluster"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "name": "prod-cluster",
            "version": "7.4.2",
            "nodes_count": 3
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    // Run with verbose flag - should still succeed
    test_cmd(&temp_dir)
        .arg("api")
        .arg("enterprise")
        .arg("get")
        .arg("/v1/cluster")
        .arg("--verbose")
        .assert()
        .success()
        .stdout(predicate::str::contains("prod-cluster"));
}

#[tokio::test]
async fn test_cloud_database_create_async_operation() {
    let temp_dir = TempDir::new().unwrap();
    let mock_server = MockServer::start().await;

    create_cloud_profile(&temp_dir, &mock_server.uri()).unwrap();

    // Mock database creation that returns a task ID
    Mock::given(method("POST"))
        .and(path("/subscriptions/123/databases"))
        .respond_with(ResponseTemplate::new(202).set_body_json(json!({
            "taskId": "task-xyz-789",
            "commandType": "databaseCreateRequest",
            "status": "received"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    // Create without --wait should return task ID immediately
    test_cmd(&temp_dir)
        .arg("api")
        .arg("cloud")
        .arg("post")
        .arg("/subscriptions/123/databases")
        .arg("--data")
        .arg(r#"{"name":"test-db","memoryLimitInGb":1}"#)
        .assert()
        .success()
        .stdout(predicate::str::contains("taskId").or(predicate::str::contains("task-xyz-789")));
}

#[tokio::test]
async fn test_enterprise_multiple_endpoints_same_server() {
    let temp_dir = TempDir::new().unwrap();
    let mock_server = MockServer::start().await;

    create_enterprise_profile(&temp_dir, &mock_server.uri()).unwrap();

    // Mock cluster info
    Mock::given(method("GET"))
        .and(path("/v1/cluster"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "name": "test-cluster"
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    // Mock nodes list
    Mock::given(method("GET"))
        .and(path("/v1/nodes"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([
            {"uid": 1, "addr": "10.0.0.1"},
            {"uid": 2, "addr": "10.0.0.2"}
        ])))
        .expect(1)
        .mount(&mock_server)
        .await;

    // First request - cluster
    test_cmd(&temp_dir)
        .arg("api")
        .arg("enterprise")
        .arg("get")
        .arg("/v1/cluster")
        .assert()
        .success()
        .stdout(predicate::str::contains("test-cluster"));

    // Second request - nodes (reusing same profile)
    test_cmd(&temp_dir)
        .arg("api")
        .arg("enterprise")
        .arg("get")
        .arg("/v1/nodes")
        .assert()
        .success()
        .stdout(predicate::str::contains("10.0.0.1"));
}

#[tokio::test]
async fn test_yaml_output_format() {
    let temp_dir = TempDir::new().unwrap();
    let mock_server = MockServer::start().await;

    create_cloud_profile(&temp_dir, &mock_server.uri()).unwrap();

    // Mock endpoint
    Mock::given(method("GET"))
        .and(path("/test"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "name": "test-resource",
            "id": 456
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    // Run with YAML output
    test_cmd(&temp_dir)
        .arg("api")
        .arg("cloud")
        .arg("get")
        .arg("/test")
        .arg("-o")
        .arg("yaml")
        .assert()
        .success()
        .stdout(
            predicate::str::contains("name:")
                .and(predicate::str::contains("test-resource"))
                .or(predicate::str::contains("id: 456")),
        );
}

#[tokio::test]
async fn test_cloud_list_all_subscriptions() {
    let temp_dir = TempDir::new().unwrap();
    let mock_server = MockServer::start().await;

    create_cloud_profile(&temp_dir, &mock_server.uri()).unwrap();

    // Mock subscriptions list endpoint
    Mock::given(method("GET"))
        .and(path("/subscriptions"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-api-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "subscriptions": [
                {
                    "id": 100,
                    "name": "production",
                    "status": "active"
                },
                {
                    "id": 200,
                    "name": "staging",
                    "status": "active"
                }
            ]
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    test_cmd(&temp_dir)
        .arg("api")
        .arg("cloud")
        .arg("get")
        .arg("/subscriptions")
        .assert()
        .success()
        .stdout(predicate::str::contains("production"))
        .stdout(predicate::str::contains("staging"));
}
