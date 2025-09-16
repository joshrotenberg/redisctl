//! Test fixtures with realistic API responses
//!
//! These fixtures are based on actual Redis Enterprise API responses
//! to ensure our mock tests catch type mismatches.
#![allow(dead_code)]

use serde_json::json;

/// Returns a realistic cluster info response from /v1/cluster
pub fn cluster_info_response() -> serde_json::Value {
    json!({
        "name": "test-cluster.local",
        "created_time": "2025-01-15T10:00:00Z",
        "cnm_http_port": 8080,
        "cnm_https_port": 9443,
        "cm_port": 8443,
        "cm_server_version": 1,
        "cm_session_timeout_minutes": 15,
        "email_alerts": false,
        "rack_aware": false,
        "bigstore_driver": "speedb",

        // Fields that were causing deserialization issues
        "password_complexity": false,  // Was incorrectly Option<Value>
        "password_expiration_duration": 0,
        "password_min_length": 8,
        "password_hashing_algorithm": "SHA-256",
        "upgrade_mode": false,  // Was incorrectly Option<String>
        "upgrade_in_progress": false,
        "mtls_certificate_authentication": false,  // Was incorrectly Option<String>
        "mtls_client_cert_subject_validation_type": "disabled",
        "sentinel_cipher_suites": [],  // Was incorrectly Option<String>
        "sentinel_cipher_suites_tls_1_3": "TLS_AES_256_GCM_SHA384:TLS_CHACHA20_POLY1305_SHA256:TLS_AES_128_GCM_SHA256",  // Was incorrectly Option<Vec<Value>>
        "sentinel_tls_mode": "allowed",
        "data_cipher_suites_tls_1_3": [],  // Array type

        // Other realistic fields
        "min_control_TLS_version": "1.2",
        "min_data_TLS_version": "1.2",
        "min_sentinel_TLS_version": "1.2",
        "control_cipher_suites": "DEFAULT:!3DES",
        "control_cipher_suites_tls_1_3": "TLS_AES_256_GCM_SHA384:TLS_CHACHA20_POLY1305_SHA256:TLS_AES_128_GCM_SHA256",
        "block_cluster_changes": false,
        "ccs_internode_encryption": true,
        "cluster_api_internal_port": 3346,
        "crdb_coordinator_port": 9081,
        "crdb_coordinator_ignore_requests": false,
        "crdt_supported_featureset_version": 8,
        "crdt_supported_protocol_versions": ["1"],
        "use_ipv6": true,
        "use_external_ipv6": true,
        "wait_command": true,
        "slave_ha": true,
        "slave_ha_grace_period": 600,
        "slave_ha_cooldown_period": 3600,
        "slave_ha_bdb_cooldown_period": 7200,
        "http_support": true,
        "handle_redirects": false,
        "handle_metrics_redirects": false,
        "robust_crdt_syncer": true,
        "s3_certificate_verification": true,
        "smtp_use_tls": false,
        "smtp_tls_mode": "none",
        "slowlog_in_sanitized_support": true,
        "options_method_forbidden": false,
        "multi_commands_opt": "batch",
        "mask_bdb_credentials": false,
        "metrics_system": 0,
        "module_upload_max_size_mb": 40,
        "mtls_authorized_subjects": [],
        "encrypt_pkeys": false,
        "entra_id_cache_ttl": 30,
        "envoy_admin_port": 8002,
        "envoy_external_authorization": false,
        "envoy_max_downstream_connections": 1024,
        "envoy_mgmt_server_port": 8004,
        "gossip_envoy_admin_port": 8006,
        "debuginfo_path": "/tmp",
        "data_cipher_list": "DEFAULT:!aNULL:!eNULL:!EXPORT:!DES:!3DES:!RC4:!MD5:!PSK",
        "availability_lag_tolerance_ms": 100,

        // Complex nested objects
        "alert_settings": {
            "cluster_ca_cert_about_to_expire": {
                "enabled": true,
                "threshold": "90"
            },
            "cluster_certs_about_to_expire": {
                "enabled": true,
                "threshold": "45"
            },
            "cluster_license_about_to_expire": {
                "enabled": true,
                "threshold": "7"
            },
            "cluster_node_operation_failed": true,
            "cluster_ocsp_query_failed": true,
            "cluster_ocsp_status_revoked": true,
            "node_checks_error": true,
            "node_ephemeral_storage": {
                "enabled": true,
                "threshold": "70"
            }
        },
        "logrotate_settings": {
            "maxage": 7,
            "maxsize": "200M",
            "rotate": 10
        },
        "reserved_ports": [],
        "system_reserved_ports": [1968, 3333, 3334, 8080, 8443, 9443],

        // Certificates (truncated for brevity)
        "proxy_certificate": "-----BEGIN CERTIFICATE-----\nMIIFiDCCA3CgAwIBAgIBATANBg...\n-----END CERTIFICATE-----",
        "syncer_certificate": "-----BEGIN CERTIFICATE-----\nMIIFhjCCA26gAwIBAgIBATANBg...\n-----END CERTIFICATE-----",
        "cluster_ssh_public_key": "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAAB..."
    })
}

