//! Suffixes endpoint tests for Redis Enterprise

use redis_enterprise::{CreateSuffixRequest, EnterpriseClient, SuffixesHandler};
use serde_json::json;
use wiremock::matchers::{basic_auth, body_json, method, path};
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

fn error_response(code: u16, message: &str) -> ResponseTemplate {
    ResponseTemplate::new(code).set_body_json(json!({
        "error": message,
        "code": code
    }))
}

fn test_suffix() -> serde_json::Value {
    json!({
        "name": "prod",
        "dns_suffix": "prod.redis.example.com",
        "use_internal_addr": true,
        "use_external_addr": false
    })
}

fn test_suffix_minimal() -> serde_json::Value {
    json!({
        "name": "test",
        "dns_suffix": "test.redis.example.com"
    })
}

fn test_suffix_external() -> serde_json::Value {
    json!({
        "name": "external",
        "dns_suffix": "external.redis.example.com",
        "use_internal_addr": false,
        "use_external_addr": true
    })
}

fn test_create_suffix_request() -> CreateSuffixRequest {
    CreateSuffixRequest {
        name: "new-suffix".to_string(),
        dns_suffix: "new.redis.example.com".to_string(),
        use_internal_addr: Some(true),
        use_external_addr: Some(false),
    }
}

fn test_create_suffix_request_minimal() -> CreateSuffixRequest {
    CreateSuffixRequest {
        name: "minimal".to_string(),
        dns_suffix: "minimal.redis.example.com".to_string(),
        use_internal_addr: None,
        use_external_addr: None,
    }
}

#[tokio::test]
async fn test_suffixes_list() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/suffixes"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            test_suffix(),
            test_suffix_minimal(),
            test_suffix_external()
        ])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = SuffixesHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let suffixes = result.unwrap();
    assert_eq!(suffixes.len(), 3);
    assert_eq!(suffixes[0].name, "prod");
    assert_eq!(
        suffixes[0].dns_suffix,
        Some("prod.redis.example.com".to_string())
    );
    assert_eq!(suffixes[0].use_internal_addr, Some(true));
    assert_eq!(suffixes[0].use_external_addr, Some(false));

    assert_eq!(suffixes[1].name, "test");
    assert_eq!(
        suffixes[1].dns_suffix,
        Some("test.redis.example.com".to_string())
    );
    assert!(suffixes[1].use_internal_addr.is_none());
    assert!(suffixes[1].use_external_addr.is_none());

    assert_eq!(suffixes[2].name, "external");
    assert_eq!(suffixes[2].use_external_addr, Some(true));
}

#[tokio::test]
async fn test_suffixes_list_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/suffixes"))
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

    let handler = SuffixesHandler::new(client);
    let result = handler.list().await;

    assert!(result.is_ok());
    let suffixes = result.unwrap();
    assert_eq!(suffixes.len(), 0);
}

#[tokio::test]
async fn test_suffixes_get() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/suffix/prod"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(test_suffix()))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = SuffixesHandler::new(client);
    let result = handler.get("prod").await;

    assert!(result.is_ok());
    let suffix = result.unwrap();
    assert_eq!(suffix.name, "prod");
    assert_eq!(
        suffix.dns_suffix,
        Some("prod.redis.example.com".to_string())
    );
    assert_eq!(suffix.use_internal_addr, Some(true));
    assert_eq!(suffix.use_external_addr, Some(false));
}

