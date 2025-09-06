//! CLI structure and command definitions
//!
//! Defines the command-line interface using clap with a three-layer architecture:
//! 1. Raw API access (`api` commands)
//! 2. Human-friendly interface (`cloud`/`enterprise` commands)
//! 3. Workflow orchestration (`workflow` commands - future)

use crate::config::DeploymentType;
use clap::{Parser, Subcommand};

/// Redis management CLI with unified access to Cloud and Enterprise
#[derive(Parser, Debug)]
#[command(name = "redisctl")]
#[command(version, about = "Redis management CLI for Cloud and Enterprise deployments", long_about = None)]
pub struct Cli {
    /// Profile to use for this command
    #[arg(long, short, global = true, env = "REDISCTL_PROFILE")]
    pub profile: Option<String>,

    /// Output format
    #[arg(long, short = 'o', global = true, value_enum, default_value = "auto")]
    pub output: OutputFormat,

    /// JMESPath query to filter output
    #[arg(long, short = 'q', global = true)]
    pub query: Option<String>,

    /// Enable verbose logging
    #[arg(long, short, global = true, action = clap::ArgAction::Count)]
    pub verbose: u8,

    #[command(subcommand)]
    pub command: Commands,
}

/// Output format options
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum OutputFormat {
    /// Automatically choose format based on command and context
    Auto,
    /// JSON output
    Json,
    /// YAML output
    Yaml,
    /// Human-readable table format
    Table,
}

/// Top-level commands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Raw API access - direct REST endpoint calls
    #[command(name = "api")]
    Api {
        /// Deployment type to target
        #[arg(value_enum)]
        deployment: DeploymentType,

        /// HTTP method
        #[arg(value_parser = parse_http_method)]
        method: HttpMethod,

        /// API endpoint path (e.g., /subscriptions)
        path: String,

        /// Request body (JSON string or @file)
        #[arg(long)]
        data: Option<String>,
    },

    /// Profile management
    #[command(subcommand, visible_alias = "prof", visible_alias = "pr")]
    Profile(ProfileCommands),

    /// Cloud-specific operations
    #[command(subcommand, visible_alias = "cl")]
    Cloud(CloudCommands),

    /// Enterprise-specific operations
    #[command(subcommand, visible_alias = "ent", visible_alias = "en")]
    Enterprise(EnterpriseCommands),

    /// Version information
    #[command(visible_alias = "ver", visible_alias = "v")]
    Version,
}

/// HTTP methods for raw API access
#[derive(Debug, Clone)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

/// Parse HTTP method case-insensitively
fn parse_http_method(s: &str) -> Result<HttpMethod, String> {
    match s.to_lowercase().as_str() {
        "get" => Ok(HttpMethod::Get),
        "post" => Ok(HttpMethod::Post),
        "put" => Ok(HttpMethod::Put),
        "patch" => Ok(HttpMethod::Patch),
        "delete" => Ok(HttpMethod::Delete),
        _ => Err(format!(
            "invalid HTTP method: {} (valid: get, post, put, patch, delete)",
            s
        )),
    }
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethod::Get => write!(f, "GET"),
            HttpMethod::Post => write!(f, "POST"),
            HttpMethod::Put => write!(f, "PUT"),
            HttpMethod::Patch => write!(f, "PATCH"),
            HttpMethod::Delete => write!(f, "DELETE"),
        }
    }
}

/// Profile management commands
#[derive(Subcommand, Debug)]
pub enum ProfileCommands {
    /// List all configured profiles
    #[command(visible_alias = "ls", visible_alias = "l")]
    List,

    /// Show details of a specific profile
    #[command(visible_alias = "sh", visible_alias = "get")]
    Show {
        /// Profile name to show
        name: String,
    },

    /// Set or create a profile
    #[command(visible_alias = "add", visible_alias = "create")]
    Set {
        /// Profile name
        name: String,

        /// Deployment type
        #[arg(long, value_enum)]
        deployment: DeploymentType,

        /// API key (for Cloud profiles)
        #[arg(long, required_if_eq("deployment", "cloud"))]
        api_key: Option<String>,

        /// API secret (for Cloud profiles)
        #[arg(long, required_if_eq("deployment", "cloud"))]
        api_secret: Option<String>,

        /// API URL (for Cloud profiles)
        #[arg(long, default_value = "https://api.redislabs.com/v1")]
        api_url: String,

        /// Enterprise URL (for Enterprise profiles)
        #[arg(long, required_if_eq("deployment", "enterprise"))]
        url: Option<String>,

        /// Username (for Enterprise profiles)
        #[arg(long, required_if_eq("deployment", "enterprise"))]
        username: Option<String>,

        /// Password (for Enterprise profiles)
        #[arg(long)]
        password: Option<String>,

        /// Allow insecure connections (for Enterprise profiles)
        #[arg(long)]
        insecure: bool,
    },