/// Returns a realistic user response from /v1/users
pub fn user_response() -> serde_json::Value {
    json!({
        "uid": 1,
        "email": "admin@cluster.local",  // NOT username!
        "name": "Administrator",  // Display name
        "role": "admin",
        "status": "active",
        "auth_method": "regular",
        "certificate_subject_line": "",
        "password_issue_date": "2025-01-15T10:00:00Z",
        "email_alerts": true,
        "role_uids": [1],
        "bdbs": [],
        "alert_audit_db_conns": false,
        "alert_bdb_backup": false,
        "alert_bdb_crdt_src_syncer": false,
        "password_expiration_duration": 0
    })
}

/// Returns a realistic license response from /v1/license
pub fn license_response() -> serde_json::Value {
    json!({
        // Note: NO license_key field!
        "key": "trial-key-123456",
        "license": "-----BEGIN LICENSE-----\n...\n-----END LICENSE-----",
        "type": "trial",
        "expired": false,
        "activation_date": "2025-01-15T00:00:00Z",
        "expiration_date": "2025-02-15T00:00:00Z",
        "cluster_name": "test-cluster",
        "owner": "Test Organization",
        "shards_limit": 100,
        "ram_shards_in_use": 0,
        "ram_shards_limit": 100,
        "flash_shards_in_use": 0,
        "flash_shards_limit": 50,
        "features": ["trial", "bigstore", "modules"]
    })
}

/// Returns a realistic node response from /v1/nodes
pub fn node_response() -> serde_json::Value {
    json!({
        "uid": 1,
        "accept_servers": true,
        "addr": "192.168.1.100",
        "architecture": "x86_64",
        "bigredis_storage_path": "/var/opt/redislabs/flash",
        "bigstore_driver": "speedb",
        "bigstore_enabled": false,
        "bigstore_free": 0,
        "cores": 8,
        "ephemeral_storage_path": "/var/opt/redislabs/tmp",
        "ephemeral_storage_size": 10737418240i64,
        "external_addr": ["192.168.1.100"],
        "hostname": "node1.cluster.local",
        "os_semantic_version": "6.1.2",
        "os_version": "Ubuntu 22.04.3 LTS",
        "persistent_storage_path": "/var/opt/redislabs/persist",
        "persistent_storage_size": 107374182400i64,
        "rack_id": "",
        "shard_count": 0,
        "shard_list": [],
        "software_version": "7.2.4-92",
        "status": "active",
        "supported_database_versions": [
            {"db_type": "redis", "version": "6.2.14"},
            {"db_type": "redis", "version": "7.2.4"}
        ],
        "total_memory": 16777216000i64,
        "uptime": 3600
    })
}

/// Returns a realistic database (BDB) response from /v1/bdbs
pub fn database_response() -> serde_json::Value {
    json!({
        "uid": 1,
        "name": "test-db",
        "type": "redis",
        "version": "7.2.4",
        "memory_size": 1073741824,
        "port": 12000,
        "status": "active",
        "ssl": false,
        "tls_mode": "disabled",
        "authentication_redis_pass": "******",
        "authentication_sasl_pass": null,
        "authentication_sasl_uname": null,
        "auto_upgrade": false,
        "backup": false,
        "backup_history": 0,
        "backup_interval": 0,
        "backup_interval_offset": 0,
        "backup_status": "idle",
        "bigstore": false,
        "bigstore_ram_size": 0,
        "crdt": false,
        "crdt_guid": null,
        "crdt_replica_id": 0,
        "crdt_sources": [],
        "crdt_sync": "disabled",
        "data_internode_encryption": false,
        "data_persistence": "disabled",
        "default_user": true,
        "dns_address_master": null,
        "email_alerts": false,
        "endpoint_host": null,
        "endpoint_ip": ["192.168.1.100"],
        "endpoint_port": 12000,
        "eviction_policy": "volatile-lru",
        "export_failure_reason": null,
        "export_progress": null,
        "export_status": null,
        "generate_text_monitor": false,
        "gradual_src_mode": "disabled",
        "gradual_src_sync_state": "idle",
        "gradual_sync_mode": "auto",
        "hash_slots_policy": null,
        "implicit_shard_key": false,
        "import_failure_reason": null,
        "import_progress": null,
        "import_status": null,
        "internal": false,
        "last_changed_time": "2025-01-15T10:00:00Z",
        "max_aof_file_size": 322122547200i64,
        "max_aof_load_time": 3600,
        "max_clients": 10000,
        "max_connections": 0,
        "max_simultaneous_backups": 1,
        "module_list": [],
        "mtls_allow_weak_hashing": false,
        "mtls_allow_outdated_cert": false,
        "mtls_client_cert": "",
        "mtls_log_file": "",
        "oss_cluster": false,
        "oss_cluster_api_preferred_ip_type": "internal",
        "oss_sharding": false,
        "persistence": "disabled",
        "proxy_policy": "single",
        "rack_aware": false,
        "redis_conf": [],
        "replica_ha": true,
        "replica_ha_priority": 0,
        "replication": false,
        "roles_permissions": [],
        "shard_block_connection_timeout": 0,
        "shard_block_cross_slot": false,
        "shard_count": 1,
        "shard_cpu_limit": "",
        "shard_list": [1],
        "sharding": false,
        "shards_placement": "dense",
        "slave_ha": true,
        "slave_ha_priority": 0,
        "snapshot_policy": [],
        "ssl_certificate": "",
        "sync": "disabled",
        "sync_sources": [],
        "syncer_mode": null,
        "wait_command": true
    })
}

