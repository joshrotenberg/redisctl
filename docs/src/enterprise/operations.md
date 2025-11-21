# Enterprise Operations

Special tools for Redis Enterprise cluster management and support.

## Overview

These commands handle operational tasks beyond day-to-day database management:

- **[Support Package](./operations/support-package.md)** - Generate diagnostic packages for Redis Support
- **[License Management](./operations/license.md)** - View, update, and validate licenses
- **[Debug Info](./operations/debuginfo.md)** - Detailed cluster diagnostics
- **[Diagnostics](./operations/diagnostics.md)** - Run health checks
- **[Migrations](./operations/migration.md)** - Data migrations between databases

## Quick Reference

### Support Package

```bash
# Generate and download
redisctl enterprise support-package cluster

# With optimization (smaller size)
redisctl enterprise support-package cluster --optimize

# Upload directly to Redis Support
redisctl enterprise support-package cluster --upload
```

### License

```bash
# View current license
redisctl enterprise license get

# Update license
redisctl enterprise license update --file license.txt
```

### Debug Info

```bash
# Full cluster debug info
redisctl enterprise debuginfo cluster

# Specific node
redisctl enterprise debuginfo node 1
```

### Diagnostics

```bash
# Run all checks
redisctl enterprise diagnostics run

# List available checks
redisctl enterprise diagnostics list-checks
```

### Migrations

```bash
# Create migration
redisctl enterprise migration create --source 1 --target 2

# Start migration
redisctl enterprise migration start <ID>

# Check status
redisctl enterprise migration get <ID>
```
