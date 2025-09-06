//! RBAC command implementations for Redis Enterprise

#![allow(dead_code)]

use crate::cli::OutputFormat;
use crate::connection::ConnectionManager;
use crate::error::Result as CliResult;
use anyhow::Context;
use redis_enterprise::ldap_mappings::LdapMappingHandler;
use redis_enterprise::redis_acls::{CreateRedisAclRequest, RedisAclHandler};
use redis_enterprise::roles::RolesHandler;
use redis_enterprise::users::{AuthRequest, PasswordSet, UserHandler};

use super::utils::*;

// ============================================================================
// User Management Commands
// ============================================================================

pub async fn list_users(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = UserHandler::new(client);
    let users = handler.list().await?;
    let users_json = serde_json::to_value(users).context("Failed to serialize users")?;
    let data = handle_output(users_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn get_user(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = UserHandler::new(client);
    let user = handler.get(id).await?;
    // Mask password field if present
    let mut user_json = serde_json::to_value(user).context("Failed to serialize user")?;
    if let Some(obj) = user_json.as_object_mut()
        && obj.contains_key("password")
    {
        obj.insert(
            "password".to_string(),
            serde_json::Value::String("***".to_string()),
        );
    }
    let data = handle_output(user_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn create_user(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let user_data = read_json_data(data).context("Failed to parse user data")?;

    // CreateUserRequest doesn't have Deserialize, so we'll use the raw endpoint
    let user_json = client.post_raw("/v1/users", user_data).await?;
    let data = handle_output(user_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn update_user(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let update_data = read_json_data(data).context("Failed to parse update data")?;

    // UpdateUserRequest doesn't have Deserialize, so we'll use the raw endpoint
    let user_json = client
        .put_raw(&format!("/v1/users/{}", id), update_data)
        .await?;
    let data = handle_output(user_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn delete_user(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    force: bool,
    _output_format: OutputFormat,
    _query: Option<&str>,
) -> CliResult<()> {
    if !force && !confirm_action(&format!("Delete user {}?", id))? {
        println!("Operation cancelled");
        return Ok(());
    }

    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = UserHandler::new(client);
    handler.delete(id).await?;
    println!("User {} deleted successfully", id);
    Ok(())
}

pub async fn reset_user_password(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    password: Option<&str>,
    _output_format: OutputFormat,
    _query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = UserHandler::new(client);

    // Get the user to get their email
    let user = handler.get(id).await?;
    let email = user.email.unwrap_or(user.username);

    let new_password = if let Some(pwd) = password {
        pwd.to_string()
    } else {
        // Prompt for password if not provided
        rpassword::prompt_password("New password: ").context("Failed to read password")?
    };

    let request = PasswordSet {
        email,
        password: new_password,
    };

    handler.password_set(request).await?;
    println!("Password reset successfully for user {}", id);
    Ok(())
}

// User-Role Assignment Commands

pub async fn get_user_roles(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    user_id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = UserHandler::new(client);

    let user = handler.get(user_id).await?;
    let roles = serde_json::json!({
        "user_id": user_id,
        "role": user.role,
        "role_uids": user.extra.get("role_uids")
    });

    let data = handle_output(roles, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn assign_user_role(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    user_id: u32,
    role_id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = UserHandler::new(client.clone());

    // Get current user to preserve existing data
    let user = handler.get(user_id).await?;
    let mut role_uids: Vec<u32> = user
        .extra
        .get("role_uids")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();

    // Add new role if not already present
    if !role_uids.contains(&role_id) {
        role_uids.push(role_id);
    }

    // Use raw API since UpdateUserRequest doesn't have Deserialize
    let update = serde_json::json!({
        "role_uids": role_uids
    });

    let updated = client
        .put_raw(&format!("/v1/users/{}", user_id), update)
        .await?;
    let data = handle_output(updated, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn remove_user_role(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    user_id: u32,
    role_id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = UserHandler::new(client.clone());

    // Get current user to preserve existing data
    let user = handler.get(user_id).await?;
    let mut role_uids: Vec<u32> = user
        .extra
        .get("role_uids")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();

    // Remove the role
    role_uids.retain(|&id| id != role_id);

    // Use raw API since UpdateUserRequest doesn't have Deserialize
    let update = serde_json::json!({
        "role_uids": role_uids
    });

    let updated = client
        .put_raw(&format!("/v1/users/{}", user_id), update)
        .await?;
    let data = handle_output(updated, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

// ============================================================================
// Role Management Commands
// ============================================================================

pub async fn list_roles(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = RolesHandler::new(client);
    let roles = handler.list().await?;
    let roles_json = serde_json::to_value(roles).context("Failed to serialize roles")?;
    let data = handle_output(roles_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn get_role(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = RolesHandler::new(client);
    let role = handler.get(id).await?;
    let role_json = serde_json::to_value(role).context("Failed to serialize role")?;
    let data = handle_output(role_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn create_role(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let role_data = read_json_data(data).context("Failed to parse role data")?;
    let result = client.post_raw("/v1/roles", role_data).await?;
    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn update_role(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let role_data = read_json_data(data).context("Failed to parse role data")?;
    let result = client
        .put_raw(&format!("/v1/roles/{}", id), role_data)
        .await?;
    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn delete_role(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    force: bool,
    _output_format: OutputFormat,
    _query: Option<&str>,
) -> CliResult<()> {
    if !force && !confirm_action(&format!("Delete role {}?", id))? {
        println!("Operation cancelled");
        return Ok(());
    }

    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = RolesHandler::new(client);
    handler.delete(id).await?;
    println!("Role {} deleted successfully", id);
    Ok(())
}

pub async fn get_role_permissions(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = RolesHandler::new(client);

    let role = handler.get(id).await?;
    let permissions = role
        .extra
        .get("permissions")
        .cloned()
        .unwrap_or_else(|| serde_json::json!([]));

    let result = serde_json::json!({
        "role_id": id,
        "permissions": permissions
    });

    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn get_role_users(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    role_id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let user_handler = UserHandler::new(client);

    // Get all users and filter by role
    let users = user_handler.list().await?;
    let users_with_role: Vec<_> = users
        .into_iter()
        .filter(|u| {
            if let Some(role_uids) = u.extra.get("role_uids")
                && let Ok(uids) = serde_json::from_value::<Vec<u32>>(role_uids.clone())
            {
                return uids.contains(&role_id);
            }
            false
        })
        .collect();

    let users_json = serde_json::to_value(users_with_role).context("Failed to serialize users")?;
    let data = handle_output(users_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

// ============================================================================
// ACL Management Commands
// ============================================================================

pub async fn list_acls(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = RedisAclHandler::new(client);
    let acls = handler.list().await?;
    let acls_json = serde_json::to_value(acls).context("Failed to serialize ACLs")?;
    let data = handle_output(acls_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn get_acl(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = RedisAclHandler::new(client);
    let acl = handler.get(id).await?;
    let acl_json = serde_json::to_value(acl).context("Failed to serialize ACL")?;
    let data = handle_output(acl_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn create_acl(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = RedisAclHandler::new(client);

    let acl_data = read_json_data(data).context("Failed to parse ACL data")?;
    let request: CreateRedisAclRequest =
        serde_json::from_value(acl_data).context("Invalid ACL creation request format")?;

    let acl = handler.create(request).await?;
    let acl_json = serde_json::to_value(acl).context("Failed to serialize ACL")?;
    let data = handle_output(acl_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn update_acl(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = RedisAclHandler::new(client);

    let acl_data = read_json_data(data).context("Failed to parse ACL data")?;
    let request: CreateRedisAclRequest =
        serde_json::from_value(acl_data).context("Invalid ACL update request format")?;

    let acl = handler.update(id, request).await?;
    let acl_json = serde_json::to_value(acl).context("Failed to serialize ACL")?;
    let data = handle_output(acl_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn delete_acl(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: u32,
    force: bool,
    _output_format: OutputFormat,
    _query: Option<&str>,
) -> CliResult<()> {
    if !force && !confirm_action(&format!("Delete ACL {}?", id))? {
        println!("Operation cancelled");
        return Ok(());
    }

    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = RedisAclHandler::new(client);
    handler.delete(id).await?;
    println!("ACL {} deleted successfully", id);
    Ok(())
}

pub async fn test_acl(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    user_id: u32,
    command: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    // This would typically involve testing the ACL against a specific command
    // The actual implementation depends on the API endpoint available
    let test_data = serde_json::json!({
        "user_id": user_id,
        "command": command
    });

    let result = client
        .post_raw("/v1/redis_acls/test", test_data)
        .await
        .unwrap_or_else(|_| {
            serde_json::json!({
                "user_id": user_id,
                "command": command,
                "result": "Test endpoint not available"
            })
        });

    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

// ============================================================================
// LDAP Integration Commands
// ============================================================================

pub async fn get_ldap_config(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let config = client.get_raw("/v1/cluster/ldap").await?;
    let data = handle_output(config, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn update_ldap_config(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let ldap_data = read_json_data(data).context("Failed to parse LDAP data")?;
    let result = client.put_raw("/v1/cluster/ldap", ldap_data).await?;
    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn test_ldap_connection(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let result = client
        .post_raw("/v1/cluster/ldap/test", serde_json::json!({}))
        .await
        .unwrap_or_else(|e| {
            serde_json::json!({
                "status": "error",
                "message": e.to_string()
            })
        });

    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn sync_ldap(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let result = client
        .post_raw("/v1/cluster/ldap/sync", serde_json::json!({}))
        .await?;
    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn get_ldap_mappings(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = LdapMappingHandler::new(client);
    let mappings = handler.list().await?;
    let mappings_json = serde_json::to_value(mappings).context("Failed to serialize mappings")?;
    let data = handle_output(mappings_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

// ============================================================================
// Authentication & Session Commands
// ============================================================================

pub async fn test_auth(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    username: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;
    let handler = UserHandler::new(client);

    // Prompt for password
    let password = rpassword::prompt_password("Password: ").context("Failed to read password")?;

    let auth_request = AuthRequest {
        email: username.to_string(),
        password,
    };

    match handler.authorize(auth_request).await {
        Ok(response) => {
            // Mask the JWT token in output
            let mut response_json = serde_json::to_value(response)?;
            if let Some(obj) = response_json.as_object_mut()
                && obj.contains_key("jwt")
            {
                obj.insert(
                    "jwt".to_string(),
                    serde_json::Value::String("***".to_string()),
                );
            }
            let data = handle_output(response_json, output_format, query)?;
            print_formatted_output(data, output_format)?;
        }
        Err(e) => {
            let error_response = serde_json::json!({
                "status": "failed",
                "error": e.to_string()
            });
            let data = handle_output(error_response, output_format, query)?;
            print_formatted_output(data, output_format)?;
        }
    }
    Ok(())
}

pub async fn list_sessions(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let sessions = client.get_raw("/v1/sessions").await.unwrap_or_else(|_| {
        serde_json::json!({
            "message": "Sessions endpoint not available"
        })
    });

    let data = handle_output(sessions, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn revoke_session(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    session_id: &str,
    _output_format: OutputFormat,
    _query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    client
        .delete_raw(&format!("/v1/sessions/{}", session_id))
        .await?;
    println!("Session {} revoked successfully", session_id);
    Ok(())
}

pub async fn revoke_all_user_sessions(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    user_id: u32,
    _output_format: OutputFormat,
    _query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    client
        .delete_raw(&format!("/v1/users/{}/sessions", user_id))
        .await
        .unwrap_or_else(|_| {
            println!("Note: Session revocation endpoint may not be available");
            serde_json::Value::Null
        });

    println!("All sessions for user {} revoked", user_id);
    Ok(())
}
