# Smart Commands

Smart commands automatically detect which backend (Cloud or Enterprise) to use based on your configuration.

## How It Works

Smart commands examine your profile or environment variables to determine the deployment type:

1. **Explicit profile**: Uses the deployment type from the profile
2. **Environment variables**: Detects based on which variables are set
3. **Ambiguous**: Prompts you to specify with `--deployment` flag

## Available Smart Commands

### Database Commands

Works with both Cloud and Enterprise databases:

```bash
# Automatically routes to correct backend
redisctl database list
redisctl database get <ID>
redisctl database create --data @db.json
```

### User Commands

```bash
# Works for both platforms
redisctl user list
redisctl user get <ID>
redisctl user create --data @user.json
```

### Cluster Commands

```bash
# Enterprise: cluster info
# Cloud: subscription info
redisctl cluster info
```

## Specifying Deployment Type

When detection is ambiguous:

```bash
# Force Cloud backend
redisctl --deployment cloud database list

# Force Enterprise backend
redisctl --deployment enterprise database list
```

## Profile-Based Routing

```bash
# Uses cloud profile
redisctl --profile prod-cloud database list

# Uses enterprise profile  
redisctl --profile prod-enterprise database list
```

## Examples

### Cross-Platform Database Management

```bash
# List databases across both platforms
echo "Cloud databases:"
redisctl --profile cloud-prod database list

echo "Enterprise databases:"
redisctl --profile enterprise-prod database list
```

### Unified Scripts

```bash
#!/bin/bash
# Works with either backend based on environment

# Set profile based on environment
if [ "$ENV" = "cloud" ]; then
  export REDISCTL_PROFILE="cloud-prod"
else
  export REDISCTL_PROFILE="enterprise-prod"
fi

# Commands work regardless of backend
redisctl database list
redisctl user list
```

## Best Practices

1. **Use profiles** for clear deployment targeting
2. **Set default profile** for your primary environment
3. **Be explicit** in scripts with `--deployment` flag
4. **Test routing** with `--dry-run` flag (if available)