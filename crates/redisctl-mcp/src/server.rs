//! MCP server implementation for Redis Cloud and Enterprise

use std::sync::Arc;

use rmcp::{
    ErrorData as RmcpError, RoleServer, ServerHandler, handler::server::router::tool::ToolRouter,
    handler::server::wrapper::Parameters, model::*, schemars, service::RequestContext, tool,
    tool_handler, tool_router,
};
use serde::Deserialize;
use tokio::sync::RwLock;
use tracing::{debug, info};

use crate::cloud_tools::CloudTools;
use crate::database_tools::{DatabaseTools, is_write_command, value_to_json};
use crate::enterprise_tools::EnterpriseTools;

/// Configuration for the MCP server
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Profile name to use for credentials
    pub profile: Option<String>,
    /// Whether the server is in read-only mode
    pub read_only: bool,
}

/// MCP server for Redis Cloud and Enterprise management
///
/// This server exposes Redis Cloud and Enterprise operations as MCP tools
/// that can be invoked by AI systems.
#[derive(Clone)]
pub struct RedisCtlMcp {
    config: Arc<ServerConfig>,
    tool_router: ToolRouter<RedisCtlMcp>,
    cloud_tools: Arc<RwLock<Option<CloudTools>>>,
    enterprise_tools: Arc<RwLock<Option<EnterpriseTools>>>,
    database_tools: Arc<RwLock<Option<DatabaseTools>>>,
}

