//! Cloud CLI command definitions

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum CloudConnectivityCommands {
    /// VPC Peering operations
    #[command(subcommand, name = "vpc-peering")]
    VpcPeering(VpcPeeringCommands),
    /// Private Service Connect operations
    #[command(subcommand, name = "psc")]
    Psc(PscCommands),
    /// Transit Gateway operations
    #[command(subcommand, name = "tgw")]
    Tgw(TgwCommands),
    /// AWS PrivateLink operations
    #[command(subcommand, name = "privatelink")]
    PrivateLink(PrivateLinkCommands),
}

/// VPC Peering Commands
#[derive(Subcommand, Debug)]
pub enum VpcPeeringCommands {
    /// Get VPC peering details
    Get {
        /// Subscription ID
        #[arg(long)]
        subscription: i32,
    },
    /// Create VPC peering
    Create {
        /// Subscription ID
        #[arg(long)]
        subscription: i32,
        /// Configuration JSON file or string (use @filename for file)
        data: String,
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },
    /// Update VPC peering
    Update {
        /// Subscription ID
        #[arg(long)]
        subscription: i32,
        /// Peering ID
        #[arg(long)]
        peering_id: i32,
        /// Configuration JSON file or string (use @filename for file)
        data: String,
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },
    /// Delete VPC peering
    Delete {
        /// Subscription ID
        #[arg(long)]
        subscription: i32,
        /// Peering ID
        #[arg(long)]
        peering_id: i32,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },
    /// List Active-Active VPC peerings
    #[command(name = "list-aa")]
    ListActiveActive {
        /// Subscription ID
        #[arg(long)]
        subscription: i32,
    },
    /// Create Active-Active VPC peering
    #[command(name = "create-aa")]
    CreateActiveActive {
        /// Subscription ID
        #[arg(long)]
        subscription: i32,
        /// Configuration JSON file or string (use @filename for file)
        data: String,
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },
    /// Update Active-Active VPC peering
    #[command(name = "update-aa")]
    UpdateActiveActive {
        /// Subscription ID
        #[arg(long)]
        subscription: i32,
        /// Peering ID
        #[arg(long)]
        peering_id: i32,
        /// Configuration JSON file or string (use @filename for file)
        data: String,
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },
    /// Delete Active-Active VPC peering
    #[command(name = "delete-aa")]
    DeleteActiveActive {
        /// Subscription ID
        #[arg(long)]
        subscription: i32,
        /// Peering ID
        #[arg(long)]
        peering_id: i32,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },
}

/// Private Service Connect (PSC) Commands
#[derive(Subcommand, Debug)]
pub enum PscCommands {
    // Standard PSC Service operations
    /// Get PSC service details
    #[command(name = "service-get")]
    ServiceGet {
        /// Subscription ID
        subscription_id: i32,
    },
    /// Create PSC service
    #[command(name = "service-create")]
    ServiceCreate {
        /// Subscription ID
        subscription_id: i32,
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },
    /// Delete PSC service
    #[command(name = "service-delete")]
    ServiceDelete {
        /// Subscription ID
        subscription_id: i32,
        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },

    // Standard PSC Endpoint operations
    /// List PSC endpoints
    #[command(name = "endpoints-list")]
    EndpointsList {
        /// Subscription ID
        subscription_id: i32,
    },
    /// Create PSC endpoint
    #[command(name = "endpoint-create")]
    EndpointCreate {
        /// Subscription ID
        subscription_id: i32,
        /// JSON file with endpoint configuration (use @filename or - for stdin)
        file: String,
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },
    /// Update PSC endpoint
    #[command(name = "endpoint-update")]
    EndpointUpdate {
        /// Subscription ID
        subscription_id: i32,
        /// Endpoint ID
        endpoint_id: i32,
        /// JSON file with endpoint configuration (use @filename or - for stdin)
        file: String,
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },
    /// Delete PSC endpoint
    #[command(name = "endpoint-delete")]
    EndpointDelete {
        /// Subscription ID
        subscription_id: i32,
        /// Endpoint ID
        endpoint_id: i32,
        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },
    /// Get PSC endpoint creation script
    #[command(name = "endpoint-creation-script")]
    EndpointCreationScript {
        /// Subscription ID
        subscription_id: i32,
        /// Endpoint ID
        endpoint_id: i32,
    },
    /// Get PSC endpoint deletion script
    #[command(name = "endpoint-deletion-script")]
    EndpointDeletionScript {
        /// Subscription ID
        subscription_id: i32,
        /// Endpoint ID
        endpoint_id: i32,
    },

