//! Enterprise tools implementation
//!
//! Wraps Redis Enterprise API client operations for MCP tool invocation.

use redis_enterprise::{
    AlertHandler, BdbHandler, ClusterHandler, CrdbHandler, CreateDatabaseRequest,
    CreateLdapMappingRequest, CreateRedisAclRequest, CreateRoleRequest, CreateUserRequest,
    DebugInfoHandler, DiagnosticRequest, DiagnosticsHandler, EndpointsHandler, EnterpriseClient,
    JobSchedulerHandler, LdapMappingHandler, LicenseHandler, LogsHandler, ModuleHandler,
    NodeHandler, ProxyHandler, RedisAclHandler, RolesHandler, ShardHandler, StatsHandler,
    UserHandler,
};
use redisctl_config::Config;
use rmcp::{ErrorData as RmcpError, model::*};
use tracing::debug;

/// Enterprise tools wrapper
#[derive(Clone)]
pub struct EnterpriseTools {
    client: EnterpriseClient,
}

impl EnterpriseTools {
    /// Create new Enterprise tools instance
    pub fn new(profile: Option<&str>) -> anyhow::Result<Self> {
        let config = Config::load()?;

        // Resolve profile name: explicit > default > error
        let profile_name = match profile {
            Some(name) => name.to_string(),
            None => config.resolve_enterprise_profile(None)?,
        };

        debug!(profile = %profile_name, "Loading Enterprise client from profile");

        let profile_config = config
            .profiles
            .get(&profile_name)
            .ok_or_else(|| anyhow::anyhow!("Enterprise profile '{}' not found", profile_name))?;

        let (url, username, password, insecure) =
            profile_config.enterprise_credentials().ok_or_else(|| {
                anyhow::anyhow!("Profile '{}' is not an Enterprise profile", profile_name)
            })?;

        let mut builder = EnterpriseClient::builder()
            .base_url(url)
            .username(username)
            .insecure(insecure);

        if let Some(pwd) = password {
            builder = builder.password(pwd);
        }

        let client = builder.build()?;

        Ok(Self { client })
    }

