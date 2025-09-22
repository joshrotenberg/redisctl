#![allow(dead_code)]

use crate::cli::OutputFormat;
use crate::commands::enterprise::utils;
use crate::connection::ConnectionManager;
use crate::error::RedisCtlError;
use anyhow::Context;
use clap::Subcommand;
use serde_json::Value;

#[derive(Debug, Clone, Subcommand)]
pub enum LdapCommands {
    /// Get LDAP configuration
    Get,

    /// Update LDAP configuration
    Update {
        /// JSON data for LDAP configuration
        #[arg(long, required = true)]
        data: String,
    },

    /// Delete LDAP configuration
    Delete,

    /// Test LDAP connection
    Test {
        /// Optional test configuration JSON
        #[arg(long)]
        data: Option<String>,
    },
}

#[derive(Debug, Clone, Subcommand)]
pub enum LdapMappingsCommands {
    /// List all LDAP mappings
    List,

    /// Get specific LDAP mapping
    Get {
        /// Mapping UID
        uid: u64,
    },

    /// Create new LDAP mapping
    Create {
        /// JSON data for new mapping
        #[arg(long, required = true)]
        data: String,
    },

    /// Update existing LDAP mapping
    Update {
        /// Mapping UID
        uid: u64,
        /// JSON data for update
        #[arg(long, required = true)]
        data: String,
    },

    /// Delete LDAP mapping
    Delete {
        /// Mapping UID
        uid: u64,
    },
}

pub async fn handle_ldap_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    cmd: LdapCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    match cmd {
        LdapCommands::Get => handle_ldap_get(conn_mgr, profile_name, output_format, query).await,
        LdapCommands::Update { data } => {
            handle_ldap_update(conn_mgr, profile_name, &data, output_format, query).await
        }
        LdapCommands::Delete => {
            handle_ldap_delete(conn_mgr, profile_name, output_format, query).await
        }
        LdapCommands::Test { data } => {
            handle_ldap_test(
                conn_mgr,
                profile_name,
                data.as_deref(),
                output_format,
                query,
            )
            .await
        }
    }
}

pub async fn handle_ldap_mappings_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    cmd: LdapMappingsCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    match cmd {
        LdapMappingsCommands::List => {
            handle_mappings_list(conn_mgr, profile_name, output_format, query).await
        }
        LdapMappingsCommands::Get { uid } => {
            handle_mappings_get(conn_mgr, profile_name, uid, output_format, query).await
        }
        LdapMappingsCommands::Create { data } => {
            handle_mappings_create(conn_mgr, profile_name, &data, output_format, query).await
        }
        LdapMappingsCommands::Update { uid, data } => {
            handle_mappings_update(conn_mgr, profile_name, uid, &data, output_format, query).await
        }
        LdapMappingsCommands::Delete { uid } => {
            handle_mappings_delete(conn_mgr, profile_name, uid, output_format, query).await
        }
    }
}

async fn handle_ldap_get(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let response = client
        .get::<Value>("/v1/cluster/ldap")
        .await
        .map_err(RedisCtlError::from)?;

    let result = if let Some(q) = query {
        utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    utils::print_formatted_output(result, output_format)
}

async fn handle_ldap_update(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let payload: Value =
        serde_json::from_str(data).context("Invalid JSON data for LDAP configuration")?;

    let response = client
        .put_raw("/v1/cluster/ldap", payload)
        .await
        .map_err(RedisCtlError::from)?;

    let result = if let Some(q) = query {
        utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    utils::print_formatted_output(result, output_format)
}

async fn handle_ldap_delete(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let response = client
        .delete_raw("/v1/cluster/ldap")
        .await
        .map_err(RedisCtlError::from)?;

    let result = if let Some(q) = query {
        utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    utils::print_formatted_output(result, output_format)
}

async fn handle_ldap_test(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    data: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let payload = if let Some(d) = data {
        serde_json::from_str(d).context("Invalid JSON data for LDAP test")?
    } else {
        serde_json::json!({})
    };

    let response = client
        .post_raw("/v1/cluster/ldap/test", payload)
        .await
        .map_err(RedisCtlError::from)?;

    let result = if let Some(q) = query {
        utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    utils::print_formatted_output(result, output_format)
}

// LDAP Mappings handlers
async fn handle_mappings_list(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let response = client
        .get::<Value>("/v1/ldap_mappings")
        .await
        .map_err(RedisCtlError::from)?;

    let result = if let Some(q) = query {
        utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    utils::print_formatted_output(result, output_format)
}

async fn handle_mappings_get(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    uid: u64,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let endpoint = format!("/v1/ldap_mappings/{}", uid);
    let response = client
        .get::<Value>(&endpoint)
        .await
        .context(format!("Failed to get LDAP mapping {}", uid))?;

    let result = if let Some(q) = query {
        utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    utils::print_formatted_output(result, output_format)
}

async fn handle_mappings_create(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let payload: Value =
        serde_json::from_str(data).context("Invalid JSON data for LDAP mapping")?;

    let response = client
        .post_raw("/v1/ldap_mappings", payload)
        .await
        .map_err(RedisCtlError::from)?;

    let result = if let Some(q) = query {
        utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    utils::print_formatted_output(result, output_format)
}

async fn handle_mappings_update(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    uid: u64,
    data: &str,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let payload: Value =
        serde_json::from_str(data).context("Invalid JSON data for LDAP mapping update")?;

    let endpoint = format!("/v1/ldap_mappings/{}", uid);
    let response = client
        .put_raw(&endpoint, payload)
        .await
        .context(format!("Failed to update LDAP mapping {}", uid))?;

    let result = if let Some(q) = query {
        utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    utils::print_formatted_output(result, output_format)
}

async fn handle_mappings_delete(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    uid: u64,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<(), RedisCtlError> {
    let client = conn_mgr.create_enterprise_client(profile_name).await?;

    let endpoint = format!("/v1/ldap_mappings/{}", uid);
    let response = client
        .delete_raw(&endpoint)
        .await
        .context(format!("Failed to delete LDAP mapping {}", uid))?;

    let result = if let Some(q) = query {
        utils::apply_jmespath(&response, q)?
    } else {
        response
    };

    utils::print_formatted_output(result, output_format)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ldap_commands() {
        use clap::CommandFactory;

        #[derive(clap::Parser)]
        struct TestCli {
            #[command(subcommand)]
            cmd: LdapCommands,
        }

        TestCli::command().debug_assert();
    }

    #[test]
    fn test_ldap_mappings_commands() {
        use clap::CommandFactory;

        #[derive(clap::Parser)]
        struct TestCli {
            #[command(subcommand)]
            cmd: LdapMappingsCommands,
        }

        TestCli::command().debug_assert();
    }
}
