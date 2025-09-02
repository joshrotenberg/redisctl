#![allow(unused_imports)]
//! Cloud API handlers

pub mod account;
pub mod acl;
pub mod api_keys;
pub mod backup;
pub mod billing;
pub mod cloud_accounts;
pub mod crdb;
pub mod database;
pub mod fixed;
pub mod logs;
pub mod metrics;
pub mod peering;
pub mod private_service_connect;
pub mod region;
pub mod sso;
pub mod subscription;
pub mod tasks;
pub mod transit_gateway;
pub mod users;

// Re-export all handlers
pub use account::CloudAccountHandler;
pub use acl::CloudAclHandler;
pub use api_keys::CloudApiKeyHandler;
pub use backup::CloudBackupHandler;
pub use billing::CloudBillingHandler;
pub use cloud_accounts::CloudAccountsHandler;
pub use crdb::CloudCrdbHandler;
pub use database::CloudDatabaseHandler;
pub use fixed::CloudFixedHandler;
pub use logs::CloudLogsHandler;
pub use metrics::CloudMetricsHandler;
pub use peering::CloudPeeringHandler;
pub use private_service_connect::CloudPrivateServiceConnectHandler;
pub use region::CloudRegionHandler;
pub use sso::CloudSsoHandler;
pub use subscription::CloudSubscriptionHandler;
pub use tasks::CloudTaskHandler;
pub use transit_gateway::CloudTransitGatewayHandler;
pub use users::CloudUserHandler;
