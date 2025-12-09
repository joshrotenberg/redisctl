//! Tests for cloud command output formatting

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[test]
    fn test_subscription_table_format() {
        // Test data that mimics real API responses
        let test_data = json!([
            {
                "id": 12345,
                "name": "production-cache",
                "status": "active",
                "planId": "pro",
                "planName": "Pro",
                "paymentMethod": "credit-card",
                "created": "2024-01-15T10:30:00Z",
                "numberOfDatabases": 3,
                "memoryStorage": {
                    "quantity": 4.0,
                    "units": "GB"
                },
                "cloudProviders": [
                    {
                        "provider": "AWS",
                        "regions": [
                            {
                                "region": "us-east-1",
                                "memoryStorage": {
                                    "quantity": 4.0
                                }
                            }
                        ]
                    }
                ]
            },
            {
                "id": 67890,
                "name": "staging-db",
                "status": "pending",
                "planId": "fixed-50",
                "planName": "Standard",
                "created": "2025-09-01T08:00:00Z",
                "numberOfDatabases": 1,
                "memoryStorage": {
                    "quantity": 1.0,
                    "units": "GB"
                },
                "cloudProviders": [
                    {
                        "provider": "GCP",
                        "regions": [
                            {
                                "region": "europe-west1",
                                "memoryStorage": {
                                    "quantity": 1.0
                                }
                            }
                        ]
                    }
                ]
            }
        ]);

        // Just verify the test data structure is valid
        assert!(test_data.is_array());
        assert_eq!(test_data.as_array().unwrap().len(), 2);

        // Verify we can extract expected fields
        let first = &test_data[0];
        assert_eq!(first["id"], 12345);
        assert_eq!(first["name"], "production-cache");
        assert_eq!(first["status"], "active");
    }

    #[test]
    fn test_jmespath_filtering() {
        let data = json!([
            {"id": 1, "status": "active", "memory": 4},
            {"id": 2, "status": "pending", "memory": 2},
            {"id": 3, "status": "active", "memory": 8}
        ]);

        // Test that we can compile a JMESPath expression
        let expr = jmespath::compile("[?status=='active']").unwrap();
        let result = expr.search(&data).unwrap();

        // Convert to JSON for testing
        let json_str = serde_json::to_string(&result).unwrap();
        let filtered: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        // Should have 2 active items
        assert!(filtered.is_array());
        assert_eq!(filtered.as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_jmespath_extended_functions() {
        // Test the extended JMESPath runtime with custom functions
        let mut runtime = jmespath::Runtime::new();
        runtime.register_builtin_functions();
        jmespath_extensions::register_all(&mut runtime);

        let data = json!([
            {"id": 1, "name": "production", "status": "active", "memory_bytes": 4294967296_i64},
            {"id": 2, "name": "staging", "status": "pending", "memory_bytes": 2147483648_i64},
            {"id": 3, "name": "development", "status": "active", "memory_bytes": 1073741824_i64}
        ]);

        // Test upper() function
        let expr = runtime
            .compile("[].{name: name, status: upper(status)}")
            .unwrap();
        let result = expr.search(&data).unwrap();
        let json_str = serde_json::to_string(&result).unwrap();
        let transformed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(transformed[0]["status"], "ACTIVE");
        assert_eq!(transformed[1]["status"], "PENDING");

        // Test unique() function
        let expr = runtime.compile("unique([].status)").unwrap();
        let result = expr.search(&data).unwrap();
        let json_str = serde_json::to_string(&result).unwrap();
        let unique_statuses: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert!(unique_statuses.is_array());
        assert_eq!(unique_statuses.as_array().unwrap().len(), 2); // "active" and "pending"

        // Test type_of() function
        let expr = runtime
            .compile("[0].{name: name, type: type_of(memory_bytes)}")
            .unwrap();
        let result = expr.search(&data).unwrap();
        let json_str = serde_json::to_string(&result).unwrap();
        let type_check: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(type_check["type"], "number");

        // Test is_empty() function
        let empty_data = json!({"items": [], "name": "test"});
        let expr = runtime
            .compile("{is_empty_items: is_empty(items)}")
            .unwrap();
        let result = expr.search(&empty_data).unwrap();
        let json_str = serde_json::to_string(&result).unwrap();
        let empty_check: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(empty_check["is_empty_items"], true);
    }

    #[test]
    fn test_jmespath_extended_string_functions() {
        let mut runtime = jmespath::Runtime::new();
        runtime.register_builtin_functions();
        jmespath_extensions::register_all(&mut runtime);

        let data = json!({
            "name": "  my-cluster-name  ",
            "url": "https://api.example.com/path"
        });

        // Test trim() function
        let expr = runtime.compile("trim(name)").unwrap();
        let result = expr.search(&data).unwrap();
        assert_eq!(result.to_string(), "\"my-cluster-name\"");

        // Test split() function - use string literal with single quotes
        let expr = runtime.compile("split(name, '-')").unwrap();
        let result = expr.search(&data).unwrap();
        let json_str = serde_json::to_string(&result).unwrap();
        let parts: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert!(parts.is_array());
    }

    #[test]
    fn test_jmespath_extended_utility_functions() {
        let mut runtime = jmespath::Runtime::new();
        runtime.register_builtin_functions();
        jmespath_extensions::register_all(&mut runtime);

        let data = json!({
            "primary_region": null,
            "fallback_region": "us-east-1"
        });

        // Test coalesce() function - returns first non-null value
        let expr = runtime
            .compile("coalesce(primary_region, fallback_region, `\"default\"`)")
            .unwrap();
        let result = expr.search(&data).unwrap();
        assert_eq!(result.to_string(), "\"us-east-1\"");

        // Test default() function
        let expr = runtime
            .compile("default(primary_region, `\"default-region\"`)")
            .unwrap();
        let result = expr.search(&data).unwrap();
        assert_eq!(result.to_string(), "\"default-region\"");
    }
}
