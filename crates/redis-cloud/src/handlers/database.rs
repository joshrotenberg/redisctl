//! Database operations handler
//!
//! This module provides comprehensive database management capabilities for Redis Cloud,
//! including CRUD operations, backups, imports, metrics, and scaling operations.
//!
//! # Examples
//!
//! ```rust,no_run
//! use redis_cloud::{CloudClient, CloudDatabaseHandler};
//! use serde_json::json;
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let client = CloudClient::builder()
//!     .api_key("your-api-key")
//!     .api_secret("your-api-secret")
//!     .build()?;
//!
//! let db_handler = CloudDatabaseHandler::new(client);
//!
//! // Get database information  
//! let db_info = db_handler.get(123, 456).await?;
//! println!("Database: {}", db_info.name);
//!
//! // Create database using raw client API
//! let database_config = json!({
//!     "name": "my-redis-db",
//!     "memory_limit_in_gb": 2.5,
//!     "support_oss_cluster_api": false,
//!     "replication": true,
//!     "data_persistence": "aof-every-1-sec"
//! });
//! # Ok(())
//! # }
//! ```

use crate::{
    Result,
    client::CloudClient,
    models::{CloudDatabase, CreateDatabaseRequest, UpdateDatabaseRequest},
};
use serde_json::Value;

/// Handler for Cloud database operations
///
/// Provides methods for managing Redis Cloud databases including creation, updates,
/// backups, imports, metrics collection, and scaling operations.
///
/// All database operations require both a subscription ID and database ID, as databases
/// are scoped within subscriptions in Redis Cloud.
pub struct CloudDatabaseHandler {
    client: CloudClient,
}

impl CloudDatabaseHandler {
    /// Create a new database handler instance
    ///
    /// # Arguments
    /// * `client` - The configured CloudClient instance
    pub fn new(client: CloudClient) -> Self {
        CloudDatabaseHandler { client }
    }

