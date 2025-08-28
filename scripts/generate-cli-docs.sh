#!/bin/bash
# Generate comprehensive CLI documentation for redisctl

set -e

OUTPUT_DIR="docs/cli-reference"
BINARY="cargo run --bin redisctl --"

echo "Generating CLI documentation..."

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Function to generate markdown for a command
generate_command_doc() {
    local cmd="$1"
    local output_file="$2"
    local title="$3"
    
    echo "# $title" > "$output_file"
    echo "" >> "$output_file"
    echo '```' >> "$output_file"
    $BINARY $cmd --help 2>/dev/null >> "$output_file" || true
    echo '```' >> "$output_file"
    echo "" >> "$output_file"
}

# Generate main command documentation
echo "Generating main command reference..."
generate_command_doc "" "$OUTPUT_DIR/README.md" "redisctl Command Reference"

# Generate Cloud commands documentation
echo "Generating Cloud commands..."
mkdir -p "$OUTPUT_DIR/cloud"

generate_command_doc "cloud" "$OUTPUT_DIR/cloud/README.md" "Redis Cloud Commands"
generate_command_doc "cloud api" "$OUTPUT_DIR/cloud/api.md" "Cloud API Commands"
generate_command_doc "cloud subscription" "$OUTPUT_DIR/cloud/subscription.md" "Cloud Subscription Commands"
generate_command_doc "cloud database" "$OUTPUT_DIR/cloud/database.md" "Cloud Database Commands"
generate_command_doc "cloud account" "$OUTPUT_DIR/cloud/account.md" "Cloud Account Commands"
generate_command_doc "cloud user" "$OUTPUT_DIR/cloud/user.md" "Cloud User Commands"
generate_command_doc "cloud region" "$OUTPUT_DIR/cloud/region.md" "Cloud Region Commands"
generate_command_doc "cloud task" "$OUTPUT_DIR/cloud/task.md" "Cloud Task Commands"
generate_command_doc "cloud acl" "$OUTPUT_DIR/cloud/acl.md" "Cloud ACL Commands"
generate_command_doc "cloud peering" "$OUTPUT_DIR/cloud/peering.md" "Cloud VPC Peering Commands"
generate_command_doc "cloud transit-gateway" "$OUTPUT_DIR/cloud/transit-gateway.md" "Cloud Transit Gateway Commands"
generate_command_doc "cloud backup" "$OUTPUT_DIR/cloud/backup.md" "Cloud Backup Commands"
generate_command_doc "cloud crdb" "$OUTPUT_DIR/cloud/crdb.md" "Cloud Active-Active Commands"
generate_command_doc "cloud api-key" "$OUTPUT_DIR/cloud/api-key.md" "Cloud API Key Commands"
generate_command_doc "cloud metrics" "$OUTPUT_DIR/cloud/metrics.md" "Cloud Metrics Commands"
generate_command_doc "cloud logs" "$OUTPUT_DIR/cloud/logs.md" "Cloud Logs Commands"
generate_command_doc "cloud cloud-account" "$OUTPUT_DIR/cloud/cloud-account.md" "Cloud Provider Account Commands"
generate_command_doc "cloud sso" "$OUTPUT_DIR/cloud/sso.md" "Cloud SSO/SAML Commands"
generate_command_doc "cloud billing" "$OUTPUT_DIR/cloud/billing.md" "Cloud Billing Commands"

# Generate Enterprise commands documentation
echo "Generating Enterprise commands..."
mkdir -p "$OUTPUT_DIR/enterprise"

generate_command_doc "enterprise" "$OUTPUT_DIR/enterprise/README.md" "Redis Enterprise Commands"
generate_command_doc "enterprise api" "$OUTPUT_DIR/enterprise/api.md" "Enterprise API Commands"
generate_command_doc "enterprise cluster" "$OUTPUT_DIR/enterprise/cluster.md" "Enterprise Cluster Commands"
generate_command_doc "enterprise database" "$OUTPUT_DIR/enterprise/database.md" "Enterprise Database Commands"
generate_command_doc "enterprise node" "$OUTPUT_DIR/enterprise/node.md" "Enterprise Node Commands"
generate_command_doc "enterprise user" "$OUTPUT_DIR/enterprise/user.md" "Enterprise User Commands"
generate_command_doc "enterprise bootstrap" "$OUTPUT_DIR/enterprise/bootstrap.md" "Enterprise Bootstrap Commands"
generate_command_doc "enterprise module" "$OUTPUT_DIR/enterprise/module.md" "Enterprise Module Commands"
generate_command_doc "enterprise role" "$OUTPUT_DIR/enterprise/role.md" "Enterprise Role Commands"
generate_command_doc "enterprise license" "$OUTPUT_DIR/enterprise/license.md" "Enterprise License Commands"
generate_command_doc "enterprise alert" "$OUTPUT_DIR/enterprise/alert.md" "Enterprise Alert Commands"
generate_command_doc "enterprise crdb" "$OUTPUT_DIR/enterprise/crdb.md" "Enterprise Active-Active Commands"
generate_command_doc "enterprise actions" "$OUTPUT_DIR/enterprise/actions.md" "Enterprise Actions Commands"
generate_command_doc "enterprise stats" "$OUTPUT_DIR/enterprise/stats.md" "Enterprise Stats Commands"
generate_command_doc "enterprise logs" "$OUTPUT_DIR/enterprise/logs.md" "Enterprise Logs Commands"