/// Returns a realistic OCSP configuration response from /v1/ocsp
pub fn ocsp_config_response() -> serde_json::Value {
    json!({
        "enabled": false,
        "ocsp_functionality": false,
        "responder_url": "",
        "response_timeout": 1,
        "query_frequency": 3600,
        "recovery_frequency": 60,
        "recovery_max_tries": 5
    })
}

/// Returns a realistic LDAP configuration response from /v1/cluster/ldap
pub fn ldap_config_response() -> serde_json::Value {
    json!({
        "bind_dn": "",
        "bind_pass": "******",
        "ca_cert": "",
        "cache_ttl": 300,
        "control_plane": false,
        "data_plane": false,
        "directory_timeout_s": 5,
        "dn_group_format": "",
        "dn_user_format": "",
        "group_attr": "",
        "group_dn": "",
        "group_query": "",
        "host": "",
        "nested_groups": false,
        "port": 389,
        "protocol": "ldap",
        "search_pass": "******",
        "search_user": "",
        "starttls": false,
        "user_attr": "",
        "user_dn": "",
        "user_query": ""
    })
}

/// Returns a realistic services list response from /v1/local/services
pub fn services_list_response() -> serde_json::Value {
    json!({
        "alert_mgr": {
            "start_time": "2025-01-15T10:00:00Z",
            "status": "RUNNING",
            "uptime": "1:23:45"
        },
        "authentication_service": {
            "start_time": "2025-01-15T10:00:00Z",
            "status": "RUNNING",
            "uptime": "1:23:45"
        },
        "call_home_agent": {
            "start_time": "2025-01-15T10:00:00Z",
            "status": "RUNNING",
            "uptime": "1:23:45"
        },
        "ccs": {
            "start_time": "2025-01-15T10:00:00Z",
            "status": "RUNNING",
            "uptime": "1:23:45"
        },
        "cluster_wd": {
            "start_time": "2025-01-15T10:00:00Z",
            "status": "RUNNING",
            "uptime": "1:23:45"
        },
        "cm_server": {
            "start_time": "2025-01-15T10:00:00Z",
            "status": "RUNNING",
            "uptime": "1:23:45"
        },
        "cnm_http": {
            "start_time": "2025-01-15T10:00:00Z",
            "status": "RUNNING",
            "uptime": "1:23:45"
        },
        "cnm_https": {
            "start_time": "2025-01-15T10:00:00Z",
            "status": "RUNNING",
            "uptime": "1:23:45"
        },
        "crdb_coordinator": {
            "start_time": "2025-01-15T10:00:00Z",
            "status": "RUNNING",
            "uptime": "1:23:45"
        },
        "dmcproxy": {
            "start_time": "2025-01-15T10:00:00Z",
            "status": "RUNNING",
            "uptime": "1:23:45"
        },
        "envoy": {
            "start_time": "2025-01-15T10:00:00Z",
            "status": "RUNNING",
            "uptime": "1:23:45"
        },
        "envoy_control_plane": {
            "start_time": "2025-01-15T10:00:00Z",
            "status": "RUNNING",
            "uptime": "1:23:45"
        },
        "heartbeatd": {
            "start_time": "2025-01-15T10:00:00Z",
            "status": "RUNNING",
            "uptime": "1:23:45"
        },
        "job_scheduler": {
            "start_time": "2025-01-15T10:00:00Z",
            "status": "RUNNING",
            "uptime": "1:23:45"
        },
        "mdns_server": {
            "start_time": "2025-01-15T10:00:00Z",
            "status": "RUNNING",
            "uptime": "1:23:45"
        },
        "metrics_aggregator": {
            "start_time": "2025-01-15T10:00:00Z",
            "status": "RUNNING",
            "uptime": "1:23:45"
        },
        "node_wd": {
            "start_time": "2025-01-15T10:00:00Z",
            "status": "RUNNING",
            "uptime": "1:23:45"
        },
        "persistence_mgr": {
            "start_time": "2025-01-15T10:00:00Z",
            "status": "RUNNING",
            "uptime": "1:23:45"
        },
        "sentinel_distributor": {
            "start_time": "2025-01-15T10:00:00Z",
            "status": "RUNNING",
            "uptime": "1:23:45"
        },
        "stats_collector": {
            "start_time": "2025-01-15T10:00:00Z",
            "status": "RUNNING",
            "uptime": "1:23:45"
        }
    })
}
