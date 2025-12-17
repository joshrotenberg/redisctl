# Enterprise Workflows

Multi-step operations that orchestrate multiple API calls with automatic handling.

## Available Workflows

### init-cluster

Initialize a new Redis Enterprise cluster from scratch.

```bash
redisctl enterprise workflow init-cluster \
  --cluster-name production \
  --username admin@cluster.local \
  --password SecurePass123!
```

**What it does:**
1. Bootstraps the cluster
2. Sets up authentication
3. Applies initial configuration
4. Optionally creates default database
5. Returns cluster details

**Options:**

| Option | Description |
|--------|-------------|
| `--cluster-name` | Cluster name |
| `--username` | Admin username |
| `--password` | Admin password |
| `--license-file` | Path to license file |
| `--create-database` | Create initial database |
| `--database-name` | Name for initial database |

**Full Example:**

```bash
redisctl enterprise workflow init-cluster \
  --cluster-name production \
  --username admin@cluster.local \
  --password SecurePass123! \
  --license-file license.txt \
  --create-database \
  --database-name default-cache
```

## When to Use Workflows

Use workflows when you need to:
- Set up a new cluster from scratch
- Perform multiple related operations
- Have automatic error handling

Use individual commands when you need:
- Fine-grained control
- Custom sequencing
- Partial operations

## Custom Workflows

For operations not covered by built-in workflows, script them:

### Add Node and Rebalance

```bash
#!/bin/bash
set -e

NEW_NODE="10.0.0.4"

# Join node to cluster
redisctl enterprise cluster join --data "{
  \"addr\": \"$NEW_NODE\",
  \"username\": \"admin@cluster.local\",
  \"password\": \"$PASSWORD\"
}"

echo "Node joined, waiting for sync..."
sleep 30

# Verify node is active
STATUS=$(redisctl enterprise node list -q "[?addr[0]=='$NEW_NODE'].status | [0]")
if [ "$STATUS" != "active" ]; then
  echo "Node not active: $STATUS"
  exit 1
fi

echo "Node $NEW_NODE successfully added"
```

### Database Migration

```bash
#!/bin/bash
set -e

SOURCE_DB=$1
TARGET_MEMORY=$2

# Create new database
NEW_DB=$(redisctl enterprise database create \
  --name "migrated-$(date +%s)" \
  --memory-size $TARGET_MEMORY \
  -q 'uid')

echo "Created database: $NEW_DB"

# Export from source
redisctl enterprise database export $SOURCE_DB --data '{
  "export_type": "rdb",
  "path": "/tmp/export.rdb"
}'

# Import to target
redisctl enterprise database import $NEW_DB --data '{
  "source_type": "rdb_file",
  "source_path": "/tmp/export.rdb"
}'

echo "Migration complete: $SOURCE_DB -> $NEW_DB"
```

### Rolling Restart

```bash
#!/bin/bash
set -e

# Get all nodes
for node in $(redisctl enterprise node list -q '[].uid' --raw); do
  echo "Restarting node $node..."
  
  # Put in maintenance
  redisctl enterprise node action $node maintenance-on
  sleep 30
  
  # Restart services (via SSH or other method)
  # ssh node$node "supervisorctl restart all"
  
  # Exit maintenance
  redisctl enterprise node action $node maintenance-off
  sleep 30
  
  # Verify healthy
  STATUS=$(redisctl enterprise node get $node -q 'status')
  if [ "$STATUS" != "active" ]; then
    echo "Node $node not healthy after restart"
    exit 1
  fi
  
  echo "Node $node restarted successfully"
done

echo "Rolling restart complete"
```

## Error Handling

Workflows handle errors gracefully:

```bash
# If workflow fails midway
$ redisctl enterprise workflow init-cluster --cluster-name prod ...
Error: Failed at step 3: License invalid

# Check what was created
$ redisctl enterprise cluster get
# Cluster exists but not fully configured

# Resume manually or clean up
```

## Comparison: Workflow vs Manual

**With workflow:**
```bash
redisctl enterprise workflow init-cluster \
  --cluster-name prod \
  --username admin@cluster.local \
  --password Pass123!
```

**Manual equivalent:**
```bash
# 1. Bootstrap cluster
redisctl api enterprise post /v1/bootstrap/create_cluster -d '{...}'

# 2. Wait for bootstrap
while [ "$(redisctl api enterprise get /v1/bootstrap -q 'status')" != "completed" ]; do
  sleep 5
done

# 3. Set credentials
redisctl api enterprise put /v1/cluster -d '{...}'

# 4. Apply license
redisctl enterprise license update --data @license.json

# 5. Create initial database
redisctl enterprise database create --name default ...
```

The workflow handles all sequencing, waiting, and error checking automatically.
