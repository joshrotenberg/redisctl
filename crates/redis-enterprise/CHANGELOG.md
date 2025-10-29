# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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