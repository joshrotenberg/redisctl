//! Billing endpoint tests for Redis Cloud

use redis_cloud::{CloudBillingHandler, CloudClient};
use serde_json::json;
use wiremock::matchers::{header, method, path, query_param};
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
    CloudClient::builder()
        .api_key("test-api-key")
        .api_secret("test-secret-key")
        .base_url(base_url)
        .build()
        .unwrap()
}

#[tokio::test]
async fn test_get_billing_info() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/billing"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "billing": {
                "accountId": 12345,
                "currentPeriod": {
                    "start": "2023-12-01T00:00:00Z",
                    "end": "2024-01-01T00:00:00Z",
                    "totalAmount": 1250.75,
                    "currency": "USD",
                    "status": "current"
                },
                "nextBillingDate": "2024-01-01T00:00:00Z",
                "paymentMethod": {
                    "id": "pm_123",
                    "type": "card",
                    "last4": "4242",
                    "isDefault": true
                },
                "billingAddress": {
                    "country": "US",
                    "city": "San Francisco",
                    "state": "CA",
                    "postalCode": "94105"
                }
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudBillingHandler::new(client);
    let result = handler.get_info().await;

    assert!(result.is_ok());
    let response = serde_json::to_value(result.unwrap()).unwrap();
    assert_eq!(response["billing"]["accountId"], 12345);
    assert_eq!(response["billing"]["currentPeriod"]["totalAmount"], 1250.75);
    assert_eq!(response["billing"]["currentPeriod"]["currency"], "USD");
}

#[tokio::test]
async fn test_get_billing_history() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/billing/history"))
        .and(query_param("start", "2023-01-01"))
        .and(query_param("end", "2023-12-31"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "billingHistory": [
                {
                    "period": "2023-12",
                    "startDate": "2023-12-01T00:00:00Z",
                    "endDate": "2023-12-31T23:59:59Z",
                    "totalAmount": 1250.75,
                    "currency": "USD",
                    "status": "paid",
                    "invoiceId": "inv_202312_001"
                },
                {
                    "period": "2023-11",
                    "startDate": "2023-11-01T00:00:00Z",
                    "endDate": "2023-11-30T23:59:59Z",
                    "totalAmount": 980.50,
                    "currency": "USD",
                    "status": "paid",
                    "invoiceId": "inv_202311_001"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudBillingHandler::new(client);
    let result = handler
        .get_history(Some("2023-01-01"), Some("2023-12-31"))
        .await;

    assert!(result.is_ok());
    let response = json!({"billingHistory": result.unwrap()});
    let history = response["billingHistory"].as_array().unwrap();
    assert_eq!(history.len(), 2);
    assert_eq!(history[0]["period"], "2023-12");
    assert_eq!(history[0]["status"], "paid");
}

#[tokio::test]
async fn test_get_billing_history_no_params() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/billing/history"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "billingHistory": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudBillingHandler::new(client);
    let result = handler.get_history(None, None).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_current_invoice() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/billing/invoice/current"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "invoice": {
                "id": "inv_current_123",
                "number": "INV-2023-12-001",
                "status": "open",
                "amount": 1250.75,
                "currency": "USD",
                "dueDate": "2024-01-01T00:00:00Z",
                "lineItems": [
                    {
                        "description": "Redis Cloud Pro - Subscription sub_123",
                        "amount": 800.00,
                        "quantity": 1
                    },
                    {
                        "description": "Additional Storage - 100GB",
                        "amount": 450.75,
                        "quantity": 1
                    }
                ]
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudBillingHandler::new(client);
    let result = handler.get_current_invoice().await;

    assert!(result.is_ok());
    let response = json!({"invoice": result.unwrap()});
    assert_eq!(response["invoice"]["status"], "open");
    assert_eq!(response["invoice"]["amount"], 1250.75);
    let line_items = response["invoice"]["lineItems"].as_array().unwrap();
    assert_eq!(line_items.len(), 2);
}

#[tokio::test]
async fn test_get_invoice() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/billing/invoices/inv_123"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "invoice": {
                "id": "inv_123",
                "number": "INV-2023-11-001",
                "status": "paid",
                "amount": 980.50,
                "currency": "USD",
                "paidDate": "2023-11-30T15:30:00Z",
                "dueDate": "2023-12-01T00:00:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudBillingHandler::new(client);
    let result = handler.get_invoice("inv_123").await;

    assert!(result.is_ok());
    let response = json!({"invoice": result.unwrap()});
    assert_eq!(response["invoice"]["id"], "inv_123");
    assert_eq!(response["invoice"]["status"], "paid");
}

#[tokio::test]
async fn test_list_invoices() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/billing/invoices"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "invoices": [
                {
                    "id": "inv_current",
                    "number": "INV-2023-12-001",
                    "status": "open",
                    "amount": 1250.75,
                    "dueDate": "2024-01-01T00:00:00Z"
                },
                {
                    "id": "inv_123",
                    "number": "INV-2023-11-001",
                    "status": "paid",
                    "amount": 980.50,
                    "paidDate": "2023-11-30T15:30:00Z"
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudBillingHandler::new(client);
    let result = handler.list_invoices().await;

    assert!(result.is_ok());
    let response = json!({"invoices": result.unwrap()});
    let invoices = response["invoices"].as_array().unwrap();
    assert_eq!(invoices.len(), 2);
    assert_eq!(invoices[0]["status"], "open");
    assert_eq!(invoices[1]["status"], "paid");
}

#[tokio::test]
async fn test_download_invoice() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/billing/invoices/inv_123/download"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "downloadUrl": "https://billing.redis.com/invoices/inv_123.pdf",
            "expiresAt": "2023-12-01T18:00:00Z"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudBillingHandler::new(client);
    let result = handler.download_invoice("inv_123").await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response["downloadUrl"].as_str().unwrap().ends_with(".pdf"));
    assert!(response["expiresAt"].is_string());
}

