#![allow(dead_code)]

use crate::cli::{CloudAclCommands, OutputFormat};
use crate::connection::ConnectionManager;
use crate::error::Result as CliResult;

use super::acl_impl;

pub async fn handle_acl_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    command: &CloudAclCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    match command {
        // Redis ACL Rules
        CloudAclCommands::ListRedisRules => {
            acl_impl::list_redis_rules(conn_mgr, profile_name, output_format, query).await
        }
        CloudAclCommands::CreateRedisRule { name, rule } => {
            acl_impl::create_redis_rule(conn_mgr, profile_name, name, rule, output_format, query)
                .await
        }
        CloudAclCommands::UpdateRedisRule { id, name, rule } => {
            acl_impl::update_redis_rule(
                conn_mgr,
                profile_name,
                *id,
                name.as_deref(),
                rule.as_deref(),
                output_format,
                query,
            )
            .await
        }
        CloudAclCommands::DeleteRedisRule { id, force } => {
            acl_impl::delete_redis_rule(conn_mgr, profile_name, *id, *force, output_format, query)
                .await
        }

        // ACL Roles
        CloudAclCommands::ListRoles => {
            acl_impl::list_roles(conn_mgr, profile_name, output_format, query).await
        }
        CloudAclCommands::CreateRole { name, redis_rules } => {
            acl_impl::create_role(
                conn_mgr,
                profile_name,
                name,
                redis_rules,
                output_format,
                query,
            )
            .await
        }
        CloudAclCommands::UpdateRole {
            id,
            name,
            redis_rules,
        } => {
            acl_impl::update_role(
                conn_mgr,
                profile_name,
                *id,
                name.as_deref(),
                redis_rules.as_deref(),
                output_format,
                query,
            )
            .await
        }
        CloudAclCommands::DeleteRole { id, force } => {
            acl_impl::delete_role(conn_mgr, profile_name, *id, *force, output_format, query).await
        }

        // ACL Users
        CloudAclCommands::ListAclUsers => {
            acl_impl::list_acl_users(conn_mgr, profile_name, output_format, query).await
        }
        CloudAclCommands::GetAclUser { id } => {
            acl_impl::get_acl_user(conn_mgr, profile_name, *id, output_format, query).await
        }
        CloudAclCommands::CreateAclUser {
            name,
            role,
            password,
        } => {
            acl_impl::create_acl_user(
                conn_mgr,
                profile_name,
                name,
                role,
                password,
                output_format,
                query,
            )
            .await
        }
        CloudAclCommands::UpdateAclUser {
            id,
            name,
            role,
            password,
        } => {
            acl_impl::update_acl_user(
                conn_mgr,
                profile_name,
                *id,
                name.as_deref(),
                role.as_deref(),
                password.as_deref(),
                output_format,
                query,
            )
            .await
        }
        CloudAclCommands::DeleteAclUser { id, force } => {
            acl_impl::delete_acl_user(conn_mgr, profile_name, *id, *force, output_format, query)
                .await
        }
    }
}