    // Active-Active PSC Service operations
    /// Get Active-Active PSC service details
    #[command(name = "aa-service-get")]
    AaServiceGet {
        /// Subscription ID
        subscription_id: i32,
    },
    /// Create Active-Active PSC service
    #[command(name = "aa-service-create")]
    AaServiceCreate {
        /// Subscription ID
        subscription_id: i32,
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },
    /// Delete Active-Active PSC service
    #[command(name = "aa-service-delete")]
    AaServiceDelete {
        /// Subscription ID
        subscription_id: i32,
        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },

    // Active-Active PSC Endpoint operations
    /// List Active-Active PSC endpoints
    #[command(name = "aa-endpoints-list")]
    AaEndpointsList {
        /// Subscription ID
        subscription_id: i32,
    },
    /// Create Active-Active PSC endpoint
    #[command(name = "aa-endpoint-create")]
    AaEndpointCreate {
        /// Subscription ID
        subscription_id: i32,
        /// JSON file with endpoint configuration (use @filename or - for stdin)
        file: String,
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },
    /// Delete Active-Active PSC endpoint
    #[command(name = "aa-endpoint-delete")]
    AaEndpointDelete {
        /// Subscription ID
        subscription_id: i32,
        /// Region ID
        region_id: i32,
        /// Endpoint ID
        endpoint_id: i32,
        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },
}

/// Transit Gateway (TGW) Commands
#[derive(Subcommand, Debug)]
pub enum TgwCommands {
    // Standard TGW Attachment operations
    /// List TGW attachments
    #[command(name = "attachments-list")]
    AttachmentsList {
        /// Subscription ID
        subscription_id: i32,
    },
    /// Create TGW attachment
    #[command(name = "attachment-create")]
    AttachmentCreate {
        /// Subscription ID
        subscription_id: i32,
        /// JSON file with attachment configuration (use @filename or - for stdin)
        file: String,
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },
    /// Create TGW attachment with ID in path
    #[command(name = "attachment-create-with-id")]
    AttachmentCreateWithId {
        /// Subscription ID
        subscription_id: i32,
        /// Transit Gateway ID
        tgw_id: String,
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },
    /// Update TGW attachment CIDRs
    #[command(name = "attachment-update")]
    AttachmentUpdate {
        /// Subscription ID
        subscription_id: i32,
        /// Attachment ID
        attachment_id: String,
        /// JSON file with CIDR configuration (use @filename or - for stdin)
        file: String,
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },
    /// Delete TGW attachment
    #[command(name = "attachment-delete")]
    AttachmentDelete {
        /// Subscription ID
        subscription_id: i32,
        /// Attachment ID
        attachment_id: String,
        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },

    // Standard TGW Invitation operations
    /// List TGW resource share invitations
    #[command(name = "invitations-list")]
    InvitationsList {
        /// Subscription ID
        subscription_id: i32,
    },
    /// Accept TGW resource share invitation
    #[command(name = "invitation-accept")]
    InvitationAccept {
        /// Subscription ID
        subscription_id: i32,
        /// Invitation ID
        invitation_id: String,
    },
    /// Reject TGW resource share invitation
    #[command(name = "invitation-reject")]
    InvitationReject {
        /// Subscription ID
        subscription_id: i32,
        /// Invitation ID
        invitation_id: String,
    },

    // Active-Active TGW Attachment operations
    /// List Active-Active TGW attachments
    #[command(name = "aa-attachments-list")]
    AaAttachmentsList {
        /// Subscription ID
        subscription_id: i32,
    },
    /// Create Active-Active TGW attachment
    #[command(name = "aa-attachment-create")]
    AaAttachmentCreate {
        /// Subscription ID
        subscription_id: i32,
        /// Region ID
        region_id: i32,
        /// JSON file with attachment configuration (use @filename or - for stdin)
        file: String,
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },
    /// Update Active-Active TGW attachment CIDRs
    #[command(name = "aa-attachment-update")]
    AaAttachmentUpdate {
        /// Subscription ID
        subscription_id: i32,
        /// Region ID
        region_id: i32,
        /// Attachment ID
        attachment_id: String,
        /// JSON file with CIDR configuration (use @filename or - for stdin)
        file: String,
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },
    /// Delete Active-Active TGW attachment
    #[command(name = "aa-attachment-delete")]
    AaAttachmentDelete {
        /// Subscription ID
        subscription_id: i32,
        /// Region ID
        region_id: i32,
        /// Attachment ID
        attachment_id: String,
        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },

