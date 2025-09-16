#[cfg(test)]
mod tests {
    use redis_enterprise::EnterpriseClient;
    use redis_enterprise::debuginfo::{DebugInfoHandler, DebugInfoRequest, TimeRange};
    use serde_json::json;
    use wiremock::matchers::{basic_auth, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn setup_mock_client(mock_server: &MockServer) -> DebugInfoHandler {
        let client = EnterpriseClient::builder()
            .base_url(mock_server.uri())
            .username("test_user")
            .password("test_pass")
            .build()
            .unwrap();
        DebugInfoHandler::new(client)
    }

    #[tokio::test]
    async fn test_create_debug_info() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        let request = DebugInfoRequest::builder()
            .node_uids(vec![1, 2])
            .include_logs(true)
            .include_metrics(true)
            .build();

        let response_body = json!({
            "task_id": "debug-task-123",
            "status": "in_progress",
            "progress": 0.0
        });

        Mock::given(method("POST"))
            .and(path("/v1/debuginfo"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let status = handler.create(request).await.unwrap();
        assert_eq!(status.task_id, "debug-task-123");
        assert_eq!(status.status, "in_progress");
    }

    #[tokio::test]
    async fn test_get_debug_info_status() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        let response_body = json!({
            "task_id": "debug-task-123",
            "status": "completed",
            "progress": 100.0,
            "download_url": "/v1/debuginfo/debug-task-123/download"
        });

        Mock::given(method("GET"))
            .and(path("/v1/debuginfo/debug-task-123"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let status = handler.status("debug-task-123").await.unwrap();
        assert_eq!(status.task_id, "debug-task-123");
        assert_eq!(status.status, "completed");
        assert_eq!(status.progress, Some(100.0));
        assert!(status.download_url.is_some());
    }

    #[tokio::test]
    async fn test_list_debug_info_tasks() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        let response_body = json!([
            {
                "task_id": "debug-task-123",
                "status": "completed",
                "progress": 100.0
            },
            {
                "task_id": "debug-task-456",
                "status": "in_progress",
                "progress": 45.0
            }
        ]);

        Mock::given(method("GET"))
            .and(path("/v1/debuginfo"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let tasks = handler.list().await.unwrap();
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].task_id, "debug-task-123");
        assert_eq!(tasks[1].task_id, "debug-task-456");
    }

    #[tokio::test]
    async fn test_download_debug_info() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        let response_body = b"debug package binary content";

        Mock::given(method("GET"))
            .and(path("/v1/debuginfo/debug-task-123/download"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_raw(response_body.to_vec(), "application/octet-stream"),
            )
            .mount(&mock_server)
            .await;

        let data = handler.download("debug-task-123").await.unwrap();
        assert_eq!(data, response_body);
    }

    #[tokio::test]
    async fn test_cancel_debug_info() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        Mock::given(method("DELETE"))
            .and(path("/v1/debuginfo/debug-task-123"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let result = handler.cancel("debug-task-123").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_all_debug_info() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        let response_body = json!({
            "nodes": [
                {"node_uid": 1, "debug_data": "node1 info"},
                {"node_uid": 2, "debug_data": "node2 info"}
            ]
        });

        Mock::given(method("GET"))
            .and(path("/v1/debuginfo/all"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let result = handler.all().await.unwrap();
        assert!(result.get("nodes").is_some());
    }

    #[tokio::test]
    async fn test_get_all_bdb_debug_info() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        let response_body = json!({
            "bdb_uid": 1,
            "debug_data": "database debug info"
        });

        Mock::given(method("GET"))
            .and(path("/v1/debuginfo/all/bdb/1"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let result = handler.all_bdb(1).await.unwrap();
        assert_eq!(result["bdb_uid"], 1);
    }

    #[tokio::test]
    async fn test_get_node_debug_info() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        let response_body = json!({
            "node_uid": 1,
            "status": "healthy",
            "debug_data": "local node debug info"
        });

        Mock::given(method("GET"))
            .and(path("/v1/debuginfo/node"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let result = handler.node().await.unwrap();
        assert_eq!(result["status"], "healthy");
    }

    #[tokio::test]
    async fn test_get_node_bdb_debug_info() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        let response_body = json!({
            "bdb_uid": 2,
            "node_uid": 1,
            "debug_data": "node specific database debug info"
        });

        Mock::given(method("GET"))
            .and(path("/v1/debuginfo/node/bdb/2"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let result = handler.node_bdb(2).await.unwrap();
        assert_eq!(result["bdb_uid"], 2);
    }

    #[tokio::test]
    async fn test_create_debug_info_with_time_range() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        let request = DebugInfoRequest::builder()
            .time_range(TimeRange {
                start: "2024-01-01T00:00:00Z".to_string(),
                end: "2024-01-02T00:00:00Z".to_string(),
            })
            .include_configs(true)
            .build();

        let response_body = json!({
            "task_id": "debug-task-789",
            "status": "queued"
        });

        Mock::given(method("POST"))
            .and(path("/v1/debuginfo"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let status = handler.create(request).await.unwrap();
        assert_eq!(status.task_id, "debug-task-789");
        assert_eq!(status.status, "queued");
    }

    // Tests for new binary endpoints

    #[tokio::test]
    async fn test_cluster_debuginfo_binary() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        // Create a simple gzip tarball for testing
        let tar_gz_data = vec![
            0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0x62, 0x18, 0x05, 0xa3,
            0x60, 0x14, 0x8c, 0x58, 0x00, 0x00, 0x00,
        ];

        Mock::given(method("GET"))
            .and(path("/v1/cluster/debuginfo"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "application/x-gzip")
                    .set_body_raw(tar_gz_data.clone(), "application/x-gzip"),
            )
            .mount(&mock_server)
            .await;

        let data = handler.cluster_debuginfo_binary().await.unwrap();
        assert_eq!(data[0..2], [0x1f, 0x8b]); // Gzip magic bytes
    }

    #[tokio::test]
    async fn test_nodes_debuginfo_binary() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        let tar_gz_data = b"fake tar.gz content for nodes";

        Mock::given(method("GET"))
            .and(path("/v1/nodes/debuginfo"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "application/x-gzip")
                    .set_body_raw(tar_gz_data.to_vec(), "application/x-gzip"),
            )
            .mount(&mock_server)
            .await;

        let data = handler.nodes_debuginfo_binary().await.unwrap();
        assert_eq!(data, tar_gz_data);
    }

    #[tokio::test]
    async fn test_node_debuginfo_binary() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        let tar_gz_data = b"fake tar.gz content for node 1";

        Mock::given(method("GET"))
            .and(path("/v1/nodes/1/debuginfo"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "application/x-gzip")
                    .set_body_raw(tar_gz_data.to_vec(), "application/x-gzip"),
            )
            .mount(&mock_server)
            .await;

        let data = handler.node_debuginfo_binary(1).await.unwrap();
        assert_eq!(data, tar_gz_data);
    }

    #[tokio::test]
    async fn test_databases_debuginfo_binary() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        let tar_gz_data = b"fake tar.gz content for all databases";

        Mock::given(method("GET"))
            .and(path("/v1/bdbs/debuginfo"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "application/x-gzip")
                    .set_body_raw(tar_gz_data.to_vec(), "application/x-gzip"),
            )
            .mount(&mock_server)
            .await;

        let data = handler.databases_debuginfo_binary().await.unwrap();
        assert_eq!(data, tar_gz_data);
    }

    #[tokio::test]
    async fn test_database_debuginfo_binary() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        let tar_gz_data = b"fake tar.gz content for database 2";

        Mock::given(method("GET"))
            .and(path("/v1/bdbs/2/debuginfo"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "application/x-gzip")
                    .set_body_raw(tar_gz_data.to_vec(), "application/x-gzip"),
            )
            .mount(&mock_server)
            .await;

        let data = handler.database_debuginfo_binary(2).await.unwrap();
        assert_eq!(data, tar_gz_data);
    }

    // Tests for deprecated binary endpoints

    #[tokio::test]
    async fn test_all_binary_deprecated() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        let tar_gz_data = b"deprecated all debuginfo endpoint";

        Mock::given(method("GET"))
            .and(path("/v1/debuginfo/all"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "application/x-gzip")
                    .set_body_raw(tar_gz_data.to_vec(), "application/x-gzip"),
            )
            .mount(&mock_server)
            .await;

        let data = handler.all_binary().await.unwrap();
        assert_eq!(data, tar_gz_data);
    }

    #[tokio::test]
    async fn test_all_bdb_binary_deprecated() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        let tar_gz_data = b"deprecated database debuginfo";

        Mock::given(method("GET"))
            .and(path("/v1/debuginfo/all/bdb/3"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "application/x-gzip")
                    .set_body_raw(tar_gz_data.to_vec(), "application/x-gzip"),
            )
            .mount(&mock_server)
            .await;

        let data = handler.all_bdb_binary(3).await.unwrap();
        assert_eq!(data, tar_gz_data);
    }

    #[tokio::test]
    async fn test_node_binary_deprecated() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        let tar_gz_data = b"deprecated node debuginfo";

        Mock::given(method("GET"))
            .and(path("/v1/debuginfo/node"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "application/x-gzip")
                    .set_body_raw(tar_gz_data.to_vec(), "application/x-gzip"),
            )
            .mount(&mock_server)
            .await;

        let data = handler.node_binary().await.unwrap();
        assert_eq!(data, tar_gz_data);
    }

    #[tokio::test]
    async fn test_node_bdb_binary_deprecated() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        let tar_gz_data = b"deprecated node bdb debuginfo";

        Mock::given(method("GET"))
            .and(path("/v1/debuginfo/node/bdb/4"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(
                ResponseTemplate::new(200)
                    .insert_header("content-type", "application/x-gzip")
                    .set_body_raw(tar_gz_data.to_vec(), "application/x-gzip"),
            )
            .mount(&mock_server)
            .await;

        let data = handler.node_bdb_binary(4).await.unwrap();
        assert_eq!(data, tar_gz_data);
    }
}
