# Installation

## Docker Setup (Recommended)

The easiest way to get started is using Docker, which provides a complete Redis Enterprise test environment.

### Prerequisites

- Docker and Docker Compose
- Make (optional, for convenience commands)

### Quick Start

```bash
# Clone the repository
git clone <repository-url>
cd redis-enterprise-rs

# Start Redis Enterprise and initialize cluster
make docker-up

# Access interactive CLI
make docker-cli

# Clean up when done
make docker-down
```

### Manual Docker Commands

```bash
# Start basic environment
docker compose up -d

# Interactive CLI session
docker compose --profile cli up cli

# Run example workflows
docker compose --profile examples up

# Monitor cluster continuously
docker compose --profile monitor up
```

## Manual Installation

### From Source

```bash
# Build the CLI
cargo build --release

# The binary will be at target/release/redis-enterprise
./target/release/redis-enterprise --help
```

### Prerequisites for Manual Build

- Rust 1.82+
- OpenSSL development headers
- pkg-config

#### On macOS
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# OpenSSL is usually already available
```

#### On Ubuntu/Debian
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install dependencies
sudo apt update
sudo apt install -y pkg-config libssl-dev
```

#### On CentOS/RHEL/Fedora
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install dependencies
sudo dnf install -y pkg-config openssl-devel
# or on older systems: sudo yum install -y pkg-config openssl-devel
```

### Verify Installation

```bash
# Check version
redis-enterprise --version

# Test connectivity (requires running cluster)
redis-enterprise cluster info --help
```

## Development Setup

### Full Development Environment

```bash
# Set up complete dev environment
make dev

# This will:
# 1. Build the CLI
# 2. Run tests
# 3. Start Docker environment
```

### Development Commands

```bash
# Run tests
make test

# Run linter
make clippy

# Format code
make fmt

# Clean build artifacts
make clean
```

## Next Steps

- [Quick Start Guide](quickstart.md) - Get started with basic commands
- [Authentication](authentication.md) - Configure credentials
- [CLI Commands](../cli/commands.md) - Complete command reference