# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.7.4](https://github.com/joshrotenberg/redisctl/compare/redis-cloud-v0.7.3...redis-cloud-v0.7.4) - 2025-12-13

### Other

- remove outdated implementation tracking file ([#492](https://github.com/joshrotenberg/redisctl/pull/492))

## [0.7.3](https://github.com/joshrotenberg/redisctl/compare/redis-cloud-v0.7.2...redis-cloud-v0.7.3) - 2025-12-09

### Added

- *(cloud)* add delete endpoint for PrivateLink ([#487](https://github.com/joshrotenberg/redisctl/pull/487))
- *(cloud)* add upgrade endpoints for Essentials databases ([#488](https://github.com/joshrotenberg/redisctl/pull/488))

## [0.7.2](https://github.com/joshrotenberg/redisctl/compare/redis-cloud-v0.7.1...redis-cloud-v0.7.2) - 2025-12-09

### Added

- *(cloud)* add task list, database flush, and available-versions commands ([#477](https://github.com/joshrotenberg/redisctl/pull/477))
- *(cloud)* add cost-report API support (Beta) ([#479](https://github.com/joshrotenberg/redisctl/pull/479))
- add user agent header to HTTP requests ([#473](https://github.com/joshrotenberg/redisctl/pull/473))
- *(redis-cloud)* add tracing instrumentation to API client ([#452](https://github.com/joshrotenberg/redisctl/pull/452))
- Add optional Tower service integration to API clients ([#447](https://github.com/joshrotenberg/redisctl/pull/447))

### Fixed

- *(release)* improve Homebrew formula auto-update ([#433](https://github.com/joshrotenberg/redisctl/pull/433))

## [0.7.1](https://github.com/joshrotenberg/redisctl/compare/redis-cloud-v0.7.0...redis-cloud-v0.7.1) - 2025-10-29

### Added

- *(redis-cloud)* add AWS PrivateLink connectivity support ([#406](https://github.com/joshrotenberg/redisctl/pull/406))

### Other

- rewrite README for presentation readiness ([#408](https://github.com/joshrotenberg/redisctl/pull/408))
- implement fixture-based validation for Enterprise API ([#352](https://github.com/joshrotenberg/redisctl/pull/352)) ([#398](https://github.com/joshrotenberg/redisctl/pull/398))

## [0.7.0](https://github.com/joshrotenberg/redisctl/compare/redis-cloud-v0.6.2...redis-cloud-v0.7.0) - 2025-10-07

### Added

- *(redis-cloud)* medium priority API coverage improvements
- *(redis-cloud)* high priority API coverage improvements
- *(redis-cloud)* expand additional response types with list fields
- *(redis-cloud)* expose all known API fields as first-class struct members

### Fixed

- add OpenAPI spec fixture for CI

### Other

- add support package optimization and upload documentation
- add Homebrew installation instructions

## [0.6.1](https://github.com/joshrotenberg/redisctl/compare/redis-cloud-v0.6.0...redis-cloud-v0.6.1) - 2025-09-16

### Added

- add serde_path_to_error for better deserialization error messages ([#349](https://github.com/joshrotenberg/redisctl/pull/349))