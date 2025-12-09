//! Cost Report Generation and Retrieval
//!
//! This module provides functionality for generating and downloading cost reports
//! in FOCUS format from Redis Cloud. FOCUS (FinOps Cost and Usage Specification)
//! is an open standard for cloud cost data.
//!
//! # Overview
//!
//! Cost reports provide detailed billing information for your Redis Cloud resources,
//! allowing you to analyze costs by subscription, database, region, and custom tags.
//!
//! # Report Generation Flow
//!
//! 1. **Generate Request**: Submit a cost report request with date range and filters
//! 2. **Track Task**: Monitor the async task until completion
//! 3. **Download Report**: Retrieve the generated report using the costReportId
//!
//! # Key Features
//!
//! - **Date Range Filtering**: Specify start and end dates (max 40 days)
//! - **Output Formats**: CSV or JSON
//! - **Subscription Filtering**: Filter by specific subscription IDs
//! - **Database Filtering**: Filter by specific database IDs
//! - **Plan Type Filtering**: Filter by "pro" or "essentials"
//! - **Region Filtering**: Filter by cloud regions
//! - **Tag Filtering**: Filter by custom key-value tags
//!
//! # Example Usage
//!
//! ```no_run
//! use redis_cloud::{CloudClient, CostReportHandler, CostReportCreateRequest, CostReportFormat};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = CloudClient::builder()
//!     .api_key("your-api-key")
//!     .api_secret("your-api-secret")
//!     .build()?;
//!
//! let handler = CostReportHandler::new(client);
//!
//! // Generate a cost report for the last month
//! let request = CostReportCreateRequest::builder()
//!     .start_date("2025-01-01")
//!     .end_date("2025-01-31")
//!     .format(CostReportFormat::Csv)
//!     .build();
//!
//! let task = handler.generate_cost_report(request).await?;
//! println!("Task ID: {:?}", task.task_id);
//!
//! // Once the task completes, download the report
//! // The costReportId is returned in the task response
//! let report_bytes = handler.download_cost_report("cost-report-12345").await?;
//! std::fs::write("cost-report.csv", report_bytes)?;
//! # Ok(())
//! # }
//! ```
//!
//! # FOCUS Format
//!
//! The cost report follows the [FOCUS specification](https://focus.finops.org/),
//! providing standardized columns including:
//! - BilledCost, EffectiveCost, ListCost, ContractedCost
//! - Resource identifiers (subscription, database)
//! - Service categories and SKU details
//! - Billing period and usage information

use crate::{CloudClient, Result, tasks::TaskStateUpdate};
use serde::{Deserialize, Serialize};
use serde_json::Value;

// ============================================================================
// Models
// ============================================================================

/// Output format for cost reports
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CostReportFormat {
    /// CSV format (default)
    #[default]
    Csv,
    /// JSON format
    Json,
}

impl std::fmt::Display for CostReportFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CostReportFormat::Csv => write!(f, "csv"),
            CostReportFormat::Json => write!(f, "json"),
        }
    }
}

/// Subscription type filter for cost reports
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SubscriptionType {
    /// Pro subscriptions (pay-as-you-go)
    Pro,
    /// Essentials subscriptions (fixed plans)
    Essentials,
}

impl std::fmt::Display for SubscriptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubscriptionType::Pro => write!(f, "pro"),
            SubscriptionType::Essentials => write!(f, "essentials"),
        }
    }
}

/// Tag filter for cost reports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    /// Tag key
    pub key: String,
    /// Tag value
    pub value: String,
}

impl Tag {
    /// Create a new tag filter
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

/// Request to generate a cost report
///
/// Cost reports are generated asynchronously. After submitting a request,
/// you'll receive a task ID that can be used to track the generation progress.
/// Once complete, use the costReportId from the task response to download the report.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CostReportCreateRequest {
    /// Start date for the report (YYYY-MM-DD format, required)
    pub start_date: String,

    /// End date for the report (YYYY-MM-DD format, required)
    /// Must be after start_date and within 40 days of start_date
    pub end_date: String,

    /// Output format (csv or json, defaults to csv)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<CostReportFormat>,

    /// Filter by subscription IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_ids: Option<Vec<i32>>,

    /// Filter by database IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_ids: Option<Vec<i32>>,

