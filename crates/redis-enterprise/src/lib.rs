//! Redis Enterprise REST API client
//!
//! This module provides a client for interacting with Redis Enterprise's REST API,
//! enabling cluster management, database operations, and monitoring.
//!
//! # Examples
//!
//! ## Creating a Client
//!
//! ```no_run
//! use redis_enterprise::EnterpriseClient;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = EnterpriseClient::builder()
//!     .base_url("https://cluster.example.com:9443")
//!     .username("admin@example.com")
//!     .password("password")
//!     .insecure(false)
//!     .build()?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Working with Databases
//!
//! ```no_run
//! use redis_enterprise::{EnterpriseClient, BdbHandler, CreateDatabaseRequestBuilder};
//!
//! # async fn example(client: EnterpriseClient) -> Result<(), Box<dyn std::error::Error>> {
//! // List all databases
//! let handler = BdbHandler::new(client.clone());
//! let databases = handler.list().await?;
//! for db in databases {
//!     println!("Database: {} ({})", db.name, db.uid);
//! }
//!
//! // Create a new database
//! let request = CreateDatabaseRequestBuilder::new()
//!     .name("my-database")
//!     .memory_size(1024 * 1024 * 1024) // 1GB
//!     .port(12000)
//!     .replication(false)
//!     .persistence("aof")
//!     .build()?;
//!
//! let new_db = handler.create(request).await?;
//! println!("Created database: {}", new_db.uid);
//!
//! // Get database stats
//! let stats = handler.stats(new_db.uid).await?;
//! println!("Ops/sec: {:?}", stats);
//! # Ok(())
//! # }
//! ```
//!
//! ## Managing Nodes
//!
//! ```no_run
//! use redis_enterprise::{EnterpriseClient, NodeHandler};
//!
//! # async fn example(client: EnterpriseClient) -> Result<(), Box<dyn std::error::Error>> {
//! let handler = NodeHandler::new(client);
//!
//! // List all nodes in the cluster
//! let nodes = handler.list().await?;
//! for node in nodes {
//!     println!("Node {}: {:?} ({})", node.uid, node.addr, node.status);
//! }
//!
//! // Get detailed node information
//! let node_info = handler.get(1).await?;
//! println!("Node memory: {:?} bytes", node_info.total_memory);
//!
//! // Check node stats
//! let stats = handler.stats(1).await?;
//! println!("CPU usage: {:?}", stats);
//! # Ok(())
//! # }
//! ```
//!
//! ## Cluster Operations
//!
//! ```no_run
//! use redis_enterprise::{EnterpriseClient, ClusterHandler};
//!
//! # async fn example(client: EnterpriseClient) -> Result<(), Box<dyn std::error::Error>> {
//! let handler = ClusterHandler::new(client);
//!
//! // Get cluster information
//! let cluster_info = handler.info().await?;
//! println!("Cluster name: {}", cluster_info.name);
//! println!("Nodes: {:?}", cluster_info.nodes);
//!
//! // Get cluster statistics
//! let stats = handler.stats().await?;
//! println!("Total memory: {:?}", stats);
//!
//! // Check license status
//! let license = handler.license().await?;
//! println!("License expires: {:?}", license);
//! # Ok(())
//! # }
//! ```
//!
//! ## User Management
//!
//! ```no_run
//! use redis_enterprise::{EnterpriseClient, UserHandler, CreateUserRequest};
//!
//! # async fn example(client: EnterpriseClient) -> Result<(), Box<dyn std::error::Error>> {
//! let handler = UserHandler::new(client);
//!
//! // List all users
//! let users = handler.list().await?;
//! for user in users {
//!     println!("User: {} ({})", user.username, user.role);
//! }
//!
//! // Create a new user
//! let request = CreateUserRequest {
//!     username: "newuser".to_string(),
//!     password: "secure_password".to_string(),
//!     role: "db_member".to_string(),
//!     email: Some("newuser@example.com".to_string()),
//!     email_alerts: Some(true),
//! };
//!
//! let new_user = handler.create(request).await?;
//! println!("Created user: {}", new_user.uid);
//! # Ok(())
//! # }
//! ```
//!
//! ## Monitoring and Alerts
//!
//! ```no_run
//! use redis_enterprise::{EnterpriseClient, AlertHandler, StatsHandler};
//!
//! # async fn example(client: EnterpriseClient) -> Result<(), Box<dyn std::error::Error>> {
//! // Check active alerts
//! let alert_handler = AlertHandler::new(client.clone());
//! let alerts = alert_handler.list().await?;
//! for alert in alerts {
//!     println!("Alert: {} - {}", alert.name, alert.severity);
//! }
//!
//! // Get cluster statistics
//! let stats_handler = StatsHandler::new(client);
//! let cluster_stats = stats_handler.cluster(None).await?;
//! println!("Cluster stats: {:?}", cluster_stats);
//! # Ok(())
//! # }
//! ```
//!
//! ## Active-Active Databases (CRDB)
//!
//! ```no_run
//! use redis_enterprise::{EnterpriseClient, CrdbHandler, CreateCrdbRequest};
//!
//! # async fn example(client: EnterpriseClient) -> Result<(), Box<dyn std::error::Error>> {
//! let handler = CrdbHandler::new(client);
//!
//! // List Active-Active databases
//! let crdbs = handler.list().await?;
//! for crdb in crdbs {
//!     println!("CRDB: {} ({})", crdb.name, crdb.guid);
//! }
//!
//! // Create an Active-Active database
//! let request = CreateCrdbRequest {
//!     name: "global-cache".to_string(),
//!     memory_size: 1024 * 1024 * 1024, // 1GB per instance
//!     instances: vec![
//!         // Define instances for each participating cluster
//!     ],
//!     encryption: Some(false),
//!     data_persistence: Some("aof".to_string()),
//!     eviction_policy: Some("allkeys-lru".to_string()),
//! };
//!
//! let new_crdb = handler.create(request).await?;
//! println!("Created CRDB: {}", new_crdb.guid);
//! # Ok(())
//! # }
//! ```