    /// Retrieve a specific database by ID
    ///
    /// Returns detailed information about a database including its configuration,
    /// status, endpoints, and current metrics.
    ///
    /// # Arguments
    /// * `subscription_id` - The ID of the subscription containing the database
    /// * `database_id` - The unique database identifier
    ///
    /// # Examples
    /// ```rust,no_run
    /// # use redis_cloud::{CloudClient, CloudDatabaseHandler};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = CloudClient::builder().api_key("key").api_secret("secret").build()?;
    /// let db_handler = CloudDatabaseHandler::new(client);
    /// let database = db_handler.get(123, 456).await?;
    /// println!("Database name: {}", database.name);
    /// println!("Memory limit: {} GB", database.memory_limit_in_gb);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self, subscription_id: u32, database_id: u32) -> Result<CloudDatabase> {
        self.client
            .get(&format!(
                "/subscriptions/{}/databases/{}",
                subscription_id, database_id
            ))
            .await
    }

    /// Create a new database in a subscription
    ///
    /// Creates a new Redis database with the specified configuration. The database
    /// will be deployed across the subscription's defined regions and cloud providers.
    ///
    /// # Arguments
    /// * `subscription_id` - The ID of the subscription to create the database in
    /// * `request` - Database configuration including name, memory, replication settings
    ///
    /// # Examples
    /// ```rust,no_run
    /// # use redis_cloud::{CloudClient, CloudDatabaseHandler};
    /// # use serde_json::json;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = CloudClient::builder().api_key("key").api_secret("secret").build()?;
    /// let db_handler = CloudDatabaseHandler::new(client);
    /// let config = json!({
    ///     "name": "production-cache",
    ///     "memory_limit_in_gb": 5.0,
    ///     "replication": true,
    ///     "data_persistence": "aof-every-1-sec",
    ///     "password": "secure-password"
    /// });
    /// // Note: create() takes a typed request struct, not JSON
    /// // let result = db_handler.create(123, create_request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        &self,
        subscription_id: u32,
        request: CreateDatabaseRequest,
    ) -> Result<Value> {
        self.client
            .post(
                &format!("/subscriptions/{}/databases", subscription_id),
                &request,
            )
            .await
    }

    /// Update an existing database configuration
    ///
    /// Modifies database settings such as memory limits, replication, persistence,
    /// and other configuration options. Some changes may require a database restart.
    ///
    /// # Arguments
    /// * `subscription_id` - The ID of the subscription containing the database
    /// * `database_id` - The unique database identifier
    /// * `request` - Updated configuration settings
    ///
    /// # Examples
    /// ```rust,no_run
    /// # use redis_cloud::{CloudClient, CloudDatabaseHandler};
    /// # use serde_json::json;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = CloudClient::builder().api_key("key").api_secret("secret").build()?;
    /// let db_handler = CloudDatabaseHandler::new(client);
    /// let updates = json!({
    ///     "memory_limit_in_gb": 10.0,
    ///     "replication": false
    /// });
    /// // Note: update() takes a typed request struct, not JSON
    /// // let updated_db = db_handler.update(123, 456, update_request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(
        &self,
        subscription_id: u32,
        database_id: u32,
        request: UpdateDatabaseRequest,
    ) -> Result<CloudDatabase> {
        self.client
            .put(
                &format!(
                    "/subscriptions/{}/databases/{}",
                    subscription_id, database_id
                ),
                &request,
            )
            .await
    }

    /// Delete a database permanently
    ///
    /// **Warning**: This operation is irreversible and will permanently delete
    /// all data in the database. Consider creating a backup before deletion.
    ///
    /// # Arguments
    /// * `subscription_id` - The ID of the subscription containing the database
    /// * `database_id` - The unique database identifier
    ///
    /// # Examples
    /// ```rust,no_run
    /// # use redis_cloud::{CloudClient, CloudDatabaseHandler};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = CloudClient::builder().api_key("key").api_secret("secret").build()?;
    /// let db_handler = CloudDatabaseHandler::new(client);
    /// let result = db_handler.delete(123, 456).await?;
    /// println!("Database deleted successfully");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(&self, subscription_id: u32, database_id: u32) -> Result<Value> {
        self.client
            .delete(&format!(
                "/subscriptions/{}/databases/{}",
                subscription_id, database_id
            ))
            .await?;
        Ok(serde_json::json!({"message": format!("Database {} deleted", database_id)}))
    }

    /// Resize database
    pub async fn resize(
        &self,
        subscription_id: u32,
        database_id: u32,
        memory_limit_in_gb: f64,
    ) -> Result<CloudDatabase> {
        let request = UpdateDatabaseRequest {
            memory_limit_in_gb: Some(memory_limit_in_gb),
            name: None,
            data_persistence: None,
            replication: None,
            data_eviction: None,
            password: None,
        };
        self.update(subscription_id, database_id, request).await
    }

    /// List all databases across all subscriptions
    pub async fn list_all(&self) -> Result<Vec<CloudDatabase>> {
        let response: Value = self.client.get("/databases").await?;
        if let Some(dbs) = response.get("databases") {
            serde_json::from_value(dbs.clone()).map_err(Into::into)
        } else {
            Ok(vec![])
        }
    }

    /// List databases for subscription as Value
    pub async fn list(&self, subscription_id: u32) -> Result<Value> {
        self.client
            .get(&format!("/subscriptions/{}/databases", subscription_id))
            .await
    }

    /// Get database as Value
    pub async fn get_raw(&self, subscription_id: u32, database_id: u32) -> Result<Value> {
        self.client
            .get(&format!(
                "/subscriptions/{}/databases/{}",
                subscription_id, database_id
            ))
            .await
    }

    /// Create database with Value
    pub async fn create_raw(&self, subscription_id: u32, request: Value) -> Result<Value> {
        self.client
            .post(
                &format!("/subscriptions/{}/databases", subscription_id),
                &request,
            )
            .await
    }

    /// Update database with Value
    pub async fn update_raw(
        &self,
        subscription_id: u32,
        database_id: u32,
        request: Value,
    ) -> Result<Value> {
        self.client
            .put(
                &format!(
                    "/subscriptions/{}/databases/{}",
                    subscription_id, database_id
                ),
                &request,
            )
            .await
    }

    /// Backup database
    pub async fn backup(&self, subscription_id: u32, database_id: u32) -> Result<Value> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/databases/{}/backup",
                    subscription_id, database_id
                ),
                &Value::Null,
            )
            .await
    }

    /// Import data
    pub async fn import(
        &self,
        subscription_id: u32,
        database_id: u32,
        request: Value,
    ) -> Result<Value> {
        self.client
            .post(
                &format!(
                    "/subscriptions/{}/databases/{}/import",
                    subscription_id, database_id
                ),
                &request,
            )
            .await
    }

    /// Get metrics
    pub async fn get_metrics(
        &self,
        subscription_id: u32,
        database_id: u32,
        metrics: &str,
        period: &str,
    ) -> Result<Value> {
        self.client
            .get(&format!(
                "/subscriptions/{}/databases/{}/metrics?metrics={}&period={}",
                subscription_id, database_id, metrics, period
            ))
            .await
    }

    /// Get database backup status
    pub async fn backup_status(&self, subscription_id: u32, database_id: u32) -> Result<Value> {
        self.client
            .get(&format!(
                "/subscriptions/{}/databases/{}/backup",
                subscription_id, database_id
            ))
            .await
    }
}
