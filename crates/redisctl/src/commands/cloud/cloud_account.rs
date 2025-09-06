#![allow(dead_code)]

use crate::cli::{CloudProviderAccountCommands, OutputFormat};
use crate::commands::cloud::cloud_account_impl;
use crate::commands::cloud::utils::create_cloud_client_raw;
use crate::connection::ConnectionManager;
use crate::error::Result as CliResult;

pub async fn handle_cloud_account_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    command: &CloudProviderAccountCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let profile = conn_mgr.get_profile(profile_name)?;
    let client = create_cloud_client_raw(profile).await?;

    match command {
        CloudProviderAccountCommands::List => {
            cloud_account_impl::handle_list(&client, output_format, query).await
        }
        CloudProviderAccountCommands::Get { account_id } => {
            cloud_account_impl::handle_get(&client, *account_id, output_format, query).await
        }
        CloudProviderAccountCommands::Create { file } => {
            cloud_account_impl::handle_create(&client, file, output_format, query).await
        }
        CloudProviderAccountCommands::Update { account_id, file } => {
            cloud_account_impl::handle_update(&client, *account_id, file, output_format, query)
                .await
        }
        CloudProviderAccountCommands::Delete { account_id, force } => {
            cloud_account_impl::handle_delete(&client, *account_id, *force).await
        }
    }
}
