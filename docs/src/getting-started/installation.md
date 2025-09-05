# Installation

## From Source

If you have Rust installed:

```bash
git clone https://github.com/joshrotenberg/redisctl.git
cd redisctl
cargo install --path crates/redisctl
```

## Pre-built Binaries

Coming soon - binaries will be available for:
- Linux (x86_64, ARM64)
- macOS (Intel, Apple Silicon)
- Windows

## Docker

```bash
docker pull redisctl/redisctl:latest
docker run redisctl/redisctl --help
```

## Verify Installation

```bash
redisctl --version
```

## Next Steps

- [Configuration](./configuration.md) - Set up your credentials