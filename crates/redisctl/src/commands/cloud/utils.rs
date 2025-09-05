//! Shared utilities for cloud command implementations

use anyhow::Context;
use chrono::{DateTime, Utc};
use colored::Colorize;
use serde_json::Value;
use tabled::Tabled;

#[cfg(unix)]
use std::io::IsTerminal;

use crate::cli::OutputFormat;
use crate::error::{RedisCtlError, Result as CliResult};
use crate::output::print_output;

/// Row structure for vertical table display (used by get commands)
#[derive(Tabled)]
pub struct DetailRow {
    #[tabled(rename = "FIELD")]
    pub field: String,
    #[tabled(rename = "VALUE")]
    pub value: String,
}

/// Truncate string to max length with ellipsis
pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else if max_len > 3 {
        format!("{}...", &s[..max_len - 3])
    } else {
        s[..max_len].to_string()
    }
}

/// Extract field from JSON value with fallback
pub fn extract_field(value: &Value, field: &str, default: &str) -> String {
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

/// Output with automatic pager for long content
pub fn output_with_pager(content: &str) {
    // Check if we should use a pager (Unix only)
    #[cfg(unix)]
    {
        use std::io::Write;
        use std::process::{Command, Stdio};

        let lines: Vec<&str> = content.lines().collect();
        if should_use_pager(&lines) {
            // Get pager command from environment or use default
            let pager_cmd = std::env::var("PAGER").unwrap_or_else(|_| "less -R".to_string());

            // Split pager command into program and args
            let mut parts = pager_cmd.split_whitespace();
            let program = parts.next().unwrap_or("less");
            let args: Vec<&str> = parts.collect();

            // Try to spawn pager process
            match Command::new(program)
                .args(&args)
                .stdin(Stdio::piped())
                .spawn()
            {
                Ok(mut child) => {
                    // Write content to pager's stdin
                    if let Some(mut stdin) = child.stdin.take() {
                        let _ = stdin.write_all(content.as_bytes());
                        let _ = stdin.flush();
                        // Close stdin to signal EOF to pager
                        drop(stdin);
                    }

                    // Wait for pager to finish
                    let _ = child.wait();
                    return;
                }
                Err(_) => {
                    // If pager fails to spawn, fall through to regular println
                }
            }
        }
    }

    println!("{}", content);
}

/// Check if we should use a pager for output (Unix only)
#[cfg(unix)]
fn should_use_pager(lines: &[&str]) -> bool {
    // Only page if we're in a TTY
    if !std::io::stdout().is_terminal() {
        return false;
    }

    // Get terminal height
    if let Some((_, height)) = terminal_size::terminal_size() {
        let term_height = height.0 as usize;
        // Use pager if output exceeds 80% of terminal height
        return lines.len() > (term_height * 8 / 10);
    }

    // Default to paging if we have more than 20 lines
    lines.len() > 20
}

/// Format status with color coding
pub fn format_status(status: String) -> String {
    match status.to_lowercase().as_str() {
        "active" => status.green().to_string(),
        "pending" => status.yellow().to_string(),
        "error" | "failed" => status.red().to_string(),
        _ => status,
    }
}

/// Format status text with color
pub fn format_status_text(status: &str) -> String {
    match status.to_lowercase().as_str() {
        "active" => status.green().to_string(),
        "suspended" | "inactive" => status.red().to_string(),
        "pending" => status.yellow().to_string(),
        _ => status.to_string(),
    }
}

/// Format date in human-readable format
pub fn format_date(date_str: String) -> String {
    if date_str.is_empty() || date_str == "—" {
        return "—".to_string();
    }

    // If it's already formatted (e.g., "2024-04-09 02:22:05"), keep it
    if date_str.contains(' ') && !date_str.contains('T') {
        return date_str;
    }

    // Try to parse as ISO8601/RFC3339
    if let Ok(dt) = DateTime::parse_from_rfc3339(&date_str) {
        let utc: DateTime<Utc> = dt.into();
        let now = Utc::now();
        let duration = now.signed_duration_since(utc);

        // Show relative time for recent items
        if duration.num_days() == 0 {
            if duration.num_hours() == 0 {
                return format!("{} min ago", duration.num_minutes());
            }
            return format!("{} hours ago", duration.num_hours());
        } else if duration.num_days() < 7 {
            return format!("{} days ago", duration.num_days());
        }

        // Show date for older items
        return utc.format("%Y-%m-%d").to_string();
    }

    // Fallback to original string
    date_str
}

/// Format memory size in human-readable format
pub fn format_memory_size(gb: f64) -> String {
    if gb < 1.0 {
        format!("{:.0}MB", gb * 1024.0)
    } else {
        format!("{:.1}GB", gb)
    }
}

/// Get short provider name for display
pub fn provider_short_name(provider: &str) -> &str {
    match provider.to_lowercase().as_str() {
        "aws" => "AWS",
        "gcp" | "google" => "GCP",
        "azure" => "Azure",
        _ => provider,
    }
}

/// Apply JMESPath query to JSON data
pub fn apply_jmespath(data: &Value, query: &str) -> CliResult<Value> {
    let expr = jmespath::compile(query)
        .with_context(|| format!("Invalid JMESPath expression: {}", query))?;

    let result = expr
        .search(data)
        .with_context(|| format!("Failed to apply JMESPath query: {}", query))?;

    // Convert jmespath Variable to serde_json Value
    let json_str = serde_json::to_string(&result).context("Failed to serialize JMESPath result")?;
    let value =
        serde_json::from_str(&json_str).context("Failed to parse JMESPath result as JSON")?;

    Ok(value)
}

/// Handle output formatting for different formats
pub fn handle_output(
    data: Value,
    _output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<Value> {
    if let Some(q) = query {
        apply_jmespath(&data, q)
    } else {
        Ok(data)
    }
}

/// Print data in requested output format
pub fn print_formatted_output(data: Value, output_format: OutputFormat) -> CliResult<()> {
    match output_format {
        OutputFormat::Json => {
            print_output(data, crate::output::OutputFormat::Json, None).map_err(|e| {
                RedisCtlError::OutputError {
                    message: e.to_string(),
                }
            })?;
        }
        OutputFormat::Yaml => {
            print_output(data, crate::output::OutputFormat::Yaml, None).map_err(|e| {
                RedisCtlError::OutputError {
                    message: e.to_string(),
                }
            })?;
        }
        _ => {} // Table format handled by individual commands
    }
    Ok(())
}
