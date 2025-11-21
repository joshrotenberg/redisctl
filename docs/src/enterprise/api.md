# Enterprise API Layer

Direct REST access to the Redis Enterprise API.

## Overview

The API layer provides raw access to all Redis Enterprise REST endpoints. Use this for:
- Scripting and automation
- Accessing endpoints not yet wrapped in human commands
- CI/CD pipelines

## Authentication

Uses Basic auth (username/password). Configure via profile or environment variables.

## Usage

```bash
# GET request
redisctl api enterprise get /v1/cluster

# POST request with data
redisctl api enterprise post /v1/bdbs -d '{"name": "test", ...}'

# PUT request
redisctl api enterprise put /v1/bdbs/1 -d '{"name": "updated"}'

# DELETE request
redisctl api enterprise delete /v1/bdbs/1
```

## Common Endpoints

- `/v1/cluster` - Cluster information
- `/v1/nodes` - Node management
- `/v1/bdbs` - Database operations
- `/v1/users` - User management
- `/v1/roles` - Role management

TODO: Move detailed content from enterprise/api-access.md
