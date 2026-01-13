//! Enterprise CLI command definitions

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum EnterpriseCommands {
    /// Action (task) operations
    #[command(subcommand)]
    Action(crate::commands::enterprise::actions::ActionCommands),
    /// Alert management operations
    #[command(subcommand)]
    Alerts(crate::commands::enterprise::alerts::AlertsCommands),
    /// Database group operations
    #[command(subcommand, name = "bdb-group")]
    BdbGroup(crate::commands::enterprise::bdb_group::BdbGroupCommands),
    /// Cluster operations
    #[command(subcommand)]
    Cluster(EnterpriseClusterCommands),
    /// Cluster manager settings
    #[command(subcommand, name = "cm-settings")]
    CmSettings(crate::commands::enterprise::cm_settings::CmSettingsCommands),

    /// Database operations
    #[command(subcommand)]
    Database(EnterpriseDatabaseCommands),

    /// Debug info collection
    #[command(subcommand)]
    DebugInfo(crate::commands::enterprise::debuginfo::DebugInfoCommands),

    /// Diagnostics operations
    #[command(subcommand)]
    Diagnostics(crate::commands::enterprise::diagnostics::DiagnosticsCommands),

    /// Endpoint operations
    #[command(subcommand)]
    Endpoint(crate::commands::enterprise::endpoint::EndpointCommands),

    /// Node operations
    #[command(subcommand)]
    Node(EnterpriseNodeCommands),

    /// Proxy management
    #[command(subcommand)]
    Proxy(crate::commands::enterprise::proxy::ProxyCommands),

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
    Ldap(crate::commands::enterprise::ldap::LdapCommands),

    /// LDAP mappings management
    #[command(subcommand, name = "ldap-mappings")]
    LdapMappings(crate::commands::enterprise::ldap::LdapMappingsCommands),

    /// Authentication & sessions
    #[command(subcommand)]
    Auth(EnterpriseAuthCommands),
    /// Bootstrap and initialization operations
    #[command(subcommand)]
    Bootstrap(crate::commands::enterprise::bootstrap::BootstrapCommands),

    /// Active-Active database (CRDB) operations
    #[command(subcommand)]
    Crdb(EnterpriseCrdbCommands),
    /// CRDB task operations
    #[command(subcommand, name = "crdb-task")]
    CrdbTask(crate::commands::enterprise::crdb_task::CrdbTaskCommands),

    /// Job scheduler operations
    #[command(subcommand, name = "job-scheduler")]
    JobScheduler(crate::commands::enterprise::job_scheduler::JobSchedulerCommands),

    /// JSON schema operations
    #[command(subcommand)]
    Jsonschema(crate::commands::enterprise::jsonschema::JsonSchemaCommands),

    /// Log operations
    #[command(subcommand)]
    Logs(crate::commands::enterprise::logs::LogsCommands),
    /// License management
    #[command(subcommand)]
    License(crate::commands::enterprise::license::LicenseCommands),

    /// Migration operations
    #[command(subcommand)]
    Migration(crate::commands::enterprise::migration::MigrationCommands),

    /// Module management operations
    #[command(subcommand)]
    Module(crate::commands::enterprise::module::ModuleCommands),

    /// OCSP certificate validation
    #[command(subcommand)]
    Ocsp(crate::commands::enterprise::ocsp::OcspCommands),

    /// Service management
    #[command(subcommand)]
    Services(crate::commands::enterprise::services::ServicesCommands),

    /// Workflow operations for multi-step tasks
    #[command(subcommand)]
    Workflow(EnterpriseWorkflowCommands),

    /// Local node operations
    #[command(subcommand)]
    Local(crate::commands::enterprise::local::LocalCommands),

    /// Shard management operations
    #[command(subcommand)]
    Shard(crate::commands::enterprise::shard::ShardCommands),

    /// Statistics and metrics operations
    #[command(subcommand)]
    Stats(EnterpriseStatsCommands),

    /// Comprehensive cluster status (cluster, nodes, databases, shards)
    Status {
        /// Show only cluster information
        #[arg(long)]
        cluster: bool,

        /// Show only nodes information
        #[arg(long)]
        nodes: bool,

        /// Show only databases information
        #[arg(long)]
        databases: bool,

        /// Show only shards information
        #[arg(long)]
        shards: bool,
    },

    /// Support package generation for troubleshooting
    #[command(subcommand, name = "support-package")]
    SupportPackage(crate::commands::enterprise::support_package::SupportPackageCommands),

    /// DNS suffix management
    #[command(subcommand)]
    Suffix(crate::commands::enterprise::suffix::SuffixCommands),

    /// Usage report operations
    #[command(subcommand, name = "usage-report")]
    UsageReport(crate::commands::enterprise::usage_report::UsageReportCommands),
}

