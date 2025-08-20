use clap::{Parser, Subcommand};
use redis_common::{DeploymentType, OutputFormat};

#[derive(Parser)]
#[command(name = "redisctl")]
#[command(about = "Unified Redis CLI for Cloud and Enterprise")]
#[command(version)]
pub struct Cli {
    /// Output format
    #[arg(short, long, value_enum, default_value = "json")]
    pub output: OutputFormat,

    /// JMESPath query to filter output
    #[arg(short, long)]
    pub query: Option<String>,

    /// Profile name to use (overrides REDISCTL_PROFILE env var)
    #[arg(short, long)]
    pub profile: Option<String>,

    /// Deployment type (auto-detected from profile if not specified)
    #[arg(short, long, value_enum)]
    pub deployment: Option<DeploymentType>,

    /// Verbose logging
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Redis Cloud commands
    Cloud {
        #[command(subcommand)]
        command: CloudCommands,
    },
    /// Redis Enterprise commands
    Enterprise {
        #[command(subcommand)]
        command: EnterpriseCommands,
    },
    /// Profile management
    Profile {
        #[command(subcommand)]
        command: ProfileCommands,
    },
    /// Database operations (smart routing)
    Database {
        #[command(subcommand)]
        command: DatabaseCommands,
    },
    /// Cluster operations (smart routing)
    Cluster {
        #[command(subcommand)]
        command: ClusterCommands,
    },
    /// User operations (smart routing)
    User {
        #[command(subcommand)]
        command: UserCommands,
    },
    /// Account operations (smart routing to Cloud subscriptions)
    Account {
        #[command(subcommand)]
        command: AccountCommands,
    },
}

#[derive(Subcommand)]
pub enum CloudCommands {
    /// Subscription management
    Subscription {
        #[command(subcommand)]
        command: SubscriptionCommands,
    },
    /// Database management
    Database {
        #[command(subcommand)]
        command: DatabaseCommands,
    },
    /// Account management
    Account {
        #[command(subcommand)]
        command: AccountCommands,
    },
    /// User management
    User {
        #[command(subcommand)]
        command: UserCommands,
    },
    /// Region information
    Region {
        #[command(subcommand)]
        command: RegionCommands,
    },
    /// Task monitoring
    Task {
        #[command(subcommand)]
        command: TaskCommands,
    },
    /// ACL management
    Acl {
        #[command(subcommand)]
        command: AclCommands,
    },
}

#[derive(Subcommand)]
pub enum EnterpriseCommands {
    /// Cluster management
    Cluster {
        #[command(subcommand)]
        command: ClusterCommands,
    },
    /// Database management
    Database {
        #[command(subcommand)]
        command: DatabaseCommands,
    },
    /// Node management
    Node {
        #[command(subcommand)]
        command: NodeCommands,
    },
    /// User management
    User {
        #[command(subcommand)]
        command: UserCommands,
    },
    /// Bootstrap operations
    Bootstrap {
        #[command(subcommand)]
        command: BootstrapCommands,
    },
    /// Module management
    Module {
        #[command(subcommand)]
        command: ModuleCommands,
    },
    /// Role management
    Role {
        #[command(subcommand)]
        command: RoleCommands,
    },
    /// License management
    License {
        #[command(subcommand)]
        command: LicenseCommands,
    },
}

#[derive(Subcommand)]
pub enum ProfileCommands {
    /// List all profiles
    List,
    /// Show profile details
    Show {
        /// Profile name (defaults to current profile)
        name: Option<String>,
    },
    /// Create or update a profile
    Set {
        /// Profile name
        name: String,
        /// Deployment type
        #[arg(value_enum)]
        deployment_type: DeploymentType,
        /// Connection URL (Enterprise) or API URL (Cloud)
        #[arg(long)]
        url: Option<String>,
        /// Username (Enterprise only)
        #[arg(long)]
        username: Option<String>,
        /// Password (Enterprise only)
        #[arg(long)]
        password: Option<String>,
        /// API Key (Cloud only)
        #[arg(long)]
        api_key: Option<String>,
        /// API Secret (Cloud only)
        #[arg(long)]
        api_secret: Option<String>,
        /// Allow insecure TLS (Enterprise only)
        #[arg(long)]
        insecure: bool,
    },
    /// Remove a profile
    Remove {
        /// Profile name
        name: String,
    },
    /// Set default profile
    Default {
        /// Profile name
        name: String,
    },
}

