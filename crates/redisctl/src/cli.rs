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

    /// User operations
    #[command(subcommand)]
    User(EnterpriseUserCommands),
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
pub enum EnterpriseClusterCommands {
    Info,
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
pub enum EnterpriseUserCommands {
    List,
}
