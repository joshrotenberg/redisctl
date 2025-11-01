# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.7.0](https://github.com/joshrotenberg/redisctl/compare/redisctl-v0.6.6...redisctl-v0.7.0) - 2025-11-01

### Added

- [**breaking**] improve CLI help text accuracy and add comprehensive test coverage ([#444](https://github.com/joshrotenberg/redisctl/pull/444))
- add payment-method commands to CLI ([#439](https://github.com/joshrotenberg/redisctl/pull/439))
- make --config-file take precedence over environment variables ([#438](https://github.com/joshrotenberg/redisctl/pull/438))

### Fixed

- *(release)* improve Homebrew formula auto-update ([#433](https://github.com/joshrotenberg/redisctl/pull/433))

### Other

- add comprehensive CLI tests with assert_cmd ([#435](https://github.com/joshrotenberg/redisctl/pull/435))

## [0.6.6](https://github.com/joshrotenberg/redisctl/compare/redisctl-v0.6.5...redisctl-v0.6.6) - 2025-10-29

### Added

- add --config-file flag for alternate configuration file ([#430](https://github.com/joshrotenberg/redisctl/pull/430))
- *(cli)* add AWS PrivateLink human-friendly commands ([#407](https://github.com/joshrotenberg/redisctl/pull/407))
- Add streaming logs support with --follow flag (Issue #70) ([#404](https://github.com/joshrotenberg/redisctl/pull/404))
- Add improved error messages with actionable suggestions (Issue #259) ([#401](https://github.com/joshrotenberg/redisctl/pull/401))

### Fixed

- handle processing-error state in async operations ([#431](https://github.com/joshrotenberg/redisctl/pull/431))

### Other

- add comprehensive presentation outline and rladmin comparison ([#415](https://github.com/joshrotenberg/redisctl/pull/415))
- Extract config/profile management to library crate ([#410](https://github.com/joshrotenberg/redisctl/pull/410))
- rewrite README for presentation readiness ([#408](https://github.com/joshrotenberg/redisctl/pull/408))
- extract profile commands from main.rs to dedicated module ([#403](https://github.com/joshrotenberg/redisctl/pull/403))

## [0.6.5](https://github.com/joshrotenberg/redisctl/compare/redisctl-v0.6.4...redisctl-v0.6.5) - 2025-10-07

### Added

- *(enterprise)* implement local node commands and expose shard commands

### Fixed

- add JSON output support to profile and version commands ([#394](https://github.com/joshrotenberg/redisctl/pull/394))

## [0.6.4](https://github.com/joshrotenberg/redisctl/compare/redisctl-v0.6.3...redisctl-v0.6.4) - 2025-10-07

### Fixed

- remove unused variable warning on Windows builds

## [0.6.3](https://github.com/joshrotenberg/redisctl/compare/redisctl-v0.6.2...redisctl-v0.6.3) - 2025-10-07

### Added

- add comprehensive Files.com API key management with secure storage
- add support package upload feature with files-sdk 0.3.1
- add support package optimization

### Fixed

- *(secure-storage)* enable platform-native keyring backends

### Other

- add support package optimization and upload documentation
- Merge pull request #371 from joshrotenberg/feat/homebrew-auto-update
- add Homebrew installation instructions

## [0.6.1](https://github.com/joshrotenberg/redisctl/compare/redisctl-v0.6.0...redisctl-v0.6.1) - 2025-09-16

### Fixed

- improve profile resolution for explicit cloud/enterprise commands ([#353](https://github.com/joshrotenberg/redisctl/pull/353))