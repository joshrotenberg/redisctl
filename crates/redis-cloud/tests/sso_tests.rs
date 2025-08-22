//! SSO/SAML configuration endpoint tests for Redis Cloud

use redis_cloud::{CloudClient, CloudConfig, CloudSsoHandler};
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

// Test helper functions
fn success_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(body)
}

fn created_response(body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(201).set_body_json(body)
}

fn error_response(status: u16, body: serde_json::Value) -> ResponseTemplate {
    ResponseTemplate::new(status).set_body_json(body)
}

fn create_test_client(base_url: String) -> CloudClient {
    let config = CloudConfig {
        api_key: "test-api-key".to_string(),
        api_secret: "test-secret-key".to_string(),
        base_url,
        timeout: std::time::Duration::from_secs(30),
    };
    CloudClient::new(config).unwrap()
}

#[tokio::test]
async fn test_get_sso_config() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/sso"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "sso": {
                "enabled": true,
                "provider": "saml",
                "entityId": "https://redis.example.com/sso",
                "ssoUrl": "https://idp.example.com/sso/redirect",
                "signOnUrl": "https://redis.example.com/sso/login",
                "logoutUrl": "https://redis.example.com/sso/logout",
                "certificateFingerprint": "AA:BB:CC:DD:EE:FF:00:11:22:33:44:55:66:77:88:99:AA:BB:CC:DD",
                "autoProvisioning": true,
                "defaultRole": "member",
                "createdAt": "2023-01-01T00:00:00Z",
                "updatedAt": "2023-12-01T10:00:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudSsoHandler::new(client);
    let result = handler.get().await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let sso = &response["sso"];
    assert_eq!(sso["enabled"], true);
    assert_eq!(sso["provider"], "saml");
    assert_eq!(sso["autoProvisioning"], true);
    assert_eq!(sso["defaultRole"], "member");
}

#[tokio::test]
async fn test_get_sso_config_disabled() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/sso"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "sso": {
                "enabled": false
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudSsoHandler::new(client);
    let result = handler.get().await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let sso = &response["sso"];
    assert_eq!(sso["enabled"], false);
}

#[tokio::test]
async fn test_update_sso_config() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/sso"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "sso": {
                "enabled": true,
                "provider": "saml",
                "entityId": "https://redis.example.com/sso",
                "ssoUrl": "https://new-idp.example.com/sso/redirect",
                "autoProvisioning": false,
                "defaultRole": "viewer",
                "updatedAt": "2023-12-01T12:00:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudSsoHandler::new(client);
    let request = json!({
        "enabled": true,
        "ssoUrl": "https://new-idp.example.com/sso/redirect",
        "autoProvisioning": false,
        "defaultRole": "viewer"
    });
    let result = handler.update(request).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let sso = &response["sso"];
    assert_eq!(sso["enabled"], true);
    assert_eq!(sso["ssoUrl"], "https://new-idp.example.com/sso/redirect");
    assert_eq!(sso["autoProvisioning"], false);
    assert_eq!(sso["defaultRole"], "viewer");
}

#[tokio::test]
async fn test_delete_sso_config() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/sso"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudSsoHandler::new(client);
    let result = handler.delete().await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["message"], "SSO configuration deleted");
}

#[tokio::test]
async fn test_test_sso_config() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/sso/test"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "test": {
                "success": true,
                "message": "SSO configuration test successful",
                "details": {
                    "entityIdResolved": true,
                    "ssoUrlAccessible": true,
                    "certificateValid": true,
                    "attributeMappingCorrect": true
                },
                "testedAt": "2023-12-01T12:00:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudSsoHandler::new(client);
    let request = json!({
        "testUser": "test@example.com",
        "validateCertificate": true
    });
    let result = handler.test(request).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let test = &response["test"];
    assert_eq!(test["success"], true);
    assert_eq!(test["message"], "SSO configuration test successful");
    let details = &test["details"];
    assert_eq!(details["certificateValid"], true);
}

