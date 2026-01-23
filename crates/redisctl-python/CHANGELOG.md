# Changelog

All notable changes to the redisctl Python package will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-01-23

### Added

- Initial release of Python bindings for Redis Cloud and Enterprise APIs
- `CloudClient` for Redis Cloud API
  - Constructor with API key, secret, optional base URL and timeout
  - `from_env()` factory method supporting multiple environment variable names
  - Async methods: `subscriptions()`, `subscription()`, `databases()`, `database()`
  - Sync methods: `subscriptions_sync()`, `subscription_sync()`, `databases_sync()`, `database_sync()`
  - Raw API access: `get()`, `post()`, `put()`, `delete()` (async and sync variants)
- `EnterpriseClient` for Redis Enterprise API
  - Constructor with base URL, username, password, optional insecure flag and timeout
  - `from_env()` factory method
  - Cluster methods: `cluster_info()`, `cluster_stats()`, `license()` (async and sync)
  - Database methods: `databases()`, `database()`, `database_stats()` (async and sync)
  - Node methods: `nodes()`, `node()`, `node_stats()` (async and sync)
  - User methods: `users()`, `user()` (async and sync)
  - Raw API access: `get()`, `post()`, `put()`, `delete()` (async and sync variants)
- `RedisCtlError` exception type for API errors
- Pre-built wheels for:
  - macOS (x86_64 and arm64)
  - Linux (x86_64 and aarch64, glibc)
  - Windows (x86_64)
- Python version support: 3.9, 3.10, 3.11, 3.12, 3.13
