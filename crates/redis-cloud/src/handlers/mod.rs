//! Cloud API handlers

pub mod account;
pub mod acl;
pub mod backup;
pub mod cloud_accounts;
pub mod database;
pub mod fixed;
pub mod logs;
pub mod metrics;
pub mod peering;
pub mod private_service_connect;
pub mod region;
pub mod subscription;
pub mod tasks;
pub mod transit_gateway;
pub mod users;

// Re-export all handlers
pub use account::CloudAccountHandler;
pub use acl::CloudAclHandler;
pub use backup::CloudBackupHandler;
pub use cloud_accounts::CloudAccountsHandler;
pub use database::CloudDatabaseHandler;
pub use fixed::CloudFixedHandler;
pub use logs::CloudLogsHandler;
pub use metrics::CloudMetricsHandler;
pub use peering::CloudPeeringHandler;
pub use private_service_connect::CloudPrivateServiceConnectHandler;
pub use region::CloudRegionHandler;
pub use subscription::CloudSubscriptionHandler;
pub use tasks::CloudTasksHandler;
pub use transit_gateway::CloudTransitGatewayHandler;
pub use users::CloudUsersHandler;