#[tokio::test]
async fn test_list_payment_methods() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/payment-methods"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "paymentMethods": [
                {
                    "id": "pm_123",
                    "type": "card",
                    "cardType": "visa",
                    "last4": "4242",
                    "expiryMonth": 12,
                    "expiryYear": 2025,
                    "isDefault": true,
                    "billingAddress": {
                        "country": "US",
                        "postalCode": "94105"
                    }
                },
                {
                    "id": "pm_456",
                    "type": "card",
                    "cardType": "mastercard",
                    "last4": "5555",
                    "expiryMonth": 6,
                    "expiryYear": 2026,
                    "isDefault": false
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudBillingHandler::new(client);
    let result = handler.list_payment_methods().await;

    assert!(result.is_ok());
    let response = json!({"paymentMethods": result.unwrap()});
    let methods = response["paymentMethods"].as_array().unwrap();
    assert_eq!(methods.len(), 2);
    assert_eq!(methods[0]["isDefault"], true);
    assert_eq!(methods[1]["isDefault"], false);
}

#[tokio::test]
async fn test_get_payment_method() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/payment-methods/123"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "paymentMethod": {
                "id": "pm_123",
                "type": "card",
                "cardType": "visa",
                "last4": "4242",
                "expiryMonth": 12,
                "expiryYear": 2025,
                "isDefault": true,
                "createdAt": "2023-01-01T00:00:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudBillingHandler::new(client);
    let result = handler.get_payment_method(123).await;

    assert!(result.is_ok());
    let response = json!({"paymentMethod": result.unwrap()});
    assert_eq!(response["paymentMethod"]["last4"], "4242");
    assert_eq!(response["paymentMethod"]["isDefault"], true);
}

#[tokio::test]
async fn test_add_payment_method() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/payment-methods"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(created_response(json!({
            "paymentMethod": {
                "id": "pm_789",
                "type": "card",
                "cardType": "amex",
                "last4": "1234",
                "expiryMonth": 3,
                "expiryYear": 2027,
                "isDefault": false,
                "createdAt": "2023-12-01T12:00:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudBillingHandler::new(client);
    let request = json!({
        "type": "card",
        "cardNumber": "378282246310005",
        "expiryMonth": 3,
        "expiryYear": 2027,
        "cvc": "123",
        "billingAddress": {
            "country": "US",
            "postalCode": "94105"
        }
    });
    let req: redis_cloud::models::billing::AddPaymentMethodRequest =
        serde_json::from_value(request).unwrap();
    let result = handler.add_payment_method(req).await;

    assert!(result.is_ok());
    let response = json!({"paymentMethod": result.unwrap()});
    assert_eq!(response["paymentMethod"]["cardType"], "amex");
    assert_eq!(response["paymentMethod"]["last4"], "1234");
}

