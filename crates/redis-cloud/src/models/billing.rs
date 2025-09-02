//! Billing and payment models for Redis Cloud

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
    pub id: u32,
    pub method_type: String,
    pub is_default: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub last_four: Option<String>,
    pub expiry_month: Option<u8>,
    pub expiry_year: Option<u16>,
    pub card_brand: Option<String>,
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
    pub postal_code: Option<String>,
    pub country: Option<String>,
    
    #[serde(flatten)]
    pub extra: Value,
}

/// Request to add a payment method
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct AddPaymentMethodRequest {
    pub method_type: String,
    #[builder(default, setter(strip_option))]
    pub card_number: Option<String>,
    #[builder(default, setter(strip_option))]
    pub expiry_month: Option<u8>,
    #[builder(default, setter(strip_option))]
    pub expiry_year: Option<u16>,
    #[builder(default, setter(strip_option))]
    pub cvv: Option<String>,
    #[builder(default, setter(strip_option))]
    pub billing_address: Option<BillingAddress>,
    #[builder(default, setter(strip_option))]
    pub set_as_default: Option<bool>,
}

/// Request to update a payment method
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct UpdatePaymentMethodRequest {
    #[builder(default, setter(strip_option))]
    pub expiry_month: Option<u8>,
    #[builder(default, setter(strip_option))]
    pub expiry_year: Option<u16>,
    #[builder(default, setter(strip_option))]
    pub billing_address: Option<BillingAddress>,
}

/// Billing alerts configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingAlerts {
    pub enabled: bool,
    pub threshold_amount: Option<f64>,
    pub notification_emails: Option<Vec<String>>,
    pub alert_frequency: Option<String>,
    
    #[serde(flatten)]
    pub extra: Value,
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

/// Cost breakdown information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostBreakdown {
    pub period: String,
    pub total_cost: f64,
    pub currency: String,
    pub categories: Option<Vec<CostCategory>>,
    
    #[serde(flatten)]
    pub extra: Value,
}

/// Cost category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostCategory {
    pub name: String,
    pub amount: f64,
    pub percentage: Option<f64>,
    pub items: Option<Vec<CostItem>>,
    
    #[serde(flatten)]
    pub extra: Value,
}

/// Cost item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostItem {
    pub resource_id: Option<String>,
    pub resource_name: Option<String>,
    pub resource_type: Option<String>,
    pub amount: f64,
    pub usage: Option<f64>,
    pub unit: Option<String>,
    
    #[serde(flatten)]
    pub extra: Value,
}

/// Usage report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageReport {
    pub start_date: String,
    pub end_date: String,
    pub total_usage: Option<f64>,
    pub total_cost: Option<f64>,
    pub currency: Option<String>,
    pub resources: Option<Vec<ResourceUsage>>,
    
    #[serde(flatten)]
    pub extra: Value,
}

/// Resource usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub resource_id: String,
    pub resource_name: Option<String>,
    pub resource_type: String,
    pub usage: f64,
    pub unit: String,
    pub cost: Option<f64>,
    pub daily_breakdown: Option<Vec<DailyUsage>>,
    
    #[serde(flatten)]
    pub extra: Value,
}

/// Daily usage breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyUsage {
    pub date: String,
    pub usage: f64,
    pub cost: Option<f64>,
    
    #[serde(flatten)]
    pub extra: Value,
}

/// Credits balance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditsBalance {
    pub total_credits: f64,
    pub used_credits: f64,
    pub available_credits: f64,
    pub currency: String,
    pub expiry_date: Option<String>,
    
    #[serde(flatten)]
    pub extra: Value,
}

/// Promo code response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromoCodeResponse {
    pub success: bool,
    pub message: String,
    pub credits_added: Option<f64>,
    pub new_balance: Option<f64>,
    
    #[serde(flatten)]
    pub extra: Value,
}

/// Billing history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingHistoryEntry {
    pub date: String,
    pub description: String,
    pub amount: f64,
    pub balance: Option<f64>,
    pub transaction_type: String,
    pub reference_id: Option<String>,
    
    #[serde(flatten)]
    pub extra: Value,
}