#[cfg(test)]
mod tests {
    use redis_enterprise::EnterpriseClient;
    use redis_enterprise::local::LocalHandler;
    use serde_json::json;
    use wiremock::matchers::{basic_auth, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn setup_mock_client(mock_server: &MockServer) -> LocalHandler {
        let client = EnterpriseClient::builder()
            .base_url(mock_server.uri())
            .username("test_user")
            .password("test_pass")
            .build()
            .unwrap();
        LocalHandler::new(client)
    }

    #[tokio::test]
    async fn test_master_healthcheck() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        let response_body = json!({
            "status": "healthy",
            "node_uid": 1,
            "role": "master",
            "uptime": 3600,
            "services": {
                "cm_server": "running",
                "mdns_server": "running",
                "pdns_server": "running"
            }
        });

        Mock::given(method("GET"))
            .and(path("/v1/local/node/master_healthcheck"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let result = handler.master_healthcheck().await.unwrap();
        assert_eq!(result["status"], "healthy");
        assert_eq!(result["role"], "master");
        assert_eq!(result["node_uid"], 1);
    }

    #[tokio::test]
    async fn test_master_healthcheck_unhealthy() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        let response_body = json!({
            "status": "unhealthy",
            "node_uid": 1,
            "role": "master",
            "errors": ["Service cm_server is not running"]
        });

        Mock::given(method("GET"))
            .and(path("/v1/local/node/master_healthcheck"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(ResponseTemplate::new(503).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let result = handler.master_healthcheck().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_services() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        let response_body = json!({
            "services": [
                {
                    "name": "cm_server",
                    "status": "running",
                    "port": 8080,
                    "pid": 1234
                },
                {
                    "name": "mdns_server",
                    "status": "running",
                    "port": 53,
                    "pid": 1235
                },
                {
                    "name": "pdns_server",
                    "status": "stopped",
                    "port": 6379,
                    "pid": null
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/v1/local/services"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let result = handler.services().await.unwrap();
        let services = result["services"].as_array().unwrap();
        assert_eq!(services.len(), 3);
        assert_eq!(services[0]["name"], "cm_server");
        assert_eq!(services[0]["status"], "running");
    }

    #[tokio::test]
    async fn test_update_services() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        let request_body = json!({
            "services": [
                {
                    "name": "pdns_server",
                    "action": "start"
                }
            ]
        });

        let response_body = json!({
            "status": "success",
            "services": [
                {
                    "name": "pdns_server",
                    "status": "starting",
                    "message": "Service start initiated"
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path("/v1/local/services"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let result = handler.services_update(request_body).await.unwrap();
        assert_eq!(result["status"], "success");
        assert_eq!(result["services"][0]["status"], "starting");
    }

    #[tokio::test]
    async fn test_update_services_restart() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        let request_body = json!({
            "services": [
                {
                    "name": "cm_server",
                    "action": "restart"
                },
                {
                    "name": "mdns_server",
                    "action": "restart"
                }
            ]
        });

        let response_body = json!({
            "status": "success",
            "services": [
                {
                    "name": "cm_server",
                    "status": "restarting"
                },
                {
                    "name": "mdns_server",
                    "status": "restarting"
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path("/v1/local/services"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let result = handler.services_update(request_body).await.unwrap();
        assert_eq!(result["status"], "success");
        assert_eq!(result["services"][0]["status"], "restarting");
        assert_eq!(result["services"][1]["status"], "restarting");
    }

    #[tokio::test]
    async fn test_update_services_stop() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        let request_body = json!({
            "services": [
                {
                    "name": "pdns_server",
                    "action": "stop"
                }
            ]
        });

        let response_body = json!({
            "status": "success",
            "services": [
                {
                    "name": "pdns_server",
                    "status": "stopped"
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path("/v1/local/services"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let result = handler.services_update(request_body).await.unwrap();
        assert_eq!(result["services"][0]["status"], "stopped");
    }

    #[tokio::test]
    async fn test_update_services_invalid_action() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        let request_body = json!({
            "services": [
                {
                    "name": "invalid_service",
                    "action": "invalid_action"
                }
            ]
        });

        Mock::given(method("POST"))
            .and(path("/v1/local/services"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(ResponseTemplate::new(400).set_body_json(json!({
                "error": "Invalid service action"
            })))
            .mount(&mock_server)
            .await;

        let result = handler.services_update(request_body).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_services_empty_list() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        let response_body = json!({
            "services": []
        });

        Mock::given(method("GET"))
            .and(path("/v1/local/services"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let result = handler.services().await.unwrap();
        let services = result["services"].as_array().unwrap();
        assert_eq!(services.len(), 0);
    }
}
