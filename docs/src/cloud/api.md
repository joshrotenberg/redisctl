# Cloud API Layer

Direct REST access to the Redis Cloud API.

## Overview

The API layer provides raw access to all Redis Cloud REST endpoints. Use this for:
- Scripting and automation
- Accessing endpoints not yet wrapped in human commands
- CI/CD pipelines

## Authentication

Uses `x-api-key` and `x-api-secret-key` headers. Configure via profile or environment variables.

## Usage

```bash
# GET request
redisctl api cloud get /subscriptions

# POST request with data
redisctl api cloud post /subscriptions -d '{"name": "test", ...}'

# PUT request
redisctl api cloud put /subscriptions/123 -d '{"name": "updated"}'

# DELETE request
redisctl api cloud delete /subscriptions/123/databases/456
```

## Common Endpoints

- `/account` - Account information
- `/subscriptions` - Subscription management
- `/subscriptions/{id}/databases` - Database operations
- `/acl/users` - User management
- `/acl/roles` - Role management
- `/tasks` - Async task status

TODO: Move detailed content from common-features/raw-api.md