#[tokio::test]
async fn test_update_payment_method() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/payment-methods/123"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "paymentMethod": {
                "id": "pm_123",
                "type": "card",
                "expiryMonth": 12,
                "expiryYear": 2026,
                "billingAddress": {
                    "country": "US",
                    "postalCode": "10001"
                },
                "updatedAt": "2023-12-01T12:00:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudBillingHandler::new(client);
    let request = json!({
        "expiryYear": 2026,
        "billingAddress": {
            "country": "US",
            "postalCode": "10001"
        }
    });
    let req: redis_cloud::models::billing::UpdatePaymentMethodRequest =
        serde_json::from_value(request).unwrap();
    let result = handler.update_payment_method(123, req).await;

    assert!(result.is_ok());
    let response = json!({"paymentMethod": result.unwrap()});
    assert_eq!(response["paymentMethod"]["expiryYear"], 2026);
    assert_eq!(
        response["paymentMethod"]["billingAddress"]["postalCode"],
        "10001"
    );
}

#[tokio::test]
async fn test_delete_payment_method() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/payment-methods/123"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudBillingHandler::new(client);
    let result = handler.delete_payment_method(123).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["message"], "Payment method 123 deleted");
}

#[tokio::test]
async fn test_set_default_payment_method() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/payment-methods/123/set-default"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "paymentMethod": {
                "id": "pm_123",
                "isDefault": true,
                "updatedAt": "2023-12-01T12:00:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudBillingHandler::new(client);
    let result = handler.set_default_payment_method(123).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["paymentMethod"]["isDefault"], true);
}

#[tokio::test]
async fn test_get_alerts() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/billing/alerts"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "alerts": {
                "emailNotifications": true,
                "thresholds": [
                    {
                        "amount": 500.0,
                        "currency": "USD",
                        "enabled": true
                    },
                    {
                        "amount": 1000.0,
                        "currency": "USD",
                        "enabled": true
                    }
                ],
                "recipients": ["admin@example.com", "billing@example.com"]
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudBillingHandler::new(client);
    let result = handler.get_alerts().await;

    assert!(result.is_ok());
    let response = json!({"alerts": result.unwrap()});
    assert_eq!(response["alerts"]["emailNotifications"], true);
    let thresholds = response["alerts"]["thresholds"].as_array().unwrap();
    assert_eq!(thresholds.len(), 2);
}

#[tokio::test]
async fn test_update_alerts() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/billing/alerts"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "alerts": {
                "emailNotifications": false,
                "thresholds": [
                    {
                        "amount": 750.0,
                        "currency": "USD",
                        "enabled": true
                    }
                ],
                "recipients": ["admin@example.com"]
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudBillingHandler::new(client);
    let request = json!({
        "emailNotifications": false,
        "thresholds": [
            {
                "amount": 750.0,
                "currency": "USD",
                "enabled": true
            }
        ],
        "recipients": ["admin@example.com"]
    });
    let req: redis_cloud::models::billing::UpdateBillingAlertsRequest =
        serde_json::from_value(request).unwrap();
    let result = handler.update_alerts(req).await;

    assert!(result.is_ok());
    let response = json!({"alerts": result.unwrap()});
    assert_eq!(response["alerts"]["emailNotifications"], false);
    let recipients = response["alerts"]["recipients"].as_array().unwrap();
    assert_eq!(recipients.len(), 1);
}