    // Active-Active TGW Invitation operations
    /// List Active-Active TGW resource share invitations
    #[command(name = "aa-invitations-list")]
    AaInvitationsList {
        /// Subscription ID
        subscription_id: i32,
    },
    /// Accept Active-Active TGW resource share invitation
    #[command(name = "aa-invitation-accept")]
    AaInvitationAccept {
        /// Subscription ID
        subscription_id: i32,
        /// Region ID
        region_id: i32,
        /// Invitation ID
        invitation_id: String,
    },
    /// Reject Active-Active TGW resource share invitation
    #[command(name = "aa-invitation-reject")]
    AaInvitationReject {
        /// Subscription ID
        subscription_id: i32,
        /// Region ID
        region_id: i32,
        /// Invitation ID
        invitation_id: String,
    },
}

/// AWS PrivateLink Commands
#[derive(Subcommand, Debug)]
pub enum PrivateLinkCommands {
    /// Get PrivateLink configuration
    Get {
        /// Subscription ID
        #[arg(long)]
        subscription: i32,
        /// Region ID (for Active-Active databases)
        #[arg(long)]
        region: Option<i32>,
    },
    /// Create PrivateLink
    Create {
        /// Subscription ID
        #[arg(long)]
        subscription: i32,
        /// Region ID (for Active-Active databases)
        #[arg(long)]
        region: Option<i32>,
        /// Configuration JSON file or string (use @filename for file)
        data: String,
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },
    /// Add principals to PrivateLink
    #[command(name = "add-principal")]
    AddPrincipal {
        /// Subscription ID
        #[arg(long)]
        subscription: i32,
        /// Region ID (for Active-Active databases)
        #[arg(long)]
        region: Option<i32>,
        /// Configuration JSON file or string (use @filename for file)
        data: String,
    },
    /// Remove principals from PrivateLink
    #[command(name = "remove-principal")]
    RemovePrincipal {
        /// Subscription ID
        #[arg(long)]
        subscription: i32,
        /// Region ID (for Active-Active databases)
        #[arg(long)]
        region: Option<i32>,
        /// Configuration JSON file or string (use @filename for file)
        data: String,
    },
    /// Get VPC endpoint creation script
    #[command(name = "get-script")]
    GetScript {
        /// Subscription ID
        #[arg(long)]
        subscription: i32,
        /// Region ID (for Active-Active databases)
        #[arg(long)]
        region: Option<i32>,
    },
}

/// Cloud Task Commands
#[derive(Subcommand, Debug)]
pub enum CloudTaskCommands {
    /// Get task status and details
    Get {
        /// Task ID (UUID format)
        id: String,
    },
    /// Wait for task to complete
    Wait {
        /// Task ID (UUID format)
        id: String,
        /// Maximum time to wait in seconds
        #[arg(long, default_value = "300")]
        timeout: u64,
        /// Polling interval in seconds
        #[arg(long, default_value = "2")]
        interval: u64,
    },
    /// Poll task status with live updates
    Poll {
        /// Task ID (UUID format)
        id: String,
        /// Polling interval in seconds
        #[arg(long, default_value = "2")]
        interval: u64,
        /// Maximum number of polls (0 = unlimited)
        #[arg(long, default_value = "0")]
        max_polls: u64,
    },
}

