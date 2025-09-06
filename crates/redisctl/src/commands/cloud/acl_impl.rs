#![allow(dead_code)]

use crate::cli::OutputFormat;
use crate::connection::ConnectionManager;
use crate::error::Result as CliResult;
use anyhow::Context;
use redis_cloud::acl::AclHandler;

use super::utils::*;

// Redis ACL Rules

pub async fn list_redis_rules(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_cloud_client(profile_name).await?;
    let handler = AclHandler::new(client);

    let rules = handler.get_all_redis_rules().await?;
    let rules_json = serde_json::to_value(rules).context("Failed to serialize Redis rules")?;
    let data = handle_output(rules_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn create_redis_rule(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    name: &str,
    rule: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_cloud_client(profile_name).await?;

    let request_data = serde_json::json!({
        "name": name,
        "rule": rule
    });

    let result = client
        .post_raw("/acl/redis-rules", request_data)
        .await
        .context("Failed to create Redis rule")?;

    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn update_redis_rule(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: i32,
    name: Option<&str>,
    rule: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_cloud_client(profile_name).await?;

    let mut update_data = serde_json::Map::new();
    if let Some(name) = name {
        update_data.insert(
            "name".to_string(),
            serde_json::Value::String(name.to_string()),
        );
    }
    if let Some(rule) = rule {
        update_data.insert(
            "rule".to_string(),
            serde_json::Value::String(rule.to_string()),
        );
    }

    let result = client
        .put_raw(
            &format!("/acl/redis-rules/{}", id),
            serde_json::Value::Object(update_data),
        )
        .await
        .context("Failed to update Redis rule")?;

    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn delete_redis_rule(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: i32,
    force: bool,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    if !force {
        let confirm = confirm_action(&format!("delete Redis rule {}", id))?;
        if !confirm {
            println!("Operation cancelled");
            return Ok(());
        }
    }

    let client = conn_mgr.create_cloud_client(profile_name).await?;
    let handler = AclHandler::new(client.clone());

    handler.delete_redis_rule(id).await?;

    let result = serde_json::json!({
        "message": format!("Redis rule {} deleted successfully", id)
    });
    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

// ACL Roles

pub async fn list_roles(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_cloud_client(profile_name).await?;
    let handler = AclHandler::new(client);

    let roles = handler.get_roles().await?;
    let roles_json = serde_json::to_value(roles).context("Failed to serialize roles")?;
    let data = handle_output(roles_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn create_role(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    name: &str,
    redis_rules: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_cloud_client(profile_name).await?;

    let rules_data = if redis_rules.starts_with('[') {
        serde_json::from_str(redis_rules).context("Failed to parse redis-rules as JSON array")?
    } else {
        // If it's a single rule ID, wrap it in an array
        serde_json::json!([{"rule_id": redis_rules.parse::<i32>().context("Invalid rule ID")?}])
    };

    let request_data = serde_json::json!({
        "name": name,
        "redis_rules": rules_data
    });

    let result = client
        .post_raw("/acl/roles", request_data)
        .await
        .context("Failed to create role")?;

    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn update_role(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: i32,
    name: Option<&str>,
    redis_rules: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_cloud_client(profile_name).await?;

    let mut update_data = serde_json::Map::new();
    if let Some(name) = name {
        update_data.insert(
            "name".to_string(),
            serde_json::Value::String(name.to_string()),
        );
    }
    if let Some(rules) = redis_rules {
        let rules_data = if rules.starts_with('[') {
            serde_json::from_str(rules).context("Failed to parse redis-rules as JSON array")?
        } else {
            serde_json::json!([{"rule_id": rules.parse::<i32>().context("Invalid rule ID")?}])
        };
        update_data.insert("redis_rules".to_string(), rules_data);
    }

    let result = client
        .put_raw(
            &format!("/acl/roles/{}", id),
            serde_json::Value::Object(update_data),
        )
        .await
        .context("Failed to update role")?;

    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn delete_role(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: i32,
    force: bool,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    if !force {
        let confirm = confirm_action(&format!("delete ACL role {}", id))?;
        if !confirm {
            println!("Operation cancelled");
            return Ok(());
        }
    }

    let client = conn_mgr.create_cloud_client(profile_name).await?;
    let handler = AclHandler::new(client.clone());

    handler.delete_acl_role(id).await?;

    let result = serde_json::json!({
        "message": format!("ACL role {} deleted successfully", id)
    });
    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

// ACL Users

pub async fn list_acl_users(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_cloud_client(profile_name).await?;
    let handler = AclHandler::new(client);

    let users = handler.get_all_acl_users().await?;
    let users_json = serde_json::to_value(users).context("Failed to serialize ACL users")?;
    let data = handle_output(users_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn get_acl_user(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: i32,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_cloud_client(profile_name).await?;
    let handler = AclHandler::new(client);

    let user = handler.get_user_by_id(id).await?;
    let user_json = serde_json::to_value(user).context("Failed to serialize ACL user")?;
    let data = handle_output(user_json, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn create_acl_user(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    name: &str,
    role: &str,
    password: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_cloud_client(profile_name).await?;

    let request_data = serde_json::json!({
        "name": name,
        "role": role,
        "password": password
    });

    let result = client
        .post_raw("/acl/users", request_data)
        .await
        .context("Failed to create ACL user")?;

    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub async fn update_acl_user(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: i32,
    name: Option<&str>,
    role: Option<&str>,
    password: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_cloud_client(profile_name).await?;

    let mut update_data = serde_json::Map::new();
    if let Some(name) = name {
        update_data.insert(
            "name".to_string(),
            serde_json::Value::String(name.to_string()),
        );
    }
    if let Some(role) = role {
        update_data.insert(
            "role".to_string(),
            serde_json::Value::String(role.to_string()),
        );
    }
    if let Some(password) = password {
        update_data.insert(
            "password".to_string(),
            serde_json::Value::String(password.to_string()),
        );
    }

    let result = client
        .put_raw(
            &format!("/acl/users/{}", id),
            serde_json::Value::Object(update_data),
        )
        .await
        .context("Failed to update ACL user")?;

    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}

pub async fn delete_acl_user(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    id: i32,
    force: bool,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    if !force {
        let confirm = confirm_action(&format!("delete ACL user {}", id))?;
        if !confirm {
            println!("Operation cancelled");
            return Ok(());
        }
    }

    let client = conn_mgr.create_cloud_client(profile_name).await?;
    let handler = AclHandler::new(client.clone());

    handler.delete_user(id).await?;

    let result = serde_json::json!({
        "message": format!("ACL user {} deleted successfully", id)
    });
    let data = handle_output(result, output_format, query)?;
    print_formatted_output(data, output_format)?;
    Ok(())
}
