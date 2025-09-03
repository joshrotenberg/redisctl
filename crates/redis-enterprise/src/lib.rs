//! Redis Enterprise REST API client
//!
//! A comprehensive Rust client library for the Redis Enterprise REST API, providing
//! full cluster management, database operations, security configuration, and monitoring
//! capabilities. This crate offers both typed and untyped API access with comprehensive
//! coverage of all Enterprise REST endpoints.
//!
//! # Features
//!
//! - **Complete API Coverage**: Full coverage of v1 endpoints plus select v2
//!   endpoints where they exist (e.g., actions, modules)
//! - **Type-Safe Operations**: Strongly typed request/response models
//! - **Flexible Authentication**: Basic auth with optional SSL verification
//! - **Async/Await Support**: Built on Tokio for high-performance async operations
//! - **Error Handling**: Comprehensive error types with context
//! - **Builder Patterns**: Ergonomic API for complex request construction
//! - **Versioned APIs**: Clear accessors for v1 and v2 where both exist
//!
//! # Quick Start
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! redis-enterprise = "0.2.0"
//! tokio = { version = "1", features = ["full"] }
//! ```
//!
//! # Environment Variables
//!
//! The client supports configuration via environment variables:
//! - `REDIS_ENTERPRISE_URL`: Base URL for the cluster (e.g., `https://cluster:9443`)
//! - `REDIS_ENTERPRISE_USER`: Username for authentication
//! - `REDIS_ENTERPRISE_PASSWORD`: Password for authentication
//! - `REDIS_ENTERPRISE_INSECURE`: Set to `true` to skip SSL verification (dev only)
//!
//! # Module Organization
//!
//! The library is organized into domain-specific modules:
//!
//! - **Core Resources**: [`bdb`], [`cluster`], [`nodes`], [`users`], [`roles`]
//! - **Monitoring**: [`stats`], [`alerts`], [`logs`], [`diagnostics`]
//! - **Advanced**: [`crdb`], [`shards`], [`proxies`], [`redis_acls`]
//! - **Configuration**: [`services`], [`cm_settings`], [`suffixes`]
//!
//! # Return Types
//!
//! Most methods return strongly-typed structs. Methods returning `serde_json::Value`:
//! - **Stats/Metrics**: Dynamic keys based on metric names
//! - **Legacy endpoints**: `start()`, `stop()` - use typed action methods instead
//! - **Variable schemas**: Endpoints with version-dependent responses
//! - **Passthrough access**: For direct API access via the CLI
//!
//! # Examples
//!
//! ## Creating a Client
//!
//! ```no_run
//! use redis_enterprise::EnterpriseClient;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // From environment variables
//! let client = EnterpriseClient::from_env()?;
//!
//! // Or using the builder
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
//! use redis_enterprise::{EnterpriseClient, BdbHandler, CreateDatabaseRequest};
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
//! let request = CreateDatabaseRequest::builder()
//!     .name("my-database")
//!     .memory_size(1024 * 1024 * 1024) // 1GB
//!     .port(12000)
//!     .replication(false)
//!     .persistence("aof")
//!     .build();
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
//! ## Versioned Endpoints (v1/v2)
//!
//! Some Enterprise endpoints have both v1 and v2 flavors. Use versioned sub-handlers
//! to make intent explicit and keep models clean.
//!
//! ```no_run
//! use redis_enterprise::{EnterpriseClient, ActionHandler, ModuleHandler};
//!
//! # async fn example(client: EnterpriseClient) -> Result<(), Box<dyn std::error::Error>> {
//! // Actions: v1 and v2
//! let actions = ActionHandler::new(client.clone());
//! let v1_actions = actions.v1().list().await?; // GET /v1/actions
//! let v2_actions = actions.v2().list().await?; // GET /v2/actions
//!
//! // Modules: v1 and v2
//! let modules = ModuleHandler::new(client.clone());
//! let all = modules.v1().list().await?;            // GET /v1/modules
//! // v2 upload body can be an opaque Value or multipart in a higher-level helper
//! let uploaded = modules.v2().upload(serde_json::json!({"name": "search", "data": "..."})).await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Typed-Only Client
//!
//! The Enterprise client exposes typed request/response APIs by default. Raw JSON
//! passthroughs are avoided except for intentionally opaque operations (for example,
//! database command passthrough or module uploads). This keeps CLI and library usage
//! consistent and safer while maintaining flexibility where the wire format is open.
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
//! let request = CreateUserRequest::builder()
//!     .email("newuser@example.com")
//!     .password("secure_password")
//!     .role("db_member")
//!     .name("New User")
//!     .email_alerts(true)
//!     .build();
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
//!
//! # Error Handling
//!
//! The library provides detailed error information for all API operations with
//! convenient error variants and helper methods:
//!
//! ```no_run
//! use redis_enterprise::{EnterpriseClient, bdb::DatabaseHandler, RestError};
//!
//! # async fn example(client: EnterpriseClient) -> Result<(), Box<dyn std::error::Error>> {
//! let handler = DatabaseHandler::new(client);
//!
//! match handler.get(999).await {
//!     Ok(db) => println!("Found database: {}", db.name),
//!     Err(RestError::NotFound) => println!("Database not found"),
//!     Err(RestError::Unauthorized) => println!("Invalid credentials"),
//!     Err(RestError::ServerError(msg)) => println!("Server error: {}", msg),
//!     Err(e) if e.is_not_found() => println!("Not found: {}", e),
//!     Err(e) => println!("Unexpected error: {}", e),
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Handler Overview
//!
//! The library exposes focused handlers per API domain to keep code organized and discoverable:
//!
//! | Handler | Purpose | Key Operations |
//! |---------|---------|----------------|
//! | `ClusterHandler` | Cluster lifecycle | info, update, license, services |
//! | `BdbHandler` | Databases (BDB) | list, get, create, update, delete, stats |
//! | `NodeHandler` | Node management | list, get, stats |
//! | `UserHandler` | Users | list, get, create, update, delete |
//! | `RoleHandler` | Roles | list, get, assign |
//! | `ModuleHandler` | Modules | list (v1), upload (v2) |
//! | `AlertHandler` | Alerts | list, acknowledge |
//! | `StatsHandler` | Monitoring | cluster/node/database stats |
//! | `LogsHandler` | Logs | cluster and database logs |
//! | `ActionHandler` | Actions | v1/v2 workflows |
//!
//! # Production Best Practices
//!
//! - **Connection Pooling**: The client reuses HTTP connections automatically
//! - **Timeout Configuration**: Set appropriate timeouts for your environment
//! - **SSL Verification**: Always enable in production (disable only for development)
//! - **Error Handling**: Implement retry logic for transient failures
//! - **Monitoring**: Log all API operations and track response times
//!
//! # API Coverage
//!
//! This crate provides complete coverage of the Redis Enterprise REST API:
//!
//! - **Cluster Management**: Bootstrap, configuration, licenses
//! - **Database Operations**: CRUD, backup/restore, configuration
//! - **Security**: Users, roles, ACLs, LDAP integration
//! - **Monitoring**: Stats, alerts, logs, diagnostics
//! - **High Availability**: Active-Active (CRDB), replication
//! - **Modules**: Redis module management
//! - **Maintenance**: Upgrades, migrations, debug info

pub mod actions;
pub mod alerts;
pub mod bdb;
pub mod bdb_groups;
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
pub mod local;
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

// Database groups
pub use bdb_groups::{BdbGroup, BdbGroupsHandler};

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

// Local endpoints
pub use local::LocalHandler;

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