#[tokio::test]
async fn test_get_cost_breakdown() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/billing/costs"))
        .and(query_param("period", "30d"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "costs": {
                "period": "30d",
                "total": 1250.75,
                "currency": "USD",
                "breakdown": [
                    {
                        "service": "Redis Cloud Pro",
                        "subscriptionId": "sub_123",
                        "amount": 800.0,
                        "percentage": 64.0
                    },
                    {
                        "service": "Additional Storage",
                        "subscriptionId": "sub_123",
                        "amount": 300.0,
                        "percentage": 24.0
                    },
                    {
                        "service": "Data Transfer",
                        "subscriptionId": "sub_456",
                        "amount": 150.75,
                        "percentage": 12.0
                    }
                ]
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudBillingHandler::new(client);
    let result = handler.get_cost_breakdown("30d").await;

    assert!(result.is_ok());
    let response = json!({"costs": result.unwrap()});
    assert_eq!(response["costs"]["total"], 1250.75);
    let breakdown = response["costs"]["breakdown"].as_array().unwrap();
    assert_eq!(breakdown.len(), 3);
    assert_eq!(breakdown[0]["service"], "Redis Cloud Pro");
}

#[tokio::test]
async fn test_get_usage() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/billing/usage"))
        .and(query_param("start", "2023-11-01"))
        .and(query_param("end", "2023-11-30"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "usage": {
                "period": {
                    "start": "2023-11-01T00:00:00Z",
                    "end": "2023-11-30T23:59:59Z"
                },
                "subscriptions": [
                    {
                        "subscriptionId": "sub_123",
                        "subscriptionName": "Production Environment",
                        "totalCost": 980.50,
                        "databases": [
                            {
                                "databaseId": "db_789",
                                "databaseName": "primary-redis",
                                "memoryUsage": 1024,
                                "storageUsage": 512,
                                "networkUsage": 1048576,
                                "cost": 650.0
                            }
                        ]
                    }
                ]
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudBillingHandler::new(client);
    let result = handler.get_usage("2023-11-01", "2023-11-30").await;

    assert!(result.is_ok());
    let response = json!({"usage": result.unwrap()});
    let subscriptions = response["usage"]["subscriptions"].as_array().unwrap();
    assert_eq!(subscriptions.len(), 1);
    assert_eq!(subscriptions[0]["totalCost"], 980.50);
}

#[tokio::test]
async fn test_get_credits() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/billing/credits"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "credits": {
                "balance": 250.0,
                "currency": "USD",
                "transactions": [
                    {
                        "id": "credit_001",
                        "type": "promotional",
                        "amount": 500.0,
                        "description": "Welcome credit",
                        "appliedDate": "2023-01-01T00:00:00Z",
                        "expiryDate": "2024-01-01T00:00:00Z"
                    },
                    {
                        "id": "credit_002",
                        "type": "usage",
                        "amount": -250.0,
                        "description": "Applied to November invoice",
                        "appliedDate": "2023-11-30T00:00:00Z"
                    }
                ]
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudBillingHandler::new(client);
    let result = handler.get_credits().await;

    assert!(result.is_ok());
    let response = json!({"credits": result.unwrap()});
    assert_eq!(response["credits"]["balance"], 250.0);
    let transactions = response["credits"]["transactions"].as_array().unwrap();
    assert_eq!(transactions.len(), 2);
    assert_eq!(transactions[0]["type"], "promotional");
}

#[tokio::test]
async fn test_apply_promo_code() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/billing/promo"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(success_response(json!({
            "promo": {
                "code": "WELCOME2024",
                "creditAmount": 100.0,
                "currency": "USD",
                "appliedAt": "2023-12-01T12:00:00Z",
                "expiryDate": "2024-12-01T00:00:00Z"
            }
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudBillingHandler::new(client);
    let result = handler.apply_promo_code("WELCOME2024").await;

    assert!(result.is_ok());
    let response = json!({"promo": result.unwrap()});
    assert_eq!(response["promo"]["code"], "WELCOME2024");
    assert!(response["promo"]["creditAmount"].is_number());
}

#[tokio::test]
async fn test_apply_promo_code_invalid() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/billing/promo"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("x-api-secret-key", "test-secret-key"))
        .respond_with(error_response(
            400,
            json!({
                "error": {
                    "type": "INVALID_PROMO_CODE",
                    "status": 400,
                    "description": "Promo code is invalid or expired"
                }
            }),
        ))
        .mount(&mock_server)
        .await;

    let client = create_test_client(mock_server.uri());
    let handler = CloudBillingHandler::new(client);
    let result = handler.apply_promo_code("INVALID_CODE").await;

    assert!(result.is_err());
}
