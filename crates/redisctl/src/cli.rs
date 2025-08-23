use crate::commands::api::ApiCommands;
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
    /// Raw API access
    Api {
        #[command(subcommand)]
        command: ApiCommands,
    },
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
    /// VPC Peering management
    Peering {
        #[command(subcommand)]
        command: PeeringCommands,
    },
    /// Transit Gateway management
    TransitGateway {
        #[command(subcommand)]
        command: TransitGatewayCommands,
    },
    /// Backup management
    Backup {
        #[command(subcommand)]
        command: BackupCommands,
    },
    /// Active-Active database management
    Crdb {
        #[command(subcommand)]
        command: CrdbCommands,
    },
    /// API Keys management
    ApiKey {
        #[command(subcommand)]
        command: ApiKeyCommands,
    },
    /// Metrics and monitoring
    Metrics {
        #[command(subcommand)]
        command: MetricsCommands,
    },
    /// Logs and audit trails
    Logs {
        #[command(subcommand)]
        command: LogsCommands,
    },
    /// Cloud account management
    CloudAccount {
        #[command(subcommand)]
        command: CloudAccountCommands,
    },
    /// Fixed plan management
    FixedPlan {
        #[command(subcommand)]
        command: FixedPlanCommands,
    },
    /// Flexible plan management
    FlexiblePlan {
        #[command(subcommand)]
        command: FlexiblePlanCommands,
    },
    /// Private Service Connect
    PrivateServiceConnect {
        #[command(subcommand)]
        command: PrivateServiceConnectCommands,
    },
}

