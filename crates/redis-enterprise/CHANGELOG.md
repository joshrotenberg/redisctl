# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.7.4](https://github.com/redis-developer/redisctl/compare/redis-enterprise-v0.7.3...redis-enterprise-v0.7.4) - 2026-01-23

### Added

- Add Python bindings via PyO3 ([#578](https://github.com/redis-developer/redisctl/pull/578))

### Fixed

- use local README.md for crates to fix sdist build ([#580](https://github.com/redis-developer/redisctl/pull/580))

## [0.7.3](https://github.com/redis-developer/redisctl/compare/redis-enterprise-v0.7.2...redis-enterprise-v0.7.3) - 2026-01-12

### Added

- add MCP server for AI integration ([#531](https://github.com/redis-developer/redisctl/pull/531))

## [0.7.2](https://github.com/redis-developer/redisctl/compare/redis-enterprise-v0.7.1...redis-enterprise-v0.7.2) - 2025-12-17

### Fixed

- support JMESPath backtick string literals and improve module upload error ([#511](https://github.com/redis-developer/redisctl/pull/511))

### Other

- update documentation URLs to new hosting location ([#509](https://github.com/redis-developer/redisctl/pull/509))

## [0.7.1](https://github.com/redis-developer/redisctl/compare/redis-enterprise-v0.7.0...redis-enterprise-v0.7.1) - 2025-12-16

### Other

- switch to GHCR for Docker images ([#500](https://github.com/redis-developer/redisctl/pull/500))
- update repository URLs for redis-developer org ([#499](https://github.com/redis-developer/redisctl/pull/499))

## [0.7.0](https://github.com/joshrotenberg/redisctl/compare/redis-enterprise-v0.6.4...redis-enterprise-v0.7.0) - 2025-12-09

### Added

- add user agent header to HTTP requests ([#473](https://github.com/joshrotenberg/redisctl/pull/473))
- *(enterprise)* add database watch command for real-time status monitoring ([#458](https://github.com/joshrotenberg/redisctl/pull/458))
- *(redis-enterprise)* add stats streaming with --follow flag ([#455](https://github.com/joshrotenberg/redisctl/pull/455))
- Add optional Tower service integration to API clients ([#447](https://github.com/joshrotenberg/redisctl/pull/447))
- add database upgrade command for Redis version upgrades ([#442](https://github.com/joshrotenberg/redisctl/pull/442))

### Fixed

- *(redis-enterprise)* remove non-existent database action methods ([#443](https://github.com/joshrotenberg/redisctl/pull/443))
- *(release)* improve Homebrew formula auto-update ([#433](https://github.com/joshrotenberg/redisctl/pull/433))

## [0.6.4](https://github.com/joshrotenberg/redisctl/compare/redis-enterprise-v0.6.3...redis-enterprise-v0.6.4) - 2025-10-29

### Added

- Add streaming logs support with --follow flag (Issue #70) ([#404](https://github.com/joshrotenberg/redisctl/pull/404))

### Other

- add comprehensive presentation outline and rladmin comparison ([#415](https://github.com/joshrotenberg/redisctl/pull/415))
- rewrite README for presentation readiness ([#408](https://github.com/joshrotenberg/redisctl/pull/408))
- implement fixture-based validation for Enterprise API ([#352](https://github.com/joshrotenberg/redisctl/pull/352)) ([#398](https://github.com/joshrotenberg/redisctl/pull/398))

## [0.6.3](https://github.com/joshrotenberg/redisctl/compare/redis-enterprise-v0.6.2...redis-enterprise-v0.6.3) - 2025-10-07

### Other

- add support package optimization and upload documentation
- add Homebrew installation instructions

## [0.6.1](https://github.com/joshrotenberg/redisctl/compare/redis-enterprise-v0.6.0...redis-enterprise-v0.6.1) - 2025-09-16

### Added

- add serde_path_to_error for better deserialization error messages ([#349](https://github.com/joshrotenberg/redisctl/pull/349))

### Fixed

- *(redis-enterprise)* correct max_aof_file_size type from String to u64 ([#351](https://github.com/joshrotenberg/redisctl/pull/351))
- *(redis-enterprise)* correct master_persistence type from String to bool ([#348](https://github.com/joshrotenberg/redisctl/pull/348))