//! Cloud connectivity command implementations

#![allow(dead_code)]

pub mod psc;
pub mod tgw;
pub mod vpc_peering;

use crate::cli::{CloudConnectivityCommands, OutputFormat};
use crate::connection::ConnectionManager;
use crate::error::Result as CliResult;

/// Handle connectivity commands
pub async fn handle_connectivity_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    command: &CloudConnectivityCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    match command {
        CloudConnectivityCommands::VpcPeering(vpc_cmd) => {
            vpc_peering::handle_vpc_peering_command(
                conn_mgr,
                profile_name,
                vpc_cmd,
                output_format,
                query,
            )
            .await
        }
        CloudConnectivityCommands::Psc(psc_cmd) => {
            psc::handle_psc_command(conn_mgr, profile_name, psc_cmd, output_format, query).await
        }
        CloudConnectivityCommands::Tgw(tgw_cmd) => {
            tgw::handle_tgw_command(conn_mgr, profile_name, tgw_cmd, output_format, query).await
        }
    }
}