/// Cloud workflow commands
#[derive(Debug, Subcommand)]
pub enum EnterpriseWorkflowCommands {
    /// List available workflows
    List,
    /// License management workflows
    #[command(subcommand)]
    License(crate::commands::enterprise::license_workflow::LicenseWorkflowCommands),

    /// Initialize a Redis Enterprise cluster
    #[command(name = "init-cluster")]
    InitCluster {
        /// Cluster name
        #[arg(long, default_value = "redis-cluster")]
        name: String,

        /// Admin username
        #[arg(long, default_value = "admin@redis.local")]
        username: String,

        /// Admin password (required)
        #[arg(long, env = "REDIS_ENTERPRISE_INIT_PASSWORD")]
        password: String,

        /// Skip creating a default database after initialization
        #[arg(long)]
        skip_database: bool,

        /// Name for the default database
        #[arg(long, default_value = "default-db")]
        database_name: String,

        /// Memory size for the default database in GB
        #[arg(long, default_value = "1")]
        database_memory_gb: i64,

        /// Async operation options
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },
}

// Placeholder command structures - will be expanded in later PRs

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
    #[command(after_help = "EXAMPLES:
    # Simple database - just name and size
    redisctl enterprise database create --name mydb --memory 1073741824

    # With replication for high availability
    redisctl enterprise database create --name prod-db --memory 2147483648 --replication

    # With persistence and eviction policy
    redisctl enterprise database create --name cache-db --memory 536870912 \\
      --persistence aof --eviction-policy volatile-lru

    # With sharding for horizontal scaling
    redisctl enterprise database create --name large-db --memory 10737418240 \\
      --sharding --shards-count 4

    # With specific port
    redisctl enterprise database create --name service-db --memory 1073741824 --port 12000

    # With modules (auto-resolves name to module)
    redisctl enterprise database create --name search-db --memory 1073741824 \\
      --module search --module ReJSON

    # Complete configuration from file
    redisctl enterprise database create --data @database.json

    # Dry run to preview without creating
    redisctl enterprise database create --name test-db --memory 1073741824 --dry-run

NOTE: Memory size is in bytes. Common values:
      - 1 GB = 1073741824 bytes
      - 2 GB = 2147483648 bytes
      - 5 GB = 5368709120 bytes
      First-class parameters override values in --data when both are provided.")]
    Create {
        /// Database name (required unless using --data)
        #[arg(long)]
        name: Option<String>,

        /// Memory size in bytes (e.g., 1073741824 for 1GB)
        #[arg(long)]
        memory: Option<u64>,

        /// TCP port (10000-19999, auto-assigned if not specified)
        #[arg(long)]
        port: Option<u16>,

        /// Enable replication for high availability
        #[arg(long)]
        replication: bool,

        /// Data persistence: aof, snapshot, or aof-and-snapshot
        #[arg(long)]
        persistence: Option<String>,

        /// Data eviction policy when memory limit reached
        #[arg(long)]
        eviction_policy: Option<String>,

        /// Enable sharding for horizontal scaling
        #[arg(long)]
        sharding: bool,

        /// Number of shards (requires --sharding)
        #[arg(long)]
        shards_count: Option<u32>,

        /// Proxy policy: single, all-master-shards, or all-nodes
        #[arg(long)]
        proxy_policy: Option<String>,

        /// Enable CRDB (Active-Active)
        #[arg(long)]
        crdb: bool,

        /// Redis password for authentication
        #[arg(long)]
        redis_password: Option<String>,

        /// Module to enable (by name, can be repeated). Use 'module list' to see available modules.
        /// Format: module_name or module_name:args (e.g., --module search --module ReJSON)
        #[arg(long = "module", value_name = "NAME[:ARGS]")]
        modules: Vec<String>,

        /// Advanced: Full database configuration as JSON string or @file.json
        #[arg(long)]
        data: Option<String>,

        /// Perform a dry run without creating the database
        #[arg(long)]
        dry_run: bool,
    },

    /// Update database configuration
    #[command(after_help = "EXAMPLES:
    # Update memory size
    redisctl enterprise database update 1 --memory 2147483648

    # Enable replication
    redisctl enterprise database update 1 --replication true

    # Update persistence and eviction policy
    redisctl enterprise database update 1 --persistence aof --eviction-policy volatile-lru

    # Update sharding configuration
    redisctl enterprise database update 1 --shards-count 8

    # Update proxy policy
    redisctl enterprise database update 1 --proxy-policy all-master-shards

    # Update Redis password
    redisctl enterprise database update 1 --redis-password newsecret

    # Advanced: Full update via JSON file
    redisctl enterprise database update 1 --data @updates.json

