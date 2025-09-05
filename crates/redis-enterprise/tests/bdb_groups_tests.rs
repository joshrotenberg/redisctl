#[cfg(test)]
mod tests {
    use redis_enterprise::EnterpriseClient;
    use redis_enterprise::bdb_groups::{
        BdbGroupsHandler, CreateBdbGroupRequest, UpdateBdbGroupRequest,
    };
    use serde_json::json;
    use wiremock::matchers::{basic_auth, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    async fn setup_mock_client(mock_server: &MockServer) -> BdbGroupsHandler {
        let client = EnterpriseClient::builder()
            .base_url(mock_server.uri())
            .username("test_user")
            .password("test_pass")
            .build()
            .unwrap();
        BdbGroupsHandler::new(client)
    }

    #[tokio::test]
    async fn test_list_bdb_groups() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        let response_body = json!([
            {
                "uid": 1,
                "name": "group1",
                "bdbs": [1, 2, 3]
            },
            {
                "uid": 2,
                "name": "group2",
                "bdbs": [4, 5]
            }
        ]);

        Mock::given(method("GET"))
            .and(path("/v1/bdb_groups"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let groups = handler.list().await.unwrap();
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].uid, 1);
        assert_eq!(groups[0].name, "group1");
        assert_eq!(groups[1].uid, 2);
        assert_eq!(groups[1].name, "group2");
    }

    #[tokio::test]
    async fn test_get_bdb_group() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        let response_body = json!({
            "uid": 1,
            "name": "test_group",
            "bdbs": [1, 2, 3],
            "sync": "enabled"
        });

        Mock::given(method("GET"))
            .and(path("/v1/bdb_groups/1"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let group = handler.get(1).await.unwrap();
        assert_eq!(group.uid, 1);
        assert_eq!(group.name, "test_group");
    }

    #[tokio::test]
    async fn test_create_bdb_group() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        let request = CreateBdbGroupRequest {
            name: "new_group".to_string(),
        };

        let response_body = json!({
            "uid": 3,
            "name": "new_group",
            "bdbs": []
        });

        Mock::given(method("POST"))
            .and(path("/v1/bdb_groups"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let group = handler.create(request).await.unwrap();
        assert_eq!(group.uid, 3);
        assert_eq!(group.name, "new_group");
    }

    #[tokio::test]
    async fn test_update_bdb_group() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        let request = UpdateBdbGroupRequest {
            name: Some("updated_group".to_string()),
        };

        let response_body = json!({
            "uid": 1,
            "name": "updated_group",
            "bdbs": [1, 2, 3]
        });

        Mock::given(method("PUT"))
            .and(path("/v1/bdb_groups/1"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        let group = handler.update(1, request).await.unwrap();
        assert_eq!(group.uid, 1);
        assert_eq!(group.name, "updated_group");
    }

    #[tokio::test]
    async fn test_delete_bdb_group() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        Mock::given(method("DELETE"))
            .and(path("/v1/bdb_groups/1"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&mock_server)
            .await;

        let result = handler.delete(1).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_bdb_group_not_found() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        Mock::given(method("GET"))
            .and(path("/v1/bdb_groups/999"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({
                "error": "BDB group not found"
            })))
            .mount(&mock_server)
            .await;

        let result = handler.get(999).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_bdb_group_with_invalid_name() {
        let mock_server = MockServer::start().await;
        let handler = setup_mock_client(&mock_server).await;

        let request = CreateBdbGroupRequest {
            name: "".to_string(),
        };

        Mock::given(method("POST"))
            .and(path("/v1/bdb_groups"))
            .and(basic_auth("test_user", "test_pass"))
            .respond_with(ResponseTemplate::new(400).set_body_json(json!({
                "error": "Invalid group name"
            })))
            .mount(&mock_server)
            .await;

        let result = handler.create(request).await;
        assert!(result.is_err());
    }
}