// Parameter structs for tools that need arguments

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SubscriptionIdParam {
    /// The subscription ID
    pub subscription_id: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DatabaseIdParam {
    /// The subscription ID
    pub subscription_id: i64,
    /// The database ID
    pub database_id: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TaskIdParam {
    /// The task ID
    pub task_id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct NodeIdParam {
    /// The node ID
    pub node_id: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct EnterpriseDatabaseIdParam {
    /// The database ID (uid)
    pub database_id: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateEnterpriseDatabaseParam {
    /// Name for the new database
    pub name: String,
    /// Memory size in MB (default: 100)
    #[serde(default)]
    pub memory_size_mb: Option<u64>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UpdateEnterpriseDatabaseParam {
    /// The database ID (uid) to update
    pub database_id: i64,
    /// Memory size in bytes (optional)
    #[serde(default)]
    pub memory_size: Option<u64>,
    /// Replication enabled (optional)
    #[serde(default)]
    pub replication: Option<bool>,
    /// Data persistence setting: disabled, aof, snapshot, aof-and-snapshot (optional)
    #[serde(default)]
    pub data_persistence: Option<String>,
    /// Eviction policy (optional)
    #[serde(default)]
    pub eviction_policy: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ExportDatabaseParam {
    /// The database ID (uid)
    pub database_id: i64,
    /// Export location (e.g., S3 URL, FTP URL, or local path)
    pub export_location: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ImportDatabaseParam {
    /// The database ID (uid)
    pub database_id: i64,
    /// Import location (e.g., S3 URL, FTP URL, or local path)
    pub import_location: String,
    /// Whether to flush the database before importing (default: false)
    #[serde(default)]
    pub flush_before_import: bool,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct RestoreDatabaseParam {
    /// The database ID (uid)
    pub database_id: i64,
    /// Specific backup UID to restore from (optional, uses latest if not specified)
    #[serde(default)]
    pub backup_uid: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UpdateClusterParam {
    /// Cluster name (optional)
    #[serde(default)]
    pub name: Option<String>,
    /// Enable/disable email alerts (optional)
    #[serde(default)]
    pub email_alerts: Option<bool>,
    /// Enable/disable rack awareness (optional)
    #[serde(default)]
    pub rack_aware: Option<bool>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UpdateNodeParam {
    /// The node ID (uid)
    pub node_id: i64,
    /// Whether the node accepts new shards (optional)
    #[serde(default)]
    pub accept_servers: Option<bool>,
    /// External IP addresses (optional)
    #[serde(default)]
    pub external_addr: Option<Vec<String>>,
    /// Rack ID where node is installed (optional)
    #[serde(default)]
    pub rack_id: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ShardIdParam {
    /// The shard UID (e.g., "1:1" for database 1, shard 1)
    pub shard_uid: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AlertIdParam {
    /// The alert UID
    pub alert_uid: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UserIdParam {
    /// The user ID (uid)
    pub user_id: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateUserParam {
    /// User's email address (used as login)
    pub email: String,
    /// User's password
    pub password: String,
    /// User's role (e.g., "admin", "db_viewer", "db_member")
    pub role: String,
    /// User's display name (optional)
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct RoleIdParam {
    /// The role ID (uid)
    pub role_id: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateRoleParam {
    /// Role name
    pub name: String,
    /// Management level (e.g., "admin", "db_viewer", "db_member") (optional)
    #[serde(default)]
    pub management: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AclIdParam {
    /// The Redis ACL ID (uid)
    pub acl_id: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateAclParam {
    /// ACL name
    pub name: String,
    /// ACL rules string (e.g., "+@all ~*")
    pub acl: String,
    /// Description (optional)
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ModuleIdParam {
    /// The module UID (e.g., "bf" for RedisBloom)
    pub module_uid: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CrdbGuidParam {
    /// The CRDB GUID (globally unique identifier)
    pub crdb_guid: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct UpdateCrdbParam {
    /// The CRDB GUID
    pub crdb_guid: String,
    /// New memory size in bytes (optional)
    #[serde(default)]
    pub memory_size: Option<u64>,
    /// Enable/disable encryption (optional)
    #[serde(default)]
    pub encryption: Option<bool>,
    /// Data persistence setting (optional)
    #[serde(default)]
    pub data_persistence: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DebugInfoTaskIdParam {
    /// The debug info task ID
    pub task_id: String,
}

// Cloud-specific parameter structs

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CloudProviderParam {
    /// Cloud provider filter (AWS, GCP, or Azure). Optional.
    #[serde(default)]
    pub provider: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateProSubscriptionParam {
    /// JSON payload for creating a Pro subscription. See Redis Cloud API docs for schema.
    pub request: serde_json::Value,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateEssentialsSubscriptionParam {
    /// Name for the new Essentials subscription
    pub name: String,
    /// Plan ID from cloud_essentials_plans_list
    pub plan_id: i64,
    /// Payment method ID (optional, use cloud_payment_methods_get to list available methods)
    #[serde(default)]
    pub payment_method_id: Option<i64>,
}

// Enterprise - LDAP Mapping parameter structs

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct LdapMappingIdParam {
    /// The LDAP mapping ID (uid)
    pub mapping_id: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CreateLdapMappingParam {
    /// Name for the LDAP mapping
    pub name: String,
    /// LDAP group distinguished name
    pub dn: String,
    /// Role identifier to map to
    pub role: String,
    /// Email address for alerts (optional)
    #[serde(default)]
    pub email: Option<String>,
}

// Enterprise - Job Scheduler parameter structs

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct JobIdParam {
    /// The scheduled job ID
    pub job_id: String,
}

// Enterprise - Proxy parameter structs

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ProxyIdParam {
    /// The proxy ID (uid)
    pub proxy_id: i64,
}

// Enterprise - Endpoint parameter structs

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct EndpointIdParam {
    /// The endpoint ID (uid)
    pub endpoint_id: String,
}

// Enterprise - Diagnostics parameter structs

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DiagnosticReportIdParam {
    /// The diagnostic report ID
    pub report_id: String,
}

// Cloud - Essentials Database parameter structs

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct EssentialsDatabaseIdParam {
    /// The Essentials subscription ID
    pub subscription_id: i64,
    /// The database ID
    pub database_id: i64,
}

// Cloud - VPC Peering parameter structs

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct VpcPeeringIdParam {
    /// The subscription ID
    pub subscription_id: i64,
    /// The VPC peering ID
    pub peering_id: i64,
}

// Cloud - Cloud Account parameter structs

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CloudAccountIdParam {
    /// The cloud account ID
    pub account_id: i64,
}

// Enterprise - CRDB Task parameter structs

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CrdbTaskIdParam {
    /// The CRDB task ID
    pub task_id: String,
}

// Cloud - Transit Gateway parameter structs

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TransitGatewayAttachmentIdParam {
    /// The subscription ID
    pub subscription_id: i64,
    /// The Transit Gateway attachment ID
    pub attachment_id: String,
}

// Enterprise - BDB Groups parameter structs

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct BdbGroupIdParam {
    /// The BDB group UID
    pub uid: i64,
}

// Enterprise - DNS Suffix parameter structs

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SuffixNameParam {
    /// The DNS suffix name
    pub name: String,
}

// Database tools parameter structs

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DatabaseExecuteParam {
    /// Redis command to execute (e.g., "GET", "INFO", "SCAN")
    pub command: String,
    /// Command arguments
    #[serde(default)]
    pub args: Vec<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DatabasePipelineCommand {
    /// Redis command to execute (e.g., "SET", "HSET", "JSON.SET")
    pub command: String,
    /// Command arguments
    #[serde(default)]
    pub args: Vec<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DatabasePipelineParam {
    /// Array of commands to execute in the pipeline
    pub commands: Vec<DatabasePipelineCommand>,
    /// Whether to execute atomically with MULTI/EXEC (default: false)
    #[serde(default)]
    pub atomic: bool,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DatabaseInfoParam {
    /// INFO section to retrieve (e.g., "server", "memory", "stats"). Optional, returns all sections if not specified.
    #[serde(default)]
    pub section: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DatabaseScanParam {
    /// Pattern to match keys (default: "*")
    #[serde(default = "default_scan_pattern")]
    pub pattern: String,
    /// Maximum number of keys to return (default: 100)
    #[serde(default = "default_scan_count")]
    pub count: usize,
}

fn default_scan_pattern() -> String {
    "*".to_string()
}

fn default_scan_count() -> usize {
    100
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DatabaseKeyParam {
    /// Redis key name
    pub key: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DatabaseSlowlogParam {
    /// Number of slowlog entries to return (default: 10)
    #[serde(default = "default_slowlog_count")]
    pub count: Option<usize>,
}

fn default_slowlog_count() -> Option<usize> {
    Some(10)
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DatabaseConfigGetParam {
    /// Pattern to match configuration parameters (e.g., "*", "max*", "timeout")
    pub pattern: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DatabaseLrangeParam {
    /// Redis key name
    pub key: String,
    /// Start index (0-based, negative values count from end)
    #[serde(default)]
    pub start: isize,
    /// Stop index (inclusive, negative values count from end, -1 means end)
    #[serde(default = "default_lrange_stop")]
    pub stop: isize,
}

fn default_lrange_stop() -> isize {
    -1
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DatabaseZrangeParam {
    /// Redis key name
    pub key: String,
    /// Start index (0-based)
    #[serde(default)]
    pub start: isize,
    /// Stop index (inclusive, -1 means end)
    #[serde(default = "default_zrange_stop")]
    pub stop: isize,
}

fn default_zrange_stop() -> isize {
    -1
}

// ========== WRITE OPERATION PARAMS ==========

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DatabaseSetParam {
    /// Redis key name
    pub key: String,
    /// Value to set
    pub value: String,
    /// Expiration time in seconds (optional, mutually exclusive with px)
    #[serde(default)]
    pub ex: Option<u64>,
    /// Expiration time in milliseconds (optional, mutually exclusive with ex)
    #[serde(default)]
    pub px: Option<u64>,
    /// Only set if key does not exist (optional)
    #[serde(default)]
    pub nx: bool,
    /// Only set if key already exists (optional)
    #[serde(default)]
    pub xx: bool,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DatabaseDelParam {
    /// Redis key(s) to delete - can be a single key or multiple keys
    pub keys: Vec<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DatabaseExpireParam {
    /// Redis key name
    pub key: String,
    /// Expiration time in seconds
    pub seconds: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DatabaseIncrbyParam {
    /// Redis key name
    pub key: String,
    /// Amount to increment by (can be negative to decrement)
    pub increment: i64,
}

// ========== HASH WRITE PARAMS ==========

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DatabaseHsetParam {
    /// Redis key name for the hash
    pub key: String,
    /// Field name within the hash
    pub field: String,
    /// Value to set
    pub value: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DatabaseHsetMultipleParam {
    /// Redis key name for the hash
    pub key: String,
    /// Field-value pairs to set (array of objects with "field" and "value" properties)
    pub fields: Vec<FieldValuePair>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FieldValuePair {
    /// Field name
    pub field: String,
    /// Field value
    pub value: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DatabaseHdelParam {
    /// Redis key name for the hash
    pub key: String,
    /// Field(s) to delete from the hash
    pub fields: Vec<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DatabaseHgetParam {
    /// Redis key name for the hash
    pub key: String,
    /// Field name to get
    pub field: String,
}

// ========== LIST WRITE PARAMS ==========

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DatabaseListPushParam {
    /// Redis key name for the list
    pub key: String,
    /// Value(s) to push onto the list
    pub values: Vec<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DatabaseLindexParam {
    /// Redis key name for the list
    pub key: String,
    /// Index to get (0-based, negative counts from end)
    pub index: isize,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DatabaseLsetParam {
    /// Redis key name for the list
    pub key: String,
    /// Index to set (0-based, negative counts from end)
    pub index: isize,
    /// Value to set at the index
    pub value: String,
}

// ========== SET WRITE PARAMS ==========

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DatabaseSetMembersParam {
    /// Redis key name for the set
    pub key: String,
    /// Member(s) to add or remove
    pub members: Vec<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DatabaseSismemberParam {
    /// Redis key name for the set
    pub key: String,
    /// Member to check
    pub member: String,
}

// ========== SORTED SET PARAMS ==========

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DatabaseZaddParam {
    /// Redis key name for the sorted set
    pub key: String,
    /// Members with scores to add (array of objects with "score" and "member" properties)
    pub members: Vec<ScoreMemberPair>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ScoreMemberPair {
    /// Score for the member
    pub score: f64,
    /// Member value
    pub member: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DatabaseZremParam {
    /// Redis key name for the sorted set
    pub key: String,
    /// Member(s) to remove
    pub members: Vec<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DatabaseZscoreParam {
    /// Redis key name for the sorted set
    pub key: String,
    /// Member to get the score for
    pub member: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DatabaseZrangebyscoreParam {
    /// Redis key name for the sorted set
    pub key: String,
    /// Minimum score (use "-inf" for negative infinity)
    pub min: String,
    /// Maximum score (use "+inf" for positive infinity)
    pub max: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DatabaseZincrbyParam {
    /// Redis key name for the sorted set
    pub key: String,
    /// Member to increment
    pub member: String,
    /// Amount to increment the score by (can be negative)
    pub increment: f64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DatabaseRenameParam {
    /// Current key name
    pub key: String,
    /// New key name
    pub new_key: String,
}

// ========== REDISEARCH PARAMS ==========

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FtSearchParam {
    /// Index name to search
    pub index: String,
    /// Search query (e.g., "@title:hello @body:world", "*" for all)
    pub query: String,
    /// Return only document IDs, not content
    #[serde(default)]
    pub nocontent: bool,
    /// Disable stemming - search terms exactly as provided
    #[serde(default)]
    pub verbatim: bool,
    /// Include relevance scores in results
    #[serde(default)]
    pub withscores: bool,
    /// Fields to return (if not specified, returns all fields)
    #[serde(default)]
    pub return_fields: Option<Vec<String>>,
    /// Field to sort results by
    #[serde(default)]
    pub sortby: Option<String>,
    /// Sort in descending order (default is ascending)
    #[serde(default)]
    pub sortby_desc: bool,
    /// Number of results to skip (for pagination)
    #[serde(default)]
    pub limit_offset: Option<i64>,
    /// Maximum number of results to return (default: 10)
    #[serde(default)]
    pub limit_num: Option<i64>,
    /// Fields to highlight matches in
    #[serde(default)]
    pub highlight_fields: Option<Vec<String>>,
    /// Opening tag for highlighting (e.g., "<b>")
    #[serde(default)]
    pub highlight_open: Option<String>,
    /// Closing tag for highlighting (e.g., "</b>")
    #[serde(default)]
    pub highlight_close: Option<String>,
    /// Language for stemming (e.g., "english", "spanish", "chinese")
    #[serde(default)]
    pub language: Option<String>,
    /// Maximum distance between query terms for phrase matching
    #[serde(default)]
    pub slop: Option<i64>,
    /// Require query terms to appear in order
    #[serde(default)]
    pub inorder: bool,
    /// Query timeout in milliseconds
    #[serde(default)]
    pub timeout: Option<i64>,
    /// Query dialect version (1, 2, or 3)
    #[serde(default)]
    pub dialect: Option<i32>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FtAggregateParam {
    /// Index name to aggregate
    pub index: String,
    /// Base filter query (e.g., "@category:{electronics}", "*" for all)
    pub query: String,
    /// Disable stemming
    #[serde(default)]
    pub verbatim: bool,
    /// Fields to load from documents (empty array loads all)
    #[serde(default)]
    pub load: Option<Vec<String>>,
    /// GROUPBY clauses with REDUCE functions
    #[serde(default)]
    pub groupby: Vec<FtGroupByParam>,
    /// APPLY transformations
    #[serde(default)]
    pub apply: Vec<FtApplyParam>,
    /// Sort by fields with direction (e.g., [["@count", "DESC"]])
    #[serde(default)]
    pub sortby: Option<Vec<Vec<String>>>,
    /// Maximum results for SORTBY optimization
    #[serde(default)]
    pub sortby_max: Option<i64>,
    /// Post-aggregation filter expression
    #[serde(default)]
    pub filter: Option<String>,
    /// Number of results to skip
    #[serde(default)]
    pub limit_offset: Option<i64>,
    /// Maximum number of results
    #[serde(default)]
    pub limit_num: Option<i64>,
    /// Query timeout in milliseconds
    #[serde(default)]
    pub timeout: Option<i64>,
    /// Query dialect version
    #[serde(default)]
    pub dialect: Option<i32>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FtGroupByParam {
    /// Properties to group by (e.g., ["@category", "@brand"])
    pub properties: Vec<String>,
    /// Reducer functions to apply
    #[serde(default)]
    pub reducers: Vec<FtReducerParam>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FtReducerParam {
    /// Reducer function (COUNT, SUM, AVG, MIN, MAX, COUNT_DISTINCT, etc.)
    pub function: String,
    /// Arguments for the reducer (e.g., ["@price"] for SUM)
    #[serde(default)]
    pub args: Vec<String>,
    /// Alias for the result
    #[serde(default)]
    pub alias: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FtApplyParam {
    /// Expression to apply (e.g., "sqrt(@foo)/log(@bar)")
    pub expression: String,
    /// Name for the result
    pub alias: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FtIndexParam {
    /// Index name
    pub index: String,
}

// ========== REDISEARCH INDEX MANAGEMENT PARAMS ==========

/// Schema field definition for FT.CREATE. Defines how a field should be indexed and queried.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FtSchemaField {
    /// Field name in HASH, or JSONPath for JSON documents (e.g., "title" or "$.product.name").
    /// For JSON, use JSONPath syntax starting with "$."
    pub name: String,

    /// Alias for the field in queries. If provided, use this name in search queries instead of
    /// the original field name. Useful for giving short names to long JSONPath expressions.
    #[serde(default)]
    pub alias: Option<String>,

    /// Field type determining indexing and query behavior:
    /// - "TEXT": Full-text search with stemming, tokenization, and scoring. Best for: titles,
    ///   descriptions, content. Supports: fuzzy matching, prefix search, phonetic matching.
    /// - "TAG": Exact-match on comma-separated or custom-delimited values. Best for: categories,
    ///   tags, status, IDs. Faster than TEXT for exact matches. Case-insensitive by default.
    /// - "NUMERIC": Numeric range queries and sorting. Best for: prices, quantities, timestamps,
    ///   ratings. Supports: >, <, >=, <=, == operators.
    /// - "GEO": Geospatial queries within radius. Store as "longitude,latitude" string.
    ///   Supports: GEOFILTER for distance-based search.
    /// - "VECTOR": Vector similarity search for AI/ML embeddings. Requires: vector_algorithm,
    ///   vector_dim, vector_distance_metric, vector_type. Used for semantic search.
    /// - "GEOSHAPE": Polygon/geometry queries. For complex geographic boundaries.
    pub field_type: String,

    /// Enable sorting by this field. Adds memory overhead but allows SORTBY in queries.
    /// Recommended for fields you'll frequently sort on (e.g., date, price, rating).
    #[serde(default)]
    pub sortable: bool,

    /// Store field without indexing. Field can be returned in results but not searched.
    /// Useful for display-only data like images URLs or formatted text.
    #[serde(default)]
    pub noindex: bool,

    /// (TEXT only) Disable stemming for exact token matching. Use for: proper names, product
    /// codes, URLs, email addresses - anything that shouldn't be stemmed.
    #[serde(default)]
    pub nostem: bool,

    /// (TEXT only) Enable phonetic matching for sounds-like search. Values:
    /// - "dm:en": Double Metaphone for English
    /// - "dm:fr": Double Metaphone for French
    /// - "dm:pt": Double Metaphone for Portuguese
    /// - "dm:es": Double Metaphone for Spanish
    ///
    /// Useful for name search where spelling varies.
    #[serde(default)]
    pub phonetic: Option<String>,

    /// (TEXT only) Field importance weight (default: 1.0). Higher weights boost relevance
    /// scores when field matches. Example: weight title at 2.0, description at 1.0.
    #[serde(default)]
    pub weight: Option<f64>,

    /// (TAG only) Character separating tag values (default: ","). Common alternatives: "|", ";".
    /// Example: with separator="|", "red|blue|green" becomes three separate tags.
    #[serde(default)]
    pub separator: Option<String>,

    /// (TAG only) Enable case-sensitive matching. By default, tags are case-insensitive.
    /// Enable when tag values are case-significant (e.g., ticker symbols, codes).
    #[serde(default)]
    pub casesensitive: bool,

    /// (TEXT only) Optimize for suffix and contains queries. Adds memory overhead but enables
    /// efficient *suffix and *contains* searches. Use sparingly on high-cardinality fields.
    #[serde(default)]
    pub withsuffixtrie: bool,

    /// (VECTOR only) Indexing algorithm: "FLAT" for brute-force (exact, slower), "HNSW" for
    /// approximate nearest neighbor (faster, uses more memory). HNSW recommended for >10K vectors.
    #[serde(default)]
    pub vector_algorithm: Option<String>,

    /// (VECTOR only) Number of dimensions in the vector. Must match your embedding model output
    /// (e.g., 384 for MiniLM, 768 for BERT, 1536 for OpenAI ada-002).
    #[serde(default)]
    pub vector_dim: Option<i64>,

    /// (VECTOR only) Distance metric for similarity:
    /// - "L2": Euclidean distance (lower = more similar)
    /// - "COSINE": Cosine similarity (higher = more similar) - best for normalized embeddings
    /// - "IP": Inner product (higher = more similar)
    #[serde(default)]
    pub vector_distance_metric: Option<String>,

    /// (VECTOR only) Vector element data type: "FLOAT32" (default), "FLOAT64", "BFLOAT16".
    /// FLOAT32 balances precision and memory. BFLOAT16 saves memory with slight precision loss.
    #[serde(default)]
    pub vector_type: Option<String>,
}

fn default_on_hash() -> String {
    "HASH".to_string()
}

/// Parameters for FT.CREATE - create a new RediSearch index with schema definition.
/// This is the most important RediSearch command for setting up full-text search.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FtCreateParam {
    /// Name for the new index. Use descriptive names like "idx:products" or "search:users".
    /// Index names should follow your key naming convention for clarity.
    pub index: String,

    /// Data structure to index: "HASH" (default) or "JSON".
    /// - HASH: Traditional Redis hashes, fields accessed by name
    /// - JSON: RedisJSON documents, fields accessed by JSONPath
    #[serde(default = "default_on_hash")]
    pub on: String,

    /// Key prefixes to index. Only keys matching these prefixes will be indexed.
    /// Examples: ["product:", "item:"] indexes all keys starting with product: or item:
    /// If not specified, indexes keys matching the default prefix.
    #[serde(default)]
    pub prefixes: Option<Vec<String>>,

    /// Filter expression to selectively index documents. Uses the same syntax as FT.SEARCH.
    /// Example: "@status:active" only indexes documents where status field equals "active".
    /// Useful for creating partial indexes to reduce memory usage.
    #[serde(default)]
    pub filter: Option<String>,

    /// Default language for TEXT field stemming. Affects how words are normalized for search.
    /// Options: "arabic", "armenian", "basque", "catalan", "danish", "dutch", "english",
    /// "finnish", "french", "german", "greek", "hindi", "hungarian", "indonesian", "irish",
    /// "italian", "lithuanian", "nepali", "norwegian", "portuguese", "romanian", "russian",
    /// "serbian", "spanish", "swedish", "tamil", "turkish", "yiddish", "chinese".
    #[serde(default)]
    pub language: Option<String>,

    /// Document field containing per-document language override. Allows multilingual indexes
    /// where each document specifies its own language for proper stemming.
    #[serde(default)]
    pub language_field: Option<String>,

    /// Default relevance score for documents (0.0-1.0). Used in ranking when no other
    /// scoring is specified. Higher scores rank documents higher in results.
    #[serde(default)]
    pub score: Option<f64>,

    /// Document field containing per-document score (0.0-1.0). Allows documents to have
    /// individual importance weights affecting their ranking in search results.
    #[serde(default)]
    pub score_field: Option<String>,

    /// Document field containing a binary payload retrievable with WITHPAYLOADS option.
    /// Useful for storing pre-computed data to return with search results.
    #[serde(default)]
    pub payload_field: Option<String>,

    /// Allow more than 32 TEXT fields in the schema. Increases memory usage but removes
    /// the default limit on text fields.
    #[serde(default)]
    pub maxtextfields: bool,

    /// Create a lightweight temporary index that expires after given seconds of inactivity.
    /// Useful for session-specific or cache-like search functionality.
    #[serde(default)]
    pub temporary: Option<i64>,

    /// Don't store term offsets. Saves memory but disables exact phrase queries and highlighting.
    /// Use when you only need simple keyword matching without phrase search.
    #[serde(default)]
    pub nooffsets: bool,

    /// Disable highlighting support. Saves memory when you don't need search term highlighting.
    #[serde(default)]
    pub nohl: bool,

    /// Don't store field bits. Saves memory but disables filtering by specific fields.
    /// Documents will still be searchable but you can't restrict search to specific fields.
    #[serde(default)]
    pub nofields: bool,

    /// Don't store term frequencies. Saves memory but affects relevance scoring accuracy.
    /// Use when you don't need TF-IDF based ranking.
    #[serde(default)]
    pub nofreqs: bool,

    /// Custom stopwords list. Stopwords are common words excluded from indexing (like "the", "a").
    /// Provide empty array [] to disable stopwords, or custom list like ["the", "a", "an"].
    /// If not specified, uses default English stopwords.
    #[serde(default)]
    pub stopwords: Option<Vec<String>>,

    /// Skip initial scan of existing keys. Index will only include documents created/modified
    /// after index creation. Useful when you'll be adding documents immediately after.
    #[serde(default)]
    pub skip_initial_scan: bool,

    /// Schema definition - array of field definitions. This is the core of index configuration.
    /// Each field specifies what data to index and how to index it. Order doesn't matter.
    pub schema: Vec<FtSchemaField>,
}

/// Parameters for FT.DROPINDEX - delete a RediSearch index.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FtDropIndexParam {
    /// Name of the index to delete
    pub index: String,

    /// Also delete the indexed documents (keys). WARNING: This permanently deletes the
    /// actual Redis keys, not just the index. Use with caution in production.
    /// If false (default), only the index is removed and documents remain.
    #[serde(default)]
    pub dd: bool,
}

/// Parameters for FT.ALTER - add new fields to an existing index schema.
/// Note: You can only ADD fields, not modify or remove existing ones.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FtAlterParam {
    /// Name of the index to modify
    pub index: String,

    /// Skip scanning existing documents for the new field. If true, only new/modified
    /// documents will have this field indexed. Useful for large indexes where rescanning
    /// would be expensive.
    #[serde(default)]
    pub skip_initial_scan: bool,

    /// New field definition to add to the schema. Uses same format as FtSchemaField.
    pub field: FtSchemaField,
}

/// Parameters for FT.EXPLAIN - get the execution plan for a query without running it.
/// Essential for debugging slow queries and understanding how RediSearch interprets your query.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FtExplainParam {
    /// Index name to explain query against
    pub index: String,

    /// Query to explain (same syntax as FT.SEARCH). Example: "@title:hello @body:world"
    /// The explanation shows how the query will be parsed and executed.
    pub query: String,

    /// Query dialect version (1, 2, 3, or 4). Different dialects have different query syntax.
    /// Use dialect 2 or higher for modern query features. Default depends on server config.
    #[serde(default)]
    pub dialect: Option<i32>,
}

/// Parameters for FT.TAGVALS - get all unique values for a TAG field.
/// Useful for building filter UIs, understanding data distribution, and debugging.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FtTagvalsParam {
    /// Index name
    pub index: String,

    /// Name of the TAG field to get values from. Must be a TAG type field in the schema.
    /// Returns all unique tag values that exist in indexed documents.
    pub field: String,
}

/// Parameters for FT.SPELLCHECK - get spelling suggestions for query terms.
/// Useful for "did you mean?" functionality in search interfaces.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FtSpellcheckParam {
    /// Index name (used as the term dictionary)
    pub index: String,

    /// Query to spellcheck. Each term in the query will be checked against the index vocabulary.
    pub query: String,

    /// Maximum Levenshtein distance for suggestions (1-4, default: 1). Higher values find
    /// more suggestions but may include less relevant ones. 1 finds typos like "helo" -> "hello".
    #[serde(default)]
    pub distance: Option<i32>,

    /// Query dialect version (1, 2, 3, or 4)
    #[serde(default)]
    pub dialect: Option<i32>,
}

// ========== REDISEARCH ALIAS PARAMS ==========

/// Parameters for FT.ALIASADD - create an alias pointing to an index.
/// Aliases allow zero-downtime index rebuilds by pointing queries to different indexes.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FtAliasAddParam {
    /// Alias name to create. Use this name in queries instead of the actual index name.
    /// Example: Create alias "products" pointing to "products_v1", later update to "products_v2".
    pub alias: String,

    /// Target index name that the alias will point to
    pub index: String,
}

/// Parameters for FT.ALIASDEL - delete an index alias.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FtAliasDelParam {
    /// Alias name to delete
    pub alias: String,
}

/// Parameters for FT.ALIASUPDATE - update an alias to point to a different index.
/// This is atomic - queries will instantly switch to the new index.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FtAliasUpdateParam {
    /// Alias name to update (will be created if doesn't exist)
    pub alias: String,

    /// New target index name
    pub index: String,
}

// ========== REDISEARCH AUTOCOMPLETE PARAMS ==========

/// Parameters for FT.SUGADD - add a suggestion to an autocomplete dictionary.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FtSugAddParam {
    /// Autocomplete dictionary key. This is a Redis key storing the suggestion trie.
    /// Example: "ac:products" for product name suggestions.
    pub key: String,

    /// Suggestion string to add. This is the text that will be suggested to users.
    pub string: String,

    /// Score/weight for this suggestion (higher = ranked higher in results).
    /// Use to prioritize popular or important suggestions.
    pub score: f64,

    /// Increment existing score instead of replacing. Useful for updating suggestion
    /// popularity based on usage (e.g., increment each time a suggestion is selected).
    #[serde(default)]
    pub incr: bool,

    /// Optional payload data to store with the suggestion. Retrieved with WITHPAYLOADS.
    /// Useful for storing metadata like IDs or categories with each suggestion.
    #[serde(default)]
    pub payload: Option<String>,
}

/// Parameters for FT.SUGGET - get autocomplete suggestions matching a prefix.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FtSugGetParam {
    /// Autocomplete dictionary key
    pub key: String,

    /// Prefix to complete. Returns suggestions starting with this string.
    /// Example: "lap" might return "laptop", "laptop case", "lap desk".
    pub prefix: String,

    /// Include fuzzy matches (Levenshtein distance 1). Helps with typos.
    /// Example: "laptp" would still match "laptop".
    #[serde(default)]
    pub fuzzy: bool,

    /// Maximum number of suggestions to return (default: 5)
    #[serde(default)]
    pub max: Option<i64>,

    /// Include suggestion scores in results
    #[serde(default)]
    pub withscores: bool,

    /// Include payloads in results (if stored with SUGADD)
    #[serde(default)]
    pub withpayloads: bool,
}

/// Parameters for FT.SUGDEL - delete a suggestion from an autocomplete dictionary.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FtSugDelParam {
    /// Autocomplete dictionary key
    pub key: String,

    /// Exact suggestion string to delete
    pub string: String,
}

/// Parameters for FT.SUGLEN - get the number of suggestions in an autocomplete dictionary.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FtSugLenParam {
    /// Autocomplete dictionary key
    pub key: String,
}

// ========== REDISEARCH SYNONYM PARAMS ==========

/// Parameters for FT.SYNDUMP - get all synonym groups from an index.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FtSynDumpParam {
    /// Index name
    pub index: String,
}

/// Parameters for FT.SYNUPDATE - add or update a synonym group.
/// Synonyms allow searching for one term to match documents containing related terms.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FtSynUpdateParam {
    /// Index name
    pub index: String,

    /// Synonym group ID. All terms in the same group are treated as synonyms.
    /// Example: group "color" might contain ["red", "crimson", "scarlet"].
    pub group_id: String,

    /// Skip scanning existing documents. If true, synonyms only apply to new documents.
    #[serde(default)]
    pub skip_initial_scan: bool,

    /// Terms to add to this synonym group. Searching for any term will match all terms in group.
    pub terms: Vec<String>,
}

// ========== REDISJSON PARAMS ==========

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct JsonGetParam {
    /// Key name
    pub key: String,
    /// JSONPath expressions (default: "$" for root)
    #[serde(default)]
    pub paths: Vec<String>,
    /// Indentation string for pretty printing
    #[serde(default)]
    pub indent: Option<String>,
    /// String to print at end of each line
    #[serde(default)]
    pub newline: Option<String>,
    /// String between key and value
    #[serde(default)]
    pub space: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct JsonSetParam {
    /// Key name
    pub key: String,
    /// JSONPath where to set the value (default: "$" for root)
    #[serde(default = "default_json_path")]
    pub path: String,
    /// JSON value to set (must be valid JSON)
    pub value: String,
    /// Only set if path does not exist
    #[serde(default)]
    pub nx: bool,
    /// Only set if path already exists
    #[serde(default)]
    pub xx: bool,
}

fn default_json_path() -> String {
    "$".to_string()
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct JsonDelParam {
    /// Key name
    pub key: String,
    /// JSONPath to delete (default: "$" deletes entire document)
    #[serde(default)]
    pub path: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct JsonPathParam {
    /// Key name
    pub key: String,
    /// JSONPath (default: "$")
    #[serde(default)]
    pub path: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct JsonArrAppendParam {
    /// Key name
    pub key: String,
    /// JSONPath to the array
    pub path: String,
    /// JSON values to append (each must be valid JSON)
    pub values: Vec<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct JsonNumIncrByParam {
    /// Key name
    pub key: String,
    /// JSONPath to the number
    pub path: String,
    /// Amount to increment by (can be negative)
    pub value: f64,
}

/// Parameters for JSON.MGET - get values from multiple keys at once.
/// Essential for batch operations when you need the same path from many documents.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct JsonMgetParam {
    /// Keys to get values from. All keys should contain JSON documents.
    /// Non-existent keys return null in the result array.
    pub keys: Vec<String>,

    /// JSONPath to extract from each document. The same path is applied to all keys.
    /// Use "$" for root, "$.field" for specific field, "$..field" for recursive.
    pub path: String,
}

/// Parameters for JSON.OBJKEYS - get all keys from a JSON object.
/// Useful for introspecting document structure and building dynamic UIs.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct JsonObjKeysParam {
    /// Key name containing the JSON document
    pub key: String,

    /// JSONPath to the object (default: "$" for root). Must point to an object, not array/scalar.
    #[serde(default = "default_json_path")]
    pub path: String,
}

/// Parameters for JSON.OBJLEN - get the number of keys in a JSON object.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct JsonObjLenParam {
    /// Key name containing the JSON document
    pub key: String,

    /// JSONPath to the object (default: "$" for root). Must point to an object.
    #[serde(default = "default_json_path")]
    pub path: String,
}

/// Parameters for JSON.ARRINDEX - find the index of an element in a JSON array.
/// Returns -1 if not found. Useful for checking if a value exists in an array.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct JsonArrIndexParam {
    /// Key name containing the JSON document
    pub key: String,

    /// JSONPath to the array
    pub path: String,

    /// JSON value to search for (must be valid JSON - use quotes for strings: "\"value\"")
    pub value: String,

    /// Start index for search (default: 0). Negative values count from end.
    #[serde(default)]
    pub start: Option<i64>,

    /// Stop index for search (default: end of array). Exclusive. Negative values count from end.
    #[serde(default)]
    pub stop: Option<i64>,
}

/// Parameters for JSON.ARRPOP - remove and return element from a JSON array.
/// Can pop from beginning, end, or specific index.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct JsonArrPopParam {
    /// Key name containing the JSON document
    pub key: String,

    /// JSONPath to the array
    #[serde(default = "default_json_path")]
    pub path: String,

    /// Index to pop from (default: -1, last element). Negative values count from end.
    /// Use 0 for first element, -1 for last element.
    #[serde(default)]
    pub index: Option<i64>,
}

/// Parameters for JSON.ARRTRIM - trim a JSON array to a specified range.
/// Elements outside the range are removed. Useful for maintaining bounded arrays.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct JsonArrTrimParam {
    /// Key name containing the JSON document
    pub key: String,

    /// JSONPath to the array
    pub path: String,

    /// Start index (inclusive). Negative values count from end.
    pub start: i64,

    /// Stop index (inclusive). Negative values count from end. Use -1 for last element.
    pub stop: i64,
}

/// Parameters for JSON.ARRINSERT - insert elements into a JSON array at a specific index.
/// Existing elements are shifted to make room.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct JsonArrInsertParam {
    /// Key name containing the JSON document
    pub key: String,

    /// JSONPath to the array
    pub path: String,

    /// Index to insert at. Elements at and after this index shift right.
    /// Negative values count from end (-1 = before last element).
    pub index: i64,

    /// JSON values to insert (each must be valid JSON)
    pub values: Vec<String>,
}

/// Parameters for JSON.CLEAR - clear arrays/objects or set numbers to 0.
/// For arrays: becomes empty []. For objects: becomes empty {}. For numbers: becomes 0.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct JsonClearParam {
    /// Key name containing the JSON document
    pub key: String,

    /// JSONPath to clear (default: "$" for root)
    #[serde(default = "default_json_path")]
    pub path: String,
}

/// Parameters for JSON.TOGGLE - toggle a boolean value.
/// true becomes false, false becomes true. Error if path is not a boolean.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct JsonToggleParam {
    /// Key name containing the JSON document
    pub key: String,

    /// JSONPath to the boolean value
    pub path: String,
}

// ========== REDISTIMESERIES PARAMS ==========

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TsAddParam {
    /// Time series key name
    pub key: String,
    /// Timestamp in milliseconds, or "*" for server time
    pub timestamp: String,
    /// Sample value
    pub value: f64,
    /// Retention period in milliseconds (only for new series)
    #[serde(default)]
    pub retention: Option<i64>,
    /// Encoding: COMPRESSED or UNCOMPRESSED
    #[serde(default)]
    pub encoding: Option<String>,
    /// Memory chunk size in bytes
    #[serde(default)]
    pub chunk_size: Option<i64>,
    /// Duplicate handling: BLOCK, FIRST, LAST, MIN, MAX, SUM
    #[serde(default)]
    pub on_duplicate: Option<String>,
    /// Labels as key-value pairs
    #[serde(default)]
    pub labels: Option<Vec<LabelPair>>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct LabelPair {
    /// Label name
    pub label: String,
    /// Label value
    pub value: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TsRangeParam {
    /// Time series key name
    pub key: String,
    /// Start timestamp ("-" for earliest)
    pub from: String,
    /// End timestamp ("+" for latest)
    pub to: String,
    /// Include latest possibly-partial bucket
    #[serde(default)]
    pub latest: bool,
    /// Filter by specific timestamps
    #[serde(default)]
    pub filter_by_ts: Option<Vec<i64>>,
    /// Minimum value filter
    #[serde(default)]
    pub filter_by_value_min: Option<f64>,
    /// Maximum value filter
    #[serde(default)]
    pub filter_by_value_max: Option<f64>,
    /// Maximum number of samples to return
    #[serde(default)]
    pub count: Option<i64>,
    /// Alignment for aggregation buckets
    #[serde(default)]
    pub align: Option<String>,
    /// Aggregation type (avg, sum, min, max, count, first, last, etc.)
    #[serde(default)]
    pub aggregation: Option<String>,
    /// Bucket duration in milliseconds
    #[serde(default)]
    pub bucket_duration: Option<i64>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct TsCreateParam {
    /// Time series key name
    pub key: String,
    /// Retention period in milliseconds
    #[serde(default)]
    pub retention: Option<i64>,
    /// Encoding: COMPRESSED or UNCOMPRESSED
    #[serde(default)]
    pub encoding: Option<String>,
    /// Memory chunk size in bytes
    #[serde(default)]
    pub chunk_size: Option<i64>,
    /// Duplicate policy: BLOCK, FIRST, LAST, MIN, MAX, SUM
    #[serde(default)]
    pub duplicate_policy: Option<String>,
    /// Labels as key-value pairs
    #[serde(default)]
    pub labels: Option<Vec<LabelPair>>,
}

// ========== REDISBLOOM PARAMS ==========

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct BfReserveParam {
    /// Bloom filter key name
    pub key: String,
    /// Desired false positive rate (0 to 1, e.g., 0.001 for 0.1%)
    pub error_rate: f64,
    /// Expected number of items
    pub capacity: u64,
    /// Sub-filter size multiplier when capacity reached (default: 2)
    #[serde(default)]
    pub expansion: Option<u32>,
    /// Prevent auto-scaling (returns error when full)
    #[serde(default)]
    pub nonscaling: bool,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct BfAddParam {
    /// Bloom filter key name
    pub key: String,
    /// Item to add
    pub item: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct BfMaddParam {
    /// Bloom filter key name
    pub key: String,
    /// Items to add
    pub items: Vec<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct BfExistsParam {
    /// Bloom filter key name
    pub key: String,
    /// Item to check
    pub item: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct BfMexistsParam {
    /// Bloom filter key name
    pub key: String,
    /// Items to check
    pub items: Vec<String>,
}

// ========== STREAMS PARAM STRUCTS ==========

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct StreamFieldPair {
    /// Field name
    pub field: String,
    /// Field value
    pub value: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct XaddParam {
    /// Stream key name
    pub key: String,
    /// Entry ID. Use "*" to auto-generate a unique ID (recommended). Or specify a timestamp-sequence ID like "1234567890123-0".
    #[serde(default = "default_xadd_id")]
    pub id: String,
    /// Field-value pairs for the stream entry (array of objects with "field" and "value" properties)
    pub fields: Vec<StreamFieldPair>,
    /// Maximum stream length. Old entries are evicted when exceeded.
    #[serde(default)]
    pub maxlen: Option<i64>,
    /// Use approximate trimming (~) for better performance. Only applies when maxlen is set.
    #[serde(default)]
    pub approximate: bool,
}

fn default_xadd_id() -> String {
    "*".to_string()
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct XreadParam {
    /// Stream key(s) to read from
    pub keys: Vec<String>,
    /// Starting ID(s) for each stream. Use "0" to read from beginning, "$" for only new entries. Must match keys length.
    pub ids: Vec<String>,
    /// Maximum number of entries to return per stream
    #[serde(default)]
    pub count: Option<i64>,
    /// Block for specified milliseconds waiting for new entries. Use 0 to block indefinitely.
    #[serde(default)]
    pub block: Option<i64>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct XrangeParam {
    /// Stream key name
    pub key: String,
    /// Start ID. Use "-" for the first entry, or a specific ID like "1234567890123-0".
    pub start: String,
    /// End ID. Use "+" for the last entry, or a specific ID like "1234567890123-0".
    pub end: String,
    /// Maximum number of entries to return
    #[serde(default)]
    pub count: Option<i64>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct XrevrangeParam {
    /// Stream key name
    pub key: String,
    /// End ID (note: reversed - this is the later ID). Use "+" for the last entry.
    pub end: String,
    /// Start ID (note: reversed - this is the earlier ID). Use "-" for the first entry.
    pub start: String,
    /// Maximum number of entries to return
    #[serde(default)]
    pub count: Option<i64>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct XinfoStreamParam {
    /// Stream key name
    pub key: String,
    /// Return full stream information including entries (more verbose)
    #[serde(default)]
    pub full: bool,
    /// Number of entries to return when using full mode
    #[serde(default)]
    pub count: Option<i64>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct XinfoConsumersParam {
    /// Stream key name
    pub key: String,
    /// Consumer group name
    pub group: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct XgroupCreateParam {
    /// Stream key name
    pub key: String,
    /// Consumer group name
    pub group: String,
    /// Starting ID for the group. Use "$" to only receive new messages, "0" to receive all existing messages.
    #[serde(default = "default_xgroup_id")]
    pub id: String,
    /// Create the stream if it doesn't exist
    #[serde(default)]
    pub mkstream: bool,
}

fn default_xgroup_id() -> String {
    "$".to_string()
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct XgroupDestroyParam {
    /// Stream key name
    pub key: String,
    /// Consumer group name to destroy
    pub group: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct XgroupDelconsumerParam {
    /// Stream key name
    pub key: String,
    /// Consumer group name
    pub group: String,
    /// Consumer name to delete
    pub consumer: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct XgroupSetidParam {
    /// Stream key name
    pub key: String,
    /// Consumer group name
    pub group: String,
    /// New last delivered ID for the group
    pub id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct XreadgroupParam {
    /// Consumer group name
    pub group: String,
    /// Consumer name (will be auto-created if doesn't exist)
    pub consumer: String,
    /// Stream key(s) to read from
    pub keys: Vec<String>,
    /// ID(s) for each stream. Use ">" to get only new messages, or a specific ID to replay pending messages.
    pub ids: Vec<String>,
    /// Maximum number of entries to return per stream
    #[serde(default)]
    pub count: Option<i64>,
    /// Block for specified milliseconds waiting for new entries. Use 0 to block indefinitely.
    #[serde(default)]
    pub block: Option<i64>,
    /// Don't add messages to the pending list (fire and forget mode)
    #[serde(default)]
    pub noack: bool,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct XackParam {
    /// Stream key name
    pub key: String,
    /// Consumer group name
    pub group: String,
    /// Message ID(s) to acknowledge
    pub ids: Vec<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct XdelParam {
    /// Stream key name
    pub key: String,
    /// Entry ID(s) to delete
    pub ids: Vec<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct XtrimParam {
    /// Stream key name
    pub key: String,
    /// Maximum stream length to trim to
    pub maxlen: i64,
    /// Use approximate trimming (~) for better performance
    #[serde(default)]
    pub approximate: bool,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct XpendingParam {
    /// Stream key name
    pub key: String,
    /// Consumer group name
    pub group: String,
    /// Start ID for range query (use "-" for first). Required with end and count for detailed output.
    #[serde(default)]
    pub start: Option<String>,
    /// End ID for range query (use "+" for last). Required with start and count for detailed output.
    #[serde(default)]
    pub end: Option<String>,
    /// Maximum number of entries to return. Required with start and end for detailed output.
    #[serde(default)]
    pub count: Option<i64>,
    /// Filter by specific consumer (optional, only with start/end/count)
    #[serde(default)]
    pub consumer: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct XclaimParam {
    /// Stream key name
    pub key: String,
    /// Consumer group name
    pub group: String,
    /// Consumer name to claim messages for
    pub consumer: String,
    /// Minimum idle time in milliseconds - only claim messages idle for at least this long
    pub min_idle_time: i64,
    /// Message ID(s) to claim
    pub ids: Vec<String>,
    /// Set the idle time (ms) of the message. If not specified, idle time is reset to 0.
    #[serde(default)]
    pub idle: Option<i64>,
    /// Set the internal message time to this Unix timestamp (ms)
    #[serde(default)]
    pub time: Option<i64>,
    /// Set the retry counter for the message
    #[serde(default)]
    pub retrycount: Option<i64>,
    /// Claim the message even if it's not in the pending list
    #[serde(default)]
    pub force: bool,
    /// Return just the message IDs, not the full messages
    #[serde(default)]
    pub justid: bool,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct XautoclaimParam {
    /// Stream key name
    pub key: String,
    /// Consumer group name
    pub group: String,
    /// Consumer name to claim messages for
    pub consumer: String,
    /// Minimum idle time in milliseconds - only claim messages idle for at least this long
    pub min_idle_time: i64,
    /// Start ID for scanning pending entries. Use "0-0" to start from the beginning.
    pub start: String,
    /// Maximum number of entries to claim
    #[serde(default)]
    pub count: Option<i64>,
    /// Return just the message IDs, not the full messages
    #[serde(default)]
    pub justid: bool,
}

// ========== PUB/SUB PARAM STRUCTS ==========

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct PublishParam {
    /// Channel name to publish to
    pub channel: String,
    /// Message to publish
    pub message: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct PubsubChannelsParam {
    /// Optional pattern to filter channels (e.g., "news.*")
    #[serde(default)]
    pub pattern: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct PubsubNumsubParam {
    /// Channel names to get subscriber counts for
    pub channels: Vec<String>,
}

impl RedisCtlMcp {
    /// Create a new MCP server instance
    pub fn new(profile: Option<&str>, read_only: bool) -> anyhow::Result<Self> {
        let config = Arc::new(ServerConfig {
            profile: profile.map(String::from),
            read_only,
        });

        info!(
            profile = ?config.profile,
            read_only = config.read_only,
            "Initializing RedisCtlMcp server"
        );

        Ok(Self {
            config,
            tool_router: Self::tool_router(),
            cloud_tools: Arc::new(RwLock::new(None)),
            enterprise_tools: Arc::new(RwLock::new(None)),
            database_tools: Arc::new(RwLock::new(None)),
        })
    }

    /// Get server configuration
    pub fn config(&self) -> &ServerConfig {
        &self.config
    }

    /// Initialize Cloud tools lazily
    async fn get_cloud_tools(&self) -> Result<CloudTools, RmcpError> {
        let mut guard = self.cloud_tools.write().await;
        if guard.is_none() {
            debug!("Initializing Cloud tools");
            let tools = CloudTools::new(self.config.profile.as_deref())
                .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;
            *guard = Some(tools);
        }
        Ok(guard.clone().unwrap())
    }

    /// Initialize Enterprise tools lazily
    async fn get_enterprise_tools(&self) -> Result<EnterpriseTools, RmcpError> {
        let mut guard = self.enterprise_tools.write().await;
        if guard.is_none() {
            debug!("Initializing Enterprise tools");
            let tools = EnterpriseTools::new(self.config.profile.as_deref())
                .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;
            *guard = Some(tools);
        }
        Ok(guard.clone().unwrap())
    }

    /// Initialize Database tools lazily
    async fn get_database_tools(&self) -> Result<DatabaseTools, RmcpError> {
        let mut guard = self.database_tools.write().await;
        if guard.is_none() {
            debug!("Initializing Database tools");
            let tools = DatabaseTools::new(self.config.profile.as_deref())
                .await
                .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;
            *guard = Some(tools);
        }
        Ok(guard.clone().unwrap())
    }
}

#[tool_router]
impl RedisCtlMcp {
    // =========================================================================
    // Cloud Tools - Read Only
    // =========================================================================

    #[tool(
        description = "Get Redis Cloud account information including account ID, name, and settings"
    )]
    async fn cloud_account_get(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: cloud_account_get");
        let tools = self.get_cloud_tools().await?;
        tools.get_account().await
    }

    #[tool(description = "List all Redis Cloud subscriptions in the account")]
    async fn cloud_subscriptions_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: cloud_subscriptions_list");
        let tools = self.get_cloud_tools().await?;
        tools.list_subscriptions().await
    }

    #[tool(description = "Get detailed information about a specific Redis Cloud subscription")]
    async fn cloud_subscription_get(
        &self,
        Parameters(params): Parameters<SubscriptionIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            subscription_id = params.subscription_id,
            "Tool called: cloud_subscription_get"
        );
        let tools = self.get_cloud_tools().await?;
        tools.get_subscription(params.subscription_id).await
    }

    #[tool(description = "List all databases in a specific Redis Cloud subscription")]
    async fn cloud_databases_list(
        &self,
        Parameters(params): Parameters<SubscriptionIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            subscription_id = params.subscription_id,
            "Tool called: cloud_databases_list"
        );
        let tools = self.get_cloud_tools().await?;
        tools.list_databases(params.subscription_id).await
    }

    #[tool(description = "Get detailed information about a specific Redis Cloud database")]
    async fn cloud_database_get(
        &self,
        Parameters(params): Parameters<DatabaseIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            subscription_id = params.subscription_id,
            database_id = params.database_id,
            "Tool called: cloud_database_get"
        );
        let tools = self.get_cloud_tools().await?;
        tools
            .get_database(params.subscription_id, params.database_id)
            .await
    }

    #[tool(description = "List recent async tasks in the Redis Cloud account")]
    async fn cloud_tasks_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: cloud_tasks_list");
        let tools = self.get_cloud_tools().await?;
        tools.list_tasks().await
    }

    #[tool(description = "Get the status of a specific Redis Cloud async task")]
    async fn cloud_task_get(
        &self,
        Parameters(params): Parameters<TaskIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(task_id = %params.task_id, "Tool called: cloud_task_get");
        let tools = self.get_cloud_tools().await?;
        tools.get_task(&params.task_id).await
    }

    // =========================================================================
    // Cloud Tools - Account & Infrastructure
    // =========================================================================

    #[tool(description = "List all payment methods configured for your Redis Cloud account")]
    async fn cloud_payment_methods_get(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: cloud_payment_methods_get");
        let tools = self.get_cloud_tools().await?;
        tools.get_payment_methods().await
    }

    #[tool(
        description = "List all available database modules (capabilities) supported in your account"
    )]
    async fn cloud_database_modules_get(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: cloud_database_modules_get");
        let tools = self.get_cloud_tools().await?;
        tools.get_database_modules().await
    }

    #[tool(
        description = "Get available regions across cloud providers (AWS, GCP, Azure) for Pro subscriptions"
    )]
    async fn cloud_regions_get(
        &self,
        Parameters(params): Parameters<CloudProviderParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(provider = ?params.provider, "Tool called: cloud_regions_get");
        let tools = self.get_cloud_tools().await?;
        tools.get_regions(params.provider.as_deref()).await
    }

    // =========================================================================
    // Cloud Tools - Pro Subscriptions (Write)
    // =========================================================================

    #[tool(
        description = "Create a new Pro subscription with advanced configuration options. Requires JSON payload with cloudProviders and databases arrays. Use cloud_regions_get to find available regions."
    )]
    async fn cloud_pro_subscription_create(
        &self,
        Parameters(params): Parameters<CreateProSubscriptionParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: cloud_pro_subscription_create");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_cloud_tools().await?;
        tools.create_subscription(params.request).await
    }

    #[tool(
        description = "Delete a Pro subscription. All databases must be deleted first. This is a destructive operation."
    )]
    async fn cloud_pro_subscription_delete(
        &self,
        Parameters(params): Parameters<SubscriptionIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            subscription_id = params.subscription_id,
            "Tool called: cloud_pro_subscription_delete"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_cloud_tools().await?;
        tools.delete_subscription(params.subscription_id).await
    }

    // =========================================================================
    // Cloud Tools - Essentials Subscriptions
    // =========================================================================

    #[tool(description = "List all Essentials (fixed) subscriptions in the account")]
    async fn cloud_essentials_subscriptions_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: cloud_essentials_subscriptions_list");
        let tools = self.get_cloud_tools().await?;
        tools.list_essentials_subscriptions().await
    }

    #[tool(description = "Get detailed information about a specific Essentials subscription")]
    async fn cloud_essentials_subscription_get(
        &self,
        Parameters(params): Parameters<SubscriptionIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            subscription_id = params.subscription_id,
            "Tool called: cloud_essentials_subscription_get"
        );
        let tools = self.get_cloud_tools().await?;
        tools
            .get_essentials_subscription(params.subscription_id)
            .await
    }

    #[tool(
        description = "Create a new Essentials subscription. Use cloud_essentials_plans_list to find available plans."
    )]
    async fn cloud_essentials_subscription_create(
        &self,
        Parameters(params): Parameters<CreateEssentialsSubscriptionParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            name = %params.name,
            plan_id = params.plan_id,
            "Tool called: cloud_essentials_subscription_create"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_cloud_tools().await?;
        tools
            .create_essentials_subscription(&params.name, params.plan_id, params.payment_method_id)
            .await
    }

    #[tool(
        description = "Delete an Essentials subscription. This is a destructive operation that cannot be undone."
    )]
    async fn cloud_essentials_subscription_delete(
        &self,
        Parameters(params): Parameters<SubscriptionIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            subscription_id = params.subscription_id,
            "Tool called: cloud_essentials_subscription_delete"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_cloud_tools().await?;
        tools
            .delete_essentials_subscription(params.subscription_id)
            .await
    }

    #[tool(
        description = "List available Essentials subscription plans with pricing. Optionally filter by cloud provider (AWS, GCP, Azure)."
    )]
    async fn cloud_essentials_plans_list(
        &self,
        Parameters(params): Parameters<CloudProviderParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(provider = ?params.provider, "Tool called: cloud_essentials_plans_list");
        let tools = self.get_cloud_tools().await?;
        tools
            .list_essentials_plans(params.provider.as_deref())
            .await
    }

    // =========================================================================
    // Enterprise Tools - Read Only
    // =========================================================================

    #[tool(
        description = "Get Redis Enterprise cluster information including name, version, and node count"
    )]
    async fn enterprise_cluster_get(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_cluster_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_cluster().await
    }

    #[tool(description = "List all nodes in the Redis Enterprise cluster with their status")]
    async fn enterprise_nodes_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_nodes_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_nodes().await
    }

    #[tool(description = "Get detailed information about a specific Redis Enterprise node")]
    async fn enterprise_node_get(
        &self,
        Parameters(params): Parameters<NodeIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(node_id = params.node_id, "Tool called: enterprise_node_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_node(params.node_id).await
    }

    #[tool(description = "List all databases (BDBs) in the Redis Enterprise cluster")]
    async fn enterprise_databases_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_databases_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_databases().await
    }

    #[tool(
        description = "Get detailed information about a specific Redis Enterprise database (BDB)"
    )]
    async fn enterprise_database_get(
        &self,
        Parameters(params): Parameters<EnterpriseDatabaseIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            database_id = params.database_id,
            "Tool called: enterprise_database_get"
        );
        let tools = self.get_enterprise_tools().await?;
        tools.get_database(params.database_id).await
    }

    #[tool(description = "Get performance statistics for a specific Redis Enterprise database")]
    async fn enterprise_database_stats(
        &self,
        Parameters(params): Parameters<EnterpriseDatabaseIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            database_id = params.database_id,
            "Tool called: enterprise_database_stats"
        );
        let tools = self.get_enterprise_tools().await?;
        tools.get_database_stats(params.database_id).await
    }

    #[tool(description = "List all shards across all databases in the Redis Enterprise cluster")]
    async fn enterprise_shards_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_shards_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_shards().await
    }

    #[tool(description = "List active alerts in the Redis Enterprise cluster")]
    async fn enterprise_alerts_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_alerts_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_alerts().await
    }

    #[tool(description = "Get recent event logs from the Redis Enterprise cluster")]
    async fn enterprise_logs_get(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_logs_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_logs().await
    }

    #[tool(
        description = "Get Redis Enterprise license information including expiration and capacity"
    )]
    async fn enterprise_license_get(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_license_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_license().await
    }

    // =========================================================================
    // Enterprise Tools - Write Operations
    // =========================================================================

    #[tool(
        description = "Create a new Redis Enterprise database. Requires name, optionally memory_size_mb (default 100)."
    )]
    async fn enterprise_database_create(
        &self,
        Parameters(params): Parameters<CreateEnterpriseDatabaseParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            name = %params.name,
            memory_size_mb = ?params.memory_size_mb,
            "Tool called: enterprise_database_create"
        );

        // Check read-only mode
        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools
            .create_database(&params.name, params.memory_size_mb)
            .await
    }

    #[tool(
        description = "Delete a Redis Enterprise database. This is a destructive operation that cannot be undone."
    )]
    async fn enterprise_database_delete(
        &self,
        Parameters(params): Parameters<EnterpriseDatabaseIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            database_id = params.database_id,
            "Tool called: enterprise_database_delete"
        );

        // Check read-only mode
        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools.delete_database(params.database_id).await
    }

    #[tool(
        description = "Update a Redis Enterprise database configuration. Supports memory_size (bytes), replication, data_persistence, and eviction_policy."
    )]
    async fn enterprise_database_update(
        &self,
        Parameters(params): Parameters<UpdateEnterpriseDatabaseParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            database_id = params.database_id,
            "Tool called: enterprise_database_update"
        );

        // Check read-only mode
        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        // Build the updates object from provided parameters
        let mut updates = serde_json::Map::new();
        if let Some(memory_size) = params.memory_size {
            updates.insert("memory_size".to_string(), serde_json::json!(memory_size));
        }
        if let Some(replication) = params.replication {
            updates.insert("replication".to_string(), serde_json::json!(replication));
        }
        if let Some(ref data_persistence) = params.data_persistence {
            updates.insert(
                "data_persistence".to_string(),
                serde_json::json!(data_persistence),
            );
        }
        if let Some(ref eviction_policy) = params.eviction_policy {
            updates.insert(
                "eviction_policy".to_string(),
                serde_json::json!(eviction_policy),
            );
        }

        if updates.is_empty() {
            return Err(RmcpError::invalid_request(
                "No updates provided. Specify at least one of: memory_size, replication, data_persistence, eviction_policy",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools
            .update_database(params.database_id, serde_json::Value::Object(updates))
            .await
    }

    #[tool(
        description = "Flush all data from a Redis Enterprise database. This is a destructive operation that cannot be undone."
    )]
    async fn enterprise_database_flush(
        &self,
        Parameters(params): Parameters<EnterpriseDatabaseIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            database_id = params.database_id,
            "Tool called: enterprise_database_flush"
        );

        // Check read-only mode
        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools.flush_database(params.database_id).await
    }

    #[tool(description = "Get performance metrics for a Redis Enterprise database")]
    async fn enterprise_database_metrics(
        &self,
        Parameters(params): Parameters<EnterpriseDatabaseIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            database_id = params.database_id,
            "Tool called: enterprise_database_metrics"
        );
        let tools = self.get_enterprise_tools().await?;
        tools.get_database_metrics(params.database_id).await
    }

    #[tool(
        description = "Export a Redis Enterprise database to a specified location (S3, FTP, etc.)"
    )]
    async fn enterprise_database_export(
        &self,
        Parameters(params): Parameters<ExportDatabaseParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            database_id = params.database_id,
            export_location = %params.export_location,
            "Tool called: enterprise_database_export"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools
            .export_database(params.database_id, &params.export_location)
            .await
    }

    #[tool(description = "Import data into a Redis Enterprise database from a specified location")]
    async fn enterprise_database_import(
        &self,
        Parameters(params): Parameters<ImportDatabaseParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            database_id = params.database_id,
            import_location = %params.import_location,
            flush_before_import = params.flush_before_import,
            "Tool called: enterprise_database_import"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools
            .import_database(
                params.database_id,
                &params.import_location,
                params.flush_before_import,
            )
            .await
    }

    #[tool(description = "Trigger a backup of a Redis Enterprise database")]
    async fn enterprise_database_backup(
        &self,
        Parameters(params): Parameters<EnterpriseDatabaseIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            database_id = params.database_id,
            "Tool called: enterprise_database_backup"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools.backup_database(params.database_id).await
    }

    #[tool(description = "Restore a Redis Enterprise database from a backup")]
    async fn enterprise_database_restore(
        &self,
        Parameters(params): Parameters<RestoreDatabaseParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            database_id = params.database_id,
            backup_uid = ?params.backup_uid,
            "Tool called: enterprise_database_restore"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools
            .restore_database(params.database_id, params.backup_uid.as_deref())
            .await
    }

    // =========================================================================
    // Enterprise Tools - Cluster Operations
    // =========================================================================

    #[tool(
        description = "Get Redis Enterprise cluster statistics including memory, CPU, and throughput metrics"
    )]
    async fn enterprise_cluster_stats(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_cluster_stats");
        let tools = self.get_enterprise_tools().await?;
        tools.get_cluster_stats().await
    }

    #[tool(description = "Get Redis Enterprise cluster settings and configuration")]
    async fn enterprise_cluster_settings(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_cluster_settings");
        let tools = self.get_enterprise_tools().await?;
        tools.get_cluster_settings().await
    }

    #[tool(
        description = "Get Redis Enterprise cluster topology showing nodes, shards, and their relationships"
    )]
    async fn enterprise_cluster_topology(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_cluster_topology");
        let tools = self.get_enterprise_tools().await?;
        tools.get_cluster_topology().await
    }

    #[tool(
        description = "Update Redis Enterprise cluster configuration. Supports name, email_alerts, and rack_aware settings."
    )]
    async fn enterprise_cluster_update(
        &self,
        Parameters(params): Parameters<UpdateClusterParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_cluster_update");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        // Build the updates object from provided parameters
        let mut updates = serde_json::Map::new();
        if let Some(ref name) = params.name {
            updates.insert("name".to_string(), serde_json::json!(name));
        }
        if let Some(email_alerts) = params.email_alerts {
            updates.insert("email_alerts".to_string(), serde_json::json!(email_alerts));
        }
        if let Some(rack_aware) = params.rack_aware {
            updates.insert("rack_aware".to_string(), serde_json::json!(rack_aware));
        }

        if updates.is_empty() {
            return Err(RmcpError::invalid_request(
                "No updates provided. Specify at least one of: name, email_alerts, rack_aware",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools
            .update_cluster(serde_json::Value::Object(updates))
            .await
    }

    // =========================================================================
    // Enterprise Tools - Node Operations
    // =========================================================================

    #[tool(
        description = "Get statistics for a specific Redis Enterprise node including CPU, memory, and network metrics"
    )]
    async fn enterprise_node_stats(
        &self,
        Parameters(params): Parameters<NodeIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            node_id = params.node_id,
            "Tool called: enterprise_node_stats"
        );
        let tools = self.get_enterprise_tools().await?;
        tools.get_node_stats(params.node_id).await
    }

    #[tool(
        description = "Update a Redis Enterprise node configuration. Supports accept_servers, external_addr, and rack_id."
    )]
    async fn enterprise_node_update(
        &self,
        Parameters(params): Parameters<UpdateNodeParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            node_id = params.node_id,
            "Tool called: enterprise_node_update"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        // Build the updates object from provided parameters
        let mut updates = serde_json::Map::new();
        if let Some(accept_servers) = params.accept_servers {
            updates.insert(
                "accept_servers".to_string(),
                serde_json::json!(accept_servers),
            );
        }
        if let Some(ref external_addr) = params.external_addr {
            updates.insert(
                "external_addr".to_string(),
                serde_json::json!(external_addr),
            );
        }
        if let Some(ref rack_id) = params.rack_id {
            updates.insert("rack_id".to_string(), serde_json::json!(rack_id));
        }

        if updates.is_empty() {
            return Err(RmcpError::invalid_request(
                "No updates provided. Specify at least one of: accept_servers, external_addr, rack_id",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools
            .update_node(params.node_id, serde_json::Value::Object(updates))
            .await
    }

    #[tool(
        description = "Remove a node from the Redis Enterprise cluster. This is a destructive operation."
    )]
    async fn enterprise_node_remove(
        &self,
        Parameters(params): Parameters<NodeIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            node_id = params.node_id,
            "Tool called: enterprise_node_remove"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools.remove_node(params.node_id).await
    }

    // =========================================================================
    // Enterprise Tools - Shard Operations
    // =========================================================================

    #[tool(description = "Get detailed information about a specific Redis Enterprise shard")]
    async fn enterprise_shard_get(
        &self,
        Parameters(params): Parameters<ShardIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(shard_uid = %params.shard_uid, "Tool called: enterprise_shard_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_shard(&params.shard_uid).await
    }

    // =========================================================================
    // Enterprise Tools - Alert Operations
    // =========================================================================

    #[tool(description = "Get detailed information about a specific Redis Enterprise alert")]
    async fn enterprise_alert_get(
        &self,
        Parameters(params): Parameters<AlertIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(alert_uid = %params.alert_uid, "Tool called: enterprise_alert_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_alert(&params.alert_uid).await
    }

    // =========================================================================
    // Enterprise Tools - User Operations
    // =========================================================================

    #[tool(description = "List all users in the Redis Enterprise cluster")]
    async fn enterprise_users_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_users_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_users().await
    }

    #[tool(description = "Get detailed information about a specific Redis Enterprise user")]
    async fn enterprise_user_get(
        &self,
        Parameters(params): Parameters<UserIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(user_id = params.user_id, "Tool called: enterprise_user_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_user(params.user_id).await
    }

    #[tool(description = "Create a new user in the Redis Enterprise cluster")]
    async fn enterprise_user_create(
        &self,
        Parameters(params): Parameters<CreateUserParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(email = %params.email, "Tool called: enterprise_user_create");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools
            .create_user(
                &params.email,
                &params.password,
                &params.role,
                params.name.as_deref(),
            )
            .await
    }

    #[tool(description = "Delete a user from the Redis Enterprise cluster")]
    async fn enterprise_user_delete(
        &self,
        Parameters(params): Parameters<UserIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            user_id = params.user_id,
            "Tool called: enterprise_user_delete"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools.delete_user(params.user_id).await
    }

    // =========================================================================
    // Enterprise Tools - Role Operations
    // =========================================================================

    #[tool(description = "List all roles in the Redis Enterprise cluster")]
    async fn enterprise_roles_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_roles_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_roles().await
    }

    #[tool(description = "Get detailed information about a specific Redis Enterprise role")]
    async fn enterprise_role_get(
        &self,
        Parameters(params): Parameters<RoleIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(role_id = params.role_id, "Tool called: enterprise_role_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_role(params.role_id).await
    }

    #[tool(description = "Create a new role in the Redis Enterprise cluster")]
    async fn enterprise_role_create(
        &self,
        Parameters(params): Parameters<CreateRoleParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(name = %params.name, "Tool called: enterprise_role_create");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools
            .create_role(&params.name, params.management.as_deref())
            .await
    }

    #[tool(description = "Delete a role from the Redis Enterprise cluster")]
    async fn enterprise_role_delete(
        &self,
        Parameters(params): Parameters<RoleIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            role_id = params.role_id,
            "Tool called: enterprise_role_delete"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools.delete_role(params.role_id).await
    }

    // =========================================================================
    // Enterprise Tools - Redis ACL Operations
    // =========================================================================

    #[tool(description = "List all Redis ACLs in the Redis Enterprise cluster")]
    async fn enterprise_acls_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_acls_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_acls().await
    }

    #[tool(description = "Get detailed information about a specific Redis ACL")]
    async fn enterprise_acl_get(
        &self,
        Parameters(params): Parameters<AclIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(acl_id = params.acl_id, "Tool called: enterprise_acl_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_acl(params.acl_id).await
    }

    #[tool(description = "Create a new Redis ACL in the Redis Enterprise cluster")]
    async fn enterprise_acl_create(
        &self,
        Parameters(params): Parameters<CreateAclParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(name = %params.name, "Tool called: enterprise_acl_create");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools
            .create_acl(&params.name, &params.acl, params.description.as_deref())
            .await
    }

    #[tool(description = "Delete a Redis ACL from the Redis Enterprise cluster")]
    async fn enterprise_acl_delete(
        &self,
        Parameters(params): Parameters<AclIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(acl_id = params.acl_id, "Tool called: enterprise_acl_delete");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools.delete_acl(params.acl_id).await
    }

    // =========================================================================
    // Enterprise Tools - Module Operations
    // =========================================================================

    #[tool(description = "List all Redis modules available in the Redis Enterprise cluster")]
    async fn enterprise_modules_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_modules_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_modules().await
    }

    #[tool(description = "Get detailed information about a specific Redis module")]
    async fn enterprise_module_get(
        &self,
        Parameters(params): Parameters<ModuleIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(module_uid = %params.module_uid, "Tool called: enterprise_module_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_module(&params.module_uid).await
    }

    // =========================================================================
    // Enterprise Tools - CRDB (Active-Active) Operations
    // =========================================================================

    #[tool(description = "List all Active-Active (CRDB) databases in the Redis Enterprise cluster")]
    async fn enterprise_crdbs_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_crdbs_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_crdbs().await
    }

    #[tool(description = "Get detailed information about a specific Active-Active (CRDB) database")]
    async fn enterprise_crdb_get(
        &self,
        Parameters(params): Parameters<CrdbGuidParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(crdb_guid = %params.crdb_guid, "Tool called: enterprise_crdb_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_crdb(&params.crdb_guid).await
    }

    #[tool(description = "Update an Active-Active (CRDB) database configuration")]
    async fn enterprise_crdb_update(
        &self,
        Parameters(params): Parameters<UpdateCrdbParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(crdb_guid = %params.crdb_guid, "Tool called: enterprise_crdb_update");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        // Build the updates object from provided parameters
        let mut updates = serde_json::Map::new();
        if let Some(memory_size) = params.memory_size {
            updates.insert("memory_size".to_string(), serde_json::json!(memory_size));
        }
        if let Some(encryption) = params.encryption {
            updates.insert("encryption".to_string(), serde_json::json!(encryption));
        }
        if let Some(ref data_persistence) = params.data_persistence {
            updates.insert(
                "data_persistence".to_string(),
                serde_json::json!(data_persistence),
            );
        }

        if updates.is_empty() {
            return Err(RmcpError::invalid_request(
                "No updates provided. Specify at least one of: memory_size, encryption, data_persistence",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools
            .update_crdb(&params.crdb_guid, serde_json::Value::Object(updates))
            .await
    }

    #[tool(
        description = "Delete an Active-Active (CRDB) database. This is a destructive operation."
    )]
    async fn enterprise_crdb_delete(
        &self,
        Parameters(params): Parameters<CrdbGuidParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(crdb_guid = %params.crdb_guid, "Tool called: enterprise_crdb_delete");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools.delete_crdb(&params.crdb_guid).await
    }

    // =========================================================================
    // Enterprise Tools - Debug Info / Support Operations
    // =========================================================================

    #[tool(description = "List debug info collection tasks in the Redis Enterprise cluster")]
    async fn enterprise_debuginfo_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_debuginfo_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_debuginfo().await
    }

    #[tool(description = "Get the status of a specific debug info collection task")]
    async fn enterprise_debuginfo_status(
        &self,
        Parameters(params): Parameters<DebugInfoTaskIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(task_id = %params.task_id, "Tool called: enterprise_debuginfo_status");
        let tools = self.get_enterprise_tools().await?;
        tools.get_debuginfo_status(&params.task_id).await
    }

    // =========================================================================
    // Enterprise Tools - LDAP Mapping Operations
    // =========================================================================

    #[tool(description = "List all LDAP mappings in the Redis Enterprise cluster")]
    async fn enterprise_ldap_mappings_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_ldap_mappings_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_ldap_mappings().await
    }

    #[tool(description = "Get detailed information about a specific LDAP mapping")]
    async fn enterprise_ldap_mapping_get(
        &self,
        Parameters(params): Parameters<LdapMappingIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            mapping_id = params.mapping_id,
            "Tool called: enterprise_ldap_mapping_get"
        );
        let tools = self.get_enterprise_tools().await?;
        tools.get_ldap_mapping(params.mapping_id as u64).await
    }

    #[tool(description = "Create a new LDAP mapping to map LDAP groups to Redis Enterprise roles")]
    async fn enterprise_ldap_mapping_create(
        &self,
        Parameters(params): Parameters<CreateLdapMappingParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(name = %params.name, "Tool called: enterprise_ldap_mapping_create");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools
            .create_ldap_mapping(
                &params.name,
                &params.dn,
                &params.role,
                params.email.as_deref(),
            )
            .await
    }

    #[tool(description = "Delete an LDAP mapping from the Redis Enterprise cluster")]
    async fn enterprise_ldap_mapping_delete(
        &self,
        Parameters(params): Parameters<LdapMappingIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            mapping_id = params.mapping_id,
            "Tool called: enterprise_ldap_mapping_delete"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools.delete_ldap_mapping(params.mapping_id as u64).await
    }

    // =========================================================================
    // Enterprise Tools - Job Scheduler Operations
    // =========================================================================

    #[tool(description = "List all scheduled jobs in the Redis Enterprise cluster")]
    async fn enterprise_jobs_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_jobs_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_jobs().await
    }

    #[tool(description = "Get detailed information about a specific scheduled job")]
    async fn enterprise_job_get(
        &self,
        Parameters(params): Parameters<JobIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(job_id = %params.job_id, "Tool called: enterprise_job_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_job(&params.job_id).await
    }

    #[tool(description = "Get execution history for a specific scheduled job")]
    async fn enterprise_job_history(
        &self,
        Parameters(params): Parameters<JobIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(job_id = %params.job_id, "Tool called: enterprise_job_history");
        let tools = self.get_enterprise_tools().await?;
        tools.get_job_history(&params.job_id).await
    }

    #[tool(description = "Trigger immediate execution of a scheduled job")]
    async fn enterprise_job_trigger(
        &self,
        Parameters(params): Parameters<JobIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(job_id = %params.job_id, "Tool called: enterprise_job_trigger");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools.trigger_job(&params.job_id).await
    }

    // =========================================================================
    // Enterprise Tools - Proxy Operations
    // =========================================================================

    #[tool(description = "List all proxies in the Redis Enterprise cluster")]
    async fn enterprise_proxies_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_proxies_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_proxies().await
    }

    #[tool(description = "Get detailed information about a specific proxy")]
    async fn enterprise_proxy_get(
        &self,
        Parameters(params): Parameters<ProxyIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            proxy_id = params.proxy_id,
            "Tool called: enterprise_proxy_get"
        );
        let tools = self.get_enterprise_tools().await?;
        tools.get_proxy(params.proxy_id as u64).await
    }

    #[tool(description = "Get statistics for a specific proxy")]
    async fn enterprise_proxy_stats(
        &self,
        Parameters(params): Parameters<ProxyIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            proxy_id = params.proxy_id,
            "Tool called: enterprise_proxy_stats"
        );
        let tools = self.get_enterprise_tools().await?;
        tools.get_proxy_stats(params.proxy_id as u64).await
    }

    #[tool(description = "List proxies for a specific database")]
    async fn enterprise_proxies_by_database(
        &self,
        Parameters(params): Parameters<EnterpriseDatabaseIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            database_id = params.database_id,
            "Tool called: enterprise_proxies_by_database"
        );
        let tools = self.get_enterprise_tools().await?;
        tools
            .list_proxies_by_database(params.database_id as u64)
            .await
    }

    // =========================================================================
    // Enterprise Tools - Endpoint Operations
    // =========================================================================

    #[tool(description = "List all database endpoints in the Redis Enterprise cluster")]
    async fn enterprise_endpoints_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_endpoints_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_endpoints().await
    }

    #[tool(description = "Get detailed information about a specific endpoint")]
    async fn enterprise_endpoint_get(
        &self,
        Parameters(params): Parameters<EndpointIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(endpoint_id = %params.endpoint_id, "Tool called: enterprise_endpoint_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_endpoint(&params.endpoint_id).await
    }

    #[tool(description = "Get statistics for a specific endpoint")]
    async fn enterprise_endpoint_stats(
        &self,
        Parameters(params): Parameters<EndpointIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(endpoint_id = %params.endpoint_id, "Tool called: enterprise_endpoint_stats");
        let tools = self.get_enterprise_tools().await?;
        tools.get_endpoint_stats(&params.endpoint_id).await
    }

    #[tool(description = "List endpoints for a specific database")]
    async fn enterprise_endpoints_by_database(
        &self,
        Parameters(params): Parameters<EnterpriseDatabaseIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            database_id = params.database_id,
            "Tool called: enterprise_endpoints_by_database"
        );
        let tools = self.get_enterprise_tools().await?;
        tools
            .list_endpoints_by_database(params.database_id as u64)
            .await
    }

    // =========================================================================
    // Enterprise Tools - Diagnostics Operations
    // =========================================================================

    #[tool(description = "List available diagnostic checks in the Redis Enterprise cluster")]
    async fn enterprise_diagnostic_checks_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_diagnostic_checks_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_diagnostic_checks().await
    }

    #[tool(description = "List diagnostic reports in the Redis Enterprise cluster")]
    async fn enterprise_diagnostic_reports_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_diagnostic_reports_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_diagnostic_reports().await
    }

    #[tool(description = "Get a specific diagnostic report")]
    async fn enterprise_diagnostic_report_get(
        &self,
        Parameters(params): Parameters<DiagnosticReportIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(report_id = %params.report_id, "Tool called: enterprise_diagnostic_report_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_diagnostic_report(&params.report_id).await
    }

    #[tool(description = "Get the most recent diagnostic report")]
    async fn enterprise_diagnostic_report_last(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_diagnostic_report_last");
        let tools = self.get_enterprise_tools().await?;
        tools.get_last_diagnostic_report().await
    }

    #[tool(description = "Run diagnostics on the Redis Enterprise cluster")]
    async fn enterprise_diagnostics_run(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_diagnostics_run");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools.run_diagnostics().await
    }

    // =========================================================================
    // Cloud Tools - Essentials Database Operations
    // =========================================================================

    #[tool(description = "List all databases in an Essentials subscription")]
    async fn cloud_essentials_databases_list(
        &self,
        Parameters(params): Parameters<SubscriptionIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            subscription_id = params.subscription_id,
            "Tool called: cloud_essentials_databases_list"
        );
        let tools = self.get_cloud_tools().await?;
        tools
            .list_essentials_databases(params.subscription_id)
            .await
    }

    #[tool(description = "Get detailed information about a specific Essentials database")]
    async fn cloud_essentials_database_get(
        &self,
        Parameters(params): Parameters<EssentialsDatabaseIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            subscription_id = params.subscription_id,
            database_id = params.database_id,
            "Tool called: cloud_essentials_database_get"
        );
        let tools = self.get_cloud_tools().await?;
        tools
            .get_essentials_database(params.subscription_id, params.database_id)
            .await
    }

    #[tool(description = "Delete an Essentials database. This is a destructive operation.")]
    async fn cloud_essentials_database_delete(
        &self,
        Parameters(params): Parameters<EssentialsDatabaseIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            subscription_id = params.subscription_id,
            database_id = params.database_id,
            "Tool called: cloud_essentials_database_delete"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_cloud_tools().await?;
        tools
            .delete_essentials_database(params.subscription_id, params.database_id)
            .await
    }

    // =========================================================================
    // Cloud Tools - VPC Peering Operations
    // =========================================================================

    #[tool(description = "Get VPC peerings for a subscription")]
    async fn cloud_vpc_peerings_get(
        &self,
        Parameters(params): Parameters<SubscriptionIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            subscription_id = params.subscription_id,
            "Tool called: cloud_vpc_peerings_get"
        );
        let tools = self.get_cloud_tools().await?;
        tools.get_vpc_peerings(params.subscription_id).await
    }

    #[tool(description = "Delete a VPC peering. This is a destructive operation.")]
    async fn cloud_vpc_peering_delete(
        &self,
        Parameters(params): Parameters<VpcPeeringIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            subscription_id = params.subscription_id,
            peering_id = params.peering_id,
            "Tool called: cloud_vpc_peering_delete"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_cloud_tools().await?;
        tools
            .delete_vpc_peering(params.subscription_id, params.peering_id)
            .await
    }

    // =========================================================================
    // Cloud Tools - Cloud Account Operations
    // =========================================================================

    #[tool(
        description = "List all cloud provider accounts (AWS, GCP, Azure) configured in your Redis Cloud account"
    )]
    async fn cloud_accounts_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: cloud_accounts_list");
        let tools = self.get_cloud_tools().await?;
        tools.list_cloud_accounts().await
    }

    #[tool(description = "Get detailed information about a specific cloud provider account")]
    async fn cloud_account_get_by_id(
        &self,
        Parameters(params): Parameters<CloudAccountIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            account_id = params.account_id,
            "Tool called: cloud_account_get_by_id"
        );
        let tools = self.get_cloud_tools().await?;
        tools.get_cloud_account(params.account_id).await
    }

    #[tool(description = "Delete a cloud provider account. This is a destructive operation.")]
    async fn cloud_account_delete(
        &self,
        Parameters(params): Parameters<CloudAccountIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            account_id = params.account_id,
            "Tool called: cloud_account_delete"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_cloud_tools().await?;
        tools.delete_cloud_account(params.account_id).await
    }

    // =========================================================================
    // Enterprise Tools - CRDB Task Operations
    // =========================================================================

    #[tool(description = "List all Active-Active (CRDB) tasks in the Redis Enterprise cluster")]
    async fn enterprise_crdb_tasks_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_crdb_tasks_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_crdb_tasks().await
    }

    #[tool(description = "Get detailed information about a specific CRDB task")]
    async fn enterprise_crdb_task_get(
        &self,
        Parameters(params): Parameters<CrdbTaskIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(task_id = %params.task_id, "Tool called: enterprise_crdb_task_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_crdb_task(&params.task_id).await
    }

    #[tool(description = "List CRDB tasks for a specific Active-Active database")]
    async fn enterprise_crdb_tasks_by_crdb(
        &self,
        Parameters(params): Parameters<CrdbGuidParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(crdb_guid = %params.crdb_guid, "Tool called: enterprise_crdb_tasks_by_crdb");
        let tools = self.get_enterprise_tools().await?;
        tools.list_crdb_tasks_by_crdb(&params.crdb_guid).await
    }

    #[tool(description = "Cancel a CRDB task")]
    async fn enterprise_crdb_task_cancel(
        &self,
        Parameters(params): Parameters<CrdbTaskIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(task_id = %params.task_id, "Tool called: enterprise_crdb_task_cancel");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools.cancel_crdb_task(&params.task_id).await
    }

    // =========================================================================
    // Cloud Tools - Private Link Operations
    // =========================================================================

    #[tool(description = "Get AWS PrivateLink configuration for a subscription")]
    async fn cloud_private_link_get(
        &self,
        Parameters(params): Parameters<SubscriptionIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            subscription_id = params.subscription_id,
            "Tool called: cloud_private_link_get"
        );
        let tools = self.get_cloud_tools().await?;
        tools.get_private_link(params.subscription_id).await
    }

    #[tool(description = "Delete AWS PrivateLink configuration for a subscription")]
    async fn cloud_private_link_delete(
        &self,
        Parameters(params): Parameters<SubscriptionIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            subscription_id = params.subscription_id,
            "Tool called: cloud_private_link_delete"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_cloud_tools().await?;
        tools.delete_private_link(params.subscription_id).await
    }

    // =========================================================================
    // Cloud Tools - Transit Gateway Operations
    // =========================================================================

    #[tool(description = "Get AWS Transit Gateway attachments for a subscription")]
    async fn cloud_transit_gateway_attachments_get(
        &self,
        Parameters(params): Parameters<SubscriptionIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            subscription_id = params.subscription_id,
            "Tool called: cloud_transit_gateway_attachments_get"
        );
        let tools = self.get_cloud_tools().await?;
        tools
            .get_transit_gateway_attachments(params.subscription_id)
            .await
    }

    #[tool(description = "Delete an AWS Transit Gateway attachment")]
    async fn cloud_transit_gateway_attachment_delete(
        &self,
        Parameters(params): Parameters<TransitGatewayAttachmentIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            subscription_id = params.subscription_id,
            attachment_id = %params.attachment_id,
            "Tool called: cloud_transit_gateway_attachment_delete"
        );

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_cloud_tools().await?;
        tools
            .delete_transit_gateway_attachment(params.subscription_id, &params.attachment_id)
            .await
    }

    // =========================================================================
    // Enterprise Tools - BDB Groups Operations
    // =========================================================================

    #[tool(description = "List all database groups in the Redis Enterprise cluster")]
    async fn enterprise_bdb_groups_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_bdb_groups_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_bdb_groups().await
    }

    #[tool(description = "Get detailed information about a specific database group")]
    async fn enterprise_bdb_group_get(
        &self,
        Parameters(params): Parameters<BdbGroupIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(uid = params.uid, "Tool called: enterprise_bdb_group_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_bdb_group(params.uid as u64).await
    }

    #[tool(description = "Delete a database group")]
    async fn enterprise_bdb_group_delete(
        &self,
        Parameters(params): Parameters<BdbGroupIdParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(uid = params.uid, "Tool called: enterprise_bdb_group_delete");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools.delete_bdb_group(params.uid as u64).await
    }

    // =========================================================================
    // Enterprise Tools - OCSP Operations
    // =========================================================================

    #[tool(description = "Get OCSP (Online Certificate Status Protocol) configuration")]
    async fn enterprise_ocsp_config_get(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_ocsp_config_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_ocsp_config().await
    }

    #[tool(description = "Get OCSP status showing certificate validation state")]
    async fn enterprise_ocsp_status_get(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_ocsp_status_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_ocsp_status().await
    }

    #[tool(description = "Test OCSP connectivity and certificate validation")]
    async fn enterprise_ocsp_test(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_ocsp_test");
        let tools = self.get_enterprise_tools().await?;
        tools.test_ocsp().await
    }

    // =========================================================================
    // Enterprise Tools - DNS Suffix Operations
    // =========================================================================

    #[tool(description = "List all DNS suffixes in the Redis Enterprise cluster")]
    async fn enterprise_suffixes_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_suffixes_list");
        let tools = self.get_enterprise_tools().await?;
        tools.list_suffixes().await
    }

    #[tool(description = "Get detailed information about a specific DNS suffix")]
    async fn enterprise_suffix_get(
        &self,
        Parameters(params): Parameters<SuffixNameParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(name = %params.name, "Tool called: enterprise_suffix_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_suffix(&params.name).await
    }

    #[tool(description = "Get cluster-level DNS suffixes")]
    async fn enterprise_cluster_suffixes_get(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: enterprise_cluster_suffixes_get");
        let tools = self.get_enterprise_tools().await?;
        tools.get_cluster_suffixes().await
    }

    #[tool(description = "Delete a DNS suffix")]
    async fn enterprise_suffix_delete(
        &self,
        Parameters(params): Parameters<SuffixNameParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(name = %params.name, "Tool called: enterprise_suffix_delete");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_enterprise_tools().await?;
        tools.delete_suffix(&params.name).await
    }

    // =========================================================================
    // Database Tools - Direct Redis Connection
    // =========================================================================

    #[tool(
        description = "Execute a Redis command directly. Use for commands not covered by specific tools. Write commands are blocked in read-only mode."
    )]
    async fn database_execute(
        &self,
        Parameters(params): Parameters<DatabaseExecuteParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(command = %params.command, args = ?params.args, "Tool called: database_execute");

        // Check if it's a write command in read-only mode
        if self.config.read_only && is_write_command(&params.command) {
            return Err(RmcpError::invalid_request(
                format!(
                    "Command '{}' is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.",
                    params.command
                ),
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let result = tools
            .execute(&params.command, &params.args)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        let json = value_to_json(&result);
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&json).unwrap_or_else(|_| json.to_string()),
        )]))
    }

    #[tool(
        description = "Execute multiple Redis commands in a single pipeline for improved performance. Reduces network round-trips by batching commands. Use atomic=true for MULTI/EXEC transactional execution."
    )]
    async fn database_pipeline(
        &self,
        Parameters(params): Parameters<DatabasePipelineParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(
            command_count = params.commands.len(),
            atomic = params.atomic,
            "Tool called: database_pipeline"
        );

        // Check if any command is a write operation in read-only mode
        if self.config.read_only {
            for cmd in &params.commands {
                if is_write_command(&cmd.command) {
                    return Err(RmcpError::invalid_request(
                        format!(
                            "Command '{}' is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.",
                            cmd.command
                        ),
                        None,
                    ));
                }
            }
        }

        // Convert to internal PipelineCommand type
        let pipeline_commands: Vec<crate::database_tools::PipelineCommand> = params
            .commands
            .iter()
            .map(|c| crate::database_tools::PipelineCommand {
                command: c.command.clone(),
                args: c.args.clone(),
            })
            .collect();

        let tools = self.get_database_tools().await?;
        let start = std::time::Instant::now();
        let results = tools
            .execute_pipeline(&pipeline_commands, params.atomic)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;
        let elapsed = start.elapsed();

        // Build response with individual command results
        let response: Vec<serde_json::Value> = params
            .commands
            .iter()
            .zip(results.iter())
            .map(|(cmd, result)| {
                serde_json::json!({
                    "command": cmd.command,
                    "args": cmd.args,
                    "result": value_to_json(result)
                })
            })
            .collect();

        let output = serde_json::json!({
            "commands": response,
            "count": params.commands.len(),
            "atomic": params.atomic,
            "execution_time_ms": elapsed.as_secs_f64() * 1000.0
        });

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&output).unwrap_or_else(|_| output.to_string()),
        )]))
    }

    #[tool(
        description = "Get Redis server information (INFO command). Returns server stats, memory usage, replication info, etc."
    )]
    async fn database_info(
        &self,
        Parameters(params): Parameters<DatabaseInfoParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(section = ?params.section, "Tool called: database_info");

        let tools = self.get_database_tools().await?;
        let result = tools
            .info(params.section.as_deref())
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Get the number of keys in the current database (DBSIZE command)")]
    async fn database_dbsize(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: database_dbsize");

        let tools = self.get_database_tools().await?;
        let result = tools
            .dbsize()
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({ "dbsize": result }).to_string(),
        )]))
    }

    #[tool(
        description = "Scan keys matching a pattern (SCAN command). Safe alternative to KEYS that doesn't block the server."
    )]
    async fn database_scan(
        &self,
        Parameters(params): Parameters<DatabaseScanParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(pattern = %params.pattern, count = params.count, "Tool called: database_scan");

        let tools = self.get_database_tools().await?;
        let keys = tools
            .scan(&params.pattern, params.count)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "pattern": params.pattern,
                "count": keys.len(),
                "keys": keys
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Get the type of a key (TYPE command). Returns string, list, set, zset, hash, stream, or none."
    )]
    async fn database_type(
        &self,
        Parameters(params): Parameters<DatabaseKeyParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, "Tool called: database_type");

        let tools = self.get_database_tools().await?;
        let key_type = tools
            .key_type(&params.key)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "type": key_type
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Get the TTL of a key in seconds (TTL command). Returns -1 if no expiration, -2 if key doesn't exist."
    )]
    async fn database_ttl(
        &self,
        Parameters(params): Parameters<DatabaseKeyParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, "Tool called: database_ttl");

        let tools = self.get_database_tools().await?;
        let ttl = tools
            .ttl(&params.key)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "ttl_seconds": ttl
            })
            .to_string(),
        )]))
    }

    #[tool(description = "Get memory usage of a key in bytes (MEMORY USAGE command)")]
    async fn database_memory_usage(
        &self,
        Parameters(params): Parameters<DatabaseKeyParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, "Tool called: database_memory_usage");

        let tools = self.get_database_tools().await?;
        let usage = tools
            .memory_usage(&params.key)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "memory_bytes": usage
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Get slow log entries (SLOWLOG GET command). Shows queries that exceeded the slowlog threshold."
    )]
    async fn database_slowlog(
        &self,
        Parameters(params): Parameters<DatabaseSlowlogParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(count = ?params.count, "Tool called: database_slowlog");

        let tools = self.get_database_tools().await?;
        let result = tools
            .slowlog_get(params.count)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        let json = value_to_json(&result);
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&json).unwrap_or_else(|_| json.to_string()),
        )]))
    }

    #[tool(description = "Get the number of entries in the slow log (SLOWLOG LEN command)")]
    async fn database_slowlog_len(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: database_slowlog_len");

        let tools = self.get_database_tools().await?;
        let len = tools
            .slowlog_len()
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({ "slowlog_len": len }).to_string(),
        )]))
    }

    #[tool(description = "Get list of connected clients (CLIENT LIST command)")]
    async fn database_client_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: database_client_list");

        let tools = self.get_database_tools().await?;
        let result = tools
            .client_list()
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Get Redis configuration values (CONFIG GET command)")]
    async fn database_config_get(
        &self,
        Parameters(params): Parameters<DatabaseConfigGetParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(pattern = %params.pattern, "Tool called: database_config_get");

        let tools = self.get_database_tools().await?;
        let result = tools
            .config_get(&params.pattern)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        // Convert vec of tuples to JSON object
        let config: serde_json::Map<String, serde_json::Value> = result
            .into_iter()
            .map(|(k, v)| (k, serde_json::Value::String(v)))
            .collect();

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&config).unwrap_or_else(|_| "{}".to_string()),
        )]))
    }

    #[tool(description = "List loaded Redis modules (MODULE LIST command)")]
    async fn database_module_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: database_module_list");

        let tools = self.get_database_tools().await?;
        let result = tools
            .module_list()
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        let json = value_to_json(&result);
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&json).unwrap_or_else(|_| json.to_string()),
        )]))
    }

    #[tool(description = "Ping the Redis server to check connectivity")]
    async fn database_ping(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: database_ping");

        let tools = self.get_database_tools().await?;
        let result = tools
            .ping()
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({ "response": result }).to_string(),
        )]))
    }

    #[tool(description = "Get the value of a string key (GET command)")]
    async fn database_get(
        &self,
        Parameters(params): Parameters<DatabaseKeyParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, "Tool called: database_get");

        let tools = self.get_database_tools().await?;
        let result = tools
            .get(&params.key)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "value": result
            })
            .to_string(),
        )]))
    }

    #[tool(description = "Check if a key exists (EXISTS command)")]
    async fn database_exists(
        &self,
        Parameters(params): Parameters<DatabaseKeyParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, "Tool called: database_exists");

        let tools = self.get_database_tools().await?;
        let exists = tools
            .exists(&params.key)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "exists": exists
            })
            .to_string(),
        )]))
    }

    #[tool(description = "Get all fields and values of a hash (HGETALL command)")]
    async fn database_hgetall(
        &self,
        Parameters(params): Parameters<DatabaseKeyParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, "Tool called: database_hgetall");

        let tools = self.get_database_tools().await?;
        let result = tools
            .hgetall(&params.key)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        // Convert vec of tuples to JSON object
        let hash: serde_json::Map<String, serde_json::Value> = result
            .into_iter()
            .map(|(k, v)| (k, serde_json::Value::String(v)))
            .collect();

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "fields": hash
            })
            .to_string(),
        )]))
    }

    #[tool(description = "Get the number of fields in a hash (HLEN command)")]
    async fn database_hlen(
        &self,
        Parameters(params): Parameters<DatabaseKeyParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, "Tool called: database_hlen");

        let tools = self.get_database_tools().await?;
        let len = tools
            .hlen(&params.key)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "length": len
            })
            .to_string(),
        )]))
    }

    #[tool(description = "Get a range of elements from a list (LRANGE command)")]
    async fn database_lrange(
        &self,
        Parameters(params): Parameters<DatabaseLrangeParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, start = params.start, stop = params.stop, "Tool called: database_lrange");

        let tools = self.get_database_tools().await?;
        let result = tools
            .lrange(&params.key, params.start, params.stop)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "start": params.start,
                "stop": params.stop,
                "values": result
            })
            .to_string(),
        )]))
    }

    #[tool(description = "Get the length of a list (LLEN command)")]
    async fn database_llen(
        &self,
        Parameters(params): Parameters<DatabaseKeyParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, "Tool called: database_llen");

        let tools = self.get_database_tools().await?;
        let len = tools
            .llen(&params.key)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "length": len
            })
            .to_string(),
        )]))
    }

    #[tool(description = "Get all members of a set (SMEMBERS command)")]
    async fn database_smembers(
        &self,
        Parameters(params): Parameters<DatabaseKeyParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, "Tool called: database_smembers");

        let tools = self.get_database_tools().await?;
        let result = tools
            .smembers(&params.key)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "members": result
            })
            .to_string(),
        )]))
    }

    #[tool(description = "Get the cardinality (size) of a set (SCARD command)")]
    async fn database_scard(
        &self,
        Parameters(params): Parameters<DatabaseKeyParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, "Tool called: database_scard");

        let tools = self.get_database_tools().await?;
        let card = tools
            .scard(&params.key)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "cardinality": card
            })
            .to_string(),
        )]))
    }

    #[tool(description = "Get a range of elements from a sorted set (ZRANGE command)")]
    async fn database_zrange(
        &self,
        Parameters(params): Parameters<DatabaseZrangeParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, start = params.start, stop = params.stop, "Tool called: database_zrange");

        let tools = self.get_database_tools().await?;
        let result = tools
            .zrange(&params.key, params.start, params.stop)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "start": params.start,
                "stop": params.stop,
                "members": result
            })
            .to_string(),
        )]))
    }

    #[tool(description = "Get the cardinality (size) of a sorted set (ZCARD command)")]
    async fn database_zcard(
        &self,
        Parameters(params): Parameters<DatabaseKeyParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, "Tool called: database_zcard");

        let tools = self.get_database_tools().await?;
        let card = tools
            .zcard(&params.key)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "cardinality": card
            })
            .to_string(),
        )]))
    }

    // ========== WRITE OPERATIONS ==========

    #[tool(
        description = "Set a string value (SET command). Creates or overwrites the key. Supports optional expiration (ex for seconds, px for milliseconds) and conditional set (nx: only if not exists, xx: only if exists). Use this to store strings, numbers, or serialized data. Returns true if set succeeded, false if NX/XX condition failed."
    )]
    async fn database_set(
        &self,
        Parameters(params): Parameters<DatabaseSetParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, "Tool called: database_set");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "SET is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.".to_string(),
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let success = tools
            .set(
                &params.key,
                &params.value,
                params.ex,
                params.px,
                params.nx,
                params.xx,
            )
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "success": success,
                "message": if success { "Value set successfully" } else { "Set failed (NX/XX condition not met)" }
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Delete one or more keys (DEL command). Removes keys and their associated values from the database. Returns the number of keys that were actually deleted (keys that didn't exist are not counted). Use this to remove data or clean up expired/unused keys."
    )]
    async fn database_del(
        &self,
        Parameters(params): Parameters<DatabaseDelParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(keys = ?params.keys, "Tool called: database_del");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "DEL is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.".to_string(),
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let deleted = tools
            .del(&params.keys)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "keys": params.keys,
                "deleted": deleted
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Set a key's expiration time in seconds (EXPIRE command). After the timeout, the key will be automatically deleted. Returns true if the timeout was set, false if the key doesn't exist. Use this to implement cache expiration, session timeouts, or temporary data."
    )]
    async fn database_expire(
        &self,
        Parameters(params): Parameters<DatabaseExpireParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, seconds = params.seconds, "Tool called: database_expire");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "EXPIRE is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.".to_string(),
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let success = tools
            .expire(&params.key, params.seconds)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "seconds": params.seconds,
                "success": success
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Remove a key's expiration (PERSIST command). Makes the key persistent (no expiration). Returns true if the timeout was removed, false if the key doesn't exist or has no timeout."
    )]
    async fn database_persist(
        &self,
        Parameters(params): Parameters<DatabaseKeyParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, "Tool called: database_persist");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "PERSIST is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.".to_string(),
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let success = tools
            .persist(&params.key)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "success": success
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Increment a key's integer value by 1 (INCR command). If the key doesn't exist, it's created with value 0 before incrementing. Returns the new value. Use this for counters, rate limiters, or sequence generators. The value must be a valid integer string."
    )]
    async fn database_incr(
        &self,
        Parameters(params): Parameters<DatabaseKeyParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, "Tool called: database_incr");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "INCR is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.".to_string(),
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let new_value = tools
            .incr(&params.key)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "value": new_value
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Decrement a key's integer value by 1 (DECR command). If the key doesn't exist, it's created with value 0 before decrementing. Returns the new value. Use this for countdown counters or decrementing stock levels."
    )]
    async fn database_decr(
        &self,
        Parameters(params): Parameters<DatabaseKeyParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, "Tool called: database_decr");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "DECR is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.".to_string(),
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let new_value = tools
            .decr(&params.key)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "value": new_value
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Increment a key's integer value by a specific amount (INCRBY command). If the key doesn't exist, it's created with value 0 before incrementing. Use negative increment to decrement. Returns the new value. Use this for counters with custom increments like adding points or adjusting balances."
    )]
    async fn database_incrby(
        &self,
        Parameters(params): Parameters<DatabaseIncrbyParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, increment = params.increment, "Tool called: database_incrby");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "INCRBY is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.".to_string(),
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let new_value = tools
            .incrby(&params.key, params.increment)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "increment": params.increment,
                "value": new_value
            })
            .to_string(),
        )]))
    }

    // ========== HASH WRITE OPERATIONS ==========

    #[tool(
        description = "Set a field in a hash (HSET command). Creates the hash if it doesn't exist. Returns 1 if the field is new, 0 if the field was updated. Use hashes to store objects like user profiles, product details, or configuration settings where you need to access individual fields."
    )]
    async fn database_hset(
        &self,
        Parameters(params): Parameters<DatabaseHsetParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, field = %params.field, "Tool called: database_hset");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "HSET is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.".to_string(),
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let added = tools
            .hset(&params.key, &params.field, &params.value)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "field": params.field,
                "added": added,
                "message": if added == 1 { "New field created" } else { "Existing field updated" }
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Set multiple fields in a hash at once (HSET with multiple field-value pairs). More efficient than multiple HSET calls. Creates the hash if it doesn't exist. Returns the number of new fields added. Use this to create or update entire objects in one operation."
    )]
    async fn database_hset_multiple(
        &self,
        Parameters(params): Parameters<DatabaseHsetMultipleParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, field_count = params.fields.len(), "Tool called: database_hset_multiple");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "HSET is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.".to_string(),
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let fields: Vec<(String, String)> = params
            .fields
            .into_iter()
            .map(|f| (f.field, f.value))
            .collect();
        let added = tools
            .hset_multiple(&params.key, &fields)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "fields_processed": fields.len(),
                "fields_added": added
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Delete one or more fields from a hash (HDEL command). Returns the number of fields that were removed (non-existing fields are not counted). Use this to remove specific properties from an object without deleting the entire hash."
    )]
    async fn database_hdel(
        &self,
        Parameters(params): Parameters<DatabaseHdelParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, fields = ?params.fields, "Tool called: database_hdel");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "HDEL is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.".to_string(),
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let deleted = tools
            .hdel(&params.key, &params.fields)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "fields": params.fields,
                "deleted": deleted
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Get a specific field from a hash (HGET command). Returns null if the field or hash doesn't exist. Use this when you only need one field from a hash instead of fetching all fields with HGETALL."
    )]
    async fn database_hget(
        &self,
        Parameters(params): Parameters<DatabaseHgetParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, field = %params.field, "Tool called: database_hget");

        let tools = self.get_database_tools().await?;
        let value = tools
            .hget(&params.key, &params.field)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "field": params.field,
                "value": value
            })
            .to_string(),
        )]))
    }

    // ========== LIST WRITE OPERATIONS ==========

    #[tool(
        description = "Push values to the left (head) of a list (LPUSH command). Creates the list if it doesn't exist. Values are inserted at the head, so the last value in the input array will be the first element in the list. Returns the new length of the list. Use this for implementing stacks (LIFO) or adding items to the front of a queue."
    )]
    async fn database_lpush(
        &self,
        Parameters(params): Parameters<DatabaseListPushParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, value_count = params.values.len(), "Tool called: database_lpush");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "LPUSH is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.".to_string(),
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let new_length = tools
            .lpush(&params.key, &params.values)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "values_pushed": params.values.len(),
                "new_length": new_length
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Push values to the right (tail) of a list (RPUSH command). Creates the list if it doesn't exist. Values are appended to the end in order. Returns the new length of the list. Use this for implementing queues (FIFO), event logs, or message lists where order matters."
    )]
    async fn database_rpush(
        &self,
        Parameters(params): Parameters<DatabaseListPushParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, value_count = params.values.len(), "Tool called: database_rpush");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "RPUSH is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.".to_string(),
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let new_length = tools
            .rpush(&params.key, &params.values)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "values_pushed": params.values.len(),
                "new_length": new_length
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Pop and return a value from the left (head) of a list (LPOP command). Removes and returns the first element. Returns null if the list is empty or doesn't exist. Use this with RPUSH for queue (FIFO) behavior or with LPUSH for stack (LIFO) behavior."
    )]
    async fn database_lpop(
        &self,
        Parameters(params): Parameters<DatabaseKeyParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, "Tool called: database_lpop");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "LPOP is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.".to_string(),
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let value = tools
            .lpop(&params.key)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "value": value
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Pop and return a value from the right (tail) of a list (RPOP command). Removes and returns the last element. Returns null if the list is empty or doesn't exist. Use this with LPUSH for queue (FIFO) behavior."
    )]
    async fn database_rpop(
        &self,
        Parameters(params): Parameters<DatabaseKeyParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, "Tool called: database_rpop");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "RPOP is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.".to_string(),
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let value = tools
            .rpop(&params.key)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "value": value
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Get an element from a list by index (LINDEX command). Index is 0-based; negative indices count from the end (-1 is the last element). Returns null if the index is out of range. Use this to peek at specific positions without removing elements."
    )]
    async fn database_lindex(
        &self,
        Parameters(params): Parameters<DatabaseLindexParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, index = params.index, "Tool called: database_lindex");

        let tools = self.get_database_tools().await?;
        let value = tools
            .lindex(&params.key, params.index)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "index": params.index,
                "value": value
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Set an element in a list at a specific index (LSET command). The index must be within the list bounds or an error is returned. Use this to update specific elements in a list without rebuilding the entire list."
    )]
    async fn database_lset(
        &self,
        Parameters(params): Parameters<DatabaseLsetParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, index = params.index, "Tool called: database_lset");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "LSET is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.".to_string(),
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        tools
            .lset(&params.key, params.index, &params.value)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "index": params.index,
                "success": true
            })
            .to_string(),
        )]))
    }

    // ========== SET WRITE OPERATIONS ==========

    #[tool(
        description = "Add members to a set (SADD command). Creates the set if it doesn't exist. Sets store unique values - duplicates are automatically ignored. Returns the number of members that were actually added (not already present). Use sets for tags, categories, unique visitor tracking, or any collection where uniqueness matters."
    )]
    async fn database_sadd(
        &self,
        Parameters(params): Parameters<DatabaseSetMembersParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, member_count = params.members.len(), "Tool called: database_sadd");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "SADD is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.".to_string(),
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let added = tools
            .sadd(&params.key, &params.members)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "members_provided": params.members.len(),
                "members_added": added
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Remove members from a set (SREM command). Returns the number of members that were actually removed (members that didn't exist are not counted). Use this to untag items, remove categories, or delete specific values from a set."
    )]
    async fn database_srem(
        &self,
        Parameters(params): Parameters<DatabaseSetMembersParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, member_count = params.members.len(), "Tool called: database_srem");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "SREM is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.".to_string(),
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let removed = tools
            .srem(&params.key, &params.members)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "members_provided": params.members.len(),
                "members_removed": removed
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Check if a member exists in a set (SISMEMBER command). Returns true if the member is in the set, false otherwise. Use this to check membership before adding, or to verify tags/permissions."
    )]
    async fn database_sismember(
        &self,
        Parameters(params): Parameters<DatabaseSismemberParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, member = %params.member, "Tool called: database_sismember");

        let tools = self.get_database_tools().await?;
        let is_member = tools
            .sismember(&params.key, &params.member)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "member": params.member,
                "is_member": is_member
            })
            .to_string(),
        )]))
    }

    // ========== SORTED SET OPERATIONS ==========

    #[tool(
        description = "Add members with scores to a sorted set (ZADD command). Creates the sorted set if it doesn't exist. Members are automatically ordered by score. If a member already exists, its score is updated. Returns the number of new members added (not updated). Use sorted sets for leaderboards, priority queues, time-series data, or any ranked data."
    )]
    async fn database_zadd(
        &self,
        Parameters(params): Parameters<DatabaseZaddParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, member_count = params.members.len(), "Tool called: database_zadd");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "ZADD is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.".to_string(),
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let members: Vec<(f64, String)> = params
            .members
            .into_iter()
            .map(|m| (m.score, m.member))
            .collect();
        let added = tools
            .zadd(&params.key, &members)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "members_provided": members.len(),
                "members_added": added
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Remove members from a sorted set (ZREM command). Returns the number of members that were actually removed. Use this to remove players from leaderboards, delete scheduled tasks, or clean up ranked data."
    )]
    async fn database_zrem(
        &self,
        Parameters(params): Parameters<DatabaseZremParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, member_count = params.members.len(), "Tool called: database_zrem");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "ZREM is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.".to_string(),
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let removed = tools
            .zrem(&params.key, &params.members)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "members_provided": params.members.len(),
                "members_removed": removed
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Get the score of a member in a sorted set (ZSCORE command). Returns null if the member doesn't exist. Use this to look up a player's score, check priority levels, or get the timestamp of a scheduled item."
    )]
    async fn database_zscore(
        &self,
        Parameters(params): Parameters<DatabaseZscoreParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, member = %params.member, "Tool called: database_zscore");

        let tools = self.get_database_tools().await?;
        let score = tools
            .zscore(&params.key, &params.member)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "member": params.member,
                "score": score
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Get the rank (position) of a member in a sorted set (ZRANK command). Rank is 0-based with the lowest score at rank 0. Returns null if the member doesn't exist. Use this to find a player's position on a leaderboard or determine priority order."
    )]
    async fn database_zrank(
        &self,
        Parameters(params): Parameters<DatabaseZscoreParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, member = %params.member, "Tool called: database_zrank");

        let tools = self.get_database_tools().await?;
        let rank = tools
            .zrank(&params.key, &params.member)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "member": params.member,
                "rank": rank
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Get the reverse rank (position from highest score) of a member in a sorted set (ZREVRANK command). Rank is 0-based with the highest score at rank 0. Returns null if the member doesn't exist. Use this for leaderboards where higher scores are better."
    )]
    async fn database_zrevrank(
        &self,
        Parameters(params): Parameters<DatabaseZscoreParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, member = %params.member, "Tool called: database_zrevrank");

        let tools = self.get_database_tools().await?;
        let rank = tools
            .zrevrank(&params.key, &params.member)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "member": params.member,
                "rank": rank
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Get a range of members from a sorted set in reverse order, highest to lowest score (ZREVRANGE command). Use start=0, stop=-1 to get all members. Perfect for leaderboards where you want the top scorers first. Use this instead of ZRANGE when higher scores should appear first."
    )]
    async fn database_zrevrange(
        &self,
        Parameters(params): Parameters<DatabaseZrangeParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, start = params.start, stop = params.stop, "Tool called: database_zrevrange");

        let tools = self.get_database_tools().await?;
        let members = tools
            .zrevrange(&params.key, params.start, params.stop)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "start": params.start,
                "stop": params.stop,
                "members": members
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Get members from a sorted set within a score range (ZRANGEBYSCORE command). Use \"-inf\" for negative infinity and \"+inf\" for positive infinity. Returns members with scores between min and max (inclusive). Use this to query time ranges, price ranges, or any score-based filtering."
    )]
    async fn database_zrangebyscore(
        &self,
        Parameters(params): Parameters<DatabaseZrangebyscoreParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, min = %params.min, max = %params.max, "Tool called: database_zrangebyscore");

        let tools = self.get_database_tools().await?;

        // Parse min/max, handling special values
        let min: f64 = match params.min.as_str() {
            "-inf" => f64::NEG_INFINITY,
            "+inf" => f64::INFINITY,
            s => s.parse().map_err(|_| {
                RmcpError::invalid_params(format!("Invalid min score: {}", s), None)
            })?,
        };
        let max: f64 = match params.max.as_str() {
            "-inf" => f64::NEG_INFINITY,
            "+inf" => f64::INFINITY,
            s => s.parse().map_err(|_| {
                RmcpError::invalid_params(format!("Invalid max score: {}", s), None)
            })?,
        };

        let members = tools
            .zrangebyscore(&params.key, min, max)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "min": params.min,
                "max": params.max,
                "members": members
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Increment a member's score in a sorted set (ZINCRBY command). Creates the member with the increment as its score if it doesn't exist. Returns the new score. Use negative increment to decrement. Perfect for updating leaderboard scores, adjusting priorities, or accumulating points."
    )]
    async fn database_zincrby(
        &self,
        Parameters(params): Parameters<DatabaseZincrbyParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, member = %params.member, increment = params.increment, "Tool called: database_zincrby");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "ZINCRBY is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.".to_string(),
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let new_score = tools
            .zincrby(&params.key, params.increment, &params.member)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "member": params.member,
                "increment": params.increment,
                "new_score": new_score
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Get a range of members with their scores from a sorted set (ZRANGE WITHSCORES). Returns members ordered from lowest to highest score, each with their score. Use this when you need both the member and its score, like displaying a leaderboard with points."
    )]
    async fn database_zrange_withscores(
        &self,
        Parameters(params): Parameters<DatabaseZrangeParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, start = params.start, stop = params.stop, "Tool called: database_zrange_withscores");

        let tools = self.get_database_tools().await?;
        let members = tools
            .zrange_withscores(&params.key, params.start, params.stop)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        let result: Vec<serde_json::Value> = members
            .into_iter()
            .map(|(member, score)| serde_json::json!({"member": member, "score": score}))
            .collect();

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "start": params.start,
                "stop": params.stop,
                "members": result
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Get a range of members with their scores in reverse order, highest to lowest (ZREVRANGE WITHSCORES). Returns members from highest to lowest score, each with their score. Perfect for leaderboards showing top players with their points."
    )]
    async fn database_zrevrange_withscores(
        &self,
        Parameters(params): Parameters<DatabaseZrangeParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, start = params.start, stop = params.stop, "Tool called: database_zrevrange_withscores");

        let tools = self.get_database_tools().await?;
        let members = tools
            .zrevrange_withscores(&params.key, params.start, params.stop)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        let result: Vec<serde_json::Value> = members
            .into_iter()
            .map(|(member, score)| serde_json::json!({"member": member, "score": score}))
            .collect();

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "start": params.start,
                "stop": params.stop,
                "members": result
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Rename a key (RENAME command). Atomically renames a key to a new name. If the new key already exists, it will be overwritten. Returns an error if the source key doesn't exist. Use this to reorganize your key namespace or implement atomic key swaps."
    )]
    async fn database_rename(
        &self,
        Parameters(params): Parameters<DatabaseRenameParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, new_key = %params.new_key, "Tool called: database_rename");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "RENAME is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.".to_string(),
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        tools
            .rename(&params.key, &params.new_key)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "old_key": params.key,
                "new_key": params.new_key,
                "success": true
            })
            .to_string(),
        )]))
    }

    // ==================== REDISEARCH TOOLS ====================

    #[tool(
        description = "Search a RediSearch index (FT.SEARCH command). Executes a full-text search query against an index, returning matching documents. Supports filters, sorting, pagination, highlighting, and scoring. Use NOCONTENT to get only document IDs for large result sets."
    )]
    async fn database_ft_search(
        &self,
        Parameters(params): Parameters<FtSearchParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(index = %params.index, query = %params.query, "Tool called: database_ft_search");

        let tools = self.get_database_tools().await?;

        use crate::database_tools::FtSearchOptions;
        let options = FtSearchOptions {
            nocontent: params.nocontent,
            verbatim: params.verbatim,
            withscores: params.withscores,
            return_fields: params.return_fields,
            sortby: params.sortby,
            sortby_desc: params.sortby_desc,
            limit_offset: params.limit_offset,
            limit_num: params.limit_num,
            highlight_fields: params.highlight_fields,
            highlight_tags_open: params.highlight_open,
            highlight_tags_close: params.highlight_close,
            language: params.language,
            slop: params.slop,
            inorder: params.inorder,
            timeout: params.timeout,
            dialect: params.dialect,
        };

        let result = tools
            .ft_search(&params.index, &params.query, &options)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "index": params.index,
                "query": params.query,
                "result": value_to_json(&result)
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Run an aggregation query on a RediSearch index (FT.AGGREGATE command). Performs complex aggregations including grouping, sorting, applying transformations, and reducing. Powerful for analytics and reporting on indexed data."
    )]
    async fn database_ft_aggregate(
        &self,
        Parameters(params): Parameters<FtAggregateParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(index = %params.index, query = %params.query, "Tool called: database_ft_aggregate");

        let tools = self.get_database_tools().await?;

        use crate::database_tools::{FtAggregateOptions, FtApply, FtGroupBy, FtReducer};

        let groupby = params
            .groupby
            .into_iter()
            .map(|g| FtGroupBy {
                properties: g.properties,
                reducers: g
                    .reducers
                    .into_iter()
                    .map(|r| FtReducer {
                        function: r.function,
                        args: r.args,
                        alias: r.alias,
                    })
                    .collect(),
            })
            .collect();

        let apply = params
            .apply
            .into_iter()
            .map(|a| FtApply {
                expression: a.expression,
                alias: a.alias,
            })
            .collect();

        // Convert sortby from Vec<Vec<String>> to Vec<(String, String)>
        let sortby = params.sortby.map(|sb| {
            sb.into_iter()
                .filter_map(|pair| {
                    if pair.len() >= 2 {
                        Some((pair[0].clone(), pair[1].clone()))
                    } else if pair.len() == 1 {
                        Some((pair[0].clone(), "ASC".to_string()))
                    } else {
                        None
                    }
                })
                .collect()
        });

        let options = FtAggregateOptions {
            verbatim: params.verbatim,
            load: params.load,
            groupby,
            apply,
            sortby,
            sortby_max: params.sortby_max,
            filter: params.filter,
            limit_offset: params.limit_offset,
            limit_num: params.limit_num,
            timeout: params.timeout,
            dialect: params.dialect,
        };

        let result = tools
            .ft_aggregate(&params.index, &params.query, &options)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "index": params.index,
                "query": params.query,
                "result": value_to_json(&result)
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Get information about a RediSearch index (FT.INFO command). Returns index schema, number of documents, indexing status, memory usage, and configuration. Useful for monitoring and debugging index performance."
    )]
    async fn database_ft_info(
        &self,
        Parameters(params): Parameters<FtIndexParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(index = %params.index, "Tool called: database_ft_info");

        let tools = self.get_database_tools().await?;
        let result = tools
            .ft_info(&params.index)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "index": params.index,
                "info": value_to_json(&result)
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "List all RediSearch indexes (FT._LIST command). Returns the names of all full-text search indexes in the database. Use FT.INFO on individual indexes for detailed information."
    )]
    async fn database_ft_list(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: database_ft_list");

        let tools = self.get_database_tools().await?;
        let indexes = tools
            .ft_list()
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "indexes": indexes
            })
            .to_string(),
        )]))
    }

    // ==================== REDISEARCH INDEX MANAGEMENT TOOLS ====================

    #[tool(
        description = "Create a new RediSearch index with schema definition (FT.CREATE command). This is the primary command for setting up full-text search. Define which keys to index using prefixes, and specify fields with their types (TEXT for full-text, TAG for exact match, NUMERIC for ranges, GEO for location, VECTOR for embeddings). Each field can have options like SORTABLE, NOSTEM, PHONETIC matching, etc. Use 'on' parameter to choose between HASH and JSON document types."
    )]
    async fn database_ft_create(
        &self,
        Parameters(params): Parameters<FtCreateParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(index = %params.index, "Tool called: database_ft_create");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "FT.CREATE is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let result = tools
            .ft_create(&params)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "index": params.index,
                "result": value_to_json(&result),
                "message": "Index created successfully"
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Delete a RediSearch index (FT.DROPINDEX command). Removes the index and optionally deletes the indexed documents. Without 'dd' flag, only the index is removed and documents remain. With 'dd' flag, both the index AND the actual Redis keys are deleted - use with caution in production!"
    )]
    async fn database_ft_dropindex(
        &self,
        Parameters(params): Parameters<FtDropIndexParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(index = %params.index, dd = %params.dd, "Tool called: database_ft_dropindex");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "FT.DROPINDEX is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let result = tools
            .ft_dropindex(&params.index, params.dd)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "index": params.index,
                "documents_deleted": params.dd,
                "result": value_to_json(&result),
                "message": if params.dd { "Index and documents deleted" } else { "Index deleted (documents preserved)" }
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Add a new field to an existing RediSearch index (FT.ALTER command). Only supports adding fields - you cannot modify or remove existing fields. Useful for evolving your search schema as requirements change. Use skip_initial_scan=true to avoid rescanning existing documents (the new field will only be indexed for new/modified documents)."
    )]
    async fn database_ft_alter(
        &self,
        Parameters(params): Parameters<FtAlterParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(index = %params.index, field = %params.field.name, "Tool called: database_ft_alter");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "FT.ALTER is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let result = tools
            .ft_alter(&params.index, params.skip_initial_scan, &params.field)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "index": params.index,
                "field_added": params.field.name,
                "result": value_to_json(&result),
                "message": "Field added to index schema"
            })
            .to_string(),
        )]))
    }

    // ==================== REDISEARCH QUERY DEBUGGING TOOLS ====================

    #[tool(
        description = "Get the execution plan for a query without running it (FT.EXPLAIN command). Essential for debugging slow queries and understanding how RediSearch parses and executes your query. Returns a textual representation of the query tree showing INTERSECT, UNION, NUMERIC, TAG operations. Use this to optimize complex queries by understanding which operations are most expensive."
    )]
    async fn database_ft_explain(
        &self,
        Parameters(params): Parameters<FtExplainParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(index = %params.index, query = %params.query, "Tool called: database_ft_explain");

        let tools = self.get_database_tools().await?;
        let result = tools
            .ft_explain(&params.index, &params.query, params.dialect)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "index": params.index,
                "query": params.query,
                "execution_plan": value_to_json(&result)
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Get all unique values for a TAG field (FT.TAGVALS command). Returns every distinct tag value that exists in the indexed documents. Useful for: building filter UIs/facets, understanding data distribution, debugging why tag filters aren't matching, validating data quality. Note: Only works on TAG type fields, not TEXT fields."
    )]
    async fn database_ft_tagvals(
        &self,
        Parameters(params): Parameters<FtTagvalsParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(index = %params.index, field = %params.field, "Tool called: database_ft_tagvals");

        let tools = self.get_database_tools().await?;
        let result = tools
            .ft_tagvals(&params.index, &params.field)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "index": params.index,
                "field": params.field,
                "values": value_to_json(&result)
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Get spelling suggestions for query terms (FT.SPELLCHECK command). Checks each term in the query against the index vocabulary and suggests corrections. Perfect for implementing 'did you mean?' functionality. The distance parameter controls how different suggestions can be (1=one character difference like typos, up to 4 for more aggressive matching). Returns suggestions ranked by how common the suggested term is in the index."
    )]
    async fn database_ft_spellcheck(
        &self,
        Parameters(params): Parameters<FtSpellcheckParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(index = %params.index, query = %params.query, "Tool called: database_ft_spellcheck");

        let tools = self.get_database_tools().await?;
        let result = tools
            .ft_spellcheck(
                &params.index,
                &params.query,
                params.distance,
                params.dialect,
            )
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "index": params.index,
                "query": params.query,
                "suggestions": value_to_json(&result)
            })
            .to_string(),
        )]))
    }

    // ==================== REDISEARCH ALIAS TOOLS ====================

    #[tool(
        description = "Create an alias pointing to an index (FT.ALIASADD command). Aliases enable zero-downtime index rebuilds: create alias 'products' -> 'products_v1', rebuild to 'products_v2', then update alias. Your application always queries 'products' and instantly switches to the new index. Aliases also allow multiple names for the same index for different use cases."
    )]
    async fn database_ft_aliasadd(
        &self,
        Parameters(params): Parameters<FtAliasAddParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(alias = %params.alias, index = %params.index, "Tool called: database_ft_aliasadd");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "FT.ALIASADD is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let result = tools
            .ft_aliasadd(&params.alias, &params.index)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "alias": params.alias,
                "index": params.index,
                "result": value_to_json(&result),
                "message": format!("Alias '{}' now points to index '{}'", params.alias, params.index)
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Delete an index alias (FT.ALIASDEL command). Removes the alias but does NOT affect the underlying index or its data. After deletion, queries using the alias name will fail until a new alias is created."
    )]
    async fn database_ft_aliasdel(
        &self,
        Parameters(params): Parameters<FtAliasDelParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(alias = %params.alias, "Tool called: database_ft_aliasdel");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "FT.ALIASDEL is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let result = tools
            .ft_aliasdel(&params.alias)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "alias": params.alias,
                "result": value_to_json(&result),
                "message": format!("Alias '{}' deleted", params.alias)
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Update an alias to point to a different index (FT.ALIASUPDATE command). This is atomic - queries instantly switch to the new index with no downtime. If the alias doesn't exist, it will be created. Use for blue-green deployments: rebuild index, test it, then atomically switch production traffic."
    )]
    async fn database_ft_aliasupdate(
        &self,
        Parameters(params): Parameters<FtAliasUpdateParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(alias = %params.alias, index = %params.index, "Tool called: database_ft_aliasupdate");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "FT.ALIASUPDATE is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let result = tools
            .ft_aliasupdate(&params.alias, &params.index)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "alias": params.alias,
                "index": params.index,
                "result": value_to_json(&result),
                "message": format!("Alias '{}' updated to point to index '{}'", params.alias, params.index)
            })
            .to_string(),
        )]))
    }

    // ==================== REDISEARCH AUTOCOMPLETE TOOLS ====================

    #[tool(
        description = "Add a suggestion to an autocomplete dictionary (FT.SUGADD command). Build type-ahead search functionality by storing suggestions with scores. Higher scores rank suggestions higher. Use 'incr' to update scores based on popularity (e.g., increment each time a suggestion is selected). Optionally store payload data like IDs or categories with each suggestion."
    )]
    async fn database_ft_sugadd(
        &self,
        Parameters(params): Parameters<FtSugAddParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, string = %params.string, score = %params.score, "Tool called: database_ft_sugadd");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "FT.SUGADD is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let result = tools
            .ft_sugadd(
                &params.key,
                &params.string,
                params.score,
                params.incr,
                params.payload.as_deref(),
            )
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "suggestion": params.string,
                "score": params.score,
                "incremented": params.incr,
                "result": value_to_json(&result),
                "message": "Suggestion added to autocomplete dictionary"
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Get autocomplete suggestions matching a prefix (FT.SUGGET command). Returns suggestions starting with the given prefix, ranked by score. Enable 'fuzzy' for typo tolerance (matches with 1 character difference). Use 'max' to limit results. 'withscores' returns ranking scores, 'withpayloads' returns stored metadata. Perfect for search-as-you-type interfaces."
    )]
    async fn database_ft_sugget(
        &self,
        Parameters(params): Parameters<FtSugGetParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, prefix = %params.prefix, "Tool called: database_ft_sugget");

        let tools = self.get_database_tools().await?;
        let result = tools
            .ft_sugget(
                &params.key,
                &params.prefix,
                params.fuzzy,
                params.max,
                params.withscores,
                params.withpayloads,
            )
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "prefix": params.prefix,
                "fuzzy": params.fuzzy,
                "suggestions": value_to_json(&result)
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Delete a suggestion from an autocomplete dictionary (FT.SUGDEL command). Removes the exact suggestion string from the dictionary. Returns 1 if the suggestion was found and deleted, 0 if not found."
    )]
    async fn database_ft_sugdel(
        &self,
        Parameters(params): Parameters<FtSugDelParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, string = %params.string, "Tool called: database_ft_sugdel");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "FT.SUGDEL is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let result = tools
            .ft_sugdel(&params.key, &params.string)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "suggestion": params.string,
                "deleted": value_to_json(&result)
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Get the number of suggestions in an autocomplete dictionary (FT.SUGLEN command). Returns the total count of unique suggestions stored. Useful for monitoring dictionary size and capacity planning."
    )]
    async fn database_ft_suglen(
        &self,
        Parameters(params): Parameters<FtSugLenParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, "Tool called: database_ft_suglen");

        let tools = self.get_database_tools().await?;
        let result = tools
            .ft_suglen(&params.key)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "count": value_to_json(&result)
            })
            .to_string(),
        )]))
    }

    // ==================== REDISEARCH SYNONYM TOOLS ====================

    #[tool(
        description = "Get all synonym groups from an index (FT.SYNDUMP command). Returns a mapping of terms to their synonym group IDs. Useful for reviewing current synonym configuration, debugging why searches aren't matching expected synonyms, and exporting synonym data for backup."
    )]
    async fn database_ft_syndump(
        &self,
        Parameters(params): Parameters<FtSynDumpParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(index = %params.index, "Tool called: database_ft_syndump");

        let tools = self.get_database_tools().await?;
        let result = tools
            .ft_syndump(&params.index)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "index": params.index,
                "synonyms": value_to_json(&result)
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Add or update a synonym group (FT.SYNUPDATE command). Synonyms make searching for one term match documents containing related terms. Example: group 'color' with terms ['red', 'crimson', 'scarlet'] - searching for 'red' finds documents with any of these terms. Each call adds terms to the group (doesn't replace). Use skip_initial_scan=true to only apply to new documents."
    )]
    async fn database_ft_synupdate(
        &self,
        Parameters(params): Parameters<FtSynUpdateParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(index = %params.index, group_id = %params.group_id, terms = ?params.terms, "Tool called: database_ft_synupdate");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "FT.SYNUPDATE is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let result = tools
            .ft_synupdate(
                &params.index,
                &params.group_id,
                params.skip_initial_scan,
                &params.terms,
            )
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "index": params.index,
                "group_id": params.group_id,
                "terms": params.terms,
                "result": value_to_json(&result),
                "message": format!("Synonym group '{}' updated with {} terms", params.group_id, params.terms.len())
            })
            .to_string(),
        )]))
    }

    // ==================== REDISJSON TOOLS ====================

    #[tool(
        description = "Get JSON value(s) from a key (JSON.GET command). Retrieves JSON data at one or more paths. Returns the JSON-encoded value. Use JSONPath syntax for paths (e.g., '$.store.book[0].title' or '$..price' for recursive). Multiple paths return an object with path keys."
    )]
    async fn database_json_get(
        &self,
        Parameters(params): Parameters<JsonGetParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, "Tool called: database_json_get");

        let tools = self.get_database_tools().await?;
        let paths = if params.paths.is_empty() {
            vec!["$".to_string()]
        } else {
            params.paths
        };
        let result = tools
            .json_get(
                &params.key,
                &paths,
                params.indent.as_deref(),
                params.newline.as_deref(),
                params.space.as_deref(),
            )
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "paths": paths,
                "value": result
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Set a JSON value at a path (JSON.SET command). Creates or updates JSON data. Use NX to only set if path doesn't exist, XX to only update existing paths. The value must be valid JSON. Path '$' sets the root."
    )]
    async fn database_json_set(
        &self,
        Parameters(params): Parameters<JsonSetParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, path = %params.path, "Tool called: database_json_set");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "JSON.SET is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.".to_string(),
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let success = tools
            .json_set(
                &params.key,
                &params.path,
                &params.value,
                params.nx,
                params.xx,
            )
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "path": params.path,
                "success": success
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Delete a JSON value at a path (JSON.DEL command). Removes the JSON value at the specified path. Returns the number of paths deleted. If path is omitted, deletes the entire key."
    )]
    async fn database_json_del(
        &self,
        Parameters(params): Parameters<JsonDelParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, "Tool called: database_json_del");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "JSON.DEL is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.".to_string(),
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let deleted = tools
            .json_del(&params.key, params.path.as_deref())
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "path": params.path,
                "deleted": deleted
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Get the type of JSON value at a path (JSON.TYPE command). Returns the JSON type: object, array, string, integer, number, boolean, or null. Useful for introspecting JSON structure."
    )]
    async fn database_json_type(
        &self,
        Parameters(params): Parameters<JsonPathParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, "Tool called: database_json_type");

        let tools = self.get_database_tools().await?;
        let result = tools
            .json_type(&params.key, params.path.as_deref())
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "path": params.path,
                "type": value_to_json(&result)
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Append values to a JSON array (JSON.ARRAPPEND command). Adds one or more JSON values to the end of the array at the specified path. Returns the new array length. Values must be valid JSON."
    )]
    async fn database_json_arrappend(
        &self,
        Parameters(params): Parameters<JsonArrAppendParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, path = %params.path, "Tool called: database_json_arrappend");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "JSON.ARRAPPEND is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.".to_string(),
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let result = tools
            .json_arrappend(&params.key, &params.path, &params.values)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "path": params.path,
                "new_length": value_to_json(&result)
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Get the length of a JSON array (JSON.ARRLEN command). Returns the number of elements in the array at the specified path. Returns null if the path doesn't exist or isn't an array."
    )]
    async fn database_json_arrlen(
        &self,
        Parameters(params): Parameters<JsonPathParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, "Tool called: database_json_arrlen");

        let tools = self.get_database_tools().await?;
        let result = tools
            .json_arrlen(&params.key, params.path.as_deref())
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "path": params.path,
                "length": value_to_json(&result)
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Increment a JSON number by a value (JSON.NUMINCRBY command). Atomically increments the number at the specified path. Returns the new value. Use negative values to decrement."
    )]
    async fn database_json_numincrby(
        &self,
        Parameters(params): Parameters<JsonNumIncrByParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, path = %params.path, value = %params.value, "Tool called: database_json_numincrby");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "JSON.NUMINCRBY is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.".to_string(),
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let result = tools
            .json_numincrby(&params.key, &params.path, params.value)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "path": params.path,
                "increment": params.value,
                "new_value": result
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Get the length of a JSON string (JSON.STRLEN command). Returns the length of the string at the specified path. Returns null if the path doesn't exist or isn't a string."
    )]
    async fn database_json_strlen(
        &self,
        Parameters(params): Parameters<JsonPathParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, "Tool called: database_json_strlen");

        let tools = self.get_database_tools().await?;
        let result = tools
            .json_strlen(&params.key, params.path.as_deref())
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "path": params.path,
                "length": value_to_json(&result)
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Get JSON values from multiple keys at once (JSON.MGET command). Efficiently retrieves the same JSONPath from many documents in a single operation. Returns an array with values for each key (null for missing keys). Essential for batch reads - much faster than multiple JSON.GET calls."
    )]
    async fn database_json_mget(
        &self,
        Parameters(params): Parameters<JsonMgetParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(keys = ?params.keys, path = %params.path, "Tool called: database_json_mget");

        let tools = self.get_database_tools().await?;
        let result = tools
            .json_mget(&params.keys, &params.path)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "keys": params.keys,
                "path": params.path,
                "values": value_to_json(&result)
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Get all keys from a JSON object (JSON.OBJKEYS command). Returns an array of field names at the specified path. Useful for introspecting document structure, building dynamic UIs, or validating schemas. Path must point to an object, not an array or scalar."
    )]
    async fn database_json_objkeys(
        &self,
        Parameters(params): Parameters<JsonObjKeysParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, path = %params.path, "Tool called: database_json_objkeys");

        let tools = self.get_database_tools().await?;
        let result = tools
            .json_objkeys(&params.key, &params.path)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "path": params.path,
                "keys": value_to_json(&result)
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Get the number of keys in a JSON object (JSON.OBJLEN command). Returns the count of fields at the specified path. Useful for checking object size without retrieving all keys. Path must point to an object."
    )]
    async fn database_json_objlen(
        &self,
        Parameters(params): Parameters<JsonObjLenParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, path = %params.path, "Tool called: database_json_objlen");

        let tools = self.get_database_tools().await?;
        let result = tools
            .json_objlen(&params.key, &params.path)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "path": params.path,
                "length": value_to_json(&result)
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Find the index of an element in a JSON array (JSON.ARRINDEX command). Returns the first index where the value is found, or -1 if not found. Supports optional start/stop indices to search within a range. The value must be valid JSON (use '\"string\"' for string values)."
    )]
    async fn database_json_arrindex(
        &self,
        Parameters(params): Parameters<JsonArrIndexParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, path = %params.path, value = %params.value, "Tool called: database_json_arrindex");

        let tools = self.get_database_tools().await?;
        let result = tools
            .json_arrindex(
                &params.key,
                &params.path,
                &params.value,
                params.start,
                params.stop,
            )
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "path": params.path,
                "value": params.value,
                "index": value_to_json(&result)
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Remove and return an element from a JSON array (JSON.ARRPOP command). By default pops the last element (-1). Use index 0 for first element. Negative indices count from end. Returns the popped value as JSON. Useful for implementing queues or stacks."
    )]
    async fn database_json_arrpop(
        &self,
        Parameters(params): Parameters<JsonArrPopParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, path = %params.path, "Tool called: database_json_arrpop");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "JSON.ARRPOP is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let result = tools
            .json_arrpop(&params.key, &params.path, params.index)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "path": params.path,
                "popped": value_to_json(&result)
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Trim a JSON array to a specified range (JSON.ARRTRIM command). Keeps only elements from start to stop (inclusive). Elements outside this range are removed. Useful for maintaining bounded arrays like activity logs or recent items. Negative indices count from end."
    )]
    async fn database_json_arrtrim(
        &self,
        Parameters(params): Parameters<JsonArrTrimParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, path = %params.path, start = %params.start, stop = %params.stop, "Tool called: database_json_arrtrim");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "JSON.ARRTRIM is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let result = tools
            .json_arrtrim(&params.key, &params.path, params.start, params.stop)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "path": params.path,
                "new_length": value_to_json(&result)
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Insert elements into a JSON array at a specific index (JSON.ARRINSERT command). Existing elements at and after the index shift right to make room. Negative indices count from end. Returns the new array length. Useful for inserting at specific positions."
    )]
    async fn database_json_arrinsert(
        &self,
        Parameters(params): Parameters<JsonArrInsertParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, path = %params.path, index = %params.index, "Tool called: database_json_arrinsert");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "JSON.ARRINSERT is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let result = tools
            .json_arrinsert(&params.key, &params.path, params.index, &params.values)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "path": params.path,
                "new_length": value_to_json(&result)
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Clear container values or set numbers to 0 (JSON.CLEAR command). For arrays: becomes []. For objects: becomes {}. For numbers: becomes 0. Strings and booleans are unchanged. Returns the count of values cleared. Useful for resetting parts of a document."
    )]
    async fn database_json_clear(
        &self,
        Parameters(params): Parameters<JsonClearParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, path = %params.path, "Tool called: database_json_clear");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "JSON.CLEAR is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let result = tools
            .json_clear(&params.key, &params.path)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "path": params.path,
                "cleared_count": value_to_json(&result)
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Toggle a boolean value (JSON.TOGGLE command). true becomes false, false becomes true. Returns the new boolean value(s). Errors if the path doesn't point to a boolean. Useful for feature flags and status toggles."
    )]
    async fn database_json_toggle(
        &self,
        Parameters(params): Parameters<JsonToggleParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, path = %params.path, "Tool called: database_json_toggle");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "JSON.TOGGLE is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.",
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let result = tools
            .json_toggle(&params.key, &params.path)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "path": params.path,
                "new_value": value_to_json(&result)
            })
            .to_string(),
        )]))
    }

    // ==================== REDISTIMESERIES TOOLS ====================

    #[tool(
        description = "Add a sample to a time series (TS.ADD command). Appends a timestamp-value pair. Use '*' for timestamp to auto-generate. Supports retention policy, encoding, chunk size, duplicate policy, and labels. Creates the key if it doesn't exist."
    )]
    async fn database_ts_add(
        &self,
        Parameters(params): Parameters<TsAddParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, timestamp = %params.timestamp, value = %params.value, "Tool called: database_ts_add");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "TS.ADD is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.".to_string(),
                None,
            ));
        }

        let tools = self.get_database_tools().await?;

        use crate::database_tools::TsAddOptions;
        let labels = params
            .labels
            .map(|l| l.into_iter().map(|lp| (lp.label, lp.value)).collect());
        let options = TsAddOptions {
            retention: params.retention,
            encoding: params.encoding,
            chunk_size: params.chunk_size,
            on_duplicate: params.on_duplicate,
            labels,
        };

        let result_ts = tools
            .ts_add(&params.key, &params.timestamp, params.value, &options)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "timestamp": result_ts,
                "value": params.value
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Get the last sample from a time series (TS.GET command). Returns the most recent timestamp-value pair. Useful for getting current/latest readings from sensors, metrics, etc."
    )]
    async fn database_ts_get(
        &self,
        Parameters(params): Parameters<DatabaseKeyParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, "Tool called: database_ts_get");

        let tools = self.get_database_tools().await?;
        let result = tools
            .ts_get(&params.key)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "sample": value_to_json(&result)
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Query a range of samples from a time series (TS.RANGE command). Returns samples between two timestamps. Supports filtering, counting, alignment, and aggregation (avg, sum, min, max, count, first, last, range, std.p, std.s, var.p, var.s)."
    )]
    async fn database_ts_range(
        &self,
        Parameters(params): Parameters<TsRangeParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, from = %params.from, to = %params.to, "Tool called: database_ts_range");

        let tools = self.get_database_tools().await?;

        use crate::database_tools::{TsAggregation, TsRangeOptions};

        // Build aggregation from separate fields if aggregation type is provided
        let aggregation = params.aggregation.map(|agg_type| TsAggregation {
            aggregator: agg_type,
            bucket_duration: params.bucket_duration.unwrap_or(1000), // default 1 second
            bucket_timestamp: None,
            empty: false,
        });

        let options = TsRangeOptions {
            latest: params.latest,
            filter_by_ts: params.filter_by_ts,
            filter_by_value_min: params.filter_by_value_min,
            filter_by_value_max: params.filter_by_value_max,
            count: params.count,
            align: params.align,
            aggregation,
        };

        let result = tools
            .ts_range(&params.key, &params.from, &params.to, &options)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "from": params.from,
                "to": params.to,
                "samples": value_to_json(&result)
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Get information about a time series (TS.INFO command). Returns metadata including retention, chunk count, memory usage, first/last timestamps, labels, and rules. Useful for monitoring and debugging."
    )]
    async fn database_ts_info(
        &self,
        Parameters(params): Parameters<DatabaseKeyParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, "Tool called: database_ts_info");

        let tools = self.get_database_tools().await?;
        let result = tools
            .ts_info(&params.key)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "info": value_to_json(&result)
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Create a new time series (TS.CREATE command). Creates an empty time series with optional retention, encoding, chunk size, duplicate policy, and labels. Use this to pre-configure a time series before adding samples."
    )]
    async fn database_ts_create(
        &self,
        Parameters(params): Parameters<TsCreateParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, "Tool called: database_ts_create");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "TS.CREATE is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.".to_string(),
                None,
            ));
        }

        let tools = self.get_database_tools().await?;

        use crate::database_tools::TsCreateOptions;
        let labels = params
            .labels
            .map(|l| l.into_iter().map(|lp| (lp.label, lp.value)).collect());
        let options = TsCreateOptions {
            retention: params.retention,
            encoding: params.encoding,
            chunk_size: params.chunk_size,
            duplicate_policy: params.duplicate_policy,
            labels,
        };

        tools
            .ts_create(&params.key, &options)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "created": true
            })
            .to_string(),
        )]))
    }

    // ==================== REDISBLOOM TOOLS ====================

    #[tool(
        description = "Create an empty Bloom filter (BF.RESERVE command). Initializes a Bloom filter with specified error rate and capacity. Use expansion factor for auto-scaling, or nonscaling to fix size. Lower error rate = more memory but fewer false positives."
    )]
    async fn database_bf_reserve(
        &self,
        Parameters(params): Parameters<BfReserveParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, error_rate = %params.error_rate, capacity = %params.capacity, "Tool called: database_bf_reserve");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "BF.RESERVE is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.".to_string(),
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        tools
            .bf_reserve(
                &params.key,
                params.error_rate,
                params.capacity,
                params.expansion,
                params.nonscaling,
            )
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "error_rate": params.error_rate,
                "capacity": params.capacity,
                "created": true
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Add an item to a Bloom filter (BF.ADD command). Adds a single item to the filter. Returns true if the item is newly added, false if it may have existed (could be false positive). Creates the filter with default parameters if it doesn't exist."
    )]
    async fn database_bf_add(
        &self,
        Parameters(params): Parameters<BfAddParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, item = %params.item, "Tool called: database_bf_add");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "BF.ADD is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.".to_string(),
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let added = tools
            .bf_add(&params.key, &params.item)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "item": params.item,
                "added": added
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Add multiple items to a Bloom filter (BF.MADD command). Adds multiple items in a single operation. Returns an array of booleans indicating whether each item was newly added. More efficient than multiple BF.ADD calls."
    )]
    async fn database_bf_madd(
        &self,
        Parameters(params): Parameters<BfMaddParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, count = %params.items.len(), "Tool called: database_bf_madd");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "BF.MADD is a write operation. Server is in read-only mode. Use --allow-writes to enable write operations.".to_string(),
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let results = tools
            .bf_madd(&params.key, &params.items)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "results": params.items.iter().zip(results.iter()).map(|(item, added)| {
                    serde_json::json!({"item": item, "added": added})
                }).collect::<Vec<_>>()
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Check if an item exists in a Bloom filter (BF.EXISTS command). Returns true if the item may exist (with false positive probability), false if it definitely doesn't exist. Bloom filters never have false negatives."
    )]
    async fn database_bf_exists(
        &self,
        Parameters(params): Parameters<BfExistsParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, item = %params.item, "Tool called: database_bf_exists");

        let tools = self.get_database_tools().await?;
        let exists = tools
            .bf_exists(&params.key, &params.item)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "item": params.item,
                "exists": exists
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Check if multiple items exist in a Bloom filter (BF.MEXISTS command). Checks multiple items in a single operation. Returns an array of booleans. More efficient than multiple BF.EXISTS calls."
    )]
    async fn database_bf_mexists(
        &self,
        Parameters(params): Parameters<BfMexistsParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, count = %params.items.len(), "Tool called: database_bf_mexists");

        let tools = self.get_database_tools().await?;
        let results = tools
            .bf_mexists(&params.key, &params.items)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "results": params.items.iter().zip(results.iter()).map(|(item, exists)| {
                    serde_json::json!({"item": item, "exists": exists})
                }).collect::<Vec<_>>()
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Get information about a Bloom filter (BF.INFO command). Returns filter metadata including capacity, size, number of filters, items inserted, and expansion rate. Useful for monitoring filter health and memory usage."
    )]
    async fn database_bf_info(
        &self,
        Parameters(params): Parameters<DatabaseKeyParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, "Tool called: database_bf_info");

        let tools = self.get_database_tools().await?;
        let result = tools
            .bf_info(&params.key)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "info": value_to_json(&result)
            })
            .to_string(),
        )]))
    }

    // ========== REDIS STREAMS TOOLS ==========

    #[tool(
        description = "Add an entry to a Redis stream (XADD command). Streams are append-only logs perfect for event sourcing, activity feeds, and message queues. Each entry has an auto-generated or specified ID and contains field-value pairs. Use maxlen to cap stream size and prevent unbounded growth."
    )]
    async fn database_xadd(
        &self,
        Parameters(params): Parameters<XaddParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, id = %params.id, fields = params.fields.len(), "Tool called: database_xadd");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "XADD is a write operation. Server is in read-only mode.",
                None,
            ));
        }

        let fields: Vec<(String, String)> = params
            .fields
            .into_iter()
            .map(|f| (f.field, f.value))
            .collect();

        let tools = self.get_database_tools().await?;
        let result = tools
            .xadd(
                &params.key,
                &params.id,
                &fields,
                params.maxlen,
                params.approximate,
            )
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "entry_id": result
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Read entries from one or more Redis streams (XREAD command). Returns entries with IDs greater than the specified IDs. Use \"0\" to read from the beginning, \"$\" to read only new entries. Supports blocking mode to wait for new entries - useful for real-time consumers."
    )]
    async fn database_xread(
        &self,
        Parameters(params): Parameters<XreadParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(keys = ?params.keys, ids = ?params.ids, "Tool called: database_xread");

        let tools = self.get_database_tools().await?;
        let result = tools
            .xread(&params.keys, &params.ids, params.count, params.block)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&value_to_json(&result))
                .unwrap_or_else(|_| "null".to_string()),
        )]))
    }

    #[tool(
        description = "Read entries from a stream within an ID range (XRANGE command). Use \"-\" for the first entry and \"+\" for the last entry. Returns entries in chronological order. Perfect for replaying events or paginating through stream history."
    )]
    async fn database_xrange(
        &self,
        Parameters(params): Parameters<XrangeParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, start = %params.start, end = %params.end, "Tool called: database_xrange");

        let tools = self.get_database_tools().await?;
        let result = tools
            .xrange(&params.key, &params.start, &params.end, params.count)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&value_to_json(&result))
                .unwrap_or_else(|_| "null".to_string()),
        )]))
    }

    #[tool(
        description = "Read entries from a stream in reverse order (XREVRANGE command). Returns entries from newest to oldest. Use \"+\" for the last entry and \"-\" for the first. Useful for getting the most recent entries first."
    )]
    async fn database_xrevrange(
        &self,
        Parameters(params): Parameters<XrevrangeParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, end = %params.end, start = %params.start, "Tool called: database_xrevrange");

        let tools = self.get_database_tools().await?;
        let result = tools
            .xrevrange(&params.key, &params.end, &params.start, params.count)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&value_to_json(&result))
                .unwrap_or_else(|_| "null".to_string()),
        )]))
    }

    #[tool(
        description = "Get the number of entries in a stream (XLEN command). Returns the count of entries currently in the stream."
    )]
    async fn database_xlen(
        &self,
        Parameters(params): Parameters<DatabaseKeyParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, "Tool called: database_xlen");

        let tools = self.get_database_tools().await?;
        let result = tools
            .xlen(&params.key)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "length": result
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Get detailed information about a stream (XINFO STREAM command). Returns metadata including length, first/last entry IDs, consumer groups, and optionally full entry data. Essential for monitoring and debugging streams."
    )]
    async fn database_xinfo_stream(
        &self,
        Parameters(params): Parameters<XinfoStreamParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, full = params.full, "Tool called: database_xinfo_stream");

        let tools = self.get_database_tools().await?;
        let result = tools
            .xinfo_stream(&params.key, params.full, params.count)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&value_to_json(&result))
                .unwrap_or_else(|_| "null".to_string()),
        )]))
    }

    #[tool(
        description = "Get information about consumer groups on a stream (XINFO GROUPS command). Returns details for each group including name, consumers count, pending messages, and last delivered ID."
    )]
    async fn database_xinfo_groups(
        &self,
        Parameters(params): Parameters<DatabaseKeyParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, "Tool called: database_xinfo_groups");

        let tools = self.get_database_tools().await?;
        let result = tools
            .xinfo_groups(&params.key)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&value_to_json(&result))
                .unwrap_or_else(|_| "null".to_string()),
        )]))
    }

    #[tool(
        description = "Get information about consumers in a group (XINFO CONSUMERS command). Returns details for each consumer including name, pending messages count, and idle time."
    )]
    async fn database_xinfo_consumers(
        &self,
        Parameters(params): Parameters<XinfoConsumersParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, group = %params.group, "Tool called: database_xinfo_consumers");

        let tools = self.get_database_tools().await?;
        let result = tools
            .xinfo_consumers(&params.key, &params.group)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&value_to_json(&result))
                .unwrap_or_else(|_| "null".to_string()),
        )]))
    }

    #[tool(
        description = "Create a consumer group on a stream (XGROUP CREATE command). Consumer groups enable multiple consumers to cooperatively process stream entries, with automatic load balancing and message acknowledgment. Use mkstream=true to create the stream if it doesn't exist."
    )]
    async fn database_xgroup_create(
        &self,
        Parameters(params): Parameters<XgroupCreateParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, group = %params.group, id = %params.id, "Tool called: database_xgroup_create");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "XGROUP CREATE is a write operation. Server is in read-only mode.",
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        tools
            .xgroup_create(&params.key, &params.group, &params.id, params.mkstream)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "group": params.group,
                "status": "created"
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Destroy a consumer group (XGROUP DESTROY command). Removes the group and all its consumers. Pending messages are lost. Use with caution in production."
    )]
    async fn database_xgroup_destroy(
        &self,
        Parameters(params): Parameters<XgroupDestroyParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, group = %params.group, "Tool called: database_xgroup_destroy");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "XGROUP DESTROY is a write operation. Server is in read-only mode.",
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let result = tools
            .xgroup_destroy(&params.key, &params.group)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "group": params.group,
                "destroyed": result == 1
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Delete a consumer from a group (XGROUP DELCONSUMER command). Returns the number of pending messages that were owned by the consumer. The pending messages become unassigned."
    )]
    async fn database_xgroup_delconsumer(
        &self,
        Parameters(params): Parameters<XgroupDelconsumerParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, group = %params.group, consumer = %params.consumer, "Tool called: database_xgroup_delconsumer");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "XGROUP DELCONSUMER is a write operation. Server is in read-only mode.",
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let result = tools
            .xgroup_delconsumer(&params.key, &params.group, &params.consumer)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "group": params.group,
                "consumer": params.consumer,
                "pending_messages_released": result
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Set the last delivered ID of a consumer group (XGROUP SETID command). Useful for resetting a group to reprocess messages or skip ahead."
    )]
    async fn database_xgroup_setid(
        &self,
        Parameters(params): Parameters<XgroupSetidParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, group = %params.group, id = %params.id, "Tool called: database_xgroup_setid");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "XGROUP SETID is a write operation. Server is in read-only mode.",
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        tools
            .xgroup_setid(&params.key, &params.group, &params.id)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "group": params.group,
                "id": params.id,
                "status": "updated"
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Read entries as a consumer in a group (XREADGROUP command). This is the primary way to consume streams with consumer groups. Use \">\" as the ID to get only new (undelivered) messages. Messages are added to the pending list until acknowledged with XACK."
    )]
    async fn database_xreadgroup(
        &self,
        Parameters(params): Parameters<XreadgroupParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(group = %params.group, consumer = %params.consumer, keys = ?params.keys, "Tool called: database_xreadgroup");

        let tools = self.get_database_tools().await?;
        let result = tools
            .xreadgroup(
                &params.group,
                &params.consumer,
                &params.keys,
                &params.ids,
                params.count,
                params.block,
                params.noack,
            )
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&value_to_json(&result))
                .unwrap_or_else(|_| "null".to_string()),
        )]))
    }

    #[tool(
        description = "Acknowledge messages as processed (XACK command). Removes messages from the pending entries list. Essential for reliable stream processing - unacknowledged messages can be reclaimed by other consumers."
    )]
    async fn database_xack(
        &self,
        Parameters(params): Parameters<XackParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, group = %params.group, ids = ?params.ids, "Tool called: database_xack");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "XACK is a write operation. Server is in read-only mode.",
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let result = tools
            .xack(&params.key, &params.group, &params.ids)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "group": params.group,
                "acknowledged": result
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Delete entries from a stream (XDEL command). Removes specific entries by ID. Note: Memory may not be immediately reclaimed due to stream's radix tree structure."
    )]
    async fn database_xdel(
        &self,
        Parameters(params): Parameters<XdelParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, ids = ?params.ids, "Tool called: database_xdel");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "XDEL is a write operation. Server is in read-only mode.",
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let result = tools
            .xdel(&params.key, &params.ids)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "deleted": result
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Trim a stream to a maximum length (XTRIM command). Removes oldest entries to cap stream size. Use approximate=true for better performance (may leave slightly more entries than specified)."
    )]
    async fn database_xtrim(
        &self,
        Parameters(params): Parameters<XtrimParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, maxlen = params.maxlen, "Tool called: database_xtrim");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "XTRIM is a write operation. Server is in read-only mode.",
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let result = tools
            .xtrim(&params.key, params.maxlen, params.approximate)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "key": params.key,
                "trimmed": result
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Get pending entries for a consumer group (XPENDING command). Without range parameters, returns a summary. With start/end/count, returns details about specific pending messages including their ID, consumer, idle time, and delivery count."
    )]
    async fn database_xpending(
        &self,
        Parameters(params): Parameters<XpendingParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, group = %params.group, "Tool called: database_xpending");

        let tools = self.get_database_tools().await?;
        let result = tools
            .xpending(
                &params.key,
                &params.group,
                params.start.as_deref(),
                params.end.as_deref(),
                params.count,
                params.consumer.as_deref(),
            )
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&value_to_json(&result))
                .unwrap_or_else(|_| "null".to_string()),
        )]))
    }

    #[tool(
        description = "Claim pending messages from another consumer (XCLAIM command). Transfers ownership of messages that have been idle for too long, enabling recovery from failed consumers. Returns the claimed messages."
    )]
    async fn database_xclaim(
        &self,
        Parameters(params): Parameters<XclaimParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, group = %params.group, consumer = %params.consumer, "Tool called: database_xclaim");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "XCLAIM is a write operation. Server is in read-only mode.",
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let result = tools
            .xclaim(
                &params.key,
                &params.group,
                &params.consumer,
                params.min_idle_time,
                &params.ids,
                params.idle,
                params.time,
                params.retrycount,
                params.force,
                params.justid,
            )
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&value_to_json(&result))
                .unwrap_or_else(|_| "null".to_string()),
        )]))
    }

    #[tool(
        description = "Auto-claim pending messages (XAUTOCLAIM command). Automatically scans and claims messages idle longer than min_idle_time. Simpler than XCLAIM - just specify a starting ID and the command finds and claims eligible messages."
    )]
    async fn database_xautoclaim(
        &self,
        Parameters(params): Parameters<XautoclaimParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(key = %params.key, group = %params.group, consumer = %params.consumer, "Tool called: database_xautoclaim");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "XAUTOCLAIM is a write operation. Server is in read-only mode.",
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let result = tools
            .xautoclaim(
                &params.key,
                &params.group,
                &params.consumer,
                params.min_idle_time,
                &params.start,
                params.count,
                params.justid,
            )
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&value_to_json(&result))
                .unwrap_or_else(|_| "null".to_string()),
        )]))
    }

    // ========== PUB/SUB TOOLS ==========

    #[tool(
        description = "Publish a message to a channel (PUBLISH command). Sends a message to all subscribers of the channel. Returns the number of clients that received the message. Note: Pub/Sub is fire-and-forget - messages are not persisted. For durable messaging, use Streams instead."
    )]
    async fn database_publish(
        &self,
        Parameters(params): Parameters<PublishParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(channel = %params.channel, "Tool called: database_publish");

        if self.config.read_only {
            return Err(RmcpError::invalid_request(
                "PUBLISH is a write operation. Server is in read-only mode.",
                None,
            ));
        }

        let tools = self.get_database_tools().await?;
        let result = tools
            .publish(&params.channel, &params.message)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "channel": params.channel,
                "receivers": result
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "List active Pub/Sub channels (PUBSUB CHANNELS command). Returns channels that have at least one subscriber. Use pattern to filter channels (e.g., \"news.*\" matches \"news.sports\", \"news.weather\")."
    )]
    async fn database_pubsub_channels(
        &self,
        Parameters(params): Parameters<PubsubChannelsParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(pattern = ?params.pattern, "Tool called: database_pubsub_channels");

        let tools = self.get_database_tools().await?;
        let result = tools
            .pubsub_channels(params.pattern.as_deref())
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "channels": result
            })
            .to_string(),
        )]))
    }

    #[tool(
        description = "Get subscriber count for specific channels (PUBSUB NUMSUB command). Returns the number of subscribers for each specified channel. Useful for monitoring channel popularity."
    )]
    async fn database_pubsub_numsub(
        &self,
        Parameters(params): Parameters<PubsubNumsubParam>,
    ) -> Result<CallToolResult, RmcpError> {
        info!(channels = ?params.channels, "Tool called: database_pubsub_numsub");

        let tools = self.get_database_tools().await?;
        let result = tools
            .pubsub_numsub(&params.channels)
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&value_to_json(&result))
                .unwrap_or_else(|_| "null".to_string()),
        )]))
    }

    #[tool(
        description = "Get the number of pattern subscriptions (PUBSUB NUMPAT command). Returns the total count of clients subscribed to patterns (via PSUBSCRIBE). Does not count channel subscriptions."
    )]
    async fn database_pubsub_numpat(&self) -> Result<CallToolResult, RmcpError> {
        info!("Tool called: database_pubsub_numpat");

        let tools = self.get_database_tools().await?;
        let result = tools
            .pubsub_numpat()
            .await
            .map_err(|e| RmcpError::internal_error(e.to_string(), None))?;

        Ok(CallToolResult::success(vec![Content::text(
            serde_json::json!({
                "pattern_subscriptions": result
            })
            .to_string(),
        )]))
    }
}

#[tool_handler]
impl ServerHandler for RedisCtlMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "Redis Cloud and Enterprise management tools. \
                Use cloud_* tools for Redis Cloud operations and \
                enterprise_* tools for Redis Enterprise operations. \
                All tools are currently read-only."
                    .to_string(),
            ),
        }
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, RmcpError> {
        info!("MCP client connected, initializing session");
        Ok(self.get_info())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_creation() {
        let server = RedisCtlMcp::new(None, true);
        assert!(server.is_ok());
        let server = server.unwrap();
        assert!(server.config().read_only);
        assert!(server.config().profile.is_none());
    }

    #[test]
    fn test_server_with_profile() {
        let server = RedisCtlMcp::new(Some("test-profile"), false);
        assert!(server.is_ok());
        let server = server.unwrap();
        assert!(!server.config().read_only);
        assert_eq!(server.config().profile, Some("test-profile".to_string()));
    }
}
