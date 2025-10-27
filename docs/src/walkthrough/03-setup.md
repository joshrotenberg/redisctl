# 3. Installation & Setup

## Installation

### Homebrew (macOS/Linux)

```bash
brew install joshrotenberg/brew/redisctl
```

### GitHub Releases

Download pre-built binaries for your platform:
- macOS (Intel & Apple Silicon)
- Linux (x86_64)
- Windows (x86_64)

[→ Latest Release](https://github.com/joshrotenberg/redisctl/releases)

### Docker

```bash
docker run ghcr.io/joshrotenberg/redisctl:latest --help
```

### Cargo (from source)

```bash
cargo install redisctl
```

## Profile Setup

### Why Profiles?

Profiles let you:
- Manage multiple clusters (dev, staging, prod)
- Store credentials securely (OS keyring)
- Override with env vars or CLI flags

### Cloud Profile

```bash
redisctl profile set prod-cloud \
  --deployment-type cloud \
  --api-key "your-api-key" \
  --api-secret "your-secret-key" \
  --use-keyring
```

### Enterprise Profile

```bash
redisctl profile set prod-cluster \
  --deployment-type enterprise \
  --url "https://cluster.example.com:9443" \
  --username "admin@cluster.local" \
  --use-keyring
# Password will be prompted and stored securely
```

### Verify Setup

```bash
# List profiles
redisctl profile list

# Test connection
redisctl api cloud get /           # Cloud
redisctl api enterprise get /v1/cluster  # Enterprise
```

## Secure Credential Storage

With `--use-keyring`:
- **macOS**: Keychain
- **Windows**: Credential Manager
- **Linux**: Secret Service

No credentials in config files or shell history!

## Override Hierarchy

```
CLI flags > Environment variables > Profile settings
```

Example:
```bash
# Override profile URL for one command
redisctl --url https://other-cluster:9443 enterprise cluster get
```

---

**← Previous:** [2. Enter redisctl](./02-solution.md)  
**Next →** [4. Raw API Layer](./04-raw-api.md)

See [Configuration Guide](../getting-started/configuration.md) for details
