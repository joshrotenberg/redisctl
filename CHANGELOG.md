# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### Comprehensive Async Operation Support (#175-#201)
- **Database Operations** - Added `--wait` flag support for all database operations including create, update, delete, import, backup, and migrate commands
- **Subscription Management** - Full async support for regular and fixed subscription operations with progress tracking
- **Active-Active Databases** - Complete async operation support for CRDB create, update, and delete operations
- **Network Connectivity** - Implemented `--wait` flags for:
  - VPC Peering operations (regular and Active-Active)
  - Private Service Connect (PSC) operations (regular and Active-Active)
  - Transit Gateway (TGW) operations including attach/detach (regular and Active-Active)
- **ACL Management** - Added async support for all 9 ACL commands:
  - Redis ACL Rules (create, update, delete)
  - ACL Roles (create, update, delete)
  - ACL Users (create, update, delete)
- **User Management** - Added `--wait` flag support for user deletion operations
- **Provider Accounts** - Full async support for cloud provider account operations (create, update, delete)
- **Fixed Plans** - Implemented async operations for fixed databases and fixed subscriptions

#### Wait Flag Options
- `--wait` - Wait for operation to complete with default 600s timeout
- `--wait-timeout <seconds>` - Configurable timeout duration for long operations
- `--wait-interval <seconds>` - Customizable polling interval (default: 10s)

### Changed

#### Code Organization Improvements
- **Parameter Grouping** - Introduced parameter structs to avoid `too_many_arguments` clippy warnings:
  - `AsyncOperationArgs` for async operation parameters
  - `ConnectivityOperationParams` for network connectivity operations
  - `CloudAccountOperationParams` for provider account operations
  - `AclOperationParams` for ACL management operations
- **Module Consolidation** - Reorganized connectivity commands under unified module structure
- **Centralized Async Handling** - All async operations now use the centralized `handle_async_response` function for consistency

### Fixed
- Fixed clippy warnings for functions with too many arguments
- Improved error handling for async operations with better context
- Enhanced progress indicators with animated spinners
- Fixed task ID extraction from various API response formats

### Documentation
- Comprehensive README update with all new async operation examples
- Updated CLAUDE.md with architectural changes and patterns
- Created FEATURES.md documenting all async operations in detail
- Added usage examples for all new `--wait` flag operations

## [0.2.0] - Previous Release

### Added
- Basic Cloud API support
- Enterprise API support
- Profile management system
- Multiple output formats (JSON, YAML, Table)
- JMESPath query filtering

### Changed
- Improved error handling
- Better configuration management

### Fixed
- Various bug fixes and improvements

## [0.1.0] - Initial Release

### Added
- Initial implementation
- Basic command structure
- Authentication support
- Raw API access