#[tokio::test]
async fn test_test_sso_config_failure() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/sso/test"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "test": {
                "success": false,
                "message": "SSO configuration test failed",
                "details": {
                    "entityIdResolved": true,
                    "ssoUrlAccessible": false,
                    "certificateValid": true,
                    "error": "Unable to reach SSO URL"
                },
                "testedAt": "2023-12-01T12:00:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudSsoHandler::new(client);
    let request = json!({
        "testUser": "test@example.com"
    });
    let result = handler.test(request).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let test = &response["test"];
    assert_eq!(test["success"], false);
    let details = &test["details"];
    assert_eq!(details["ssoUrlAccessible"], false);
    assert_eq!(details["error"], "Unable to reach SSO URL");
}

#[tokio::test]
async fn test_get_saml_config() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/sso/saml"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "saml": {
                "entityId": "https://redis.example.com/sso",
                "ssoUrl": "https://idp.example.com/sso/redirect",
                "sloUrl": "https://idp.example.com/sso/logout",
                "certificate": "-----BEGIN CERTIFICATE-----\nMIIBkT...",
                "certificateFingerprint": "AA:BB:CC:DD:EE:FF:00:11:22:33:44:55:66:77:88:99:AA:BB:CC:DD",
                "nameIdFormat": "urn:oasis:names:tc:SAML:2.0:nameid-format:emailAddress",
                "attributeMapping": {
                    "email": "http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress",
                    "firstName": "http://schemas.xmlsoap.org/ws/2005/05/identity/claims/givenname",
                    "lastName": "http://schemas.xmlsoap.org/ws/2005/05/identity/claims/surname"
                },
                "signRequest": true,
                "encryptAssertion": false
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudSsoHandler::new(client);
    let result = handler.get_saml().await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let saml = &response["saml"];
    assert_eq!(saml["entityId"], "https://redis.example.com/sso");
    assert_eq!(saml["signRequest"], true);
    assert_eq!(saml["encryptAssertion"], false);
    let attr_mapping = &saml["attributeMapping"];
    assert!(attr_mapping["email"].is_string());
}

#[tokio::test]
async fn test_update_saml_config() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/sso/saml"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "saml": {
                "entityId": "https://redis.example.com/sso",
                "ssoUrl": "https://updated-idp.example.com/sso/redirect",
                "signRequest": false,
                "encryptAssertion": true,
                "updatedAt": "2023-12-01T12:00:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudSsoHandler::new(client);
    let request = json!({
        "ssoUrl": "https://updated-idp.example.com/sso/redirect",
        "signRequest": false,
        "encryptAssertion": true
    });
    let result = handler.update_saml(request).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let saml = &response["saml"];
    assert_eq!(
        saml["ssoUrl"],
        "https://updated-idp.example.com/sso/redirect"
    );
    assert_eq!(saml["signRequest"], false);
    assert_eq!(saml["encryptAssertion"], true);
}

#[tokio::test]
async fn test_get_saml_metadata() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/sso/saml/metadata"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "metadata": {
                "xml": "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<EntityDescriptor...",
                "downloadUrl": "https://redis.example.com/sso/saml/metadata.xml",
                "entityId": "https://redis.example.com/sso",
                "acsUrl": "https://redis.example.com/sso/saml/acs",
                "sloUrl": "https://redis.example.com/sso/saml/slo"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudSsoHandler::new(client);
    let result = handler.get_saml_metadata().await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let metadata = &response["metadata"];
    assert!(metadata["xml"].as_str().unwrap().starts_with("<?xml"));
    assert_eq!(metadata["entityId"], "https://redis.example.com/sso");
}

#[tokio::test]
async fn test_upload_saml_cert() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/sso/saml/certificate"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(created_response(json!({
            "certificate": {
                "fingerprint": "FF:EE:DD:CC:BB:AA:99:88:77:66:55:44:33:22:11:00:FF:EE:DD:CC",
                "subject": "CN=idp.example.com",
                "issuer": "CN=CA Certificate Authority",
                "validFrom": "2023-01-01T00:00:00Z",
                "validTo": "2025-01-01T00:00:00Z",
                "uploadedAt": "2023-12-01T12:00:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudSsoHandler::new(client);
    let request = json!({
        "certificate": "-----BEGIN CERTIFICATE-----\nMIIBkTCB...\n-----END CERTIFICATE-----"
    });
    let result = handler.upload_saml_cert(request).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let cert = &response["certificate"];
    assert_eq!(cert["subject"], "CN=idp.example.com");
    assert_eq!(cert["issuer"], "CN=CA Certificate Authority");
    assert!(cert["uploadedAt"].is_string());
}