NOTE: First-class parameters override values in --data when both are provided.")]
    Update {
        /// Database ID
        id: u32,

        /// New database name
        #[arg(long)]
        name: Option<String>,

        /// Memory size in bytes (e.g., 1073741824 for 1GB)
        #[arg(long)]
        memory: Option<u64>,

        /// Enable/disable replication
        #[arg(long)]
        replication: Option<bool>,

        /// Data persistence: disabled, aof, snapshot, or aof-and-snapshot
        #[arg(long)]
        persistence: Option<String>,

        /// Data eviction policy when memory limit reached
        #[arg(long)]
        eviction_policy: Option<String>,

        /// Number of shards
        #[arg(long)]
        shards_count: Option<u32>,

        /// Proxy policy: single, all-master-shards, or all-nodes
        #[arg(long)]
        proxy_policy: Option<String>,

        /// Redis password for authentication
        #[arg(long)]
        redis_password: Option<String>,

        /// Advanced: Full update configuration as JSON string or @file.json
        #[arg(long)]
        data: Option<String>,
    },

    /// Delete a database
    Delete {
        /// Database ID
        id: u32,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },

    /// Watch database status changes in real-time
    Watch {
        /// Database ID
        id: u32,
        /// Poll interval in seconds
        #[arg(long, default_value = "5")]
        poll_interval: u64,
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

    /// Upgrade database Redis version
    Upgrade {
        /// Database ID
        id: u32,
        /// Target Redis version (defaults to latest)
        #[arg(long)]
        version: Option<String>,
        /// Preserve master/replica roles (requires extra failover)
        #[arg(long)]
        preserve_roles: bool,
        /// Restart shards even if no version change
        #[arg(long)]
        force_restart: bool,
        /// Allow data loss in non-replicated, non-persistent databases
        #[arg(long)]
        may_discard_data: bool,
        /// Force data discard even if replicated/persistent
        #[arg(long)]
        force_discard: bool,
        /// Keep current CRDT protocol version
        #[arg(long)]
        keep_crdt_protocol_version: bool,
        /// Maximum parallel shard upgrades
        #[arg(long)]
        parallel_shards_upgrade: Option<u32>,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
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
    #[command(after_help = "EXAMPLES:
    # Create user with email and password
    redisctl enterprise user create --email admin@example.com --password secret123 --role admin

    # Create user with display name
    redisctl enterprise user create --email user@example.com --password secret123 --role db_viewer --name \"John Doe\"

    # Create user with email alerts enabled
    redisctl enterprise user create --email ops@example.com --password secret123 --role db_member --email-alerts

    # Create user with RBAC role IDs
    redisctl enterprise user create --email rbac@example.com --password secret123 --role db_viewer --role-uid 1 --role-uid 2

    # Advanced: Full configuration via JSON file
    redisctl enterprise user create --data @user.json

NOTE: First-class parameters override values in --data when both are provided.")]
    Create {
        /// User's email address (used as login)
        #[arg(long)]
        email: Option<String>,

        /// User's password
        #[arg(long)]
        password: Option<String>,

        /// User's role (admin, db_viewer, db_member, cluster_viewer, cluster_member, none)
        #[arg(long)]
        role: Option<String>,

        /// User's display name
        #[arg(long)]
        name: Option<String>,

        /// Enable email alerts for this user
        #[arg(long)]
        email_alerts: bool,

        /// Role UID for RBAC (can be repeated)
        #[arg(long = "role-uid")]
        role_uids: Vec<u32>,

        /// Authentication method (regular, external, certificate)
        #[arg(long)]
        auth_method: Option<String>,

        /// Advanced: Full user configuration as JSON string or @file.json
        #[arg(long)]
        data: Option<String>,
    },

    /// Update user
    #[command(after_help = "EXAMPLES:
    # Update user's name
    redisctl enterprise user update 1 --name \"Jane Doe\"

    # Update user's role
    redisctl enterprise user update 1 --role admin

    # Update user's password
    redisctl enterprise user update 1 --password newsecret123

    # Enable email alerts
    redisctl enterprise user update 1 --email-alerts true

    # Update RBAC role assignments
    redisctl enterprise user update 1 --role-uid 1 --role-uid 3

    # Advanced: Full update via JSON file
    redisctl enterprise user update 1 --data @updates.json

NOTE: First-class parameters override values in --data when both are provided.")]
    Update {
        /// User ID
        id: u32,

        /// New email address
        #[arg(long)]
        email: Option<String>,

        /// New password
        #[arg(long)]
        password: Option<String>,

        /// New role
        #[arg(long)]
        role: Option<String>,

        /// New display name
        #[arg(long)]
        name: Option<String>,

        /// Enable/disable email alerts
        #[arg(long)]
        email_alerts: Option<bool>,

        /// Role UID for RBAC (can be repeated, replaces existing)
        #[arg(long = "role-uid")]
        role_uids: Vec<u32>,

        /// Advanced: Full update configuration as JSON string or @file.json
        #[arg(long)]
        data: Option<String>,
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

#[derive(Subcommand, Debug, Clone)]
pub enum EnterpriseStatsCommands {
    /// Get database statistics
    Database {
        /// Database ID
        id: u32,
        /// Stream stats continuously
        #[arg(long, short = 'f')]
        follow: bool,
        /// Poll interval in seconds (for --follow)
        #[arg(long, default_value = "5")]
        poll_interval: u64,
    },

    /// Get database shard statistics
    DatabaseShards {
        /// Database ID
        id: u32,
    },

    /// Get database metrics over time
    DatabaseMetrics {
        /// Database ID
        id: u32,
        /// Time interval (1m, 5m, 1h, 1d)
        #[arg(long, default_value = "1h")]
        interval: String,
    },

    /// Get node statistics
    Node {
        /// Node ID
        id: u32,
        /// Stream stats continuously
        #[arg(long, short = 'f')]
        follow: bool,
        /// Poll interval in seconds (for --follow)
        #[arg(long, default_value = "5")]
        poll_interval: u64,
    },

    /// Get node metrics over time
    NodeMetrics {
        /// Node ID
        id: u32,
        /// Time interval (1m, 5m, 1h, 1d)
        #[arg(long, default_value = "1h")]
        interval: String,
    },

    /// Get cluster-wide statistics
    Cluster {
        /// Stream stats continuously
        #[arg(long, short = 'f')]
        follow: bool,
        /// Poll interval in seconds (for --follow)
        #[arg(long, default_value = "5")]
        poll_interval: u64,
    },

    /// Get cluster metrics over time
    ClusterMetrics {
        /// Time interval (1m, 5m, 1h, 1d)
        #[arg(long, default_value = "1h")]
        interval: String,
    },

    /// Get listener statistics
    Listener,

    /// Export statistics in various formats
    Export {
        /// Export format (json, prometheus, csv)
        #[arg(long, default_value = "json")]
        format: String,
        /// Time interval for time-series data (1m, 5m, 1h, 1d)
        #[arg(long)]
        interval: Option<String>,
    },
}