#[tokio::test]
async fn test_suffixes_get_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/suffix/nonexistent"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(404, "Suffix not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = SuffixesHandler::new(client);
    let result = handler.get("nonexistent").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_suffixes_create() {
    let mock_server = MockServer::start().await;
    let request = test_create_suffix_request();

    Mock::given(method("POST"))
        .and(path("/v1/suffix"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&request))
        .respond_with(created_response(json!({
            "name": "new-suffix",
            "dns_suffix": "new.redis.example.com",
            "use_internal_addr": true,
            "use_external_addr": false
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = SuffixesHandler::new(client);
    let result = handler.create(request).await;

    assert!(result.is_ok());
    let suffix = result.unwrap();
    assert_eq!(suffix.name, "new-suffix");
    assert_eq!(suffix.dns_suffix, Some("new.redis.example.com".to_string()));
    assert_eq!(suffix.use_internal_addr, Some(true));
    assert_eq!(suffix.use_external_addr, Some(false));
}

#[tokio::test]
async fn test_suffixes_create_minimal() {
    let mock_server = MockServer::start().await;
    let request = test_create_suffix_request_minimal();

    Mock::given(method("POST"))
        .and(path("/v1/suffix"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&request))
        .respond_with(created_response(json!({
            "name": "minimal",
            "dns_suffix": "minimal.redis.example.com"
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = SuffixesHandler::new(client);
    let result = handler.create(request).await;

    assert!(result.is_ok());
    let suffix = result.unwrap();
    assert_eq!(suffix.name, "minimal");
    assert_eq!(
        suffix.dns_suffix,
        Some("minimal.redis.example.com".to_string())
    );
    assert!(suffix.use_internal_addr.is_none());
    assert!(suffix.use_external_addr.is_none());
}

#[tokio::test]
async fn test_suffixes_create_duplicate() {
    let mock_server = MockServer::start().await;
    let request = CreateSuffixRequest {
        name: "existing".to_string(),
        dns_suffix: "existing.redis.example.com".to_string(),
        use_internal_addr: None,
        use_external_addr: None,
    };

    Mock::given(method("POST"))
        .and(path("/v1/suffix"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&request))
        .respond_with(error_response(409, "Suffix already exists"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = SuffixesHandler::new(client);
    let result = handler.create(request).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_suffixes_create_invalid() {
    let mock_server = MockServer::start().await;
    let request = CreateSuffixRequest {
        name: "".to_string(),
        dns_suffix: "invalid-dns".to_string(),
        use_internal_addr: None,
        use_external_addr: None,
    };

    Mock::given(method("POST"))
        .and(path("/v1/suffix"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&request))
        .respond_with(error_response(400, "Invalid suffix configuration"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = SuffixesHandler::new(client);
    let result = handler.create(request).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_suffixes_update() {
    let mock_server = MockServer::start().await;
    let request = test_create_suffix_request();

    Mock::given(method("PUT"))
        .and(path("/v1/suffix/new-suffix"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&request))
        .respond_with(success_response(json!({
            "name": "new-suffix",
            "dns_suffix": "new.redis.example.com",
            "use_internal_addr": true,
            "use_external_addr": false
        })))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = SuffixesHandler::new(client);
    let result = handler.update("new-suffix", request).await;

    assert!(result.is_ok());
    let suffix = result.unwrap();
    assert_eq!(suffix.name, "new-suffix");
    assert_eq!(suffix.dns_suffix, Some("new.redis.example.com".to_string()));
}

#[tokio::test]
async fn test_suffixes_update_nonexistent() {
    let mock_server = MockServer::start().await;
    let request = test_create_suffix_request();

    Mock::given(method("PUT"))
        .and(path("/v1/suffix/nonexistent"))
        .and(basic_auth("admin", "password"))
        .and(body_json(&request))
        .respond_with(error_response(404, "Suffix not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = SuffixesHandler::new(client);
    let result = handler.update("nonexistent", request).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_suffixes_delete() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/suffix/test"))
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

    let handler = SuffixesHandler::new(client);
    let result = handler.delete("test").await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_suffixes_delete_nonexistent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/suffix/nonexistent"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(404, "Suffix not found"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = SuffixesHandler::new(client);
    let result = handler.delete("nonexistent").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_suffixes_delete_in_use() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v1/suffix/prod"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(409, "Suffix is in use by databases"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = SuffixesHandler::new(client);
    let result = handler.delete("prod").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_suffixes_cluster_suffixes() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/cluster/suffixes"))
        .and(basic_auth("admin", "password"))
        .respond_with(success_response(json!([
            {
                "name": "cluster-default",
                "dns_suffix": "cluster.redis.example.com",
                "use_internal_addr": true,
                "use_external_addr": true
            },
            {
                "name": "cluster-internal",
                "dns_suffix": "internal.cluster.redis.example.com",
                "use_internal_addr": true,
                "use_external_addr": false
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = SuffixesHandler::new(client);
    let result = handler.cluster_suffixes().await;

    assert!(result.is_ok());
    let suffixes = result.unwrap();
    assert_eq!(suffixes.len(), 2);
    assert_eq!(suffixes[0].name, "cluster-default");
    assert_eq!(suffixes[0].use_internal_addr, Some(true));
    assert_eq!(suffixes[0].use_external_addr, Some(true));

    assert_eq!(suffixes[1].name, "cluster-internal");
    assert_eq!(suffixes[1].use_internal_addr, Some(true));
    assert_eq!(suffixes[1].use_external_addr, Some(false));
}

#[tokio::test]
async fn test_suffixes_cluster_suffixes_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/cluster/suffixes"))
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

    let handler = SuffixesHandler::new(client);
    let result = handler.cluster_suffixes().await;

    assert!(result.is_ok());
    let suffixes = result.unwrap();
    assert_eq!(suffixes.len(), 0);
}

#[tokio::test]
async fn test_suffixes_cluster_suffixes_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v1/cluster/suffixes"))
        .and(basic_auth("admin", "password"))
        .respond_with(error_response(500, "Internal server error"))
        .mount(&mock_server)
        .await;

    let client = EnterpriseClient::builder()
        .base_url(mock_server.uri())
        .username("admin")
        .password("password")
        .build()
        .unwrap();

    let handler = SuffixesHandler::new(client);
    let result = handler.cluster_suffixes().await;

    assert!(result.is_err());
}
