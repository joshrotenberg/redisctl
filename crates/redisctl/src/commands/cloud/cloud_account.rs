#![allow(dead_code)]

use crate::cli::{CloudProviderAccountCommands, OutputFormat};
use crate::commands::cloud::cloud_account_impl::{self, CloudAccountOperationParams};
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
        CloudProviderAccountCommands::Create { file, async_ops } => {
            let params = CloudAccountOperationParams {
                conn_mgr,
                profile_name,
                client: &client,
                async_ops,
                output_format,
                query,
            };
            cloud_account_impl::handle_create(&params, file).await
        }
        CloudProviderAccountCommands::Update {
            account_id,
            file,
            async_ops,
        } => {
            let params = CloudAccountOperationParams {
                conn_mgr,
                profile_name,
                client: &client,
                async_ops,
                output_format,
                query,
            };
            cloud_account_impl::handle_update(&params, *account_id, file).await
        }
        CloudProviderAccountCommands::Delete {
            account_id,
            force,
            async_ops,
        } => {
            let params = CloudAccountOperationParams {
                conn_mgr,
                profile_name,
                client: &client,
                async_ops,
                output_format,
                query,
            };
            cloud_account_impl::handle_delete(&params, *account_id, *force).await
        }
    }
}
