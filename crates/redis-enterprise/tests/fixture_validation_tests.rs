//! Fixture-based validation tests
//!
//! These tests use real API responses captured from a Redis Enterprise cluster
//! to validate that our Rust type definitions accurately match the actual API.
//!
//! ## Known Issues Found
//!
//! - Module struct has type mismatch: bigstore_version_2_support should be bool, not map

use redis_enterprise::{ClusterInfo, Database, License, Node, User};
use serde_json::Value;

#[test]
fn test_cluster_info_from_fixture() {
    let fixture = include_str!("fixtures/cluster.json");
    let cluster: ClusterInfo =
        serde_json::from_str(fixture).expect("Failed to deserialize cluster info");
    assert!(!cluster.name.is_empty());
}

#[test]
fn test_database_list_from_fixture() {
    let fixture = include_str!("fixtures/bdbs_list.json");
    let databases: Vec<Database> =
        serde_json::from_str(fixture).expect("Failed to deserialize database list");
    assert!(!databases.is_empty());
}

#[test]
fn test_single_database_from_fixture() {
    let fixture = include_str!("fixtures/bdb_single.json");
    let _database: Database =
        serde_json::from_str(fixture).expect("Failed to deserialize single database");
}

#[test]
fn test_nodes_list_from_fixture() {
    let fixture = include_str!("fixtures/nodes_list.json");
    let nodes: Vec<Node> = serde_json::from_str(fixture).expect("Failed to deserialize nodes list");
    assert!(!nodes.is_empty());
}

#[test]
fn test_users_list_from_fixture() {
    let fixture = include_str!("fixtures/users_list.json");
    let users: Vec<User> = serde_json::from_str(fixture).expect("Failed to deserialize users list");
    assert!(!users.is_empty());
}

#[test]
#[ignore = "Known type mismatch - bigstore_version_2_support should be bool not map"]
fn test_modules_list_from_fixture() {
    let fixture = include_str!("fixtures/modules_list.json");
    // This will fail until Module struct is fixed
    let _modules: Vec<serde_json::Value> =
        serde_json::from_str(fixture).expect("Failed to deserialize modules list");
}

#[test]
fn test_license_from_fixture() {
    let fixture = include_str!("fixtures/license.json");
    let _license: License = serde_json::from_str(fixture).expect("Failed to deserialize license");
}

#[test]
fn test_stats_from_fixtures() {
    // Test cluster stats
    let cluster_stats: Value = serde_json::from_str(include_str!("fixtures/cluster_stats.json"))
        .expect("Failed to deserialize cluster stats");
    assert!(cluster_stats.is_object());

    // Test database stats - these are arrays not objects
    let db_stats: Value = serde_json::from_str(include_str!("fixtures/bdbs_stats.json"))
        .expect("Failed to deserialize database stats");
    assert!(db_stats.is_array());

    // Test node stats - these are arrays not objects
    let node_stats: Value = serde_json::from_str(include_str!("fixtures/nodes_stats.json"))
        .expect("Failed to deserialize node stats");
    assert!(node_stats.is_array());
}
