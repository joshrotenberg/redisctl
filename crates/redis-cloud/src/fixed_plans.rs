//! Fixed plan management for Redis Cloud Essentials
//!
//! ## Overview
//! - Query available fixed plans
//! - Get plan details and pricing

use crate::client::CloudClient;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Fixed plan information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixedPlan {
    pub id: u32,
    pub name: String,
    pub size: f64,
    
    #[serde(rename = "sizeMeasurementUnit")]
    pub size_measurement_unit: String,
    
    pub provider: String,
    pub region: String,
    
    #[serde(rename = "regionId")]
    pub region_id: u32,
    
    pub price: f64,
    
    #[serde(rename = "priceCurrency")]
    pub price_currency: String,
    
    #[serde(rename = "pricePeriod")]
    pub price_period: String,
    
    #[serde(rename = "maximumDatabases")]
    pub maximum_databases: u32,
    
    pub availability: String,
    pub connections: String,
    
    #[serde(rename = "supportDataPersistence")]
    pub support_data_persistence: bool,
    
    #[serde(rename = "supportInstantAndDailyBackups")]
    pub support_instant_and_daily_backups: bool,
    
    #[serde(rename = "supportReplication")]
    pub support_replication: bool,
    
    #[serde(rename = "supportClustering")]
    pub support_clustering: bool,
}

/// Fixed plan handler
pub struct FixedPlanHandler {
    client: CloudClient,
}

impl FixedPlanHandler {
    pub fn new(client: CloudClient) -> Self {
        FixedPlanHandler { client }
    }
    
    /// List all available fixed plans
    pub async fn list(&self) -> Result<Vec<FixedPlan>> {
        let response = self.client.get_raw("/fixed/plans").await?;
        
        // The response has a "plans" field containing the array
        if let Some(plans) = response.get("plans") {
            let plans: Vec<FixedPlan> = serde_json::from_value(plans.clone())?;
            Ok(plans)
        } else {
            Ok(vec![])
        }
    }
    
    /// Get a specific fixed plan by ID
    pub async fn get(&self, plan_id: u32) -> Result<FixedPlan> {
        let response = self.client
            .get_raw(&format!("/fixed/plans/{}", plan_id))
            .await?;
        let plan: FixedPlan = serde_json::from_value(response)?;
        Ok(plan)
    }
    
    /// Get fixed plans for a specific subscription
    pub async fn get_subscription_plans(&self, subscription_id: u32) -> Result<Value> {
        self.client
            .get_raw(&format!("/fixed/plans/subscriptions/{}", subscription_id))
            .await
    }
}