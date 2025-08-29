#![allow(dead_code)]

use anyhow::{Context, Result};
use comfy_table::Table;
use jmespath::compile;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum OutputFormat {
    Json,
    Yaml,
    Table,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Json
    }
}

pub fn print_output<T: Serialize>(
    data: T,
    format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let mut json_value = serde_json::to_value(data)?;

    // Apply JMESPath query if provided
    if let Some(query_str) = query {
        let expr = compile(query_str).context("Invalid JMESPath expression")?;
        // Convert Value to string then parse as Variable
        let json_str = serde_json::to_string(&json_value)?;
        let data = jmespath::Variable::from_json(&json_str)
            .map_err(|e| anyhow::anyhow!("Failed to parse JSON for JMESPath: {}", e))?;
        let result = expr.search(&data).context("JMESPath query failed")?;
        // Convert result back to JSON string then parse as Value
        let result_str = result.to_string();
        json_value =
            serde_json::from_str(&result_str).context("Failed to parse JMESPath result")?;
    }

    match format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&json_value)?);
        }
        OutputFormat::Yaml => {
            println!("{}", serde_yaml::to_string(&json_value)?);
        }
        OutputFormat::Table => {
            print_as_table(&json_value)?;
        }
    }

    Ok(())
}

fn print_as_table(value: &Value) -> Result<()> {
    match value {
        Value::Array(arr) if !arr.is_empty() => {
            let mut table = Table::new();

            // Get headers from first object
            if let Value::Object(first) = &arr[0] {
                let headers: Vec<String> = first.keys().cloned().collect();
                table.set_header(&headers);

                // Add rows
                for item in arr {
                    if let Value::Object(obj) = item {
                        let row: Vec<String> = headers
                            .iter()
                            .map(|h| format_value(obj.get(h).unwrap_or(&Value::Null)))
                            .collect();
                        table.add_row(row);
                    }
                }
            } else {
                // Simple array of values
                table.set_header(vec!["Value"]);
                for item in arr {
                    table.add_row(vec![format_value(item)]);
                }
            }

            println!("{}", table);
        }
        Value::Object(obj) => {
            let mut table = Table::new();
            table.set_header(vec!["Key", "Value"]);

            for (key, val) in obj {
                table.add_row(vec![key.clone(), format_value(val)]);
            }

            println!("{}", table);
        }
        _ => {
            println!("{}", format_value(value));
        }
    }

    Ok(())
}

fn format_value(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.clone(),
        Value::Array(arr) => format!("[{} items]", arr.len()),
        Value::Object(obj) => format!("{{{} fields}}", obj.len()),
    }
}
