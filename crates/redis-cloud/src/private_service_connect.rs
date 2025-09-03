//! Private Service Connect operations handler

use crate::{
    Result,
    client::CloudClient,
    models::{PscCreateRequest, PscEndpoint, PscScripts, PscService, PscUpdateRequest},
};

/// Handler for Cloud Private Service Connect operations
pub struct CloudPrivateServiceConnectHandler {
    client: CloudClient,
}

impl CloudPrivateServiceConnectHandler {
    pub fn new(client: CloudClient) -> Self {
        CloudPrivateServiceConnectHandler { client }
    }

    /// List all private service connect services for a subscription
    pub async fn list(&self, subscription_id: u32) -> Result<Vec<PscService>> {
        let v: serde_json::Value = self
            .client
            .get(&format!(
                "/subscriptions/{}/private-service-connect",
                subscription_id
            ))
            .await?;
        if v.is_array() {
            serde_json::from_value(v).map_err(Into::into)
        } else if let Some(arr) = v
            .get("services")
            .or_else(|| v.get("privateServiceConnects"))
        {
            serde_json::from_value(arr.clone()).map_err(Into::into)
        } else {
            Ok(vec![])
        }
    }

    /// Get private service connect service details
    pub async fn get(&self, subscription_id: u32, psc_service_id: &str) -> Result<PscService> {
        let v: serde_json::Value = self
            .client
            .get(&format!(
                "/subscriptions/{}/private-service-connect/{}",
                subscription_id, psc_service_id
            ))
            .await?;
        if let Some(obj) = v.get("privateServiceConnect") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
    }

    /// Create private service connect service
    pub async fn create(
        &self,
        subscription_id: u32,
        service: PscCreateRequest,
    ) -> Result<PscService> {
        let v: serde_json::Value = self
            .client
            .post(
                &format!("/subscriptions/{}/private-service-connect", subscription_id),
                &service,
            )
            .await?;
        if let Some(obj) = v.get("privateServiceConnect") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
    }

    /// Update private service connect service
    pub async fn update(
        &self,
        subscription_id: u32,
        psc_service_id: &str,
        service: PscUpdateRequest,
    ) -> Result<PscService> {
        let v: serde_json::Value = self
            .client
            .put(
                &format!(
                    "/subscriptions/{}/private-service-connect/{}",
                    subscription_id, psc_service_id
                ),
                &service,
            )
            .await?;
        if let Some(obj) = v.get("privateServiceConnect") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
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
    ) -> Result<PscEndpoint> {
        let v: serde_json::Value = self
            .client
            .get(&format!(
                "/subscriptions/{}/private-service-connect/{}/endpoints/{}",
                subscription_id, psc_service_id, endpoint_id
            ))
            .await?;
        if let Some(obj) = v.get("endpoint") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
    }

    /// Get endpoint creation scripts
    pub async fn get_creation_scripts(
        &self,
        subscription_id: u32,
        psc_service_id: &str,
        endpoint_id: &str,
    ) -> Result<PscScripts> {
        let v: serde_json::Value = self
            .client
            .get(&format!(
                "/subscriptions/{}/private-service-connect/{}/endpoints/{}/creationScripts",
                subscription_id, psc_service_id, endpoint_id
            ))
            .await?;
        if let Some(obj) = v.get("scripts") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
    }

    /// Get endpoint deletion scripts
    pub async fn get_deletion_scripts(
        &self,
        subscription_id: u32,
        psc_service_id: &str,
        endpoint_id: &str,
    ) -> Result<PscScripts> {
        let v: serde_json::Value = self
            .client
            .get(&format!(
                "/subscriptions/{}/private-service-connect/{}/endpoints/{}/deletionScripts",
                subscription_id, psc_service_id, endpoint_id
            ))
            .await?;
        if let Some(obj) = v.get("scripts") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
    }

    /// List regional private service connect services
    pub async fn list_regional(
        &self,
        subscription_id: u32,
        region_id: &str,
    ) -> Result<Vec<PscService>> {
        let v: serde_json::Value = self
            .client
            .get(&format!(
                "/subscriptions/{}/regions/{}/private-service-connect",
                subscription_id, region_id
            ))
            .await?;
        if v.is_array() {
            serde_json::from_value(v).map_err(Into::into)
        } else if let Some(arr) = v
            .get("services")
            .or_else(|| v.get("privateServiceConnects"))
        {
            serde_json::from_value(arr.clone()).map_err(Into::into)
        } else {
            Ok(vec![])
        }
    }

    /// Get regional private service connect service
    pub async fn get_regional(
        &self,
        subscription_id: u32,
        region_id: &str,
        psc_service_id: &str,
    ) -> Result<PscService> {
        let v: serde_json::Value = self
            .client
            .get(&format!(
                "/subscriptions/{}/regions/{}/private-service-connect/{}",
                subscription_id, region_id, psc_service_id
            ))
            .await?;
        if let Some(obj) = v.get("privateServiceConnect") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
    }

    /// Create regional private service connect service
    pub async fn create_regional(
        &self,
        subscription_id: u32,
        region_id: &str,
        service: PscCreateRequest,
    ) -> Result<PscService> {
        let v: serde_json::Value = self
            .client
            .post(
                &format!(
                    "/subscriptions/{}/regions/{}/private-service-connect",
                    subscription_id, region_id
                ),
                &service,
            )
            .await?;
        if let Some(obj) = v.get("privateServiceConnect") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
    }

    /// Update regional private service connect service
    pub async fn update_regional(
        &self,
        subscription_id: u32,
        region_id: &str,
        psc_service_id: &str,
        service: PscUpdateRequest,
    ) -> Result<PscService> {
        let v: serde_json::Value = self
            .client
            .put(
                &format!(
                    "/subscriptions/{}/regions/{}/private-service-connect/{}",
                    subscription_id, region_id, psc_service_id
                ),
                &service,
            )
            .await?;
        if let Some(obj) = v.get("privateServiceConnect") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
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
    ) -> Result<PscEndpoint> {
        let v: serde_json::Value = self
            .client
            .get(&format!(
                "/subscriptions/{}/regions/{}/private-service-connect/{}/endpoints/{}",
                subscription_id, region_id, psc_service_id, endpoint_id
            ))
            .await?;
        if let Some(obj) = v.get("endpoint") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
    }

    /// Get regional endpoint creation scripts
    pub async fn get_regional_creation_scripts(
        &self,
        subscription_id: u32,
        region_id: &str,
        psc_service_id: &str,
        endpoint_id: &str,
    ) -> Result<PscScripts> {
        let v: serde_json::Value = self.client.get(&format!(
                "/subscriptions/{}/regions/{}/private-service-connect/{}/endpoints/{}/creationScripts",
                subscription_id, region_id, psc_service_id, endpoint_id
            ))
            .await?;
        if let Some(obj) = v.get("scripts") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
    }

    /// Get regional endpoint deletion scripts
    pub async fn get_regional_deletion_scripts(
        &self,
        subscription_id: u32,
        region_id: &str,
        psc_service_id: &str,
        endpoint_id: &str,
    ) -> Result<PscScripts> {
        let v: serde_json::Value = self.client.get(&format!(
                "/subscriptions/{}/regions/{}/private-service-connect/{}/endpoints/{}/deletionScripts",
                subscription_id, region_id, psc_service_id, endpoint_id
            ))
            .await?;
        if let Some(obj) = v.get("scripts") {
            serde_json::from_value(obj.clone()).map_err(Into::into)
        } else {
            serde_json::from_value(v).map_err(Into::into)
        }
    }
}