    fn to_result(&self, value: serde_json::Value) -> Result<CallToolResult, RmcpError> {
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string()),
        )]))
    }

    fn to_error(&self, err: impl std::fmt::Display) -> RmcpError {
        RmcpError::internal_error(err.to_string(), None)
    }

    /// Get cluster information
    pub async fn get_cluster(&self) -> Result<CallToolResult, RmcpError> {
        let handler = ClusterHandler::new(self.client.clone());
        let cluster = handler.info().await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(cluster).map_err(|e| self.to_error(e))?)
    }

    /// List all nodes
    pub async fn list_nodes(&self) -> Result<CallToolResult, RmcpError> {
        let handler = NodeHandler::new(self.client.clone());
        let nodes = handler.list().await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(nodes).map_err(|e| self.to_error(e))?)
    }

    /// Get a specific node
    pub async fn get_node(&self, node_id: i64) -> Result<CallToolResult, RmcpError> {
        let handler = NodeHandler::new(self.client.clone());
        let node = handler
            .get(node_id as u32)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(node).map_err(|e| self.to_error(e))?)
    }

    /// List all databases
    pub async fn list_databases(&self) -> Result<CallToolResult, RmcpError> {
        let handler = BdbHandler::new(self.client.clone());
        let dbs = handler.list().await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(dbs).map_err(|e| self.to_error(e))?)
    }

    /// Get a specific database
    pub async fn get_database(&self, database_id: i64) -> Result<CallToolResult, RmcpError> {
        let handler = BdbHandler::new(self.client.clone());
        let db = handler
            .get(database_id as u32)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(db).map_err(|e| self.to_error(e))?)
    }

    /// Get database statistics
    pub async fn get_database_stats(&self, database_id: i64) -> Result<CallToolResult, RmcpError> {
        let handler = StatsHandler::new(self.client.clone());
        let stats = handler
            .database(database_id as u32, None)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(stats).map_err(|e| self.to_error(e))?)
    }

    /// List all shards
    pub async fn list_shards(&self) -> Result<CallToolResult, RmcpError> {
        let handler = ShardHandler::new(self.client.clone());
        let shards = handler.list().await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(shards).map_err(|e| self.to_error(e))?)
    }

    /// List active alerts
    pub async fn list_alerts(&self) -> Result<CallToolResult, RmcpError> {
        let handler = AlertHandler::new(self.client.clone());
        let alerts = handler.list().await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(alerts).map_err(|e| self.to_error(e))?)
    }

    /// Get cluster logs
    pub async fn get_logs(&self) -> Result<CallToolResult, RmcpError> {
        let handler = LogsHandler::new(self.client.clone());
        let logs = handler.list(None).await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(logs).map_err(|e| self.to_error(e))?)
    }

    /// Get license information
    pub async fn get_license(&self) -> Result<CallToolResult, RmcpError> {
        let handler = LicenseHandler::new(self.client.clone());
        let license = handler.get().await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(license).map_err(|e| self.to_error(e))?)
    }

    /// Create a new database
    pub async fn create_database(
        &self,
        name: &str,
        memory_size_mb: Option<u64>,
    ) -> Result<CallToolResult, RmcpError> {
        let handler = BdbHandler::new(self.client.clone());

        let memory_size = memory_size_mb.unwrap_or(100) * 1024 * 1024; // Default 100MB

        let request = CreateDatabaseRequest::builder()
            .name(name)
            .memory_size(memory_size)
            .build();

        let db = handler
            .create(request)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(db).map_err(|e| self.to_error(e))?)
    }

    /// Delete a database
    pub async fn delete_database(&self, database_id: i64) -> Result<CallToolResult, RmcpError> {
        let handler = BdbHandler::new(self.client.clone());
        handler
            .delete(database_id as u32)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::json!({
            "success": true,
            "message": format!("Database {} deleted successfully", database_id)
        }))
    }

    /// Update a database
    pub async fn update_database(
        &self,
        database_id: i64,
        updates: serde_json::Value,
    ) -> Result<CallToolResult, RmcpError> {
        let handler = BdbHandler::new(self.client.clone());
        let db = handler
            .update(database_id as u32, updates)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(db).map_err(|e| self.to_error(e))?)
    }

    /// Flush all data from a database
    pub async fn flush_database(&self, database_id: i64) -> Result<CallToolResult, RmcpError> {
        let handler = BdbHandler::new(self.client.clone());
        let result = handler
            .flush(database_id as u32)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(result).map_err(|e| self.to_error(e))?)
    }

    /// Get database metrics
    pub async fn get_database_metrics(
        &self,
        database_id: i64,
    ) -> Result<CallToolResult, RmcpError> {
        let handler = BdbHandler::new(self.client.clone());
        let metrics = handler
            .metrics(database_id as u32)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(metrics)
    }

    /// Export database to a location
    pub async fn export_database(
        &self,
        database_id: i64,
        export_location: &str,
    ) -> Result<CallToolResult, RmcpError> {
        let handler = BdbHandler::new(self.client.clone());
        let result = handler
            .export(database_id as u32, export_location)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(result).map_err(|e| self.to_error(e))?)
    }

    /// Import data into a database
    pub async fn import_database(
        &self,
        database_id: i64,
        import_location: &str,
        flush_before_import: bool,
    ) -> Result<CallToolResult, RmcpError> {
        let handler = BdbHandler::new(self.client.clone());
        let result = handler
            .import(database_id as u32, import_location, flush_before_import)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(result).map_err(|e| self.to_error(e))?)
    }

    /// Trigger database backup
    pub async fn backup_database(&self, database_id: i64) -> Result<CallToolResult, RmcpError> {
        let handler = BdbHandler::new(self.client.clone());
        let result = handler
            .backup(database_id as u32)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(result).map_err(|e| self.to_error(e))?)
    }

    /// Restore database from backup
    pub async fn restore_database(
        &self,
        database_id: i64,
        backup_uid: Option<&str>,
    ) -> Result<CallToolResult, RmcpError> {
        let handler = BdbHandler::new(self.client.clone());
        let result = handler
            .restore(database_id as u32, backup_uid)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(result).map_err(|e| self.to_error(e))?)
    }

    /// Get cluster statistics
    pub async fn get_cluster_stats(&self) -> Result<CallToolResult, RmcpError> {
        let handler = ClusterHandler::new(self.client.clone());
        let stats = handler.stats().await.map_err(|e| self.to_error(e))?;
        self.to_result(stats)
    }

    /// Get cluster settings
    pub async fn get_cluster_settings(&self) -> Result<CallToolResult, RmcpError> {
        let handler = ClusterHandler::new(self.client.clone());
        let settings = handler.settings().await.map_err(|e| self.to_error(e))?;
        self.to_result(settings)
    }

    /// Get cluster topology
    pub async fn get_cluster_topology(&self) -> Result<CallToolResult, RmcpError> {
        let handler = ClusterHandler::new(self.client.clone());
        let topology = handler.topology().await.map_err(|e| self.to_error(e))?;
        self.to_result(topology)
    }

    /// Update cluster configuration
    pub async fn update_cluster(
        &self,
        updates: serde_json::Value,
    ) -> Result<CallToolResult, RmcpError> {
        let handler = ClusterHandler::new(self.client.clone());
        let result = handler
            .update(updates)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(result)
    }

    /// Get node statistics
    pub async fn get_node_stats(&self, node_id: i64) -> Result<CallToolResult, RmcpError> {
        let handler = NodeHandler::new(self.client.clone());
        let stats = handler
            .stats(node_id as u32)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(stats).map_err(|e| self.to_error(e))?)
    }

    /// Update node configuration
    pub async fn update_node(
        &self,
        node_id: i64,
        updates: serde_json::Value,
    ) -> Result<CallToolResult, RmcpError> {
        let handler = NodeHandler::new(self.client.clone());
        let node = handler
            .update(node_id as u32, updates)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(node).map_err(|e| self.to_error(e))?)
    }

    /// Remove node from cluster
    pub async fn remove_node(&self, node_id: i64) -> Result<CallToolResult, RmcpError> {
        let handler = NodeHandler::new(self.client.clone());
        handler
            .remove(node_id as u32)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::json!({
            "success": true,
            "message": format!("Node {} removed from cluster", node_id)
        }))
    }

    /// Get a specific shard
    pub async fn get_shard(&self, shard_uid: &str) -> Result<CallToolResult, RmcpError> {
        let handler = ShardHandler::new(self.client.clone());
        let shard = handler.get(shard_uid).await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(shard).map_err(|e| self.to_error(e))?)
    }

    /// Get a specific alert
    pub async fn get_alert(&self, alert_uid: &str) -> Result<CallToolResult, RmcpError> {
        let handler = AlertHandler::new(self.client.clone());
        let alert = handler.get(alert_uid).await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(alert).map_err(|e| self.to_error(e))?)
    }

    // =========================================================================
    // User Operations
    // =========================================================================

    /// List all users
    pub async fn list_users(&self) -> Result<CallToolResult, RmcpError> {
        let handler = UserHandler::new(self.client.clone());
        let users = handler.list().await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(users).map_err(|e| self.to_error(e))?)
    }

    /// Get a specific user
    pub async fn get_user(&self, user_id: i64) -> Result<CallToolResult, RmcpError> {
        let handler = UserHandler::new(self.client.clone());
        let user = handler
            .get(user_id as u32)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(user).map_err(|e| self.to_error(e))?)
    }

    /// Create a new user
    pub async fn create_user(
        &self,
        email: &str,
        password: &str,
        role: &str,
        name: Option<&str>,
    ) -> Result<CallToolResult, RmcpError> {
        let handler = UserHandler::new(self.client.clone());
        // Build request - name is optional
        let request = match name {
            Some(n) => CreateUserRequest::builder()
                .email(email)
                .password(password)
                .role(role)
                .name(n)
                .build(),
            None => CreateUserRequest::builder()
                .email(email)
                .password(password)
                .role(role)
                .build(),
        };
        let user = handler
            .create(request)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(user).map_err(|e| self.to_error(e))?)
    }

    /// Delete a user
    pub async fn delete_user(&self, user_id: i64) -> Result<CallToolResult, RmcpError> {
        let handler = UserHandler::new(self.client.clone());
        handler
            .delete(user_id as u32)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::json!({
            "success": true,
            "message": format!("User {} deleted successfully", user_id)
        }))
    }

    // =========================================================================
    // Role Operations
    // =========================================================================

    /// List all roles
    pub async fn list_roles(&self) -> Result<CallToolResult, RmcpError> {
        let handler = RolesHandler::new(self.client.clone());
        let roles = handler.list().await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(roles).map_err(|e| self.to_error(e))?)
    }

    /// Get a specific role
    pub async fn get_role(&self, role_id: i64) -> Result<CallToolResult, RmcpError> {
        let handler = RolesHandler::new(self.client.clone());
        let role = handler
            .get(role_id as u32)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(role).map_err(|e| self.to_error(e))?)
    }

    /// Create a new role
    pub async fn create_role(
        &self,
        name: &str,
        management: Option<&str>,
    ) -> Result<CallToolResult, RmcpError> {
        let handler = RolesHandler::new(self.client.clone());
        // Build request - management is optional
        let request = match management {
            Some(m) => CreateRoleRequest::builder()
                .name(name)
                .management(m)
                .build(),
            None => CreateRoleRequest::builder().name(name).build(),
        };
        let role = handler
            .create(request)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(role).map_err(|e| self.to_error(e))?)
    }

    /// Delete a role
    pub async fn delete_role(&self, role_id: i64) -> Result<CallToolResult, RmcpError> {
        let handler = RolesHandler::new(self.client.clone());
        handler
            .delete(role_id as u32)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::json!({
            "success": true,
            "message": format!("Role {} deleted successfully", role_id)
        }))
    }

    // =========================================================================
    // Redis ACL Operations
    // =========================================================================

    /// List all Redis ACLs
    pub async fn list_acls(&self) -> Result<CallToolResult, RmcpError> {
        let handler = RedisAclHandler::new(self.client.clone());
        let acls = handler.list().await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(acls).map_err(|e| self.to_error(e))?)
    }

    /// Get a specific Redis ACL
    pub async fn get_acl(&self, acl_id: i64) -> Result<CallToolResult, RmcpError> {
        let handler = RedisAclHandler::new(self.client.clone());
        let acl = handler
            .get(acl_id as u32)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(acl).map_err(|e| self.to_error(e))?)
    }

    /// Create a new Redis ACL
    pub async fn create_acl(
        &self,
        name: &str,
        acl: &str,
        description: Option<&str>,
    ) -> Result<CallToolResult, RmcpError> {
        let handler = RedisAclHandler::new(self.client.clone());
        // Build request - description is optional
        let request = match description {
            Some(d) => CreateRedisAclRequest::builder()
                .name(name)
                .acl(acl)
                .description(d)
                .build(),
            None => CreateRedisAclRequest::builder().name(name).acl(acl).build(),
        };
        let result = handler
            .create(request)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(result).map_err(|e| self.to_error(e))?)
    }

    /// Delete a Redis ACL
    pub async fn delete_acl(&self, acl_id: i64) -> Result<CallToolResult, RmcpError> {
        let handler = RedisAclHandler::new(self.client.clone());
        handler
            .delete(acl_id as u32)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::json!({
            "success": true,
            "message": format!("Redis ACL {} deleted successfully", acl_id)
        }))
    }

    // =========================================================================
    // Module Operations
    // =========================================================================

    /// List all modules
    pub async fn list_modules(&self) -> Result<CallToolResult, RmcpError> {
        let handler = ModuleHandler::new(self.client.clone());
        let modules = handler.list().await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(modules).map_err(|e| self.to_error(e))?)
    }

    /// Get a specific module
    pub async fn get_module(&self, module_uid: &str) -> Result<CallToolResult, RmcpError> {
        let handler = ModuleHandler::new(self.client.clone());
        let module = handler
            .get(module_uid)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(module).map_err(|e| self.to_error(e))?)
    }

    // =========================================================================
    // CRDB (Active-Active) Operations
    // =========================================================================

    /// List all CRDBs
    pub async fn list_crdbs(&self) -> Result<CallToolResult, RmcpError> {
        let handler = CrdbHandler::new(self.client.clone());
        let crdbs = handler.list().await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(crdbs).map_err(|e| self.to_error(e))?)
    }

    /// Get a specific CRDB
    pub async fn get_crdb(&self, crdb_guid: &str) -> Result<CallToolResult, RmcpError> {
        let handler = CrdbHandler::new(self.client.clone());
        let crdb = handler.get(crdb_guid).await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(crdb).map_err(|e| self.to_error(e))?)
    }

    /// Update a CRDB
    pub async fn update_crdb(
        &self,
        crdb_guid: &str,
        updates: serde_json::Value,
    ) -> Result<CallToolResult, RmcpError> {
        let handler = CrdbHandler::new(self.client.clone());
        let crdb = handler
            .update(crdb_guid, updates)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(crdb).map_err(|e| self.to_error(e))?)
    }

    /// Delete a CRDB
    pub async fn delete_crdb(&self, crdb_guid: &str) -> Result<CallToolResult, RmcpError> {
        let handler = CrdbHandler::new(self.client.clone());
        handler
            .delete(crdb_guid)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::json!({
            "success": true,
            "message": format!("CRDB {} deleted successfully", crdb_guid)
        }))
    }

    // =========================================================================
    // Debug Info / Support Operations
    // =========================================================================

    /// List debug info collection tasks
    pub async fn list_debuginfo(&self) -> Result<CallToolResult, RmcpError> {
        let handler = DebugInfoHandler::new(self.client.clone());
        let tasks = handler.list().await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(tasks).map_err(|e| self.to_error(e))?)
    }

    /// Get debug info task status
    pub async fn get_debuginfo_status(&self, task_id: &str) -> Result<CallToolResult, RmcpError> {
        let handler = DebugInfoHandler::new(self.client.clone());
        let status = handler
            .status(task_id)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(status).map_err(|e| self.to_error(e))?)
    }

    // =========================================================================
    // LDAP Mapping Operations
    // =========================================================================

    /// List all LDAP mappings
    pub async fn list_ldap_mappings(&self) -> Result<CallToolResult, RmcpError> {
        let handler = LdapMappingHandler::new(self.client.clone());
        let mappings = handler.list().await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(mappings).map_err(|e| self.to_error(e))?)
    }

    /// Get a specific LDAP mapping
    pub async fn get_ldap_mapping(&self, uid: u64) -> Result<CallToolResult, RmcpError> {
        let handler = LdapMappingHandler::new(self.client.clone());
        let mapping = handler
            .get(uid as u32)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(mapping).map_err(|e| self.to_error(e))?)
    }

    /// Create a new LDAP mapping
    pub async fn create_ldap_mapping(
        &self,
        name: &str,
        dn: &str,
        role: &str,
        email: Option<&str>,
    ) -> Result<CallToolResult, RmcpError> {
        let handler = LdapMappingHandler::new(self.client.clone());
        let request = match email {
            Some(e) => CreateLdapMappingRequest::builder()
                .name(name)
                .dn(dn)
                .role(role)
                .email(e)
                .build(),
            None => CreateLdapMappingRequest::builder()
                .name(name)
                .dn(dn)
                .role(role)
                .build(),
        };
        let mapping = handler
            .create(request)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(mapping).map_err(|e| self.to_error(e))?)
    }

    /// Delete an LDAP mapping
    pub async fn delete_ldap_mapping(&self, uid: u64) -> Result<CallToolResult, RmcpError> {
        let handler = LdapMappingHandler::new(self.client.clone());
        handler
            .delete(uid as u32)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::json!({
            "success": true,
            "message": format!("LDAP mapping {} deleted successfully", uid)
        }))
    }

    // =========================================================================
    // Job Scheduler Operations
    // =========================================================================

    /// List all scheduled jobs
    pub async fn list_jobs(&self) -> Result<CallToolResult, RmcpError> {
        let handler = JobSchedulerHandler::new(self.client.clone());
        let jobs = handler.list().await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(jobs).map_err(|e| self.to_error(e))?)
    }

    /// Get a specific scheduled job
    pub async fn get_job(&self, job_id: &str) -> Result<CallToolResult, RmcpError> {
        let handler = JobSchedulerHandler::new(self.client.clone());
        let job = handler.get(job_id).await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(job).map_err(|e| self.to_error(e))?)
    }

    /// Get job execution history
    pub async fn get_job_history(&self, job_id: &str) -> Result<CallToolResult, RmcpError> {
        let handler = JobSchedulerHandler::new(self.client.clone());
        let history = handler
            .history(job_id)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(history).map_err(|e| self.to_error(e))?)
    }

    /// Trigger a job execution
    pub async fn trigger_job(&self, job_id: &str) -> Result<CallToolResult, RmcpError> {
        let handler = JobSchedulerHandler::new(self.client.clone());
        let execution = handler
            .trigger(job_id)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(execution).map_err(|e| self.to_error(e))?)
    }

    // =========================================================================
    // Proxy Operations
    // =========================================================================

    /// List all proxies
    pub async fn list_proxies(&self) -> Result<CallToolResult, RmcpError> {
        let handler = ProxyHandler::new(self.client.clone());
        let proxies = handler.list().await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(proxies).map_err(|e| self.to_error(e))?)
    }

    /// Get a specific proxy
    pub async fn get_proxy(&self, uid: u64) -> Result<CallToolResult, RmcpError> {
        let handler = ProxyHandler::new(self.client.clone());
        let proxy = handler
            .get(uid as u32)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(proxy).map_err(|e| self.to_error(e))?)
    }

    /// Get proxy statistics
    pub async fn get_proxy_stats(&self, uid: u64) -> Result<CallToolResult, RmcpError> {
        let handler = ProxyHandler::new(self.client.clone());
        let stats = handler
            .stats(uid as u32)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(stats).map_err(|e| self.to_error(e))?)
    }

    /// Get proxies for a specific database
    pub async fn list_proxies_by_database(
        &self,
        database_id: u64,
    ) -> Result<CallToolResult, RmcpError> {
        let handler = ProxyHandler::new(self.client.clone());
        let proxies = handler
            .list_by_database(database_id as u32)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(proxies).map_err(|e| self.to_error(e))?)
    }

    // =========================================================================
    // Endpoint Operations
    // =========================================================================

    /// List all endpoints
    pub async fn list_endpoints(&self) -> Result<CallToolResult, RmcpError> {
        let handler = EndpointsHandler::new(self.client.clone());
        let endpoints = handler.list().await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(endpoints).map_err(|e| self.to_error(e))?)
    }

    /// Get a specific endpoint
    pub async fn get_endpoint(&self, uid: &str) -> Result<CallToolResult, RmcpError> {
        let handler = EndpointsHandler::new(self.client.clone());
        let endpoint = handler.get(uid).await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(endpoint).map_err(|e| self.to_error(e))?)
    }

    /// Get endpoint statistics
    pub async fn get_endpoint_stats(&self, uid: &str) -> Result<CallToolResult, RmcpError> {
        let handler = EndpointsHandler::new(self.client.clone());
        let stats = handler.stats(uid).await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(stats).map_err(|e| self.to_error(e))?)
    }

    /// Get endpoints for a specific database
    pub async fn list_endpoints_by_database(
        &self,
        database_id: u64,
    ) -> Result<CallToolResult, RmcpError> {
        let handler = EndpointsHandler::new(self.client.clone());
        let endpoints = handler
            .list_by_database(database_id as u32)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(endpoints).map_err(|e| self.to_error(e))?)
    }

    // =========================================================================
    // Diagnostics Operations
    // =========================================================================

    /// List diagnostic checks
    pub async fn list_diagnostic_checks(&self) -> Result<CallToolResult, RmcpError> {
        let handler = DiagnosticsHandler::new(self.client.clone());
        let checks = handler.list_checks().await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(checks).map_err(|e| self.to_error(e))?)
    }

    /// List diagnostic reports
    pub async fn list_diagnostic_reports(&self) -> Result<CallToolResult, RmcpError> {
        let handler = DiagnosticsHandler::new(self.client.clone());
        let reports = handler.list_reports().await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(reports).map_err(|e| self.to_error(e))?)
    }

    /// Get a specific diagnostic report
    pub async fn get_diagnostic_report(
        &self,
        report_id: &str,
    ) -> Result<CallToolResult, RmcpError> {
        let handler = DiagnosticsHandler::new(self.client.clone());
        let report = handler
            .get_report(report_id)
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(report).map_err(|e| self.to_error(e))?)
    }

    /// Get the last diagnostic report
    pub async fn get_last_diagnostic_report(&self) -> Result<CallToolResult, RmcpError> {
        let handler = DiagnosticsHandler::new(self.client.clone());
        let report = handler
            .get_last_report()
            .await
            .map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(report).map_err(|e| self.to_error(e))?)
    }

    /// Run diagnostics
    pub async fn run_diagnostics(&self) -> Result<CallToolResult, RmcpError> {
        let handler = DiagnosticsHandler::new(self.client.clone());
        // Run all diagnostics by default (empty request)
        let request = DiagnosticRequest::builder().build();
        let report = handler.run(request).await.map_err(|e| self.to_error(e))?;
        self.to_result(serde_json::to_value(report).map_err(|e| self.to_error(e))?)
    }
}
