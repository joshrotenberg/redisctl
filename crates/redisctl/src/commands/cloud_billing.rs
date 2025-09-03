use crate::output::{OutputFormat, print_output};
use anyhow::Result;
use redis_cloud::{CloudBillingHandler, CloudClient};

use crate::cli::BillingCommands;

pub async fn handle_billing_command(
    command: BillingCommands,
    client: &CloudClient,
    output_format: OutputFormat,
    query: Option<&str>,
) -> Result<()> {
    let billing_handler = CloudBillingHandler::new(client.clone());

    let result = match command {
        BillingCommands::Info => {
            let info = billing_handler.get_info().await?;
            Ok(serde_json::to_value(info)?)
        }
        BillingCommands::InvoiceList { .. } => {
            let list = billing_handler.list_invoices().await?;
            Ok(serde_json::to_value(list)?)
        }
        BillingCommands::InvoiceGet { id } => {
            let inv = billing_handler.get_invoice(&id).await?;
            Ok(serde_json::to_value(inv)?)
        }
        BillingCommands::InvoiceDownload { id, output } => {
            // Note: The API returns JSON, not actual PDF data.
            // This would need to be handled differently in production
            let data = billing_handler.download_invoice(&id).await?;
            let filename = output.unwrap_or_else(|| format!("invoice_{}.json", id));
            let json_str = serde_json::to_string_pretty(&data)?;
            std::fs::write(&filename, json_str)?;
            println!("Invoice data saved to {}", filename);
            return Ok(());
        }
        BillingCommands::InvoiceCurrent => {
            let inv = billing_handler.get_current_invoice().await?;
            Ok(serde_json::to_value(inv)?)
        }
        BillingCommands::PaymentMethodList => {
            let methods = billing_handler.list_payment_methods().await?;
            Ok(serde_json::to_value(methods)?)
        }
        BillingCommands::PaymentMethodGet { id } => {
            let method_id: u32 = id.parse()?;
            let method = billing_handler.get_payment_method(method_id).await?;
            Ok(serde_json::to_value(method)?)
        }
        BillingCommands::PaymentMethodAdd { data } => {
            let payment_method: serde_json::Value = serde_json::from_str(&data)?;
            // Keep raw for CLI free-form payloads
            client.post_raw("/payment-methods", payment_method).await
        }
        BillingCommands::PaymentMethodUpdate { id, data } => {
            let method_id: u32 = id.parse()?;
            let payment_method: serde_json::Value = serde_json::from_str(&data)?;
            client
                .put_raw(&format!("/payment-methods/{}", method_id), payment_method)
                .await
        }
        BillingCommands::PaymentMethodDelete { id, force } => {
            if !force {
                println!(
                    "Are you sure you want to delete payment method {}? Use --force to confirm",
                    id
                );
                return Ok(());
            }
            let method_id: u32 = id.parse()?;
            billing_handler.delete_payment_method(method_id).await
        }
        BillingCommands::PaymentMethodDefault { id } => {
            let method_id: u32 = id.parse()?;
            billing_handler.set_default_payment_method(method_id).await
        }
        BillingCommands::CostBreakdown { subscription } => {
            // The API expects a period string, not a subscription ID
            // Using "current" as a default period
            let period = subscription
                .map(|s| s.to_string())
                .unwrap_or_else(|| "current".to_string());
            let cb = billing_handler.get_cost_breakdown(&period).await?;
            Ok(serde_json::to_value(cb)?)
        }
        BillingCommands::Usage { from, to } => {
            // Both dates are required for the usage API
            let start = from.as_deref().unwrap_or("2024-01-01");
            let end = to.as_deref().unwrap_or("2024-12-31");
            let usage = billing_handler.get_usage(start, end).await?;
            Ok(serde_json::to_value(usage)?)
        }
        BillingCommands::History { months } => {
            // The API takes start_date and end_date, not months
            // We'll calculate the date range based on months
            let end_date = chrono::Local::now().format("%Y-%m-%d").to_string();
            let start_date = if let Some(m) = months {
                chrono::Local::now()
                    .checked_sub_months(chrono::Months::new(m))
                    .unwrap_or_else(chrono::Local::now)
                    .format("%Y-%m-%d")
                    .to_string()
            } else {
                // Default to 6 months
                chrono::Local::now()
                    .checked_sub_months(chrono::Months::new(6))
                    .unwrap_or_else(chrono::Local::now)
                    .format("%Y-%m-%d")
                    .to_string()
            };
            let hist = billing_handler
                .get_history(Some(&start_date), Some(&end_date))
                .await?;
            Ok(serde_json::to_value(hist)?)
        }
        BillingCommands::Credits => {
            let credits = billing_handler.get_credits().await?;
            Ok(serde_json::to_value(credits)?)
        }
        BillingCommands::PromoApply { code } => {
            let resp = billing_handler.apply_promo_code(&code).await?;
            Ok(serde_json::to_value(resp)?)
        }
        BillingCommands::AlertList => {
            let alerts = billing_handler.get_alerts().await?;
            Ok(serde_json::to_value(alerts)?)
        }
        BillingCommands::AlertUpdate { data } => {
            let alerts: serde_json::Value = serde_json::from_str(&data)?;
            client.put_raw("/billing/alerts", alerts).await
        }
    };

    print_output(result?, output_format, query)
}