    /// Filter by subscription type (pro or essentials)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_type: Option<SubscriptionType>,

    /// Filter by regions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,

    /// Filter by tags (key-value pairs)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,

    /// Additional fields for forward compatibility
    #[serde(flatten)]
    pub extra: Value,
}

impl CostReportCreateRequest {
    /// Create a new cost report request builder
    pub fn builder() -> CostReportCreateRequestBuilder {
        CostReportCreateRequestBuilder::default()
    }

    /// Create a simple request with just date range
    pub fn new(start_date: impl Into<String>, end_date: impl Into<String>) -> Self {
        Self {
            start_date: start_date.into(),
            end_date: end_date.into(),
            ..Default::default()
        }
    }
}

/// Builder for CostReportCreateRequest
#[derive(Debug, Clone, Default)]
pub struct CostReportCreateRequestBuilder {
    start_date: Option<String>,
    end_date: Option<String>,
    format: Option<CostReportFormat>,
    subscription_ids: Option<Vec<i32>>,
    database_ids: Option<Vec<i32>>,
    subscription_type: Option<SubscriptionType>,
    regions: Option<Vec<String>>,
    tags: Option<Vec<Tag>>,
}

impl CostReportCreateRequestBuilder {
    /// Set the start date (required, YYYY-MM-DD format)
    pub fn start_date(mut self, date: impl Into<String>) -> Self {
        self.start_date = Some(date.into());
        self
    }

    /// Set the end date (required, YYYY-MM-DD format)
    pub fn end_date(mut self, date: impl Into<String>) -> Self {
        self.end_date = Some(date.into());
        self
    }

    /// Set the output format
    pub fn format(mut self, format: CostReportFormat) -> Self {
        self.format = Some(format);
        self
    }

    /// Filter by subscription IDs
    pub fn subscription_ids(mut self, ids: Vec<i32>) -> Self {
        self.subscription_ids = Some(ids);
        self
    }

    /// Filter by database IDs
    pub fn database_ids(mut self, ids: Vec<i32>) -> Self {
        self.database_ids = Some(ids);
        self
    }

    /// Filter by subscription type
    pub fn subscription_type(mut self, sub_type: SubscriptionType) -> Self {
        self.subscription_type = Some(sub_type);
        self
    }

    /// Filter by regions
    pub fn regions(mut self, regions: Vec<String>) -> Self {
        self.regions = Some(regions);
        self
    }

    /// Filter by tags
    pub fn tags(mut self, tags: Vec<Tag>) -> Self {
        self.tags = Some(tags);
        self
    }

    /// Add a single tag filter
    pub fn tag(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let tag = Tag::new(key, value);
        match &mut self.tags {
            Some(tags) => tags.push(tag),
            None => self.tags = Some(vec![tag]),
        }
        self
    }

    /// Build the request
    ///
    /// # Panics
    /// Panics if start_date or end_date is not set
    pub fn build(self) -> CostReportCreateRequest {
        CostReportCreateRequest {
            start_date: self.start_date.expect("start_date is required"),
            end_date: self.end_date.expect("end_date is required"),
            format: self.format,
            subscription_ids: self.subscription_ids,
            database_ids: self.database_ids,
            subscription_type: self.subscription_type,
            regions: self.regions,
            tags: self.tags,
            extra: Value::Null,
        }
    }
}

// ============================================================================
// Handler
// ============================================================================

/// Handler for cost report operations
///
/// Provides methods to generate and download cost reports in FOCUS format.
pub struct CostReportHandler {
    client: CloudClient,
}

impl CostReportHandler {
    /// Create a new handler
    pub fn new(client: CloudClient) -> Self {
        Self { client }
    }

