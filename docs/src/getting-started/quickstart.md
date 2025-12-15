# Quick Start

Get running in 60 seconds with Docker.

## Try It Now

### Redis Cloud

```bash
# Set your credentials
export REDIS_CLOUD_API_KEY="your-api-key"
export REDIS_CLOUD_SECRET_KEY="your-secret-key"

# Run a command
docker run --rm \
  -e REDIS_CLOUD_API_KEY \
  -e REDIS_CLOUD_SECRET_KEY \
  redis-developer/redisctl cloud subscription list
```

### Redis Enterprise

```bash
# Set your credentials
export REDIS_ENTERPRISE_URL="https://cluster.example.com:9443"
export REDIS_ENTERPRISE_USER="admin@cluster.local"
export REDIS_ENTERPRISE_PASSWORD="your-password"
export REDIS_ENTERPRISE_INSECURE="true"  # for self-signed certs

# Run a command
docker run --rm \
  -e REDIS_ENTERPRISE_URL \
  -e REDIS_ENTERPRISE_USER \
  -e REDIS_ENTERPRISE_PASSWORD \
  -e REDIS_ENTERPRISE_INSECURE \
  redis-developer/redisctl enterprise cluster get
```

That's it! You just ran your first redisctl command.

## Next Steps

Choose your path:

- **New to redisctl?** - Start with the [Walkthrough](../walkthrough/overview.md) to understand the 3-tier model
- **Redis Cloud Users** - Jump to [Cloud Overview](../cloud/overview.md) for Cloud-specific commands
- **Redis Enterprise Users** - Jump to [Enterprise Overview](../enterprise/overview.md) for Enterprise-specific commands
- **Ready to install?** - See Installation for Homebrew, binaries, and more
- **Developers** - Check out [Libraries](../developer/libraries.md) for using redisctl as a Rust library