#[derive(Subcommand)]
pub enum EnterpriseCommands {
    /// Raw API access
    Api {
        #[command(subcommand)]
        command: ApiCommands,
    },
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
    /// Export data
    Export {
        /// Database ID
        id: String,
        /// Export format (rdb, json)
        #[arg(long, default_value = "rdb")]
        format: String,
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
    /// Show account information (different from show)
    Info,
    /// Show account owner information
    Owner,
    /// List users for this account
    Users,
    /// List payment methods
    PaymentMethods,
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
    /// Get subscription pricing
    Pricing {
        /// Subscription ID
        id: String,
    },
    /// List subscription databases
    Databases {
        /// Subscription ID
        id: String,
    },
    /// Get CIDR whitelist
    CidrList {
        /// Subscription ID
        id: String,
    },
    /// Update CIDR whitelist
    CidrUpdate {
        /// Subscription ID
        id: String,
        /// CIDR blocks (comma-separated)
        #[arg(long)]
        cidrs: String,
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
    /// Add a new node to the cluster
    Add {
        /// Node IP address
        #[arg(long)]
        addr: String,
        /// Node username (default: admin)
        #[arg(long, default_value = "admin")]
        username: String,
        /// Node password
        #[arg(long)]
        password: String,
        /// External address
        #[arg(long)]
        external_addr: Option<String>,
    },
    /// Remove a node from the cluster
    Remove {
        /// Node ID
        id: String,
        /// Skip confirmation
        #[arg(long)]
        force: bool,
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
    /// Wait for task completion
    Wait {
        /// Task ID
        id: String,
        /// Timeout in seconds (default: 300)
        #[arg(long, default_value = "300")]
        timeout: u64,
    },
}

#[derive(Subcommand)]
pub enum AclCommands {
    /// List ACL rules
    List {
        /// Subscription ID
        subscription_id: u32,
        /// Database ID
        database_id: u32,
    },
    /// Show ACL rule details
    Show {
        /// Subscription ID
        subscription_id: u32,
        /// Database ID
        database_id: u32,
        /// ACL rule ID
        acl_id: u32,
    },
    /// Create ACL rule
    Create {
        /// Subscription ID
        subscription_id: u32,
        /// Database ID
        database_id: u32,
        /// ACL name
        name: String,
        /// ACL rule
        #[arg(long)]
        rule: String,
    },
    /// Update ACL rule
    Update {
        /// Subscription ID
        subscription_id: u32,
        /// Database ID
        database_id: u32,
        /// ACL rule ID
        acl_id: u32,
        /// New ACL rule
        #[arg(long)]
        rule: String,
    },
    /// Delete ACL rule
    Delete {
        /// Subscription ID
        subscription_id: u32,
        /// Database ID
        database_id: u32,
        /// ACL rule ID
        acl_id: u32,
        /// Force deletion without confirmation
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand)]
pub enum PeeringCommands {
    /// List VPC peerings
    List {
        /// Subscription ID
        subscription_id: u32,
    },
    /// Show peering details
    Show {
        /// Subscription ID
        subscription_id: u32,
        /// Peering ID
        peering_id: String,
    },
    /// Create VPC peering
    Create {
        /// Subscription ID
        subscription_id: u32,
        /// AWS account ID or GCP project ID
        #[arg(long)]
        provider_account_id: String,
        /// VPC ID (AWS) or network name (GCP)
        #[arg(long)]
        vpc_id: String,
        /// VPC CIDR
        #[arg(long)]
        vpc_cidr: String,
        /// Region
        #[arg(long)]
        region: String,
    },
    /// Delete VPC peering
    Delete {
        /// Subscription ID
        subscription_id: u32,
        /// Peering ID
        peering_id: String,
        /// Force deletion without confirmation
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand)]
pub enum TransitGatewayCommands {
    /// List Transit Gateway attachments
    List {
        /// Subscription ID
        subscription_id: u32,
    },
    /// Show Transit Gateway attachment details
    Show {
        /// Subscription ID
        subscription_id: u32,
        /// Transit Gateway ID
        tgw_id: String,
    },
    /// Create Transit Gateway attachment
    Create {
        /// Subscription ID
        subscription_id: u32,
        /// Transit Gateway ID
        tgw_id: String,
        /// AWS account ID
        #[arg(long)]
        aws_account_id: String,
        /// VPC CIDRs
        #[arg(long)]
        cidrs: Vec<String>,
    },
    /// Delete Transit Gateway attachment
    Delete {
        /// Subscription ID
        subscription_id: u32,
        /// Transit Gateway ID
        tgw_id: String,
        /// Force deletion without confirmation
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand)]
pub enum BackupCommands {
    /// List backups
    List {
        /// Subscription ID
        subscription_id: u32,
        /// Database ID
        database_id: u32,
    },
    /// Show backup details
    Show {
        /// Subscription ID
        subscription_id: u32,
        /// Database ID
        database_id: u32,
        /// Backup ID
        backup_id: u32,
    },
    /// Create backup
    Create {
        /// Subscription ID
        subscription_id: u32,
        /// Database ID
        database_id: u32,
    },
    /// Restore from backup
    Restore {
        /// Subscription ID
        subscription_id: u32,
        /// Database ID
        database_id: u32,
        /// Backup ID
        backup_id: u32,
    },
    /// Delete backup
    Delete {
        /// Subscription ID
        subscription_id: u32,
        /// Database ID
        database_id: u32,
        /// Backup ID
        backup_id: u32,
        /// Force deletion without confirmation
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand)]
pub enum CrdbCommands {
    /// List Active-Active databases
    List,
    /// Show Active-Active database details
    Show {
        /// CRDB ID
        crdb_id: u32,
    },
    /// Create Active-Active database
    Create {
        /// Database name
        name: String,
        /// Memory limit per region (MB)
        #[arg(long)]
        memory_limit: u64,
        /// Participating regions
        #[arg(long)]
        regions: Vec<String>,
    },
    /// Update Active-Active database
    Update {
        /// CRDB ID
        crdb_id: u32,
        /// New name
        #[arg(long)]
        name: Option<String>,
        /// New memory limit (MB)
        #[arg(long)]
        memory_limit: Option<u64>,
    },
    /// Delete Active-Active database
    Delete {
        /// CRDB ID
        crdb_id: u32,
        /// Force deletion without confirmation
        #[arg(long)]
        force: bool,
    },
    /// Add region to Active-Active database
    AddRegion {
        /// CRDB ID
        crdb_id: u32,
        /// Region to add
        region: String,
    },
    /// Remove region from Active-Active database
    RemoveRegion {
        /// CRDB ID
        crdb_id: u32,
        /// Region ID to remove
        region_id: u32,
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

#[derive(Subcommand)]
pub enum ApiKeyCommands {
    /// List API keys
    List,
    /// Show API key details
    Show {
        /// API key ID
        key_id: u32,
    },
    /// Create API key
    Create {
        /// API key name
        name: String,
        /// API key role
        #[arg(long)]
        role: String,
    },
    /// Update API key
    Update {
        /// API key ID
        key_id: u32,
        /// New name
        #[arg(long)]
        name: Option<String>,
        /// New role
        #[arg(long)]
        role: Option<String>,
    },
    /// Delete API key
    Delete {
        /// API key ID
        key_id: u32,
        /// Force deletion without confirmation
        #[arg(long)]
        force: bool,
    },
    /// Regenerate API key
    Regenerate {
        /// API key ID
        key_id: u32,
    },
    /// Enable API key
    Enable {
        /// API key ID
        key_id: u32,
    },
    /// Disable API key
    Disable {
        /// API key ID
        key_id: u32,
    },
}

#[derive(Subcommand)]
pub enum MetricsCommands {
    /// Get database metrics
    Database {
        /// Subscription ID
        subscription_id: u32,
        /// Database ID
        database_id: u32,
        /// Metric name
        #[arg(long)]
        metric: String,
        /// Time period
        #[arg(long, default_value = "1hour")]
        period: String,
    },
    /// Get subscription metrics
    Subscription {
        /// Subscription ID
        subscription_id: u32,
        /// Metric name
        #[arg(long)]
        metric: String,
        /// Time period
        #[arg(long, default_value = "1hour")]
        period: String,
    },
}

#[derive(Subcommand)]
pub enum LogsCommands {
    /// Get database logs
    Database {
        /// Subscription ID
        subscription_id: u32,
        /// Database ID
        database_id: u32,
        /// Log type (slowlog, audit)
        #[arg(long, default_value = "slowlog")]
        log_type: String,
        /// Number of entries to retrieve
        #[arg(long, default_value = "100")]
        limit: u32,
        /// Offset for pagination
        #[arg(long, default_value = "0")]
        offset: u32,
    },
    /// Get system logs
    System {
        /// Number of entries to retrieve
        #[arg(long, default_value = "100")]
        limit: u32,
        /// Offset for pagination
        #[arg(long, default_value = "0")]
        offset: u32,
    },
    /// Get session logs
    Session {
        /// Number of entries to retrieve
        #[arg(long, default_value = "100")]
        limit: u32,
        /// Offset for pagination
        #[arg(long, default_value = "0")]
        offset: u32,
    },
}

#[derive(Subcommand)]
pub enum CloudAccountCommands {
    /// List cloud accounts
    List,
    /// Show cloud account details
    Show {
        /// Cloud account ID
        account_id: String,
    },
    /// Create cloud account
    Create {
        /// Account name
        #[arg(long)]
        name: String,
        /// Provider (AWS, GCP, Azure)
        #[arg(long)]
        provider: String,
        /// Access key ID
        #[arg(long)]
        access_key_id: String,
        /// Secret access key
        #[arg(long)]
        secret_access_key: String,
    },
    /// Update cloud account
    Update {
        /// Cloud account ID
        account_id: String,
        /// Account name
        #[arg(long)]
        name: Option<String>,
        /// Access key ID
        #[arg(long)]
        access_key_id: Option<String>,
        /// Secret access key
        #[arg(long)]
        secret_access_key: Option<String>,
    },
    /// Delete cloud account
    Delete {
        /// Cloud account ID
        account_id: String,
        /// Force deletion without confirmation
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand)]
pub enum FixedPlanCommands {
    /// List fixed plans
    List,
    /// Show fixed plan details
    Show {
        /// Plan ID
        plan_id: u32,
    },
    /// List available plans for region
    Plans {
        /// Region name
        region: String,
    },
}

#[derive(Subcommand)]
pub enum FlexiblePlanCommands {
    /// List flexible plans
    List,
    /// Show flexible plan details
    Show {
        /// Plan ID
        plan_id: u32,
    },
    /// Create flexible plan
    Create {
        /// Plan name
        #[arg(long)]
        name: String,
        /// Memory limit in GB
        #[arg(long)]
        memory_limit_in_gb: f64,
        /// Maximum number of databases
        #[arg(long)]
        maximum_databases: u32,
    },
    /// Update flexible plan
    Update {
        /// Plan ID
        plan_id: u32,
        /// Plan name
        #[arg(long)]
        name: Option<String>,
        /// Memory limit in GB
        #[arg(long)]
        memory_limit_in_gb: Option<f64>,
        /// Maximum number of databases
        #[arg(long)]
        maximum_databases: Option<u32>,
    },
    /// Delete flexible plan
    Delete {
        /// Plan ID
        plan_id: u32,
        /// Force deletion without confirmation
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand)]
pub enum PrivateServiceConnectCommands {
    /// List Private Service Connect endpoints
    List {
        /// Subscription ID
        subscription_id: u32,
    },
    /// Show Private Service Connect endpoint details
    Show {
        /// Subscription ID
        subscription_id: u32,
        /// Endpoint ID
        endpoint_id: u32,
    },
    /// Create Private Service Connect endpoint
    Create {
        /// Subscription ID
        subscription_id: u32,
        /// Service name
        #[arg(long)]
        service_name: String,
        /// Allowed principals (comma-separated)
        #[arg(long)]
        allowed_principals: String,
    },
    /// Update Private Service Connect endpoint
    Update {
        /// Subscription ID
        subscription_id: u32,
        /// Endpoint ID
        endpoint_id: u32,
        /// Service name
        #[arg(long)]
        service_name: Option<String>,
        /// Allowed principals (comma-separated)
        #[arg(long)]
        allowed_principals: Option<String>,
    },
    /// Delete Private Service Connect endpoint
    Delete {
        /// Subscription ID
        subscription_id: u32,
        /// Endpoint ID
        endpoint_id: u32,
        /// Force deletion without confirmation
        #[arg(long)]
        force: bool,
    },
}