    /// Remove a profile
    #[command(visible_alias = "rm", visible_alias = "del", visible_alias = "delete")]
    Remove {
        /// Profile name to remove
        name: String,
    },

    /// Set the default profile
    #[command(visible_alias = "def")]
    Default {
        /// Profile name to set as default
        name: String,
    },
}

/// Cloud-specific commands (placeholder for now)
#[derive(Subcommand, Debug)]
pub enum CloudCommands {
    /// Account operations
    #[command(subcommand)]
    Account(CloudAccountCommands),

    /// Subscription operations
    #[command(subcommand)]
    Subscription(CloudSubscriptionCommands),

    /// Database operations
    #[command(subcommand)]
    Database(CloudDatabaseCommands),

    /// User operations
    #[command(subcommand)]
    User(CloudUserCommands),

    /// ACL (Access Control List) operations
    #[command(subcommand)]
    Acl(CloudAclCommands),
}

/// Enterprise-specific commands (placeholder for now)
#[derive(Subcommand, Debug)]
pub enum EnterpriseCommands {
    /// Cluster operations
    #[command(subcommand)]
    Cluster(EnterpriseClusterCommands),

    /// Database operations
    #[command(subcommand)]
    Database(EnterpriseDatabaseCommands),

    /// Node operations
    #[command(subcommand)]
    Node(EnterpriseNodeCommands),

    /// User operations
    #[command(subcommand)]
    User(EnterpriseUserCommands),

    /// Role operations
    #[command(subcommand)]
    Role(EnterpriseRoleCommands),

    /// ACL operations
    #[command(subcommand)]
    Acl(EnterpriseAclCommands),

    /// LDAP integration
    #[command(subcommand)]
    Ldap(EnterpriseLdapCommands),

    /// Authentication & sessions
    #[command(subcommand)]
    Auth(EnterpriseAuthCommands),

    /// Active-Active database (CRDB) operations
    #[command(subcommand)]
    Crdb(EnterpriseCrdbCommands),
}

// Placeholder command structures - will be expanded in later PRs

#[derive(Subcommand, Debug)]
pub enum CloudAccountCommands {
    /// Get account information
    Get,

    /// Get payment methods configured for the account
    GetPaymentMethods,

    /// List supported regions
    ListRegions {
        /// Filter by cloud provider (aws, gcp, azure)
        #[arg(long)]
        provider: Option<String>,
    },

    /// List supported Redis modules
    ListModules,

    /// Get data persistence options
    GetPersistenceOptions,

    /// Get system logs
    GetSystemLogs {
        /// Maximum number of logs to return
        #[arg(long, default_value = "100")]
        limit: Option<u32>,

        /// Offset for pagination
        #[arg(long, default_value = "0")]
        offset: Option<u32>,
    },

    /// Get session/audit logs
    GetSessionLogs {
        /// Maximum number of logs to return
        #[arg(long, default_value = "100")]
        limit: Option<u32>,

        /// Offset for pagination
        #[arg(long, default_value = "0")]
        offset: Option<u32>,
    },

    /// Get search module scaling factors
    GetSearchScaling,
}

#[derive(Subcommand, Debug)]
pub enum CloudSubscriptionCommands {
    /// List all subscriptions
    List,

    /// Get detailed subscription information
    Get {
        /// Subscription ID
        id: u32,
    },

    /// Create a new subscription
    Create {
        /// Subscription configuration as JSON string or @file.json
        #[arg(long)]
        data: String,
    },

    /// Update subscription configuration
    Update {
        /// Subscription ID
        id: u32,
        /// Update configuration as JSON string or @file.json
        #[arg(long)]
        data: String,
    },

    /// Delete a subscription
    Delete {
        /// Subscription ID
        id: u32,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },

    /// Get available Redis versions
    RedisVersions {
        /// Filter by subscription ID (optional)
        #[arg(long)]
        subscription: Option<u32>,
    },

    /// Get subscription pricing information
    GetPricing {
        /// Subscription ID
        id: u32,
    },

    /// Get CIDR allowlist
    GetCidrAllowlist {
        /// Subscription ID
        id: u32,
    },

