//! Private Service Connect operations handler

use crate::{client::CloudClient, Result};
use serde_json::Value;

/// Handler for Cloud Private Service Connect operations
pub struct CloudPrivateServiceConnectHandler {
    client: CloudClient,
}

impl CloudPrivateServiceConnectHandler {
    pub fn new(client: CloudClient) -> Self {
        CloudPrivateServiceConnectHandler { client }
    }

    /// List all private service connect services for a subscription
    pub async fn list(&self, subscription_id: u32) -> Result<Value> {
        self.client
            .get(&format!(
                "/subscriptions/{}/private-service-connect",
                subscription_id
            ))
            .await
    }

    /// Get private service connect service details
    pub async fn get(&self, subscription_id: u32, psc_service_id: &str) -> Result<Value> {
        self.client
            .get(&format!(
                "/subscriptions/{}/private-service-connect/{}",
                subscription_id, psc_service_id
            ))
            .await
    }

    /// Create private service connect service
    pub async fn create(&self, subscription_id: u32, service: Value) -> Result<Value> {
        self.client
            .post(
                &format!("/subscriptions/{}/private-service-connect", subscription_id),
                &service,
            )
            .await
    }

    /// Update private service connect service
    pub async fn update(
        &self,
        subscription_id: u32,
        psc_service_id: &str,
        service: Value,
    ) -> Result<Value> {
        self.client
            .put(
                &format!(
                    "/subscriptions/{}/private-service-connect/{}",
                    subscription_id, psc_service_id
                ),
                &service,
            )
            .await
    }

    /// Delete private service connect service
    pub async fn delete(&self, subscription_id: u32, psc_service_id: &str) -> Result<()> {
        self.client
            .delete(&format!(
                "/subscriptions/{}/private-service-connect/{}",
                subscription_id, psc_service_id
            ))
            .await
    }

    /// Get private service connect endpoint
    pub async fn get_endpoint(
        &self,
        subscription_id: u32,
        psc_service_id: &str,
        endpoint_id: &str,
    ) -> Result<Value> {
        self.client
            .get(&format!(
                "/subscriptions/{}/private-service-connect/{}/endpoints/{}",
                subscription_id, psc_service_id, endpoint_id
            ))
            .await
    }

    /// Get endpoint creation scripts
    pub async fn get_creation_scripts(
        &self,
        subscription_id: u32,
        psc_service_id: &str,
        endpoint_id: &str,
    ) -> Result<Value> {
        self.client
            .get(&format!(
                "/subscriptions/{}/private-service-connect/{}/endpoints/{}/creationScripts",
                subscription_id, psc_service_id, endpoint_id
            ))
            .await
    }

    /// Get endpoint deletion scripts
    pub async fn get_deletion_scripts(
        &self,
        subscription_id: u32,
        psc_service_id: &str,
        endpoint_id: &str,
    ) -> Result<Value> {
        self.client
            .get(&format!(
                "/subscriptions/{}/private-service-connect/{}/endpoints/{}/deletionScripts",
                subscription_id, psc_service_id, endpoint_id
            ))
            .await
    }

    /// List regional private service connect services
    pub async fn list_regional(&self, subscription_id: u32, region_id: &str) -> Result<Value> {
        self.client
            .get(&format!(
                "/subscriptions/{}/regions/{}/private-service-connect",
                subscription_id, region_id
            ))
            .await
    }

    /// Get regional private service connect service
    pub async fn get_regional(
        &self,
        subscription_id: u32,
        region_id: &str,
        psc_service_id: &str,
    ) -> Result<Value> {
        self.client
            .get(&format!(
                "/subscriptions/{}/regions/{}/private-service-connect/{}",
                subscription_id, region_id, psc_service_id
            ))
            .await
    }

    /// Create regional private service connect service
    pub async fn create_regional(
        &self,
        subscription_id: u32,
        region_id: &str,
        service: Value,
    ) -> Result<Value> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/regions/{}/private-service-connect",
                    subscription_id, region_id
                ),
                &service,
            )
            .await
    }

    /// Update regional private service connect service
    pub async fn update_regional(
        &self,
        subscription_id: u32,
        region_id: &str,
        psc_service_id: &str,
        service: Value,
    ) -> Result<Value> {
        self.client
            .put(
                &format!(
                    "/subscriptions/{}/regions/{}/private-service-connect/{}",
                    subscription_id, region_id, psc_service_id
                ),
                &service,
            )
            .await
    }

    /// Delete regional private service connect service
    pub async fn delete_regional(
        &self,
        subscription_id: u32,
        region_id: &str,
        psc_service_id: &str,
    ) -> Result<()> {
        self.client
            .delete(&format!(
                "/subscriptions/{}/regions/{}/private-service-connect/{}",
                subscription_id, region_id, psc_service_id
            ))
            .await
    }

    /// Get regional endpoint
    pub async fn get_regional_endpoint(
        &self,
        subscription_id: u32,
        region_id: &str,
        psc_service_id: &str,
        endpoint_id: &str,
    ) -> Result<Value> {
        self.client
            .get(&format!(
                "/subscriptions/{}/regions/{}/private-service-connect/{}/endpoints/{}",
                subscription_id, region_id, psc_service_id, endpoint_id
            ))
            .await
    }

    /// Get regional endpoint creation scripts
    pub async fn get_regional_creation_scripts(
        &self,
        subscription_id: u32,
        region_id: &str,
        psc_service_id: &str,
        endpoint_id: &str,
    ) -> Result<Value> {
        self.client
            .get(&format!(
                "/subscriptions/{}/regions/{}/private-service-connect/{}/endpoints/{}/creationScripts",
                subscription_id, region_id, psc_service_id, endpoint_id
            ))
            .await
    }

    /// Get regional endpoint deletion scripts
    pub async fn get_regional_deletion_scripts(
        &self,
        subscription_id: u32,
        region_id: &str,
        psc_service_id: &str,
        endpoint_id: &str,
    ) -> Result<Value> {
        self.client
            .get(&format!(
                "/subscriptions/{}/regions/{}/private-service-connect/{}/endpoints/{}/deletionScripts",
                subscription_id, region_id, psc_service_id, endpoint_id
            ))
            .await
    }
}