#[tokio::test]
async fn test_list_sso_users() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/sso/users"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "users": [
                {
                    "id": 1,
                    "email": "john.doe@example.com",
                    "firstName": "John",
                    "lastName": "Doe",
                    "role": "admin",
                    "status": "active",
                    "lastLogin": "2023-12-01T10:30:00Z",
                    "createdAt": "2023-01-01T00:00:00Z",
                    "ssoMapping": {
                        "nameId": "john.doe@example.com",
                        "attributes": {
                            "department": "Engineering",
                            "groups": ["developers", "admins"]
                        }
                    }
                },
                {
                    "id": 2,
                    "email": "jane.smith@example.com",
                    "firstName": "Jane",
                    "lastName": "Smith",
                    "role": "member",
                    "status": "active",
                    "lastLogin": "2023-11-30T15:45:00Z",
                    "createdAt": "2023-02-01T00:00:00Z",
                    "ssoMapping": {
                        "nameId": "jane.smith@example.com",
                        "attributes": {
                            "department": "Marketing"
                        }
                    }
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudSsoHandler::new(client);
    let result = handler.list_users().await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let users = response["users"].as_array().unwrap();
    assert_eq!(users.len(), 2);
    assert_eq!(users[0]["email"], "john.doe@example.com");
    assert_eq!(users[0]["role"], "admin");
    assert_eq!(users[1]["email"], "jane.smith@example.com");
    assert_eq!(users[1]["role"], "member");
}

#[tokio::test]
async fn test_get_sso_user() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/sso/users/1"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "user": {
                "id": 1,
                "email": "john.doe@example.com",
                "firstName": "John",
                "lastName": "Doe",
                "role": "admin",
                "status": "active",
                "lastLogin": "2023-12-01T10:30:00Z",
                "createdAt": "2023-01-01T00:00:00Z",
                "ssoMapping": {
                    "nameId": "john.doe@example.com",
                    "attributes": {
                        "department": "Engineering",
                        "title": "Senior Engineer",
                        "groups": ["developers", "admins"]
                    }
                },
                "permissions": ["account:read", "subscription:*", "database:*"]
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudSsoHandler::new(client);
    let result = handler.get_user(1).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let user = &response["user"];
    assert_eq!(user["id"], 1);
    assert_eq!(user["email"], "john.doe@example.com");
    assert_eq!(user["role"], "admin");
    let mapping = &user["ssoMapping"];
    let groups = mapping["attributes"]["groups"].as_array().unwrap();
    assert_eq!(groups.len(), 2);
}

#[tokio::test]
async fn test_create_user_mapping() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/sso/users"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(created_response(json!({
            "user": {
                "id": 3,
                "email": "bob.wilson@example.com",
                "firstName": "Bob",
                "lastName": "Wilson",
                "role": "viewer",
                "status": "active",
                "createdAt": "2023-12-01T12:00:00Z",
                "ssoMapping": {
                    "nameId": "bob.wilson@example.com",
                    "attributes": {
                        "department": "Sales"
                    }
                }
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudSsoHandler::new(client);
    let request = json!({
        "email": "bob.wilson@example.com",
        "firstName": "Bob",
        "lastName": "Wilson",
        "role": "viewer",
        "ssoMapping": {
            "nameId": "bob.wilson@example.com",
            "attributes": {
                "department": "Sales"
            }
        }
    });
    let result = handler.create_user_mapping(request).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let user = &response["user"];
    assert_eq!(user["email"], "bob.wilson@example.com");
    assert_eq!(user["role"], "viewer");
    assert_eq!(user["ssoMapping"]["attributes"]["department"], "Sales");
}

