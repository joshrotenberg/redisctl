//! Billing and payment operations handler

use crate::{client::CloudClient, Result};
use serde_json::Value;

/// Handler for Cloud billing and payment operations
pub struct CloudBillingHandler {
    client: CloudClient,
}

impl CloudBillingHandler {
    pub fn new(client: CloudClient) -> Self {
        CloudBillingHandler { client }
    }

    /// Get current billing information
    pub async fn get_info(&self) -> Result<Value> {
        self.client.get("/billing").await
    }

    /// Get billing history
    pub async fn get_history(&self, start_date: Option<&str>, end_date: Option<&str>) -> Result<Value> {
        let mut path = "/billing/history".to_string();
        if let (Some(start), Some(end)) = (start_date, end_date) {
            path = format!("{}?start={}&end={}", path, start, end);
        }
        self.client.get(&path).await
    }

    /// Get current invoice
    pub async fn get_current_invoice(&self) -> Result<Value> {
        self.client.get("/billing/invoice/current").await
    }

    /// Get invoice by ID
    pub async fn get_invoice(&self, invoice_id: &str) -> Result<Value> {
        self.client
            .get(&format!("/billing/invoices/{}", invoice_id))
            .await
    }

    /// List all invoices
    pub async fn list_invoices(&self) -> Result<Value> {
        self.client.get("/billing/invoices").await
    }

    /// Download invoice PDF
    pub async fn download_invoice(&self, invoice_id: &str) -> Result<Value> {
        self.client
            .get(&format!("/billing/invoices/{}/download", invoice_id))
            .await
    }

    /// Get payment methods
    pub async fn list_payment_methods(&self) -> Result<Value> {
        self.client.get("/payment-methods").await
    }

    /// Get payment method by ID
    pub async fn get_payment_method(&self, method_id: u32) -> Result<Value> {
        self.client
            .get(&format!("/payment-methods/{}", method_id))
            .await
    }

    /// Add payment method
    pub async fn add_payment_method(&self, request: Value) -> Result<Value> {
        self.client.post("/payment-methods", &request).await
    }

    /// Update payment method
    pub async fn update_payment_method(&self, method_id: u32, request: Value) -> Result<Value> {
        self.client
            .put(&format!("/payment-methods/{}", method_id), &request)
            .await
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
        self.client.get("/billing/alerts").await
    }

    /// Update billing alerts configuration
    pub async fn update_alerts(&self, request: Value) -> Result<Value> {
        self.client.put("/billing/alerts", &request).await
    }

    /// Get cost breakdown
    pub async fn get_cost_breakdown(&self, period: &str) -> Result<Value> {
        self.client
            .get(&format!("/billing/costs?period={}", period))
            .await
    }

    /// Get usage report
    pub async fn get_usage(&self, start_date: &str, end_date: &str) -> Result<Value> {
        self.client
            .get(&format!(
                "/billing/usage?start={}&end={}",
                start_date, end_date
            ))
            .await
    }

    /// Get credits balance
    pub async fn get_credits(&self) -> Result<Value> {
        self.client.get("/billing/credits").await
    }

    /// Apply promo code
    pub async fn apply_promo_code(&self, code: &str) -> Result<Value> {
        let request = serde_json::json!({ "code": code });
        self.client.post("/billing/promo", &request).await
    }
}