/// Cloud Fixed Database Commands
#[derive(Subcommand, Debug)]
pub enum CloudFixedDatabaseCommands {
    /// List all databases in a fixed subscription
    List {
        /// Subscription ID
        subscription_id: i32,
    },
    /// Get details of a specific fixed database
    Get {
        /// Database ID (format: subscription_id:database_id)
        id: String,
    },
    /// Create a new database in a fixed subscription
    Create {
        /// Subscription ID
        subscription_id: i32,
        /// JSON file with database configuration (use @filename or - for stdin)
        file: String,
        /// Async operation options
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },
    /// Update fixed database configuration
    Update {
        /// Database ID (format: subscription_id:database_id)
        id: String,
        /// JSON file with update configuration (use @filename or - for stdin)
        file: String,
        /// Async operation options
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },
    /// Delete a fixed database
    Delete {
        /// Database ID (format: subscription_id:database_id)
        id: String,
        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
        /// Async operation options
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },
    /// Get backup status for fixed database
    #[command(name = "backup-status")]
    BackupStatus {
        /// Database ID (format: subscription_id:database_id)
        id: String,
    },
    /// Trigger manual backup
    Backup {
        /// Database ID (format: subscription_id:database_id)
        id: String,
        /// Async operation options
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },
    /// Get import status
    #[command(name = "import-status")]
    ImportStatus {
        /// Database ID (format: subscription_id:database_id)
        id: String,
    },
    /// Import data into fixed database
    Import {
        /// Database ID (format: subscription_id:database_id)
        id: String,
        /// JSON file with import configuration (use @filename or - for stdin)
        file: String,
        /// Async operation options
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },
    /// Get slow query log
    #[command(name = "slow-log")]
    SlowLog {
        /// Database ID (format: subscription_id:database_id)
        id: String,
        /// Maximum number of entries to return
        #[arg(long, default_value = "100")]
        limit: i32,
        /// Offset for pagination
        #[arg(long, default_value = "0")]
        offset: i32,
    },
    /// List tags for fixed database
    #[command(name = "list-tags")]
    ListTags {
        /// Database ID (format: subscription_id:database_id)
        id: String,
    },
    /// Add a tag
    #[command(name = "add-tag")]
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
    /// Update all tags
    #[command(name = "update-tags")]
    UpdateTags {
        /// Database ID (format: subscription_id:database_id)
        id: String,
        /// JSON file with tags (use @filename or - for stdin)
        file: String,
    },
    /// Update specific tag
    #[command(name = "update-tag")]
    UpdateTag {
        /// Database ID (format: subscription_id:database_id)
        id: String,
        /// Tag key
        #[arg(long)]
        key: String,
        /// Tag value
        #[arg(long)]
        value: String,
    },
    /// Delete a tag
    #[command(name = "delete-tag")]
    DeleteTag {
        /// Database ID (format: subscription_id:database_id)
        id: String,
        /// Tag key
        #[arg(long)]
        key: String,
    },
}

/// Cloud Fixed Subscription Commands
#[derive(Subcommand, Debug)]
pub enum CloudFixedSubscriptionCommands {
    /// List all available fixed subscription plans
    #[command(name = "list-plans")]
    ListPlans {
        /// Filter by cloud provider (AWS, GCP, Azure)
        #[arg(long)]
        provider: Option<String>,
    },
    /// Get plans for a specific subscription
    #[command(name = "get-plans")]
    GetPlans {
        /// Subscription ID
        #[arg(long)]
        subscription: i32,
    },
    /// Get details of a specific plan
    #[command(name = "get-plan")]
    GetPlan {
        /// Plan ID
        id: i32,
    },
    /// List all fixed subscriptions
    List,
    /// Get details of a fixed subscription
    Get {
        /// Subscription ID
        id: i32,
    },
    /// Create a new fixed subscription
    Create {
        /// JSON file with subscription configuration (use @filename or - for stdin)
        file: String,
        /// Async operation options
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },
    /// Update fixed subscription
    Update {
        /// Subscription ID
        id: i32,
        /// JSON file with update configuration (use @filename or - for stdin)
        file: String,
        /// Async operation options
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },
    /// Delete a fixed subscription
    Delete {
        /// Subscription ID
        id: i32,
        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
        /// Async operation options
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },
    /// Get available Redis versions for fixed subscription
    #[command(name = "redis-versions")]
    RedisVersions {
        /// Subscription ID
        #[arg(long)]
        subscription: i32,
    },
}

/// Cloud Provider Account Commands
#[derive(Subcommand, Debug)]
pub enum CloudProviderAccountCommands {
    /// List all cloud provider accounts
    List,
    /// Get cloud provider account details
    Get {
        /// Cloud account ID
        account_id: i32,
    },
    /// Create a new cloud provider account
    Create {
        /// JSON file containing the cloud account configuration
        /// For GCP, this should be the service account JSON file
        /// Use @filename to read from file
        file: String,
        /// Async operation arguments
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },
    /// Update a cloud provider account
    Update {
        /// Cloud account ID
        account_id: i32,
        /// JSON file containing updated cloud account configuration
        /// Use @filename to read from file
        file: String,
        /// Async operation arguments
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },
    /// Delete a cloud provider account
    Delete {
        /// Cloud account ID
        account_id: i32,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
        /// Async operation arguments
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },
}

/// Cloud-specific commands (placeholder for now)
#[derive(Subcommand, Debug)]
pub enum CloudCommands {
    /// Account operations
    #[command(subcommand)]
    Account(CloudAccountCommands),