#[tokio::test]
async fn test_update_user_mapping() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/sso/users/1"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "user": {
                "id": 1,
                "email": "john.doe@example.com",
                "role": "owner",
                "status": "active",
                "updatedAt": "2023-12-01T12:00:00Z",
                "ssoMapping": {
                    "nameId": "john.doe@example.com",
                    "attributes": {
                        "department": "Engineering",
                        "title": "Engineering Manager"
                    }
                }
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudSsoHandler::new(client);
    let request = json!({
        "role": "owner",
        "ssoMapping": {
            "attributes": {
                "title": "Engineering Manager"
            }
        }
    });
    let result = handler.update_user_mapping(1, request).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let user = &response["user"];
    assert_eq!(user["role"], "owner");
    assert_eq!(
        user["ssoMapping"]["attributes"]["title"],
        "Engineering Manager"
    );
}

#[tokio::test]
async fn test_delete_user_mapping() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/sso/users/1"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudSsoHandler::new(client);
    let result = handler.delete_user_mapping(1).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["message"], "SSO user mapping 1 deleted");
}

#[tokio::test]
async fn test_list_sso_groups() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/sso/groups"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "groups": [
                {
                    "id": 1,
                    "name": "developers",
                    "displayName": "Software Developers",
                    "role": "member",
                    "memberCount": 15,
                    "createdAt": "2023-01-01T00:00:00Z",
                    "permissions": ["database:read", "database:write"]
                },
                {
                    "id": 2,
                    "name": "admins",
                    "displayName": "System Administrators",
                    "role": "admin",
                    "memberCount": 3,
                    "createdAt": "2023-01-01T00:00:00Z",
                    "permissions": ["*"]
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudSsoHandler::new(client);
    let result = handler.list_groups().await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let groups = response["groups"].as_array().unwrap();
    assert_eq!(groups.len(), 2);
    assert_eq!(groups[0]["name"], "developers");
    assert_eq!(groups[0]["role"], "member");
    assert_eq!(groups[1]["name"], "admins");
    assert_eq!(groups[1]["role"], "admin");
}

#[tokio::test]
async fn test_map_group() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/sso/groups"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(created_response(json!({
            "group": {
                "id": 3,
                "name": "qa-team",
                "displayName": "QA Team",
                "role": "viewer",
                "memberCount": 0,
                "createdAt": "2023-12-01T12:00:00Z",
                "permissions": ["database:read"]
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudSsoHandler::new(client);
    let request = json!({
        "name": "qa-team",
        "displayName": "QA Team",
        "role": "viewer",
        "permissions": ["database:read"]
    });
    let result = handler.map_group(request).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let group = &response["group"];
    assert_eq!(group["name"], "qa-team");
    assert_eq!(group["role"], "viewer");
    let permissions = group["permissions"].as_array().unwrap();
    assert_eq!(permissions[0], "database:read");
}

#[tokio::test]
async fn test_update_group_mapping() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/sso/groups/1"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "group": {
                "id": 1,
                "name": "developers",
                "displayName": "Senior Software Developers",
                "role": "admin",
                "memberCount": 15,
                "updatedAt": "2023-12-01T12:00:00Z",
                "permissions": ["database:*", "subscription:read"]
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudSsoHandler::new(client);
    let request = json!({
        "displayName": "Senior Software Developers",
        "role": "admin",
        "permissions": ["database:*", "subscription:read"]
    });
    let result = handler.update_group_mapping(1, request).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let group = &response["group"];
    assert_eq!(group["displayName"], "Senior Software Developers");
    assert_eq!(group["role"], "admin");
}

#[tokio::test]
async fn test_delete_group_mapping() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/sso/groups/1"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudSsoHandler::new(client);
    let result = handler.delete_group_mapping(1).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["message"], "SSO group mapping 1 deleted");
}

#[tokio::test]
async fn test_sso_not_configured_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/sso"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            404,
            json!({
                "error": {
                    "type": "SSO_NOT_CONFIGURED",
                    "status": 404,
                    "description": "SSO is not configured for this account"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudSsoHandler::new(client);
    let result = handler.get().await;

    assert!(result.is_err());
}