pub mod actions;
pub mod alerts;
pub mod bdb;
pub mod bootstrap;
pub mod client;
pub mod cluster;
pub mod cm_settings;
pub mod crdb;
pub mod crdb_tasks;
pub mod debuginfo;
pub mod diagnostics;
pub mod endpoints;
pub mod error;
pub mod job_scheduler;
pub mod jsonschema;
pub mod ldap_mappings;
pub mod license;
pub mod logs;
pub mod migrations;
pub mod modules;
pub mod nodes;
pub mod ocsp;
pub mod proxies;
pub mod redis_acls;
pub mod roles;
pub mod services;
pub mod shards;
pub mod stats;
pub mod suffixes;
pub mod types;
pub mod usage_report;
pub mod users;

#[cfg(test)]
mod lib_tests;

// Core client and error types
pub use client::{EnterpriseClient, EnterpriseClientBuilder};
pub use error::{RestError, Result};

// Database management
pub use bdb::{
    BdbHandler, CreateDatabaseRequest, CreateDatabaseRequestBuilder, Database, ModuleConfig,
};

// Cluster management
pub use cluster::{
    BootstrapRequest, ClusterHandler, ClusterInfo, ClusterNode, LicenseInfo, NodeInfo,
};

// Node management
pub use nodes::{Node, NodeActionRequest, NodeHandler, NodeStats};

// User management
pub use users::{CreateUserRequest, Role, RoleHandler, UpdateUserRequest, User, UserHandler};

// Module management
pub use modules::{Module, ModuleHandler, UploadModuleRequest};

// Action tracking
pub use actions::{Action, ActionHandler};

// Logs
pub use logs::{LogEntry, LogsHandler, LogsQuery};

// Active-Active databases
pub use crdb::{Crdb, CrdbHandler, CrdbInstance, CreateCrdbInstance, CreateCrdbRequest};

// Statistics
pub use stats::{StatsHandler, StatsInterval, StatsQuery, StatsResponse};

// Alerts
pub use alerts::{Alert, AlertHandler, AlertSettings};

// Redis ACLs
pub use redis_acls::{CreateRedisAclRequest, RedisAcl, RedisAclHandler};

// Shards
pub use shards::{Shard, ShardHandler, ShardStats};

// Proxies
pub use proxies::{Proxy, ProxyHandler, ProxyStats};

// LDAP mappings
pub use ldap_mappings::{
    CreateLdapMappingRequest, LdapConfig, LdapMapping, LdapMappingHandler, LdapServer,
};

// OCSP
pub use ocsp::{OcspConfig, OcspHandler, OcspStatus, OcspTestResult};

// Bootstrap
pub use bootstrap::{
    BootstrapConfig, BootstrapHandler, BootstrapStatus, ClusterBootstrap, CredentialsBootstrap,
    NodeBootstrap, NodePaths,
};

// Cluster Manager settings
pub use cm_settings::{CmSettings, CmSettingsHandler};

// CRDB tasks
pub use crdb_tasks::{CrdbTask, CrdbTasksHandler, CreateCrdbTaskRequest};

// Debug info
pub use debuginfo::{DebugInfoHandler, DebugInfoRequest, DebugInfoStatus, TimeRange};

// Diagnostics
pub use diagnostics::{
    DiagnosticReport, DiagnosticRequest, DiagnosticResult, DiagnosticSummary, DiagnosticsHandler,
};

// Endpoints
pub use endpoints::{Endpoint, EndpointStats, EndpointsHandler};

// Job scheduler
pub use job_scheduler::{
    CreateScheduledJobRequest, JobExecution, JobSchedulerHandler, ScheduledJob,
};

// JSON Schema
pub use jsonschema::JsonSchemaHandler;

// License
pub use license::{License, LicenseHandler, LicenseUpdateRequest, LicenseUsage};

// Migrations
pub use migrations::{CreateMigrationRequest, Migration, MigrationEndpoint, MigrationsHandler};

// Roles
pub use roles::{BdbRole, CreateRoleRequest, RoleInfo, RolesHandler};

// Services
pub use services::{
    NodeServiceStatus, Service, ServiceConfigRequest, ServiceStatus, ServicesHandler,
};

// Suffixes
pub use suffixes::{CreateSuffixRequest, Suffix, SuffixesHandler};

// Usage report
pub use usage_report::{
    DatabaseUsage, NodeUsage, UsageReport, UsageReportConfig, UsageReportHandler, UsageSummary,
};