# Generate Profile commands documentation
echo "Generating Profile commands..."
mkdir -p "$OUTPUT_DIR/profile"
generate_command_doc "profile" "$OUTPUT_DIR/profile/README.md" "Profile Management Commands"
generate_command_doc "profile list" "$OUTPUT_DIR/profile/list.md" "Profile List Command"
generate_command_doc "profile set" "$OUTPUT_DIR/profile/set.md" "Profile Set Command"
generate_command_doc "profile get" "$OUTPUT_DIR/profile/get.md" "Profile Get Command"
generate_command_doc "profile remove" "$OUTPUT_DIR/profile/remove.md" "Profile Remove Command"
generate_command_doc "profile default" "$OUTPUT_DIR/profile/default.md" "Profile Default Command"

# Generate Smart-routed commands documentation
echo "Generating smart-routed commands..."
mkdir -p "$OUTPUT_DIR/smart"
generate_command_doc "database" "$OUTPUT_DIR/smart/database.md" "Database Commands (Smart Routing)"
generate_command_doc "cluster" "$OUTPUT_DIR/smart/cluster.md" "Cluster Commands (Smart Routing)"
generate_command_doc "user" "$OUTPUT_DIR/smart/user.md" "User Commands (Smart Routing)"
generate_command_doc "account" "$OUTPUT_DIR/smart/account.md" "Account Commands (Smart Routing)"

echo "Documentation generation complete!"
echo "Output directory: $OUTPUT_DIR"

# Generate index file
cat > "$OUTPUT_DIR/index.md" << 'EOF'
# redisctl CLI Reference

Complete command reference for the Redis unified CLI tool.

## Command Categories

### [Main Commands](README.md)
The main redisctl commands and global options.

### [Cloud Commands](cloud/README.md)
Commands for managing Redis Cloud deployments:
- [API Access](cloud/api.md)
- [Subscriptions](cloud/subscription.md)
- [Databases](cloud/database.md)
- [Users](cloud/user.md)
- [ACLs](cloud/acl.md)
- [Backups](cloud/backup.md)
- [VPC Peering](cloud/peering.md)
- [Transit Gateway](cloud/transit-gateway.md)
- [Metrics](cloud/metrics.md)
- [Logs](cloud/logs.md)
- [SSO/SAML](cloud/sso.md)
- [Billing](cloud/billing.md)
- [And more...](cloud/)

### [Enterprise Commands](enterprise/README.md)
Commands for managing Redis Enterprise deployments:
- [API Access](enterprise/api.md)
- [Clusters](enterprise/cluster.md)
- [Databases](enterprise/database.md)
- [Nodes](enterprise/node.md)
- [Users](enterprise/user.md)
- [Bootstrap](enterprise/bootstrap.md)
- [Modules](enterprise/module.md)
- [Roles](enterprise/role.md)
- [License](enterprise/license.md)
- [Alerts](enterprise/alert.md)
- [Stats](enterprise/stats.md)
- [Logs](enterprise/logs.md)
- [And more...](enterprise/)

### [Profile Management](profile/README.md)
Commands for managing configuration profiles:
- [List Profiles](profile/list.md)
- [Set Profile](profile/set.md)
- [Get Profile](profile/get.md)
- [Remove Profile](profile/remove.md)
- [Set Default](profile/default.md)

### [Smart-Routed Commands](smart/)
Commands that automatically detect deployment type:
- [Database Operations](smart/database.md)
- [Cluster Operations](smart/cluster.md)
- [User Operations](smart/user.md)
- [Account Operations](smart/account.md)

## Environment Variables

### Redis Cloud
- `REDIS_CLOUD_API_KEY` - API key for authentication
- `REDIS_CLOUD_API_SECRET` - API secret for authentication
- `REDIS_CLOUD_API_URL` - Custom API URL (optional)

### Redis Enterprise
- `REDIS_ENTERPRISE_URL` - Cluster API URL
- `REDIS_ENTERPRISE_USER` - Username for authentication
- `REDIS_ENTERPRISE_PASSWORD` - Password for authentication
- `REDIS_ENTERPRISE_INSECURE` - Allow insecure TLS (true/false)

### General
- `REDISCTL_PROFILE` - Default profile to use
- `RUST_LOG` - Logging level (error, warn, info, debug, trace)

## Output Formats

All commands support multiple output formats via the `-o/--output` flag:
- `json` - JSON format (default)
- `yaml` - YAML format
- `table` - Human-readable table format

## Query Filtering

Use JMESPath queries with the `-q/--query` flag to filter output:
```bash
redisctl database list -q "[?status=='active'].name"
redisctl user list -q "[].{email:email,role:role}"
```

## Examples

See the [examples directory](../../examples/) for common usage patterns.
EOF

echo "Index file generated at $OUTPUT_DIR/index.md"