#[derive(Subcommand, Clone)]
pub enum DatabaseCommands {
    /// List databases
    List,
    /// Show database details
    Show {
        /// Database ID
        id: String,
    },
    /// Create database
    Create {
        /// Database name
        name: String,
        /// Memory limit (MB)
        #[arg(long)]
        memory_limit: Option<u64>,
        /// Redis modules
        #[arg(long)]
        modules: Vec<String>,
    },
    /// Update database
    Update {
        /// Database ID
        id: String,
        /// New name
        #[arg(long)]
        name: Option<String>,
        /// Memory limit (MB)
        #[arg(long)]
        memory_limit: Option<u64>,
    },
    /// Delete database
    Delete {
        /// Database ID
        id: String,
        /// Skip confirmation
        #[arg(long)]
        force: bool,
    },
    /// Backup database
    Backup {
        /// Database ID
        id: String,
    },
    /// Import data
    Import {
        /// Database ID
        id: String,
        /// Import URL
        url: String,
    },
}

#[derive(Subcommand, Clone)]
pub enum ClusterCommands {
    /// Show cluster information
    Info,
    /// List cluster nodes
    Nodes,
    /// Show cluster settings
    Settings,
    /// Update cluster settings
    Update {
        /// Setting name
        name: String,
        /// Setting value
        value: String,
    },
}

#[derive(Subcommand, Clone)]
pub enum UserCommands {
    /// List users
    List,
    /// Show user details
    Show {
        /// User ID
        id: String,
    },
    /// Create user
    Create {
        /// Username
        name: String,
        /// Email
        #[arg(long)]
        email: Option<String>,
        /// Password
        #[arg(long)]
        password: Option<String>,
        /// Roles
        #[arg(long)]
        roles: Vec<String>,
    },
    /// Update user
    Update {
        /// User ID
        id: String,
        /// New email
        #[arg(long)]
        email: Option<String>,
        /// New password
        #[arg(long)]
        password: Option<String>,
    },
    /// Delete user
    Delete {
        /// User ID
        id: String,
        /// Skip confirmation
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand, Clone)]
pub enum AccountCommands {
    /// List accounts/subscriptions
    List,
    /// Show account/subscription details
    Show {
        /// Account/Subscription ID
        id: String,
    },
}

#[derive(Subcommand)]
pub enum SubscriptionCommands {
    /// List subscriptions
    List,
    /// Show subscription details
    Show {
        /// Subscription ID
        id: String,
    },
    /// Create subscription
    Create {
        /// Subscription name
        name: String,
        /// Cloud provider
        #[arg(long)]
        provider: String,
        /// Region
        #[arg(long)]
        region: String,
    },
    /// Update subscription
    Update {
        /// Subscription ID
        id: String,
        /// New name
        #[arg(long)]
        name: Option<String>,
    },
    /// Delete subscription
    Delete {
        /// Subscription ID
        id: String,
        /// Skip confirmation
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand)]
pub enum NodeCommands {
    /// List nodes
    List,
    /// Show node details
    Show {
        /// Node ID
        id: String,
    },
    /// Update node
    Update {
        /// Node ID
        id: String,
        /// External address
        #[arg(long)]
        external_addr: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum RegionCommands {
    /// List available regions
    List,
}

#[derive(Subcommand)]
pub enum TaskCommands {
    /// List tasks
    List,
    /// Show task details
    Show {
        /// Task ID
        id: String,
    },
}

#[derive(Subcommand)]
pub enum AclCommands {
    /// List ACL rules
    List,
    /// Show ACL rule details
    Show {
        /// ACL rule ID
        id: String,
    },
}

#[derive(Subcommand)]
pub enum BootstrapCommands {
    /// Create initial cluster setup
    Create {
        /// License key
        #[arg(long)]
        license: String,
        /// Admin email
        #[arg(long)]
        email: String,
        /// Admin password
        #[arg(long)]
        password: String,
    },
}

#[derive(Subcommand)]
pub enum ModuleCommands {
    /// List modules
    List,
    /// Show module details
    Show {
        /// Module ID
        id: String,
    },
    /// Upload module
    Upload {
        /// Module file path
        path: String,
    },
}

#[derive(Subcommand)]
pub enum RoleCommands {
    /// List roles
    List,
    /// Show role details
    Show {
        /// Role ID
        id: String,
    },
    /// Create role
    Create {
        /// Role name
        name: String,
        /// Permissions
        #[arg(long)]
        permissions: Vec<String>,
    },
    /// Update role
    Update {
        /// Role ID
        id: String,
        /// New permissions
        #[arg(long)]
        permissions: Vec<String>,
    },
    /// Delete role
    Delete {
        /// Role ID
        id: String,
        /// Skip confirmation
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand)]
pub enum LicenseCommands {
    /// Show license information
    Info,
    /// Update license
    Update {
        /// License key
        key: String,
    },
}
