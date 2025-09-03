use crate::commands::api::ApiCommands;
use crate::config::DeploymentType;
use crate::output::OutputFormat;
use clap::{Parser, Subcommand};

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
    /// Authentication testing and management
    Auth {
        #[command(subcommand)]
        command: AuthCommands,
    },
    /// Configuration management
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
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
    /// SSO/SAML management
    Sso {
        #[command(subcommand)]
        command: SsoCommands,
    },
    /// Billing and payment management
    Billing {
        #[command(subcommand)]
        command: BillingCommands,
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
    /// Alert management
    Alert {
        #[command(subcommand)]
        command: AlertCommands,
    },
    /// Active-Active database (CRDB) management
    Crdb {
        #[command(subcommand)]
        command: EnterpriseCrdbCommands,
    },
    /// Action/task management
    Actions {
        #[command(subcommand)]
        command: EnterpriseActionCommands,
    },
    /// Statistics and metrics
    Stats {
        #[command(subcommand)]
        command: EnterpriseStatsCommands,
    },
    /// Log management
    Logs {
        #[command(subcommand)]
        command: EnterpriseLogsCommands,
    },
    /// Redis ACL management
    RedisAcl {
        #[command(subcommand)]
        command: RedisAclCommands,
    },
    /// Shard management
    Shard {
        #[command(subcommand)]
        command: ShardCommands,
    },
    /// Proxy management
    Proxy {
        #[command(subcommand)]
        command: ProxyCommands,
    },
    /// Service management
    Service {
        #[command(subcommand)]
        command: ServiceCommands,
    },
    /// CRDB task management
    CrdbTask {
        #[command(subcommand)]
        command: CrdbTaskCommands,
    },
    /// Debug info collection
    DebugInfo {
        #[command(subcommand)]
        command: DebugInfoCommands,
    },
    /// Diagnostics operations
    Diagnostics {
        #[command(subcommand)]
        command: DiagnosticsCommands,
    },
    /// Endpoint management
    Endpoint {
        #[command(subcommand)]
        command: EndpointCommands,
    },
    /// Migration management
    Migration {
        #[command(subcommand)]
        command: MigrationCommands,
    },
    /// OCSP certificate management
    Ocsp {
        #[command(subcommand)]
        command: OcspCommands,
    },
    /// Usage report management
    UsageReport {
        #[command(subcommand)]
        command: UsageReportCommands,
    },
    /// Job scheduler management
    JobScheduler {
        #[command(subcommand)]
        command: JobSchedulerCommands,
    },
    /// JSON schema operations
    JsonSchema {
        #[command(subcommand)]
        command: JsonSchemaCommands,
    },
    /// LDAP mapping management
    LdapMapping {
        #[command(subcommand)]
        command: LdapMappingCommands,
    },
    /// DNS suffix management
    Suffix {
        #[command(subcommand)]
        command: SuffixCommands,
    },
    /// Authentication configuration
    Auth {
        #[command(subcommand)]
        command: AuthCommands,
    },
    /// CRDT configuration
    Crdt {
        #[command(subcommand)]
        command: CrdtCommands,
    },
    /// Client certificate management
    ClientCert {
        #[command(subcommand)]
        command: ClientCertCommands,
    },
    /// Cluster Manager server management
    CmServer {
        #[command(subcommand)]
        command: CmServerCommands,
    },
    /// CCS server management
    CcsServer {
        #[command(subcommand)]
        command: CcsServerCommands,
    },
    /// DMC server management
    DmcServer {
        #[command(subcommand)]
        command: DmcServerCommands,
    },
    /// PDN server management
    PdnServer {
        #[command(subcommand)]
        command: PdnServerCommands,
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
    /// Show node statistics
    Stats {
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
        /// Cluster name
        #[arg(long)]
        name: String,
        /// Admin username (usually email)
        #[arg(long)]
        username: String,
        /// Admin password
        #[arg(long)]
        password: String,
        /// License file path (optional)
        #[arg(long)]
        license_file: Option<String>,
        /// Enable rack awareness
        #[arg(long)]
        rack_aware: bool,
        /// DNS suffixes (optional)
        #[arg(long)]
        dns_suffixes: Option<Vec<String>>,
    },
    /// Get bootstrap status
    Status,
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
pub enum AlertCommands {
    /// List all alerts
    List,
    /// Show alert details
    Show {
        /// Alert UID
        uid: String,
    },
    /// List alerts for a database
    Database {
        /// Database UID
        uid: u32,
    },
    /// List alerts for a node
    Node {
        /// Node UID
        uid: u32,
    },
    /// List cluster alerts
    Cluster,
    /// Clear/acknowledge an alert
    Clear {
        /// Alert UID
        uid: String,
    },
    /// Clear all alerts
    ClearAll,
    /// Get alert settings
    Settings {
        /// Alert name
        name: String,
    },
    /// Update alert settings
    UpdateSettings {
        /// Alert name
        name: String,
        /// Enable/disable alert
        #[arg(long)]
        enabled: Option<bool>,
        /// Email recipients (comma-separated)
        #[arg(long)]
        emails: Option<String>,
        /// Webhook URL
        #[arg(long)]
        webhook_url: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum EnterpriseActionCommands {
    /// List all actions/tasks
    List,
    /// Show action details
    Show {
        /// Action UID
        uid: String,
    },
    /// Cancel a running action
    Cancel {
        /// Action UID
        uid: String,
    },
}

#[derive(Subcommand)]
pub enum EnterpriseStatsCommands {
    /// Get cluster statistics
    Cluster {
        /// Time interval (e.g., "1hour", "1day")
        #[arg(long)]
        interval: Option<String>,
    },
    /// Get node statistics
    Node {
        /// Node UID
        uid: u32,
        /// Time interval
        #[arg(long)]
        interval: Option<String>,
    },
    /// Get database statistics
    Database {
        /// Database UID
        uid: u32,
        /// Time interval
        #[arg(long)]
        interval: Option<String>,
    },
    /// Get shard statistics
    Shard {
        /// Shard UID
        uid: String,
        /// Time interval
        #[arg(long)]
        interval: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum EnterpriseLogsCommands {
    /// List log entries
    List {
        /// Filter by severity
        #[arg(long)]
        severity: Option<String>,
        /// Filter by module
        #[arg(long)]
        module: Option<String>,
        /// Limit number of entries
        #[arg(long)]
        limit: Option<u32>,
    },
    /// Show specific log entry
    Show {
        /// Log entry ID
        id: u64,
    },
}

#[derive(Subcommand)]
pub enum EnterpriseCrdbCommands {
    /// List all Active-Active databases
    List,
    /// Show CRDB details
    Show {
        /// CRDB GUID
        guid: String,
    },
    /// Create new Active-Active database
    Create {
        /// Database name
        name: String,
        /// Memory size in bytes
        #[arg(long)]
        memory_size: u64,
        /// Cluster instances (format: cluster_url:username:password)
        #[arg(long)]
        instances: Vec<String>,
        /// Enable encryption
        #[arg(long)]
        encryption: bool,
        /// Data persistence type
        #[arg(long)]
        persistence: Option<String>,
        /// Eviction policy
        #[arg(long)]
        eviction_policy: Option<String>,
    },
    /// Update CRDB configuration
    Update {
        /// CRDB GUID
        guid: String,
        /// Memory size in bytes
        #[arg(long)]
        memory_size: Option<u64>,
        /// Data persistence type
        #[arg(long)]
        persistence: Option<String>,
        /// Eviction policy
        #[arg(long)]
        eviction_policy: Option<String>,
    },
    /// Delete Active-Active database
    Delete {
        /// CRDB GUID
        guid: String,
        /// Force deletion without confirmation
        #[arg(long)]
        force: bool,
    },
    /// Get CRDB tasks
    Tasks {
        /// CRDB GUID
        guid: String,
    },
}

#[derive(Subcommand)]
pub enum RedisAclCommands {
    /// List all Redis ACLs
    List,
    /// Show Redis ACL details
    Show {
        /// ACL UID
        uid: u32,
    },
    /// Create new Redis ACL
    Create {
        /// ACL name
        name: String,
        /// ACL rules
        #[arg(long)]
        acl: String,
        /// ACL description
        #[arg(long)]
        description: Option<String>,
    },
    /// Update Redis ACL
    Update {
        /// ACL UID
        uid: u32,
        /// New ACL rules
        #[arg(long)]
        acl: Option<String>,
        /// New description
        #[arg(long)]
        description: Option<String>,
    },
    /// Delete Redis ACL
    Delete {
        /// ACL UID
        uid: u32,
        /// Force deletion without confirmation
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand)]
pub enum ShardCommands {
    /// List all shards
    List {
        /// Filter by database UID
        #[arg(long)]
        database: Option<u32>,
        /// Filter by node UID
        #[arg(long)]
        node: Option<u32>,
    },
    /// Show shard details
    Show {
        /// Shard UID
        uid: u32,
    },
    /// Get shard statistics
    Stats {
        /// Shard UID
        uid: u32,
        /// Time interval
        #[arg(long)]
        interval: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum ProxyCommands {
    /// List all proxies
    List {
        /// Filter by node UID
        #[arg(long)]
        node: Option<u32>,
    },
    /// Show proxy details
    Show {
        /// Proxy UID
        uid: u32,
    },
    /// Get proxy statistics
    Stats {
        /// Proxy UID
        uid: u32,
    },
    /// Reload proxy configuration
    Reload {
        /// Proxy UID
        uid: u32,
    },
}

#[derive(Subcommand)]
pub enum ServiceCommands {
    /// List all services
    List {
        /// Filter by node UID
        #[arg(long)]
        node: Option<u32>,
    },
    /// Show service details
    Show {
        /// Service name
        name: String,
        /// Node UID
        #[arg(long)]
        node: Option<u32>,
    },
    /// Restart service
    Restart {
        /// Service name
        name: String,
        /// Node UID
        #[arg(long)]
        node: Option<u32>,
    },
}

#[derive(Subcommand)]
pub enum CrdbTaskCommands {
    /// List all CRDB tasks
    List {
        /// Filter by CRDB GUID
        #[arg(long)]
        crdb: Option<String>,
    },
    /// Show CRDB task details
    Show {
        /// Task UID
        uid: String,
    },
    /// Create CRDB task
    Create {
        /// Task type
        task_type: String,
        /// CRDB GUID
        #[arg(long)]
        crdb: String,
    },
}

#[derive(Subcommand)]
pub enum DebugInfoCommands {
    /// Collect debug information
    Collect {
        /// Time range start
        #[arg(long)]
        from: Option<String>,
        /// Time range end
        #[arg(long)]
        to: Option<String>,
    },
    /// Get debug info status
    Status {
        /// Request ID
        id: String,
    },
    /// Download debug info
    Download {
        /// Request ID
        id: String,
        /// Output path
        #[arg(long)]
        output: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum DiagnosticsCommands {
    /// Run diagnostics
    Run {
        /// Diagnostic type
        #[arg(long)]
        diagnostic_type: Option<String>,
    },
    /// Get diagnostics status
    Status,
    /// Download diagnostics report
    Download {
        /// Output path
        #[arg(long)]
        output: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum EndpointCommands {
    /// List all endpoints
    List {
        /// Filter by database UID
        #[arg(long)]
        database: Option<u32>,
    },
    /// Show endpoint details
    Show {
        /// Endpoint UID
        uid: u32,
    },
    /// Get endpoint statistics
    Stats {
        /// Endpoint UID
        uid: u32,
    },
}

#[derive(Subcommand)]
pub enum MigrationCommands {
    /// List all migrations
    List,
    /// Show migration details
    Show {
        /// Migration UID
        uid: String,
    },
    /// Create migration
    Create {
        /// Source endpoint
        #[arg(long)]
        source: String,
        /// Target database UID
        #[arg(long)]
        target: u32,
    },
    /// Get migration status
    Status {
        /// Migration UID
        uid: String,
    },
}

#[derive(Subcommand)]
pub enum OcspCommands {
    /// Get OCSP status
    Status,
    /// Test OCSP configuration
    Test {
        /// Server URL
        #[arg(long)]
        server: Option<String>,
    },
    /// Update OCSP configuration
    Update {
        /// Enable/disable OCSP
        #[arg(long)]
        enabled: bool,
        /// OCSP server URL
        #[arg(long)]
        server: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum UsageReportCommands {
    /// Get usage report
    Get {
        /// Time period (e.g., "2024-01")
        #[arg(long)]
        period: Option<String>,
    },
    /// Download usage report
    Download {
        /// Time period
        #[arg(long)]
        period: Option<String>,
        /// Output path
        #[arg(long)]
        output: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum JobSchedulerCommands {
    /// List scheduled jobs
    List,
    /// Show job details
    Show {
        /// Job ID
        id: String,
    },
    /// Create scheduled job
    Create {
        /// Job name
        name: String,
        /// Cron expression
        #[arg(long)]
        cron: String,
        /// Job command
        #[arg(long)]
        command: String,
    },
    /// Delete scheduled job
    Delete {
        /// Job ID
        id: String,
        /// Force deletion
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand)]
pub enum JsonSchemaCommands {
    /// Get JSON schema for an endpoint
    Get {
        /// Endpoint path (e.g., "/v1/bdbs")
        path: String,
    },
}

#[derive(Subcommand)]
pub enum LdapMappingCommands {
    /// List LDAP mappings
    List,
    /// Show LDAP mapping details
    Show {
        /// Mapping ID
        id: u32,
    },
    /// Create LDAP mapping
    Create {
        /// LDAP DN
        dn: String,
        /// Redis Enterprise role
        #[arg(long)]
        role: String,
    },
    /// Update LDAP mapping
    Update {
        /// Mapping ID
        id: u32,
        /// New role
        #[arg(long)]
        role: Option<String>,
    },
    /// Delete LDAP mapping
    Delete {
        /// Mapping ID
        id: u32,
        /// Force deletion
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand)]
pub enum SuffixCommands {
    /// List DNS suffixes
    List,
    /// Show suffix details
    Show {
        /// Suffix ID
        id: String,
    },
    /// Create DNS suffix
    Create {
        /// Suffix name
        name: String,
        /// DNS suffix value
        dns_suffix: String,
    },
    /// Update DNS suffix
    Update {
        /// Suffix ID
        id: String,
        /// New DNS suffix value
        dns_suffix: String,
    },
    /// Delete DNS suffix
    Delete {
        /// Suffix ID
        id: String,
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

#[derive(Subcommand)]
pub enum SsoCommands {
    /// Show SSO configuration
    Show,
    /// Update SSO configuration
    Update {
        /// Provider name (e.g., okta, azure)
        #[arg(long)]
        provider: String,
        /// SSO login URL
        #[arg(long)]
        login_url: String,
        /// SSO logout URL
        #[arg(long)]
        logout_url: Option<String>,
        /// Enable SSO
        #[arg(long)]
        enabled: Option<bool>,
    },
    /// Delete SSO configuration
    Delete {
        /// Skip confirmation
        #[arg(long)]
        force: bool,
    },
    /// Test SSO configuration
    Test {
        /// Test user email
        #[arg(long)]
        email: String,
    },

    // SAML specific commands
    /// Show SAML configuration
    SamlShow,
    /// Update SAML configuration
    SamlUpdate {
        /// SAML issuer URL
        #[arg(long)]
        issuer: String,
        /// SAML SSO URL
        #[arg(long)]
        sso_url: String,
        /// SAML certificate content
        #[arg(long)]
        certificate: Option<String>,
    },
    /// Get SAML metadata
    SamlMetadata,
    /// Upload SAML certificate
    SamlUploadCert {
        /// Certificate file path or content
        certificate: String,
    },

    // User mapping commands
    /// List SSO users
    UserList,
    /// Show SSO user details
    UserShow {
        /// User ID
        id: u32,
    },
    /// Create SSO user mapping
    UserCreate {
        /// SSO user email
        #[arg(long)]
        email: String,
        /// Local user ID to map to
        #[arg(long)]
        local_user_id: u32,
        /// User role
        #[arg(long)]
        role: String,
    },
    /// Update SSO user mapping
    UserUpdate {
        /// User ID
        id: u32,
        /// Local user ID to map to
        #[arg(long)]
        local_user_id: Option<u32>,
        /// User role
        #[arg(long)]
        role: Option<String>,
    },
    /// Delete SSO user mapping
    UserDelete {
        /// User ID
        id: u32,
        /// Skip confirmation
        #[arg(long)]
        force: bool,
    },

    // Group mapping commands
    /// List SSO groups
    GroupList,
    /// Create SSO group mapping
    GroupCreate {
        /// SSO group name
        #[arg(long)]
        name: String,
        /// Local role to map to
        #[arg(long)]
        role: String,
    },
    /// Update SSO group mapping
    GroupUpdate {
        /// Group ID
        id: u32,
        /// Local role to map to
        #[arg(long)]
        role: String,
    },
    /// Delete SSO group mapping
    GroupDelete {
        /// Group ID
        id: u32,
        /// Skip confirmation
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand)]
pub enum BillingCommands {
    /// Get billing information
    Info,
    /// List invoices
    InvoiceList {
        /// Number of invoices to return
        #[arg(long)]
        limit: Option<u32>,
        /// Filter by status
        #[arg(long)]
        status: Option<String>,
    },
    /// Get invoice details
    InvoiceGet {
        /// Invoice ID
        id: String,
    },
    /// Download invoice PDF
    InvoiceDownload {
        /// Invoice ID
        id: String,
        /// Output file path
        #[arg(long)]
        output: Option<String>,
    },
    /// Get current month invoice
    InvoiceCurrent,
    /// List payment methods
    PaymentMethodList,
    /// Get payment method details
    PaymentMethodGet {
        /// Payment method ID
        id: String,
    },
    /// Add payment method
    PaymentMethodAdd {
        /// Payment method JSON data
        #[arg(long)]
        data: String,
    },
    /// Update payment method
    PaymentMethodUpdate {
        /// Payment method ID
        id: String,
        /// Payment method JSON data
        #[arg(long)]
        data: String,
    },
    /// Delete payment method
    PaymentMethodDelete {
        /// Payment method ID
        id: String,
        /// Skip confirmation
        #[arg(long)]
        force: bool,
    },
    /// Set default payment method
    PaymentMethodDefault {
        /// Payment method ID
        id: String,
    },
    /// Get cost breakdown
    CostBreakdown {
        /// Subscription ID (optional)
        #[arg(long)]
        subscription: Option<u32>,
    },
    /// Get usage report
    Usage {
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        from: Option<String>,
        /// End date (YYYY-MM-DD)
        #[arg(long)]
        to: Option<String>,
    },
    /// Get billing history
    History {
        /// Number of months
        #[arg(long)]
        months: Option<u32>,
    },
    /// Get available credits
    Credits,
    /// Apply promo code
    PromoApply {
        /// Promo code
        code: String,
    },
    /// Get billing alerts
    AlertList,
    /// Update billing alerts
    AlertUpdate {
        /// Alert settings JSON
        #[arg(long)]
        data: String,
    },
}

#[derive(Subcommand)]
pub enum AuthCommands {
    /// Test authentication credentials
    Test {
        /// Profile to test (defaults to current profile)
        #[arg(long)]
        profile: Option<String>,
        /// Test a specific deployment type
        #[arg(long, value_enum)]
        deployment: Option<DeploymentType>,
    },
    /// Interactive setup wizard for new profiles
    Setup,
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Show current configuration and active profile
    Show {
        /// Show sensitive values (passwords, API keys)
        #[arg(long)]
        show_secrets: bool,
    },
    /// Show configuration file path
    Path,
    /// Validate configuration
    Validate {
        /// Profile to validate (defaults to all profiles)
        #[arg(long)]
        profile: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum CrdtCommands {
    /// List CRDTs
    List,
    /// Show CRDT details
    Show {
        /// CRDT ID
        id: String,
    },
    /// Create CRDT
    Create {
        /// Database ID
        database_id: String,
    },
    /// Update CRDT
    Update {
        /// CRDT ID
        id: String,
        /// Database ID
        #[arg(long)]
        database_id: Option<String>,
    },
    /// Delete CRDT
    Delete {
        /// CRDT ID
        id: String,
    },
}

#[derive(Subcommand)]
pub enum ClientCertCommands {
    /// List client certificates
    List,
    /// Show certificate details
    Show {
        /// Certificate ID
        id: String,
    },
    /// Create client certificate
    Create {
        /// Certificate name
        name: String,
        /// Certificate content
        cert: String,
    },
    /// Delete client certificate
    Delete {
        /// Certificate ID
        id: String,
    },
}

#[derive(Subcommand)]
pub enum CmServerCommands {
    /// List CM servers
    List,
    /// Show CM server details
    Show {
        /// Server ID
        id: String,
    },
    /// Get CM server statistics
    Stats {
        /// Server ID
        id: String,
    },
}

#[derive(Subcommand)]
pub enum CcsServerCommands {
    /// List CCS servers
    List,
    /// Show CCS server details
    Show {
        /// Server ID
        id: String,
    },
    /// Get CCS server statistics
    Stats {
        /// Server ID
        id: String,
    },
}

#[derive(Subcommand)]
pub enum DmcServerCommands {
    /// List DMC servers
    List,
    /// Show DMC server details
    Show {
        /// Server ID
        id: String,
    },
    /// Get DMC server statistics
    Stats {
        /// Server ID
        id: String,
    },
}

#[derive(Subcommand)]
pub enum PdnServerCommands {
    /// List PDN servers
    List,
    /// Show PDN server details
    Show {
        /// Server ID
        id: String,
    },
    /// Get PDN server statistics
    Stats {
        /// Server ID
        id: String,
    },
}
