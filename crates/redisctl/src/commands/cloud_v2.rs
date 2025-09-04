//! Cloud commands with tabled output styles for comparison

#![allow(dead_code)]

use crate::cli::{CloudUserCommands, OutputFormat};
use crate::connection::ConnectionManager;
use crate::error::Result as CliResult;
use crate::output::print_output;
use anyhow::Context;
use chrono::{DateTime, Utc};
use colored::Colorize;
use serde_json::Value;
use tabled::{Table, Tabled, settings::Style};

/// User row for tabled
#[derive(Tabled)]
struct UserRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "NAME")]
    name: String,
    #[tabled(rename = "EMAIL")]
    email: String,
    #[tabled(rename = "ROLE")]
    role: String,
    #[tabled(rename = "STATUS")]
    status: String,
    #[tabled(rename = "MFA")]
    mfa: String,
    #[tabled(rename = "LAST LOGIN")]
    last_login: String,
    #[tabled(rename = "CREATED")]
    created: String,
}

/// Table style options for comparison
pub enum TableStyle {
    Blank,    // No borders, just spaces (like gh)
    Markdown, // GitHub markdown style
    Psql,     // PostgreSQL style (minimal)
    Ascii,    // Simple ASCII borders
    Modern,   // Clean modern look
    Sharp,    // Sharp corners
}

/// Handle cloud user commands v2
pub async fn handle_user_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    command: &CloudUserCommands,
    output_format: OutputFormat,
    query: Option<&str>,
    table_style: Option<TableStyle>,
) -> CliResult<()> {
    match command {
        CloudUserCommands::List => {
            list_users(conn_mgr, profile_name, output_format, query, table_style).await
        }
        CloudUserCommands::Get { .. } => {
            // Not implemented in v2 test module
            Ok(())
        }
    }
}

/// List users with tabled
async fn list_users(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
    table_style: Option<TableStyle>,
) -> CliResult<()> {
    let client = conn_mgr.create_cloud_client(profile_name).await?;

    // Get raw user data
    let response = client
        .get_raw("/users")
        .await
        .context("Failed to fetch users")?;

    // Apply JMESPath query if provided
    let data = if let Some(q) = query {
        apply_jmespath(&response, q)?
    } else {
        response
    };

    // Format output based on requested format
    match output_format {
        OutputFormat::Auto | OutputFormat::Table => {
            print_users_table(&data, table_style.unwrap_or(TableStyle::Blank))?;
        }
        OutputFormat::Json => {
            print_output(data, crate::output::OutputFormat::Json, None)?;
        }
        OutputFormat::Yaml => {
            print_output(data, crate::output::OutputFormat::Yaml, None)?;
        }
    }

    Ok(())
}

/// Print users table with selected style
fn print_users_table(data: &Value, style: TableStyle) -> CliResult<()> {
    // Extract users from response
    let users = if let Some(users_array) = data.get("users").and_then(|u| u.as_array()) {
        users_array.clone()
    } else if let Value::Array(arr) = data {
        arr.clone()
    } else if data.is_object() {
        vec![data.clone()]
    } else {
        println!("No users found");
        return Ok(());
    };

    if users.is_empty() {
        println!("No users found");
        return Ok(());
    }

    // Build user rows
    let mut rows = Vec::new();
    for user in users {
        rows.push(UserRow {
            id: extract_field(&user, "id", "—"),
            name: extract_user_name(&user),
            email: extract_field(&user, "email", "—"),
            role: format_role(&user),
            status: format_status(&user),
            mfa: format_mfa(&user),
            last_login: format_last_login(&user),
            created: format_date(extract_field(&user, "signUp", "")),
        });
    }

    // Create table and apply style
    let mut table = Table::new(&rows);

    match style {
        TableStyle::Blank => {
            println!("\n=== Style: Blank (GitHub CLI style) ===\n");
            table.with(Style::blank());
        }
        TableStyle::Markdown => {
            println!("\n=== Style: Markdown ===\n");
            table.with(Style::markdown());
        }
        TableStyle::Psql => {
            println!("\n=== Style: PostgreSQL ===\n");
            table.with(Style::psql());
        }
        TableStyle::Ascii => {
            println!("\n=== Style: ASCII ===\n");
            table.with(Style::ascii());
        }
        TableStyle::Modern => {
            println!("\n=== Style: Modern ===\n");
            table.with(Style::modern());
        }
        TableStyle::Sharp => {
            println!("\n=== Style: Sharp ===\n");
            table.with(Style::sharp());
        }
    }

    println!("{}", table);
    Ok(())
}

