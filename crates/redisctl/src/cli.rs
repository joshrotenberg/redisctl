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
    List,
}

#[derive(Subcommand, Debug)]
pub enum EnterpriseUserCommands {
    List,
}