    /// Update CIDR allowlist
    UpdateCidrAllowlist {
        /// Subscription ID
        id: u32,
        /// CIDR blocks as JSON array or @file.json
        #[arg(long)]
        cidrs: String,
    },

    /// Get maintenance windows
    GetMaintenanceWindows {
        /// Subscription ID
        id: u32,
    },

    /// Update maintenance windows
    UpdateMaintenanceWindows {
        /// Subscription ID
        id: u32,
        /// Maintenance windows configuration as JSON or @file.json
        #[arg(long)]
        data: String,
    },

    /// List Active-Active regions
    ListAaRegions {
        /// Subscription ID
        id: u32,
    },

    /// Add region to Active-Active subscription
    AddAaRegion {
        /// Subscription ID
        id: u32,
        /// Region configuration as JSON or @file.json
        #[arg(long)]
        data: String,
    },

    /// Delete regions from Active-Active subscription
    DeleteAaRegions {
        /// Subscription ID
        id: u32,
        /// Regions to delete as JSON array or @file.json
        #[arg(long)]
        regions: String,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum CloudDatabaseCommands {
    /// List all databases across subscriptions
    List {
        /// Filter by subscription ID
        #[arg(long)]
        subscription: Option<u32>,
    },

    /// Get detailed database information
    Get {
        /// Database ID (format: subscription_id:database_id for fixed, or just database_id for flexible)
        id: String,
    },

    /// Create a new database
    Create {
        /// Subscription ID
        #[arg(long)]
        subscription: u32,
        /// Database configuration as JSON string or @file.json
        #[arg(long)]
        data: String,
    },

    /// Update database configuration
    Update {
        /// Database ID (format: subscription_id:database_id)
        id: String,
        /// Update configuration as JSON string or @file.json
        #[arg(long)]
        data: String,
    },

    /// Delete a database
    Delete {
        /// Database ID (format: subscription_id:database_id)
        id: String,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },

    /// Get database backup status
    BackupStatus {
        /// Database ID (format: subscription_id:database_id)
        id: String,
    },

    /// Trigger manual database backup
    Backup {
        /// Database ID (format: subscription_id:database_id)
        id: String,
    },

    /// Get database import status
    ImportStatus {
        /// Database ID (format: subscription_id:database_id)
        id: String,
    },

    /// Import data into database
    Import {
        /// Database ID (format: subscription_id:database_id)
        id: String,
        /// Import configuration as JSON string or @file.json
        #[arg(long)]
        data: String,
    },

    /// Get database certificate
    GetCertificate {
        /// Database ID (format: subscription_id:database_id)
        id: String,
    },

    /// Get slow query log
    SlowLog {
        /// Database ID (format: subscription_id:database_id)
        id: String,
        /// Maximum number of entries to return
        #[arg(long, default_value = "100")]
        limit: u32,
        /// Offset for pagination
        #[arg(long, default_value = "0")]
        offset: u32,
    },

    /// List database tags
    ListTags {
        /// Database ID (format: subscription_id:database_id)
        id: String,
    },

    /// Add a tag to database
    AddTag {
        /// Database ID (format: subscription_id:database_id)
        id: String,
        /// Tag key
        #[arg(long)]
        key: String,
        /// Tag value
        #[arg(long)]
        value: String,
    },

    /// Update database tags
    UpdateTags {
        /// Database ID (format: subscription_id:database_id)
        id: String,
        /// Tags as JSON string or @file.json
        #[arg(long)]
        data: String,
    },

    /// Delete a tag from database
    DeleteTag {
        /// Database ID (format: subscription_id:database_id)
        id: String,
        /// Tag key to delete
        #[arg(long)]
        key: String,
    },

    /// Flush Active-Active database
    FlushCrdb {
        /// Database ID (format: subscription_id:database_id)
        id: String,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },

    /// Get Redis version upgrade status
    UpgradeStatus {
        /// Database ID (format: subscription_id:database_id)
        id: String,
    },

    /// Upgrade Redis version
    UpgradeRedis {
        /// Database ID (format: subscription_id:database_id)
        id: String,
        /// Target Redis version
        #[arg(long)]
        version: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum CloudUserCommands {
    /// List all users
    List,

    /// Get detailed user information
    Get {
        /// User ID
        id: u32,
    },
}

#[derive(Subcommand, Debug)]
pub enum CloudAclCommands {
    // Redis ACL Rules
    /// List all Redis ACL rules
    #[command(name = "list-redis-rules")]
    ListRedisRules,

    /// Create a new Redis ACL rule
    #[command(name = "create-redis-rule")]
    CreateRedisRule {
        /// Rule name
        #[arg(long)]
        name: String,
        /// Redis ACL rule (e.g., "+@read")
        #[arg(long)]
        rule: String,
    },

    /// Update an existing Redis ACL rule
    #[command(name = "update-redis-rule")]
    UpdateRedisRule {
        /// Rule ID
        id: i32,
        /// New rule name
        #[arg(long)]
        name: Option<String>,
        /// New Redis ACL rule
        #[arg(long)]
        rule: Option<String>,
    },

    /// Delete a Redis ACL rule
    #[command(name = "delete-redis-rule")]
    DeleteRedisRule {
        /// Rule ID
        id: i32,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },

    // ACL Roles
    /// List all ACL roles
    #[command(name = "list-roles")]
    ListRoles,

    /// Create a new ACL role
    #[command(name = "create-role")]
    CreateRole {
        /// Role name
        #[arg(long)]
        name: String,
        /// Redis rules (JSON array or single rule ID)
        #[arg(long, value_name = "JSON|ID")]
        redis_rules: String,
    },

    /// Update an existing ACL role
    #[command(name = "update-role")]
    UpdateRole {
        /// Role ID
        id: i32,
        /// New role name
        #[arg(long)]
        name: Option<String>,
        /// New Redis rules (JSON array or single rule ID)
        #[arg(long, value_name = "JSON|ID")]
        redis_rules: Option<String>,
    },

    /// Delete an ACL role
    #[command(name = "delete-role")]
    DeleteRole {
        /// Role ID
        id: i32,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },

    // ACL Users
    /// List all ACL users
    #[command(name = "list-acl-users")]
    ListAclUsers,

    /// Get ACL user details
    #[command(name = "get-acl-user")]
    GetAclUser {
        /// ACL user ID
        id: i32,
    },

    /// Create a new ACL user
    #[command(name = "create-acl-user")]
    CreateAclUser {
        /// Username
        #[arg(long)]
        name: String,
        /// Role name
        #[arg(long)]
        role: String,
        /// Password
        #[arg(long)]
        password: String,
    },

    /// Update an ACL user
    #[command(name = "update-acl-user")]
    UpdateAclUser {
        /// ACL user ID
        id: i32,
        /// New username
        #[arg(long)]
        name: Option<String>,
        /// New role name
        #[arg(long)]
        role: Option<String>,
        /// New password
        #[arg(long)]
        password: Option<String>,
    },

    /// Delete an ACL user
    #[command(name = "delete-acl-user")]
    DeleteAclUser {
        /// ACL user ID
        id: i32,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum EnterpriseClusterCommands {
    /// Get cluster configuration
    Get,

    /// Update cluster configuration
    Update {
        /// Cluster configuration data (JSON file or inline)
        #[arg(long, value_name = "FILE|JSON")]
        data: String,
    },

    /// Get cluster policies
    #[command(name = "get-policy")]
    GetPolicy,

    /// Update cluster policies
    #[command(name = "update-policy")]
    UpdatePolicy {
        /// Policy data (JSON file or inline)
        #[arg(long, value_name = "FILE|JSON")]
        data: String,
    },

    /// Get license information
    #[command(name = "get-license")]
    GetLicense,

    /// Update license
    #[command(name = "update-license")]
    UpdateLicense {
        /// License key file or content
        #[arg(long, value_name = "FILE|KEY")]
        license: String,
    },

    /// Bootstrap new cluster
    Bootstrap {
        /// Bootstrap configuration (JSON file or inline)
        #[arg(long, value_name = "FILE|JSON")]
        data: String,
    },

    /// Join node to cluster
    Join {
        /// Join configuration (JSON file or inline)
        #[arg(long, value_name = "FILE|JSON")]
        data: String,
    },

    /// Recover cluster
    Recover {
        /// Recovery configuration (JSON file or inline)
        #[arg(long, value_name = "FILE|JSON")]
        data: String,
    },

    /// Reset cluster (dangerous!)
    Reset {
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },

    /// Get cluster statistics
    Stats,

    /// Get cluster metrics
    Metrics {
        /// Time interval (e.g., "1h", "5m")
        #[arg(long)]
        interval: Option<String>,
    },

    /// Get active alerts
    Alerts,

    /// Get cluster events
    Events {
        /// Maximum number of events to return
        #[arg(long, default_value = "100")]
        limit: Option<u32>,
    },

    /// Get audit log
    #[command(name = "audit-log")]
    AuditLog {
        /// From date (e.g., "2024-01-01")
        #[arg(long)]
        from: Option<String>,
    },

    /// Enable maintenance mode
    #[command(name = "maintenance-mode-enable")]
    MaintenanceModeEnable,

    /// Disable maintenance mode
    #[command(name = "maintenance-mode-disable")]
    MaintenanceModeDisable,

    /// Collect debug information
    #[command(name = "debug-info")]
    DebugInfo,

    /// Check cluster health status
    #[command(name = "check-status")]
    CheckStatus,

    /// Get cluster certificates
    #[command(name = "get-certificates")]
    GetCertificates,

    /// Update certificates
    #[command(name = "update-certificates")]
    UpdateCertificates {
        /// Certificate data (JSON file or inline)
        #[arg(long, value_name = "FILE|JSON")]
        data: String,
    },

    /// Rotate certificates
    #[command(name = "rotate-certificates")]
    RotateCertificates,

    /// Get OCSP configuration
    #[command(name = "get-ocsp")]
    GetOcsp,

    /// Update OCSP configuration
    #[command(name = "update-ocsp")]
    UpdateOcsp {
        /// OCSP configuration data (JSON file or inline)
        #[arg(long, value_name = "FILE|JSON")]
        data: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum EnterpriseDatabaseCommands {
    /// List all databases
    List,

    /// Get database details
    Get {
        /// Database ID
        id: u32,
    },

    /// Create a new database
    Create {
        /// Database configuration as JSON string or @file.json
        #[arg(long)]
        data: String,
        /// Perform a dry run without creating the database
        #[arg(long)]
        dry_run: bool,
    },

    /// Update database configuration
    Update {
        /// Database ID
        id: u32,
        /// Update configuration as JSON string or @file.json
        #[arg(long)]
        data: String,
    },

    /// Delete a database
    Delete {
        /// Database ID
        id: u32,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },

    /// Export database
    Export {
        /// Database ID
        id: u32,
        /// Export configuration as JSON string or @file.json
        #[arg(long)]
        data: String,
    },

    /// Import to database
    Import {
        /// Database ID
        id: u32,
        /// Import configuration as JSON string or @file.json
        #[arg(long)]
        data: String,
    },

    /// Trigger database backup
    Backup {
        /// Database ID
        id: u32,
    },

    /// Restore database
    Restore {
        /// Database ID
        id: u32,
        /// Restore configuration as JSON string or @file.json
        #[arg(long)]
        data: String,
    },

    /// Flush database data
    Flush {
        /// Database ID
        id: u32,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },

    /// Get database shards info
    GetShards {
        /// Database ID
        id: u32,
    },

    /// Update sharding configuration
    UpdateShards {
        /// Database ID
        id: u32,
        /// Shards configuration as JSON string or @file.json
        #[arg(long)]
        data: String,
    },

    /// Get enabled modules
    GetModules {
        /// Database ID
        id: u32,
    },

    /// Update modules configuration
    UpdateModules {
        /// Database ID
        id: u32,
        /// Modules configuration as JSON string or @file.json
        #[arg(long)]
        data: String,
    },

    /// Get ACL configuration
    GetAcl {
        /// Database ID
        id: u32,
    },

    /// Update ACL configuration
    UpdateAcl {
        /// Database ID
        id: u32,
        /// ACL configuration as JSON string or @file.json
        #[arg(long)]
        data: String,
    },

    /// Get database statistics
    Stats {
        /// Database ID
        id: u32,
    },

    /// Get database metrics
    Metrics {
        /// Database ID
        id: u32,
        /// Time interval (e.g., "1h", "24h")
        #[arg(long)]
        interval: Option<String>,
    },

    /// Get slow query log
    Slowlog {
        /// Database ID
        id: u32,
        /// Limit number of entries
        #[arg(long)]
        limit: Option<u32>,
    },

    /// Get connected clients
    ClientList {
        /// Database ID
        id: u32,
    },
}

#[derive(Subcommand, Debug)]
pub enum EnterpriseNodeCommands {
    /// List all nodes in cluster
    List,

    /// Get node details
    Get {
        /// Node ID
        id: u32,
    },

    /// Add node to cluster
    Add {
        /// Node configuration (JSON file or inline)
        #[arg(long, value_name = "FILE|JSON")]
        data: String,
    },

    /// Remove node from cluster
    Remove {
        /// Node ID
        id: u32,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },

    /// Update node configuration
    Update {
        /// Node ID
        id: u32,
        /// Update data (JSON file or inline)
        #[arg(long, value_name = "FILE|JSON")]
        data: String,
    },

    /// Get node status
    Status {
        /// Node ID
        id: u32,
    },

    /// Get node statistics
    Stats {
        /// Node ID
        id: u32,
    },

    /// Get node metrics
    Metrics {
        /// Node ID
        id: u32,
        /// Time interval (e.g., "1h", "5m")
        #[arg(long)]
        interval: Option<String>,
    },

    /// Run health check on node
    Check {
        /// Node ID
        id: u32,
    },

    /// Get node-specific alerts
    Alerts {
        /// Node ID
        id: u32,
    },

    /// Put node in maintenance mode
    #[command(name = "maintenance-enable")]
    MaintenanceEnable {
        /// Node ID
        id: u32,
    },

    /// Remove node from maintenance mode
    #[command(name = "maintenance-disable")]
    MaintenanceDisable {
        /// Node ID
        id: u32,
    },

    /// Rebalance shards on node
    Rebalance {
        /// Node ID
        id: u32,
    },

    /// Drain node before removal
    Drain {
        /// Node ID
        id: u32,
    },

    /// Restart node services
    Restart {
        /// Node ID
        id: u32,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },

    /// Get node configuration
    #[command(name = "get-config")]
    GetConfig {
        /// Node ID
        id: u32,
    },

    /// Update node configuration
    #[command(name = "update-config")]
    UpdateConfig {
        /// Node ID
        id: u32,
        /// Configuration data (JSON file or inline)
        #[arg(long, value_name = "FILE|JSON")]
        data: String,
    },

    /// Get rack awareness configuration
    #[command(name = "get-rack")]
    GetRack {
        /// Node ID
        id: u32,
    },

    /// Set rack ID
    #[command(name = "set-rack")]
    SetRack {
        /// Node ID
        id: u32,
        /// Rack identifier
        #[arg(long)]
        rack: String,
    },

    /// Get node role
    #[command(name = "get-role")]
    GetRole {
        /// Node ID
        id: u32,
    },

    /// Get resource utilization
    Resources {
        /// Node ID
        id: u32,
    },

    /// Get memory usage details
    Memory {
        /// Node ID
        id: u32,
    },

    /// Get CPU usage details
    Cpu {
        /// Node ID
        id: u32,
    },

    /// Get storage usage details
    Storage {
        /// Node ID
        id: u32,
    },

    /// Get network statistics
    Network {
        /// Node ID
        id: u32,
    },
}

#[derive(Subcommand, Debug)]
pub enum EnterpriseUserCommands {
    /// List all users
    List,

    /// Get user details
    Get {
        /// User ID
        id: u32,
    },

    /// Create new user
    Create {
        /// User data (JSON file or inline)
        #[arg(long, value_name = "FILE|JSON")]
        data: String,
    },

    /// Update user
    Update {
        /// User ID
        id: u32,
        /// Update data (JSON file or inline)
        #[arg(long, value_name = "FILE|JSON")]
        data: String,
    },

    /// Delete user
    Delete {
        /// User ID
        id: u32,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },

    /// Reset user password
    #[command(name = "reset-password")]
    ResetPassword {
        /// User ID
        id: u32,
        /// New password (will prompt if not provided)
        #[arg(long)]
        password: Option<String>,
    },

    /// Get user's roles
    #[command(name = "get-roles")]
    GetRoles {
        /// User ID
        #[arg(name = "user-id")]
        user_id: u32,
    },

    /// Assign role to user
    #[command(name = "assign-role")]
    AssignRole {
        /// User ID
        #[arg(name = "user-id")]
        user_id: u32,
        /// Role ID to assign
        #[arg(long)]
        role: u32,
    },

    /// Remove role from user
    #[command(name = "remove-role")]
    RemoveRole {
        /// User ID
        #[arg(name = "user-id")]
        user_id: u32,
        /// Role ID to remove
        #[arg(long)]
        role: u32,
    },
}

#[derive(Subcommand, Debug)]
pub enum EnterpriseRoleCommands {
    /// List all roles
    List,

    /// Get role details
    Get {
        /// Role ID
        id: u32,
    },

    /// Create custom role
    Create {
        /// Role data (JSON file or inline)
        #[arg(long, value_name = "FILE|JSON")]
        data: String,
    },

    /// Update role
    Update {
        /// Role ID
        id: u32,
        /// Update data (JSON file or inline)
        #[arg(long, value_name = "FILE|JSON")]
        data: String,
    },

    /// Delete custom role
    Delete {
        /// Role ID
        id: u32,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },

    /// Get role permissions
    #[command(name = "get-permissions")]
    GetPermissions {
        /// Role ID
        id: u32,
    },

    /// Get users with specific role
    #[command(name = "get-users")]
    GetUsers {
        /// Role ID
        #[arg(name = "role-id")]
        role_id: u32,
    },
}

#[derive(Subcommand, Debug)]
pub enum EnterpriseAclCommands {
    /// List all ACLs
    List,

    /// Get ACL details
    Get {
        /// ACL ID
        id: u32,
    },

    /// Create ACL
    Create {
        /// ACL data (JSON file or inline)
        #[arg(long, value_name = "FILE|JSON")]
        data: String,
    },

    /// Update ACL
    Update {
        /// ACL ID
        id: u32,
        /// Update data (JSON file or inline)
        #[arg(long, value_name = "FILE|JSON")]
        data: String,
    },

    /// Delete ACL
    Delete {
        /// ACL ID
        id: u32,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },

    /// Test ACL permissions
    Test {
        /// User ID
        #[arg(long)]
        user: u32,
        /// Redis command to test
        #[arg(long)]
        command: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum EnterpriseLdapCommands {
    /// Get LDAP configuration
    #[command(name = "get-config")]
    GetConfig,

    /// Update LDAP configuration
    #[command(name = "update-config")]
    UpdateConfig {
        /// LDAP config data (JSON file or inline)
        #[arg(long, value_name = "FILE|JSON")]
        data: String,
    },

    /// Test LDAP connection
    #[command(name = "test-connection")]
    TestConnection,

    /// Sync users from LDAP
    Sync,

    /// Get LDAP role mappings
    #[command(name = "get-mappings")]
    GetMappings,
}

#[derive(Subcommand, Debug)]
pub enum EnterpriseAuthCommands {
    /// Test authentication
    Test {
        /// Username/email to test
        #[arg(long)]
        user: String,
    },

    /// List active sessions
    #[command(name = "session-list")]
    SessionList,

    /// Revoke session
    #[command(name = "session-revoke")]
    SessionRevoke {
        /// Session ID
        #[arg(name = "session-id")]
        session_id: String,
    },

    /// Revoke all user sessions
    #[command(name = "session-revoke-all")]
    SessionRevokeAll {
        /// User ID
        #[arg(long)]
        user: u32,
    },
}

#[derive(Subcommand, Debug)]
pub enum EnterpriseCrdbCommands {
    // CRDB Lifecycle Management
    /// List all Active-Active databases
    List,

    /// Get CRDB details
    Get {
        /// CRDB ID
        id: u32,
    },

    /// Create Active-Active database
    Create {
        /// CRDB configuration as JSON string or @file.json
        #[arg(long)]
        data: String,
    },

    /// Update CRDB configuration
    Update {
        /// CRDB ID
        id: u32,
        /// Update configuration as JSON string or @file.json
        #[arg(long)]
        data: String,
    },

    /// Delete CRDB
    Delete {
        /// CRDB ID
        id: u32,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },

    // Participating Clusters Management
    /// Get participating clusters
    #[command(name = "get-clusters")]
    GetClusters {
        /// CRDB ID
        id: u32,
    },

    /// Add cluster to CRDB
    #[command(name = "add-cluster")]
    AddCluster {
        /// CRDB ID
        id: u32,
        /// Cluster configuration as JSON string or @file.json
        #[arg(long)]
        data: String,
    },

    /// Remove cluster from CRDB
    #[command(name = "remove-cluster")]
    RemoveCluster {
        /// CRDB ID
        id: u32,
        /// Cluster ID to remove
        #[arg(long)]
        cluster: u32,
    },

    /// Update cluster configuration in CRDB
    #[command(name = "update-cluster")]
    UpdateCluster {
        /// CRDB ID
        id: u32,
        /// Cluster ID to update
        #[arg(long)]
        cluster: u32,
        /// Update configuration as JSON string or @file.json
        #[arg(long)]
        data: String,
    },

    // Instance Management
    /// Get all CRDB instances
    #[command(name = "get-instances")]
    GetInstances {
        /// CRDB ID
        id: u32,
    },

    /// Get specific CRDB instance
    #[command(name = "get-instance")]
    GetInstance {
        /// CRDB ID
        #[arg(name = "crdb-id")]
        crdb_id: u32,
        /// Instance ID
        #[arg(long)]
        instance: u32,
    },

    /// Update CRDB instance
    #[command(name = "update-instance")]
    UpdateInstance {
        /// CRDB ID
        #[arg(name = "crdb-id")]
        crdb_id: u32,
        /// Instance ID
        #[arg(long)]
        instance: u32,
        /// Update configuration as JSON string or @file.json
        #[arg(long)]
        data: String,
    },

    /// Flush CRDB instance data
    #[command(name = "flush-instance")]
    FlushInstance {
        /// CRDB ID
        #[arg(name = "crdb-id")]
        crdb_id: u32,
        /// Instance ID
        #[arg(long)]
        instance: u32,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },

    // Replication & Sync
    /// Get replication status
    #[command(name = "get-replication-status")]
    GetReplicationStatus {
        /// CRDB ID
        id: u32,
    },

    /// Get replication lag metrics
    #[command(name = "get-lag")]
    GetLag {
        /// CRDB ID
        id: u32,
    },

    /// Force synchronization
    #[command(name = "force-sync")]
    ForceSync {
        /// CRDB ID
        id: u32,
        /// Source cluster ID
        #[arg(long)]
        source: u32,
    },

    /// Pause replication
    #[command(name = "pause-replication")]
    PauseReplication {
        /// CRDB ID
        id: u32,
    },

    /// Resume replication
    #[command(name = "resume-replication")]
    ResumeReplication {
        /// CRDB ID
        id: u32,
    },

    // Conflict Resolution
    /// Get conflict history
    #[command(name = "get-conflicts")]
    GetConflicts {
        /// CRDB ID
        id: u32,
        /// Maximum number of conflicts to return
        #[arg(long)]
        limit: Option<u32>,
    },

    /// Get conflict resolution policy
    #[command(name = "get-conflict-policy")]
    GetConflictPolicy {
        /// CRDB ID
        id: u32,
    },

    /// Update conflict resolution policy
    #[command(name = "update-conflict-policy")]
    UpdateConflictPolicy {
        /// CRDB ID
        id: u32,
        /// Policy configuration as JSON string or @file.json
        #[arg(long)]
        data: String,
    },

    /// Manually resolve conflict
    #[command(name = "resolve-conflict")]
    ResolveConflict {
        /// CRDB ID
        id: u32,
        /// Conflict ID
        #[arg(long)]
        conflict: String,
        /// Resolution method
        #[arg(long)]
        resolution: String,
    },

    // Tasks & Jobs
    /// Get CRDB tasks
    #[command(name = "get-tasks")]
    GetTasks {
        /// CRDB ID
        id: u32,
    },

    /// Get specific task details
    #[command(name = "get-task")]
    GetTask {
        /// CRDB ID
        #[arg(name = "crdb-id")]
        crdb_id: u32,
        /// Task ID
        #[arg(long)]
        task: String,
    },

    /// Retry failed task
    #[command(name = "retry-task")]
    RetryTask {
        /// CRDB ID
        #[arg(name = "crdb-id")]
        crdb_id: u32,
        /// Task ID
        #[arg(long)]
        task: String,
    },

    /// Cancel running task
    #[command(name = "cancel-task")]
    CancelTask {
        /// CRDB ID
        #[arg(name = "crdb-id")]
        crdb_id: u32,
        /// Task ID
        #[arg(long)]
        task: String,
    },

    // Monitoring & Metrics
    /// Get CRDB statistics
    Stats {
        /// CRDB ID
        id: u32,
    },

    /// Get CRDB metrics
    Metrics {
        /// CRDB ID
        id: u32,
        /// Time interval (e.g., "1h", "24h")
        #[arg(long)]
        interval: Option<String>,
    },

    /// Get connection details per instance
    #[command(name = "get-connections")]
    GetConnections {
        /// CRDB ID
        id: u32,
    },

    /// Get throughput metrics
    #[command(name = "get-throughput")]
    GetThroughput {
        /// CRDB ID
        id: u32,
    },

    /// Run health check
    #[command(name = "health-check")]
    HealthCheck {
        /// CRDB ID
        id: u32,
    },

    // Backup & Recovery
    /// Create CRDB backup
    Backup {
        /// CRDB ID
        id: u32,
        /// Backup configuration as JSON string or @file.json
        #[arg(long)]
        data: String,
    },

    /// Restore CRDB
    Restore {
        /// CRDB ID
        id: u32,
        /// Restore configuration as JSON string or @file.json
        #[arg(long)]
        data: String,
    },

    /// List available backups
    #[command(name = "get-backups")]
    GetBackups {
        /// CRDB ID
        id: u32,
    },

    /// Export CRDB data
    Export {
        /// CRDB ID
        id: u32,
        /// Export configuration as JSON string or @file.json
        #[arg(long)]
        data: String,
    },
}
