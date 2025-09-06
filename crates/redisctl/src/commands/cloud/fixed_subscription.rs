//! Fixed subscription command implementations

#![allow(dead_code)]

use crate::cli::{CloudFixedSubscriptionCommands, OutputFormat};
use crate::commands::cloud::utils::{
    confirm_action, handle_output, print_formatted_output, read_file_input,
};
use crate::connection::ConnectionManager;
use crate::error::Result as CliResult;
use anyhow::Context;
use redis_cloud::fixed::subscriptions::{
    FixedSubscriptionCreateRequest, FixedSubscriptionHandler, FixedSubscriptionUpdateRequest,
};

/// Handle fixed subscription commands
pub async fn handle_fixed_subscription_command(
    conn_mgr: &ConnectionManager,
    profile_name: Option<&str>,
    command: &CloudFixedSubscriptionCommands,
    output_format: OutputFormat,
    query: Option<&str>,
) -> CliResult<()> {
    let client = conn_mgr
        .create_cloud_client(profile_name)
        .await
        .context("Failed to create Cloud client")?;

    let handler = FixedSubscriptionHandler::new(client);

    match command {
        CloudFixedSubscriptionCommands::ListPlans { provider } => {
            let plans = if let Some(provider_filter) = provider {
                // If provider specified, fetch all plans and filter
                let all_plans = handler
                    .list_plans(None, None)
                    .await
                    .context("Failed to list fixed subscription plans")?;

                // Convert to JSON for filtering
                let mut json_plans = serde_json::to_value(all_plans)?;

                // Filter by provider if the structure supports it
                if let Some(plans_array) = json_plans.as_array_mut() {
                    plans_array.retain(|plan| {
                        plan.get("cloudProvider")
                            .and_then(|p| p.as_str())
                            .map(|p| p.eq_ignore_ascii_case(provider_filter))
                            .unwrap_or(false)
                    });
                }

                json_plans
            } else {
                // No filter, get all plans
                let plans = handler
                    .list_plans(None, None)
                    .await
                    .context("Failed to list fixed subscription plans")?;
                serde_json::to_value(plans)?
            };

            let data = handle_output(plans, output_format, query)?;
            print_formatted_output(data, output_format)?;
            Ok(())
        }

        CloudFixedSubscriptionCommands::GetPlans { subscription } => {
            let plans = handler
                .get_plans_by_subscription_id(*subscription)
                .await
                .context("Failed to get subscription plans")?;

            let json_response =
                serde_json::to_value(plans).context("Failed to serialize response")?;
            let data = handle_output(json_response, output_format, query)?;
            print_formatted_output(data, output_format)?;
            Ok(())
        }

        CloudFixedSubscriptionCommands::GetPlan { id } => {
            let plan = handler
                .get_plan_by_id(*id)
                .await
                .context("Failed to get plan details")?;

            let json_response =
                serde_json::to_value(plan).context("Failed to serialize response")?;
            let data = handle_output(json_response, output_format, query)?;
            print_formatted_output(data, output_format)?;
            Ok(())
        }

        CloudFixedSubscriptionCommands::List => {
            let subscriptions = handler
                .list()
                .await
                .context("Failed to list fixed subscriptions")?;

            let json_response =
                serde_json::to_value(subscriptions).context("Failed to serialize response")?;
            let data = handle_output(json_response, output_format, query)?;
            print_formatted_output(data, output_format)?;
            Ok(())
        }

        CloudFixedSubscriptionCommands::Get { id } => {
            let subscription = handler
                .get_by_id(*id)
                .await
                .context("Failed to get fixed subscription")?;

            let json_response =
                serde_json::to_value(subscription).context("Failed to serialize response")?;
            let data = handle_output(json_response, output_format, query)?;
            print_formatted_output(data, output_format)?;
            Ok(())
        }

        CloudFixedSubscriptionCommands::Create { file } => {
            let json_string = read_file_input(file)?;
            let request: FixedSubscriptionCreateRequest =
                serde_json::from_str(&json_string).context("Invalid subscription configuration")?;

            let result = handler
                .create(&request)
                .await
                .context("Failed to create fixed subscription")?;

            // Check if response contains a task ID
            let json_result =
                serde_json::to_value(&result).context("Failed to serialize response")?;
            if let Some(task_id) = json_result.get("taskId").and_then(|v| v.as_str()) {
                eprintln!(
                    "Fixed subscription creation initiated. Task ID: {}",
                    task_id
                );
                eprintln!(
                    "Use 'redisctl cloud task wait {}' to monitor progress",
                    task_id
                );
            }

            let data = handle_output(json_result, output_format, query)?;
            print_formatted_output(data, output_format)?;
            Ok(())
        }

        CloudFixedSubscriptionCommands::Update { id, file } => {
            let json_string = read_file_input(file)?;
            let request: FixedSubscriptionUpdateRequest =
                serde_json::from_str(&json_string).context("Invalid update configuration")?;

            let result = handler
                .update(*id, &request)
                .await
                .context("Failed to update fixed subscription")?;

            // Check if response contains a task ID
            let json_result =
                serde_json::to_value(&result).context("Failed to serialize response")?;
            if let Some(task_id) = json_result.get("taskId").and_then(|v| v.as_str()) {
                eprintln!("Fixed subscription update initiated. Task ID: {}", task_id);
                eprintln!(
                    "Use 'redisctl cloud task wait {}' to monitor progress",
                    task_id
                );
            }

            let data = handle_output(json_result, output_format, query)?;
            print_formatted_output(data, output_format)?;
            Ok(())
        }

        CloudFixedSubscriptionCommands::Delete { id, yes } => {
            if !yes {
                let prompt = format!("Delete fixed subscription {}?", id);
                if !confirm_action(&prompt)? {
                    eprintln!("Operation cancelled");
                    return Ok(());
                }
            }

            let result = handler
                .delete_by_id(*id)
                .await
                .context("Failed to delete fixed subscription")?;

            // Check if response contains a task ID
            let json_result =
                serde_json::to_value(&result).context("Failed to serialize response")?;
            if let Some(task_id) = json_result.get("taskId").and_then(|v| v.as_str()) {
                eprintln!(
                    "Fixed subscription deletion initiated. Task ID: {}",
                    task_id
                );
                eprintln!(
                    "Use 'redisctl cloud task wait {}' to monitor progress",
                    task_id
                );
            } else {
                eprintln!("Fixed subscription deleted successfully");
            }

            let data = handle_output(json_result, output_format, query)?;
            print_formatted_output(data, output_format)?;
            Ok(())
        }

        CloudFixedSubscriptionCommands::RedisVersions { subscription } => {
            let versions = handler
                .get_redis_versions(*subscription)
                .await
                .context("Failed to get Redis versions")?;

            let json_response =
                serde_json::to_value(versions).context("Failed to serialize response")?;
            let data = handle_output(json_response, output_format, query)?;
            print_formatted_output(data, output_format)?;
            Ok(())
        }
    }
}
