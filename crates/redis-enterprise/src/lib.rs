//! Redis Enterprise REST API client
//!
//! This module provides a client for interacting with Redis Enterprise's REST API,
//! enabling cluster management, database operations, and monitoring.

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
pub mod usage_report;
pub mod users;

#[cfg(test)]
mod lib_tests;

// Core client and error types
pub use client::{EnterpriseClient, EnterpriseClientBuilder, EnterpriseConfig};
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