    /// Payment method operations
    #[command(subcommand, name = "payment-method")]
    PaymentMethod(CloudPaymentMethodCommands),

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
    /// Cloud provider account operations
    #[command(subcommand, name = "provider-account")]
    ProviderAccount(CloudProviderAccountCommands),
    /// Task operations
    #[command(subcommand)]
    Task(CloudTaskCommands),
    /// Network connectivity operations (VPC, PSC, TGW)
    #[command(subcommand)]
    Connectivity(CloudConnectivityCommands),
    /// Fixed database operations
    #[command(subcommand, name = "fixed-database")]
    FixedDatabase(CloudFixedDatabaseCommands),
    /// Fixed subscription operations
    #[command(subcommand, name = "fixed-subscription")]
    FixedSubscription(CloudFixedSubscriptionCommands),
    /// Workflow operations for multi-step tasks
    #[command(subcommand)]
    Workflow(CloudWorkflowCommands),
}
#[derive(Debug, Subcommand)]
pub enum CloudWorkflowCommands {
    /// List available workflows
    List,
    /// Complete subscription setup with optional database
    #[command(name = "subscription-setup")]
    SubscriptionSetup(crate::workflows::cloud::subscription_setup::SubscriptionSetupArgs),
}

/// Enterprise workflow commands
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
pub enum CloudPaymentMethodCommands {
    /// List payment methods configured for the account
    List,
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
    #[command(after_help = "EXAMPLES:
    # Simple subscription - just name, provider, and region via --data
    redisctl cloud subscription create --name prod-subscription \\
      --data '{\"cloudProviders\":[{\"regions\":[{\"region\":\"us-east-1\"}]}],\"databases\":[{\"name\":\"db1\",\"memoryLimitInGb\":1}]}'

    # With payment method
    redisctl cloud subscription create --name dev-subscription \\
      --payment-method marketplace \\
      --data '{\"cloudProviders\":[{\"regions\":[{\"region\":\"us-west-2\"}]}],\"databases\":[{\"name\":\"db1\",\"memoryLimitInGb\":1}]}'

    # With auto-tiering (RAM+Flash)
    redisctl cloud subscription create --name large-subscription \\
      --memory-storage ram-and-flash \\
      --data '{\"cloudProviders\":[{\"provider\":\"AWS\",\"regions\":[{\"region\":\"eu-west-1\"}]}],\"databases\":[{\"name\":\"db1\",\"memoryLimitInGb\":10}]}'

    # Complete configuration from file
    redisctl cloud subscription create --data @subscription.json

    # Dry run to preview deployment
    redisctl cloud subscription create --dry-run --data @subscription.json

NOTE: Subscription creation requires complex nested structures for cloud providers,
      regions, and databases. Use --data for the required cloudProviders and databases
      arrays. First-class parameters (--name, --payment-method, etc.) override values
      in --data when both are provided.")]
    Create {
        /// Subscription name
        #[arg(long)]
        name: Option<String>,

        /// Dry run - create deployment plan without provisioning resources
        #[arg(long)]
        dry_run: bool,

        /// Deployment type: single-region or active-active
        #[arg(long, value_parser = ["single-region", "active-active"])]
        deployment_type: Option<String>,

        /// Payment method: credit-card or marketplace
        #[arg(long, value_parser = ["credit-card", "marketplace"], default_value = "credit-card")]
        payment_method: String,

        /// Payment method ID (required if payment-method is credit-card)
        #[arg(long)]
        payment_method_id: Option<i32>,

        /// Memory storage: ram or ram-and-flash (Auto Tiering)
        #[arg(long, value_parser = ["ram", "ram-and-flash"], default_value = "ram")]
        memory_storage: String,

        /// Persistent storage encryption: cloud-provider-managed-key or customer-managed-key
        #[arg(long, value_parser = ["cloud-provider-managed-key", "customer-managed-key"], default_value = "cloud-provider-managed-key")]
        persistent_storage_encryption: String,

        /// Advanced: Full subscription configuration as JSON string or @file.json
        /// REQUIRED: Must include cloudProviders array with regions and databases array
        #[arg(long)]
        data: Option<String>,

        /// Async operation options
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },

    /// Update subscription configuration
    Update {
        /// Subscription ID
        id: u32,
        /// Update configuration as JSON string or @file.json
        #[arg(long)]
        data: String,
        /// Async operation options
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },

