use criterion::{Criterion, black_box, criterion_group, criterion_main};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

// Sample response data that mimics a real API response
fn sample_database_list() -> Value {
    json!({
        "subscription": {
            "id": 123456,
            "name": "Production Subscription"
        },
        "databases": [
            {
                "databaseId": 1001,
                "databaseName": "cache-prod-01",
                "protocol": "redis",
                "provider": "AWS",
                "region": "us-east-1",
                "redisVersionCompliance": "7.0",
                "respVersion": "resp3",
                "status": "active",
                "memoryStorage": "ram",
                "memoryLimitInGb": 10.0,
                "memoryUsedInMb": 4567.89,
                "numberOfShards": 1,
                "throughputMeasurement": {
                    "by": "operations-per-second",
                    "value": 25000
                },
                "replication": true,
                "dataPersistence": "aof-every-1-second",
                "dataEvictionPolicy": "allkeys-lru",
                "activated": "2024-01-15T10:30:00Z",
                "lastModified": "2024-08-20T15:45:00Z",
                "publicEndpoint": "redis-12345.c1.us-east-1-1.ec2.cloud.redislabs.com:12345",
                "privateEndpoint": "redis-12345.internal.c1.us-east-1-1.ec2.cloud.redislabs.com:12345",
                "replicaOf": null,
                "clustering": {
                    "enabled": false,
                    "numberOfShards": 1
                },
                "security": {
                    "sslClientAuthentication": true,
                    "tlsClientAuthentication": true,
                    "defaultUser": true,
                    "dataAccessControl": true
                },
                "modules": [
                    {
                        "name": "RedisJSON",
                        "version": "2.6.0"
                    },
                    {
                        "name": "RedisSearch",
                        "version": "2.8.0"
                    }
                ],
                "alerts": [],
                "backupEnabled": true,
                "backupInterval": 24,
                "backupIntervalOffset": 3,
                "sourceIps": ["0.0.0.0/0"]
            },
            // Add more databases for realistic payload size
            {
                "databaseId": 1002,
                "databaseName": "cache-prod-02",
                "protocol": "redis",
                "provider": "AWS",
                "region": "us-west-2",
                "status": "active",
                "memoryLimitInGb": 5.0,
                "memoryUsedInMb": 2345.67,
                "throughputMeasurement": {
                    "by": "operations-per-second",
                    "value": 15000
                }
            },
            {
                "databaseId": 1003,
                "databaseName": "session-store",
                "protocol": "redis",
                "provider": "GCP",
                "region": "us-central1",
                "status": "active",
                "memoryLimitInGb": 8.0,
                "memoryUsedInMb": 6789.01,
                "throughputMeasurement": {
                    "by": "operations-per-second",
                    "value": 50000
                }
            }
        ],
        "links": [
            {
                "rel": "self",
                "type": "GET",
                "href": "https://api.redislabs.com/v1/subscriptions/123456/databases"
            }
        ]
    })
}

// Simplified typed structures for benchmarking
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DatabaseList {
    subscription: Subscription,
    databases: Vec<Database>,
    links: Vec<Link>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Subscription {
    id: u32,
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Database {
    #[serde(rename = "databaseId")]
    database_id: u32,
    #[serde(rename = "databaseName")]
    database_name: String,
    protocol: String,
    provider: Option<String>,
    region: Option<String>,
    status: String,
    #[serde(rename = "memoryLimitInGb")]
    memory_limit_in_gb: f64,
    #[serde(rename = "memoryUsedInMb")]
    memory_used_in_mb: Option<f64>,
    #[serde(flatten)]
    other: Value, // Capture remaining fields
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Link {
    rel: String,
    #[serde(rename = "type")]
    link_type: String,
    href: String,
}

fn benchmark_typed_approach(c: &mut Criterion) {
    let json_data = sample_database_list();
    let json_str = json_data.to_string();

    c.bench_function("typed_deserialize_serialize", |b| {
        b.iter(|| {
            // Simulate the typed approach: JSON -> Struct -> JSON
            let parsed: DatabaseList = serde_json::from_str(&json_str).unwrap();
            let _output = serde_json::to_value(&parsed).unwrap();
            black_box(_output);
        });
    });
}

fn benchmark_raw_approach(c: &mut Criterion) {
    let json_data = sample_database_list();
    let json_str = json_data.to_string();

    c.bench_function("raw_value_passthrough", |b| {
        b.iter(|| {
            // Simulate the raw approach: JSON -> Value -> JSON (passthrough)
            let parsed: Value = serde_json::from_str(&json_str).unwrap();
            let _output = parsed.clone(); // In real usage, this would just be passed through
            black_box(_output);
        });
    });
}

fn benchmark_hybrid_approach(c: &mut Criterion) {
    let json_data = sample_database_list();
    let json_str = json_data.to_string();

    c.bench_function("hybrid_partial_parsing", |b| {
        b.iter(|| {
            // Simulate a hybrid approach: Parse only what we need
            let parsed: Value = serde_json::from_str(&json_str).unwrap();

            // Extract specific fields without full deserialization
            if let Some(databases) = parsed.get("databases").and_then(|d| d.as_array()) {
                for db in databases {
                    let _id = db.get("databaseId");
                    let _name = db.get("databaseName");
                    let _status = db.get("status");
                    black_box((_id, _name, _status));
                }
            }

            let _output = parsed;
            black_box(_output);
        });
    });
}

fn benchmark_large_payload(c: &mut Criterion) {
    // Create a larger payload with many databases
    let mut large_payload = json!({
        "subscription": {
            "id": 123456,
            "name": "Production Subscription"
        },
        "databases": [],
        "links": []
    });

    // Add 100 databases to simulate a large response
    let databases = (0..100)
        .map(|i| {
            json!({
                "databaseId": 2000 + i,
                "databaseName": format!("database-{}", i),
                "protocol": "redis",
                "provider": "AWS",
                "region": "us-east-1",
                "status": "active",
                "memoryLimitInGb": 10.0,
                "memoryUsedInMb": 5000.0 + i as f64 * 10.0,
                "throughputMeasurement": {
                    "by": "operations-per-second",
                    "value": 10000 + i * 100
                },
                "modules": [
                    {"name": "RedisJSON", "version": "2.6.0"},
                    {"name": "RedisSearch", "version": "2.8.0"}
                ],
                "security": {
                    "sslClientAuthentication": true,
                    "dataAccessControl": true
                }
            })
        })
        .collect::<Vec<_>>();

    large_payload["databases"] = json!(databases);
    let json_str = large_payload.to_string();

    let mut group = c.benchmark_group("large_payload_100_databases");

    group.bench_function("typed", |b| {
        b.iter(|| {
            let parsed: Value = serde_json::from_str(&json_str).unwrap();
            // Would normally deserialize to typed struct here
            let _output = serde_json::to_string(&parsed).unwrap();
            black_box(_output);
        });
    });

    group.bench_function("raw", |b| {
        b.iter(|| {
            let parsed: Value = serde_json::from_str(&json_str).unwrap();
            let _output = serde_json::to_string(&parsed).unwrap();
            black_box(_output);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_typed_approach,
    benchmark_raw_approach,
    benchmark_hybrid_approach,
    benchmark_large_payload
);

criterion_main!(benches);
