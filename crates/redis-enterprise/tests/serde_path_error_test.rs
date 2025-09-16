use redis_enterprise::bdb::DatabaseInfo;

#[test]
fn test_serde_path_to_error_improvement() {
    // Create a test JSON with incorrect type for master_persistence
    // Previously this would give a generic error, now it should show the exact field path
    let bad_json = r#"{
        "uid": 1,
        "name": "test-db",
        "type": "redis",
        "memory_size": 1073741824,
        "port": 12000,
        "status": "active",
        "master_persistence": "should-be-bool",
        "data_persistence": "disabled"
    }"#;

    // Test with standard serde_json
    let standard_result: Result<DatabaseInfo, _> = serde_json::from_str(bad_json);
    assert!(standard_result.is_err());
    let standard_error = standard_result.unwrap_err();
    println!("Standard serde_json error: {}", standard_error);

    // Test with serde_path_to_error
    let deserializer = &mut serde_json::Deserializer::from_str(bad_json);
    let path_result: Result<DatabaseInfo, _> = serde_path_to_error::deserialize(deserializer);
    assert!(path_result.is_err());

    let path_error = path_result.unwrap_err();
    println!("Improved error with path:");
    println!("  Field path: {}", path_error.path());
    println!("  Error: {}", path_error.inner());

    // Verify the path includes "master_persistence"
    assert!(path_error.path().to_string().contains("master_persistence"));
}

#[test]
fn test_correct_types_work() {
    // Test that correctly typed JSON still works
    let good_json = r#"{
        "uid": 1,
        "name": "test-db",
        "type": "redis",
        "memory_size": 1073741824,
        "port": 12000,
        "status": "active",
        "master_persistence": false,
        "data_persistence": "disabled"
    }"#;

    let result: Result<DatabaseInfo, _> = serde_json::from_str(good_json);
    assert!(result.is_ok());

    let db = result.unwrap();
    assert_eq!(db.uid, 1);
    assert_eq!(db.name, "test-db".to_string());
    assert_eq!(db.master_persistence, Some(false));
}