// Helper functions (same as before but returning String for Display)

fn extract_field(value: &Value, field: &str, default: &str) -> String {
    value
        .get(field)
        .and_then(|v| match v {
            Value::String(s) => Some(s.clone()),
            Value::Number(n) => Some(n.to_string()),
            Value::Bool(b) => Some(b.to_string()),
            _ => None,
        })
        .unwrap_or_else(|| default.to_string())
}

fn extract_user_name(user: &Value) -> String {
    let first = extract_field(user, "firstName", "");
    let last = extract_field(user, "lastName", "");

    if !first.is_empty() || !last.is_empty() {
        format!("{} {}", first, last).trim().to_string()
    } else {
        extract_field(user, "name", "—")
    }
}

fn format_role(user: &Value) -> String {
    if let Some(role) = user.get("role").and_then(|r| r.as_str()) {
        match role.to_lowercase().as_str() {
            "owner" => role.to_uppercase().blue().to_string(),
            "admin" => role.to_string().yellow().to_string(),
            _ => role.to_string(),
        }
    } else {
        "Member".to_string()
    }
}

fn format_status(user: &Value) -> String {
    let status = extract_field(user, "status", "active");
    match status.to_lowercase().as_str() {
        "active" => status.green().to_string(),
        "inactive" | "disabled" => status.red().to_string(),
        "pending" | "invited" => status.yellow().to_string(),
        _ => status,
    }
}

fn format_mfa(user: &Value) -> String {
    if let Some(options) = user.get("options") {
        if let Some(mfa) = options.get("mfaEnabled").and_then(|m| m.as_bool()) {
            if mfa {
                "✓".green().to_string()
            } else {
                "✗".red().to_string()
            }
        } else {
            "—".to_string()
        }
    } else {
        "—".to_string()
    }
}

fn format_last_login(user: &Value) -> String {
    let login_field = extract_field(user, "lastLoginTimestamp", "");
    if login_field.is_empty() {
        return "Never".dimmed().to_string();
    }
    format_date(login_field)
}

fn format_date(date_str: String) -> String {
    if date_str.is_empty() || date_str == "—" {
        return "—".to_string();
    }

    // If it's already a nicely formatted date, keep it
    if !date_str.contains('T') && !date_str.contains('Z') {
        return date_str;
    }

    // Try to parse as ISO8601/RFC3339
    if let Ok(dt) = DateTime::parse_from_rfc3339(&date_str) {
        let utc: DateTime<Utc> = dt.into();
        return utc.format("%Y-%m-%d").to_string();
    }

    date_str
}

fn apply_jmespath(data: &Value, query: &str) -> CliResult<Value> {
    let expr = jmespath::compile(query)
        .with_context(|| format!("Invalid JMESPath expression: {}", query))?;

    let result = expr
        .search(data)
        .with_context(|| format!("Failed to apply JMESPath query: {}", query))?;

    let json_str = serde_json::to_string(&result).context("Failed to serialize JMESPath result")?;
    let value =
        serde_json::from_str(&json_str).context("Failed to parse JMESPath result as JSON")?;

    Ok(value)
}

/// Test function to show all styles
pub async fn test_all_styles(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr.create_cloud_client(profile_name).await?;

    let response = client
        .get_raw("/users")
        .await
        .context("Failed to fetch users")?;

    let styles = vec![
        TableStyle::Blank,
        TableStyle::Psql,
        TableStyle::Markdown,
        TableStyle::Ascii,
        TableStyle::Modern,
        TableStyle::Sharp,
    ];

    for style in styles {
        print_users_table(&response, style)?;
        println!();
    }

    Ok(())
}
