# Enterprise Quick Examples

Examples showing the three-tier model for Redis Enterprise.

## API Layer

Direct REST access:

```bash
# Get cluster info
redisctl api enterprise get /v1/cluster

# List databases
redisctl api enterprise get /v1/bdbs

# Get node details
redisctl api enterprise get /v1/nodes/1
```

## Human Commands

Type-safe operations:

```bash
# Get cluster status
redisctl enterprise cluster get

# List databases
redisctl enterprise database list

# Create a database
redisctl enterprise database create --name mydb --memory 1024

# Stream stats continuously
redisctl enterprise stats cluster --follow
```

## Workflows

Multi-step operations:

```bash
# Initialize a new cluster
redisctl enterprise workflow init-cluster --name mycluster --nodes 3
```

## Operations

Special Enterprise features:

```bash
# Generate support package
redisctl enterprise support-package cluster --upload

# Check license
redisctl enterprise license get
```