    /// Delete a subscription
    Delete {
        /// Subscription ID
        id: u32,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
        /// Async operation options
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
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
    #[command(after_help = "EXAMPLES:
    # Simple database - just name and size
    redisctl cloud database create --subscription 123 --name mydb --memory 1

    # Production database with high availability
    redisctl cloud database create \\
      --subscription 123 \\
      --name prod-cache \\
      --memory 10 \\
      --replication \\
      --data-persistence aof-every-1-second

    # Advanced: Mix flags with JSON for rare options
    redisctl cloud database create \\
      --subscription 123 \\
      --name mydb \\
      --memory 5 \\
      --data '{\"modules\": [{\"name\": \"RedisJSON\"}]}'
")]
    Create {
        /// Subscription ID
        #[arg(long)]
        subscription: u32,

        /// Database name (required unless using --data)
        /// Limited to 40 characters: letters, digits, hyphens
        /// Must start with letter, end with letter or digit
        #[arg(long)]
        name: Option<String>,

        /// Memory limit in GB (e.g., 1, 5, 10, 50)
        /// Alternative to --dataset-size
        #[arg(long, conflicts_with = "dataset_size")]
        memory: Option<f64>,

        /// Dataset size in GB (alternative to --memory)
        /// If replication enabled, total memory will be 2x this value
        #[arg(long, conflicts_with = "memory")]
        dataset_size: Option<f64>,

        /// Database protocol
        #[arg(long, value_parser = ["redis", "memcached"], default_value = "redis")]
        protocol: String,

        /// Enable replication for high availability
        #[arg(long)]
        replication: bool,

        /// Data persistence policy
        /// Options: none, aof-every-1-second, aof-every-write, snapshot-every-1-hour,
        ///          snapshot-every-6-hours, snapshot-every-12-hours
        #[arg(long)]
        data_persistence: Option<String>,

        /// Data eviction policy when memory limit reached
        /// Options: volatile-lru, volatile-ttl, volatile-random, allkeys-lru,
        ///          allkeys-lfu, allkeys-random, noeviction, volatile-lfu
        #[arg(long, default_value = "volatile-lru")]
        eviction_policy: String,

        /// Redis version (e.g., "7.2", "7.0", "6.2")
        #[arg(long)]
        redis_version: Option<String>,

        /// Enable OSS Cluster API support
        #[arg(long)]
        oss_cluster: bool,

        /// TCP port (10000-19999, auto-assigned if not specified)
        #[arg(long)]
        port: Option<i32>,

        /// Advanced: Full database configuration as JSON string or @file.json
        /// CLI flags take precedence over values in JSON
        #[arg(long)]
        data: Option<String>,

        /// Async operation options
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },

    /// Update database configuration
    Update {
        /// Database ID (format: subscription_id:database_id)
        id: String,
        /// Update configuration as JSON string or @file.json
        #[arg(long)]
        data: String,
        /// Async operation options
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },

    /// Delete a database
    Delete {
        /// Database ID (format: subscription_id:database_id)
        id: String,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
        /// Async operation options
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
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
        /// Async operation options
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
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
        /// Async operation options
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
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

    /// Update user information
    Update {
        /// User ID
        id: u32,
        /// New name for the user
        #[arg(long)]
        name: Option<String>,
        /// New role for the user (owner, manager, viewer, billing_admin)
        #[arg(long)]
        role: Option<String>,
        /// Enable/disable email alerts
        #[arg(long)]
        alerts_email: Option<bool>,
        /// Enable/disable SMS alerts
        #[arg(long)]
        alerts_sms: Option<bool>,
    },

    /// Delete a user
    Delete {
        /// User ID
        id: u32,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
        /// Async operation arguments
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
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
        /// Async operation options
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
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
        /// Async operation options
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },

    /// Delete a Redis ACL rule
    #[command(name = "delete-redis-rule")]
    DeleteRedisRule {
        /// Rule ID
        id: i32,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
        /// Async operation options
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
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
        /// Async operation options
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
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
        /// Async operation options
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },

    /// Delete an ACL role
    #[command(name = "delete-role")]
    DeleteRole {
        /// Role ID
        id: i32,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
        /// Async operation options
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
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
        /// Async operation options
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
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
        /// Async operation options
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },

    /// Delete an ACL user
    #[command(name = "delete-acl-user")]
    DeleteAclUser {
        /// ACL user ID
        id: i32,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
        /// Async operation options
        #[command(flatten)]
        async_ops: crate::commands::cloud::async_utils::AsyncOperationArgs,
    },
}
