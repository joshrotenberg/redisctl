//! Billing and payment operations handler
//!
//! This module provides comprehensive billing and payment management for Redis Cloud,
//! including invoice management, payment method handling, cost analysis, and usage reporting.

use crate::{Result, client::CloudClient};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

/// Billing information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingInfo {
    pub account_id: Option<u32>,
    pub balance: Option<f64>,
    pub currency: Option<String>,
    pub billing_cycle: Option<String>,
    pub next_billing_date: Option<String>,
    pub payment_method_id: Option<u32>,
    pub status: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Invoice information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: String,
    pub invoice_number: Option<String>,
    pub date: String,
    pub due_date: Option<String>,
    pub amount: f64,
    pub currency: String,
    pub status: String,
    pub payment_status: Option<String>,
    pub period_start: Option<String>,
    pub period_end: Option<String>,
    pub items: Option<Vec<InvoiceItem>>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Invoice line item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceItem {
    pub description: String,
    pub quantity: Option<f64>,
    pub unit_price: Option<f64>,
    pub amount: f64,
    pub resource_type: Option<String>,
    pub resource_id: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Payment method information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethod {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "type")]
    pub method_type: String,
    pub is_default: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    #[serde(rename = "last4")]
    pub last_four: Option<String>,
    #[serde(rename = "expiryMonth")]
    pub expiry_month: Option<u8>,
    #[serde(rename = "expiryYear")]
    pub expiry_year: Option<u16>,
    #[serde(rename = "cardType")]
    pub card_brand: Option<String>,
    #[serde(rename = "billingAddress")]
    pub billing_address: Option<BillingAddress>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Billing address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingAddress {
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    #[serde(rename = "postalCode")]
    pub postal_code: Option<String>,
    pub country: Option<String>,

    #[serde(flatten)]
    pub extra: Value,
}

/// Request to add a payment method
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct AddPaymentMethodRequest {
    #[serde(rename = "type")]
    pub method_type: String,
    #[builder(default, setter(strip_option))]
    #[serde(rename = "cardNumber")]
    pub card_number: Option<String>,
    #[builder(default, setter(strip_option))]
    #[serde(rename = "expiryMonth")]
    pub expiry_month: Option<u8>,
    #[builder(default, setter(strip_option))]
    #[serde(rename = "expiryYear")]
    pub expiry_year: Option<u16>,
    #[builder(default, setter(strip_option))]
    #[serde(rename = "cvc")]
    pub cvv: Option<String>,
    #[builder(default, setter(strip_option))]
    #[serde(rename = "billingAddress")]
    pub billing_address: Option<BillingAddress>,
    #[builder(default, setter(strip_option))]
    #[serde(rename = "setAsDefault")]
    pub set_as_default: Option<bool>,
}

/// Request to update a payment method
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UpdatePaymentMethodRequest {
    #[builder(default, setter(strip_option))]
    #[serde(rename = "expiryMonth")]
    pub expiry_month: Option<u8>,
    #[builder(default, setter(strip_option))]
    #[serde(rename = "expiryYear")]
    pub expiry_year: Option<u16>,
    #[builder(default, setter(strip_option))]
    #[serde(rename = "billingAddress")]
    pub billing_address: Option<BillingAddress>,
}

/// Request to update billing alerts
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UpdateBillingAlertsRequest {
    #[builder(default, setter(strip_option))]
    pub enabled: Option<bool>,
    #[builder(default, setter(strip_option))]
    pub threshold_amount: Option<f64>,
    #[builder(default, setter(strip_option))]
    pub notification_emails: Option<Vec<String>>,
    #[builder(default, setter(strip_option))]
    pub alert_frequency: Option<String>,
}

/// Handler for Cloud billing and payment operations
///
/// Provides access to billing information, invoice management, payment methods,
/// cost analysis, and usage reporting. Essential for monitoring and managing
/// Redis Cloud costs and payment configuration.
pub struct CloudBillingHandler {
    client: CloudClient,
}

impl CloudBillingHandler {
    pub fn new(client: CloudClient) -> Self {
        CloudBillingHandler { client }
    }

    /// Get current billing information
    pub async fn get_info(&self) -> Result<BillingInfo> {
        self.client.get("/billing").await
    }

