# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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