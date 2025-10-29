# 3. Installation & Setup

## Installation

**Homebrew (macOS/Linux):**
```bash
brew tap joshrotenberg/brew
brew install redisctl
```

**GitHub Releases:**  
Download pre-built binaries: [Latest Release](https://github.com/joshrotenberg/redisctl/releases)

**Docker:**
```bash
docker run ghcr.io/joshrotenberg/redisctl:latest --help
```

**Cargo (from source):**
```bash
cargo install redisctl
```

## Profile Setup

Profiles let you manage multiple clusters (dev, staging, prod) and store credentials securely.

**Cloud Profile:**
```bash
redisctl profile set prod --deployment cloud \
  --api-key "your-api-key" \
  --api-secret "your-secret-key"
```

**Enterprise Profile:**
```bash
redisctl profile set local --deployment enterprise \
  --url https://localhost:9443 \
  --username admin@redis.local \
  --password Redis123! \
  --insecure
```

**List Profiles:**
```bash
redisctl profile list
```

**Use a Profile:**
```bash
# With flag
redisctl -p prod cloud subscription list

# With environment variable
export REDISCTL_PROFILE=prod
redisctl cloud subscription list
```

## Credential Storage

**Plaintext** (default) - Stored in config.toml

**Environment Variables:**
```bash
export REDIS_CLOUD_API_KEY="your-key"
export REDIS_CLOUD_SECRET_KEY="your-secret"
```

**OS Keyring** (requires `secure-storage` feature):
```bash
redisctl profile set prod --deployment cloud \
  --api-key "$KEY" \
  --api-secret "$SECRET" \
  --use-keyring
```

## Credential Priority

**IMPORTANT:** Credentials are used in this order (highest to lowest):

1. **Environment Variables** ← Will override profiles!
2. **Profile Configuration**
3. **CLI Flags**

If you have `REDIS_CLOUD_API_KEY` set in your environment, it will be used instead of your profile credentials. Use `unset REDIS_CLOUD_API_KEY` to use profile credentials.

## Docker Compose Demo

Try the complete Enterprise demo:

```bash
git clone https://github.com/joshrotenberg/redisctl
cd redisctl
docker compose up -d

# Watch initialization
docker compose logs -f redis-enterprise-init
```

See `docker-compose.yml` for annotated examples of every command type.

---

**Previous:** [2. Enter redisctl](./02-solution.md)  
**Next:** [4. Raw API Access](./04-raw-api.md)
