//! Tests for the Enterprise library

#[cfg(test)]
mod tests {
    use crate::{EnterpriseClient, RestError, Result};
    use wiremock::matchers::{basic_auth, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_enterprise_client_builder_default() {
        let builder = EnterpriseClient::builder();
        // Builder defaults are tested through build
        let client = builder.username("test").password("test").build();
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_enterprise_client_creation() {
        let result = EnterpriseClient::builder()
            .base_url("https://example.com")
            .username("test_user")
            .password("test_pass")
            .timeout(std::time::Duration::from_secs(10))
            .insecure(false)
            .build();

        // Client should be created successfully
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_enterprise_client_get_request() {
        // Start a background HTTP server on a random local port
        let mock_server = MockServer::start().await;

        // Arrange the behaviour of the MockServer adding a Mock
        Mock::given(method("GET"))
            .and(path("/test"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"status": "ok"})),
            )
            .mount(&mock_server)
            .await;

        let client = EnterpriseClient::builder()
            .base_url(mock_server.uri())
            .username("test_user")
            .password("test_pass")
            .timeout(std::time::Duration::from_secs(10))
            .insecure(false)
            .build()
            .unwrap();
        let result: Result<serde_json::Value> = client.get("/test").await;

        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value["status"], "ok");
    }

    #[tokio::test]
    async fn test_enterprise_client_post_request() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/test"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(
                ResponseTemplate::new(201).set_body_json(serde_json::json!({"created": true})),
            )
            .mount(&mock_server)
            .await;

        let client = EnterpriseClient::builder()
            .base_url(mock_server.uri())
            .username("test_user")
            .password("test_pass")
            .timeout(std::time::Duration::from_secs(10))
            .insecure(false)
            .build()
            .unwrap();
        let test_data = serde_json::json!({"name": "test"});
        let result: Result<serde_json::Value> = client.post("/test", &test_data).await;

        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value["created"], true);
    }

    #[tokio::test]
    async fn test_enterprise_client_error_handling() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/error"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(
                ResponseTemplate::new(404).set_body_json(serde_json::json!({"error": "Not found"})),
            )
            .mount(&mock_server)
            .await;

        let client = EnterpriseClient::builder()
            .base_url(mock_server.uri())
            .username("test_user")
            .password("test_pass")
            .timeout(std::time::Duration::from_secs(10))
            .insecure(false)
            .build()
            .unwrap();
        let result: Result<serde_json::Value> = client.get("/error").await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.is_not_found(),
            "Expected not found error, got: {:?}",
            err
        );
    }

    #[tokio::test]
    async fn test_enterprise_client_authentication_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/auth-test"))
            .respond_with(
                ResponseTemplate::new(401)
                    .set_body_json(serde_json::json!({"error": "Unauthorized"})),
            )
            .mount(&mock_server)
            .await;

        let client = EnterpriseClient::builder()
            .base_url(mock_server.uri())
            .username("wrong_user")
            .password("wrong_pass")
            .timeout(std::time::Duration::from_secs(10))
            .insecure(false)
            .build()
            .unwrap();
        let result: Result<serde_json::Value> = client.get("/auth-test").await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.is_unauthorized(),
            "Expected unauthorized error, got: {:?}",
            err
        );
    }

    #[test]
    fn test_rest_error_display() {
        let err = RestError::AuthenticationFailed;
        assert_eq!(err.to_string(), "Authentication failed");

        let err = RestError::ApiError {
            code: 400,
            message: "Bad request".to_string(),
        };
        assert_eq!(err.to_string(), "API error: Bad request (code: 400)");

        let err = RestError::ConnectionError("Connection refused".to_string());
        assert_eq!(err.to_string(), "Connection error: Connection refused");
    }

    #[tokio::test]
    async fn test_enterprise_client_delete_request() {
        let mock_server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/test/123"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let client = EnterpriseClient::builder()
            .base_url(mock_server.uri())
            .username("test_user")
            .password("test_pass")
            .timeout(std::time::Duration::from_secs(10))
            .insecure(false)
            .build()
            .unwrap();
        let result = client.delete("/test/123").await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_database_action_start() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/bdbs/1/actions/start"))
            .and(basic_auth("admin", "password"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"status": "started"})),
            )
            .mount(&mock_server)
            .await;

        let client = EnterpriseClient::builder()
            .base_url(mock_server.uri())
            .username("admin")
            .password("password")
            .build()
            .unwrap();

        let handler = crate::bdb::DatabaseHandler::new(client);
        let result = handler.start(1).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap()["status"], "started");
    }

    #[tokio::test]
    async fn test_database_action_stop() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/bdbs/1/actions/stop"))
            .and(basic_auth("admin", "password"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"status": "stopped"})),
            )
            .mount(&mock_server)
            .await;

        let client = EnterpriseClient::builder()
            .base_url(mock_server.uri())
            .username("admin")
            .password("password")
            .build()
            .unwrap();

        let handler = crate::bdb::DatabaseHandler::new(client);
        let result = handler.stop(1).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap()["status"], "stopped");
    }

    #[tokio::test]
    async fn test_database_action_export() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/bdbs/1/actions/export"))
            .and(basic_auth("admin", "password"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!({"task_id": "export-123"})),
            )
            .mount(&mock_server)
            .await;

        let client = EnterpriseClient::builder()
            .base_url(mock_server.uri())
            .username("admin")
            .password("password")
            .build()
            .unwrap();

        let handler = crate::bdb::DatabaseHandler::new(client);
        let result = handler.export(1, "ftp://backup/db1.rdb").await;

        assert!(result.is_ok());
        // ExportResponse doesn't have task_id, check action_uid instead
        assert!(result.unwrap().extra["task_id"].is_string());
    }

    #[tokio::test]
    async fn test_database_action_import() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/bdbs/1/actions/import"))
            .and(basic_auth("admin", "password"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!({"task_id": "import-456"})),
            )
            .mount(&mock_server)
            .await;

        let client = EnterpriseClient::builder()
            .base_url(mock_server.uri())
            .username("admin")
            .password("password")
            .build()
            .unwrap();

        let handler = crate::bdb::DatabaseHandler::new(client);
        let result = handler.import(1, "ftp://backup/db1.rdb", true).await;

        assert!(result.is_ok());
        // ImportResponse doesn't have task_id, check action_uid instead
        assert!(result.unwrap().extra["task_id"].is_string());
    }

    #[tokio::test]
    async fn test_database_action_backup() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/bdbs/1/actions/backup"))
            .and(basic_auth("admin", "password"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!({"backup_id": "backup-789"})),
            )
            .mount(&mock_server)
            .await;

        let client = EnterpriseClient::builder()
            .base_url(mock_server.uri())
            .username("admin")
            .password("password")
            .build()
            .unwrap();

        let handler = crate::bdb::DatabaseHandler::new(client);
        let result = handler.backup(1).await;

        assert!(result.is_ok());
        // BackupResponse has backup_uid field
        assert!(result.unwrap().extra["backup_id"].is_string());
    }

    #[tokio::test]
    async fn test_database_get_shards() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/v1/bdbs/1/shards"))
            .and(basic_auth("admin", "password"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                {"shard_id": 1, "role": "master"},
                {"shard_id": 2, "role": "slave"}
            ])))
            .mount(&mock_server)
            .await;

        let client = EnterpriseClient::builder()
            .base_url(mock_server.uri())
            .username("admin")
            .password("password")
            .build()
            .unwrap();

        let handler = crate::bdb::DatabaseHandler::new(client);
        let result = handler.shards(1).await;

        assert!(result.is_ok());
        let shards = result.unwrap();
        assert!(shards.is_array());
    }

    #[tokio::test]
    async fn test_cluster_join_node() {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/v1/bootstrap/join"))
            .and(basic_auth("admin", "password"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"status": "joined"})),
            )
            .mount(&mock_server)
            .await;

        let client = EnterpriseClient::builder()
            .base_url(mock_server.uri())
            .username("admin")
            .password("password")
            .build()
            .unwrap();

        let handler = crate::cluster::ClusterHandler::new(client);
        let result = handler.join_node("192.168.1.10", "admin", "password").await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap()["status"], "joined");
    }
}
