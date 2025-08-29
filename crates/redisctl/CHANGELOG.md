# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2025-08-29

### Added
- **Interactive Setup Wizard** (`redisctl auth setup`) - Guided configuration for new users
- **Authentication Testing** (`redisctl auth test`) - Verify credentials before use
- **Configuration Management Commands**:
  - `redisctl config show` - Display current configuration and active profile
  - `redisctl config path` - Show configuration file location
  - `redisctl config validate` - Validate profile configurations
- **Environment Variable Standardization**:
  - `REDISCTL_PROFILE` - Set active profile
  - `REDIS_CLOUD_API_KEY` / `REDIS_CLOUD_API_SECRET` - Cloud credentials
  - `REDIS_ENTERPRISE_URL` / `REDIS_ENTERPRISE_USER` / `REDIS_ENTERPRISE_PASSWORD` - Enterprise credentials
  - `REDIS_ENTERPRISE_INSECURE` - Skip SSL verification

### Changed
- Authentication now works without any configuration file (environment variables only)
- Improved error messages with actionable suggestions
- Better SSL certificate handling for Enterprise deployments

### Fixed
- Docker image publishing workflow
- Configuration priority handling
- Enterprise authentication with self-signed certificates

### Removed
- Optional redis-cloud and redis-enterprise binaries (use unified redisctl binary)

## [0.1.1] - 2025-08-20

### Initial Release
- Unified CLI for Redis Cloud and Enterprise
- Smart command routing based on deployment type
- Profile-based configuration management
- Support for all Redis Cloud and Enterprise REST API endpoints