    /// Generate a cost report (Beta)
    ///
    /// Generates a cost report in FOCUS format for the specified time period
    /// and filters. The maximum date range is 40 days.
    ///
    /// This is an asynchronous operation. The returned TaskStateUpdate contains
    /// a task_id that can be used to track progress. Once complete, the task
    /// response will contain the costReportId needed to download the report.
    ///
    /// POST /cost-report
    ///
    /// # Arguments
    /// * `request` - The cost report generation request with date range and filters
    ///
    /// # Returns
    /// A TaskStateUpdate with the task ID for tracking the generation
    ///
    /// # Example
    /// ```no_run
    /// # use redis_cloud::{CloudClient, CostReportHandler, CostReportCreateRequest};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = CloudClient::builder().api_key("k").api_secret("s").build()?;
    /// let handler = CostReportHandler::new(client);
    /// let request = CostReportCreateRequest::new("2025-01-01", "2025-01-31");
    /// let task = handler.generate_cost_report(request).await?;
    /// println!("Task ID: {:?}", task.task_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn generate_cost_report(
        &self,
        request: CostReportCreateRequest,
    ) -> Result<TaskStateUpdate> {
        self.client.post("/cost-report", &request).await
    }

    /// Generate a cost report and return raw JSON response
    ///
    /// POST /cost-report
    pub async fn generate_cost_report_raw(
        &self,
        request: CostReportCreateRequest,
    ) -> Result<Value> {
        let body = serde_json::to_value(request).map_err(crate::CloudError::from)?;
        self.client.post_raw("/cost-report", body).await
    }

    /// Download a generated cost report (Beta)
    ///
    /// Returns the generated cost report file in FOCUS format. The costReportId
    /// is obtained from the task response after the generation task completes.
    ///
    /// GET /cost-report/{costReportId}
    ///
    /// # Arguments
    /// * `cost_report_id` - The cost report ID from the completed generation task
    ///
    /// # Returns
    /// The raw bytes of the cost report file (CSV or JSON depending on request)
    ///
    /// # Example
    /// ```no_run
    /// # use redis_cloud::{CloudClient, CostReportHandler};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = CloudClient::builder().api_key("k").api_secret("s").build()?;
    /// let handler = CostReportHandler::new(client);
    /// let report = handler.download_cost_report("cost-report-12345").await?;
    /// std::fs::write("cost-report.csv", report)?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download_cost_report(&self, cost_report_id: &str) -> Result<Vec<u8>> {
        self.client
            .get_bytes(&format!("/cost-report/{}", cost_report_id))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cost_report_request_builder() {
        let request = CostReportCreateRequest::builder()
            .start_date("2025-01-01")
            .end_date("2025-01-31")
            .format(CostReportFormat::Csv)
            .subscription_ids(vec![123, 456])
            .regions(vec!["us-east-1".to_string()])
            .tag("env", "prod")
            .build();

        assert_eq!(request.start_date, "2025-01-01");
        assert_eq!(request.end_date, "2025-01-31");
        assert_eq!(request.format, Some(CostReportFormat::Csv));
        assert_eq!(request.subscription_ids, Some(vec![123, 456]));
        assert_eq!(request.regions, Some(vec!["us-east-1".to_string()]));
        assert!(request.tags.is_some());
        let tags = request.tags.unwrap();
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0].key, "env");
        assert_eq!(tags[0].value, "prod");
    }

    #[test]
    fn test_cost_report_request_simple() {
        let request = CostReportCreateRequest::new("2025-01-01", "2025-01-31");
        assert_eq!(request.start_date, "2025-01-01");
        assert_eq!(request.end_date, "2025-01-31");
        assert!(request.format.is_none());
    }

    #[test]
    fn test_cost_report_format_display() {
        assert_eq!(CostReportFormat::Csv.to_string(), "csv");
        assert_eq!(CostReportFormat::Json.to_string(), "json");
    }

    #[test]
    fn test_subscription_type_display() {
        assert_eq!(SubscriptionType::Pro.to_string(), "pro");
        assert_eq!(SubscriptionType::Essentials.to_string(), "essentials");
    }

    #[test]
    fn test_tag_creation() {
        let tag = Tag::new("environment", "production");
        assert_eq!(tag.key, "environment");
        assert_eq!(tag.value, "production");
    }

    #[test]
    fn test_request_serialization() {
        let request = CostReportCreateRequest::builder()
            .start_date("2025-01-01")
            .end_date("2025-01-31")
            .format(CostReportFormat::Json)
            .subscription_type(SubscriptionType::Pro)
            .build();

        let json = serde_json::to_value(&request).unwrap();
        assert_eq!(json["startDate"], "2025-01-01");
        assert_eq!(json["endDate"], "2025-01-31");
        assert_eq!(json["format"], "json");
        assert_eq!(json["subscriptionType"], "pro");
    }
}