    /// Get billing history
    pub async fn get_history(
        &self,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<Value> {
        let mut path = "/billing/history".to_string();
        if let (Some(start), Some(end)) = (start_date, end_date) {
            path = format!("{}?start={}&end={}", path, start, end);
        }
        let v: Value = self.client.get(&path).await?;
        Ok(v.get("billingHistory").cloned().unwrap_or(v))
    }

    /// Get current invoice
    pub async fn get_current_invoice(&self) -> Result<Value> {
        let v: Value = self.client.get("/billing/invoice/current").await?;
        Ok(v.get("invoice").cloned().unwrap_or(v))
    }

    /// Get invoice by ID
    pub async fn get_invoice(&self, invoice_id: &str) -> Result<Value> {
        let v: Value = self
            .client
            .get(&format!("/billing/invoices/{}", invoice_id))
            .await?;
        Ok(v.get("invoice").cloned().unwrap_or(v))
    }

    /// List all invoices
    pub async fn list_invoices(&self) -> Result<Value> {
        let v: Value = self.client.get("/billing/invoices").await?;
        Ok(v.get("invoices").cloned().unwrap_or(v))
    }

    /// Download invoice PDF
    pub async fn download_invoice(&self, invoice_id: &str) -> Result<Value> {
        self.client
            .get(&format!("/billing/invoices/{}/download", invoice_id))
            .await
    }

    /// Get payment methods
    pub async fn list_payment_methods(&self) -> Result<Value> {
        let v: Value = self.client.get("/payment-methods").await?;
        Ok(v.get("paymentMethods").cloned().unwrap_or(v))
    }

    /// Get payment method by ID
    pub async fn get_payment_method(&self, method_id: u32) -> Result<Value> {
        let v: Value = self
            .client
            .get(&format!("/payment-methods/{}", method_id))
            .await?;
        Ok(v.get("paymentMethod").cloned().unwrap_or(v))
    }

    /// Add payment method
    pub async fn add_payment_method(&self, request: AddPaymentMethodRequest) -> Result<Value> {
        let v: Value = self.client.post("/payment-methods", &request).await?;
        Ok(v.get("paymentMethod").cloned().unwrap_or(v))
    }

    /// Update payment method
    pub async fn update_payment_method(
        &self,
        method_id: u32,
        request: UpdatePaymentMethodRequest,
    ) -> Result<Value> {
        let v: Value = self
            .client
            .put(&format!("/payment-methods/{}", method_id), &request)
            .await?;
        Ok(v.get("paymentMethod").cloned().unwrap_or(v))
    }

    /// Delete payment method
    pub async fn delete_payment_method(&self, method_id: u32) -> Result<Value> {
        self.client
            .delete(&format!("/payment-methods/{}", method_id))
            .await?;
        Ok(serde_json::json!({"message": format!("Payment method {} deleted", method_id)}))
    }

    /// Set default payment method
    pub async fn set_default_payment_method(&self, method_id: u32) -> Result<Value> {
        self.client
            .post(
                &format!("/payment-methods/{}/set-default", method_id),
                &Value::Null,
            )
            .await
    }

    /// Get billing alerts configuration
    pub async fn get_alerts(&self) -> Result<Value> {
        let v: Value = self.client.get("/billing/alerts").await?;
        Ok(v.get("alerts").cloned().unwrap_or(v))
    }

    /// Update billing alerts configuration
    pub async fn update_alerts(&self, request: UpdateBillingAlertsRequest) -> Result<Value> {
        let v: Value = self.client.put("/billing/alerts", &request).await?;
        Ok(v.get("alerts").cloned().unwrap_or(v))
    }

    /// Get cost breakdown
    pub async fn get_cost_breakdown(&self, period: &str) -> Result<Value> {
        let v: Value = self
            .client
            .get(&format!("/billing/costs?period={}", period))
            .await?;
        Ok(v.get("costs").cloned().unwrap_or(v))
    }

    /// Get usage report
    pub async fn get_usage(&self, start_date: &str, end_date: &str) -> Result<Value> {
        let v: Value = self
            .client
            .get(&format!(
                "/billing/usage?start={}&end={}",
                start_date, end_date
            ))
            .await?;
        Ok(v.get("usage").cloned().unwrap_or(v))
    }

    /// Get credits balance
    pub async fn get_credits(&self) -> Result<Value> {
        let v: Value = self.client.get("/billing/credits").await?;
        Ok(v.get("credits").cloned().unwrap_or(v))
    }

    /// Apply promo code
    pub async fn apply_promo_code(&self, code: &str) -> Result<Value> {
        let request = serde_json::json!({ "code": code });
        let v: Value = self.client.post("/billing/promo", &request).await?;
        Ok(v.get("promo").cloned().unwrap_or(v))
    }
}
