# Enterprise Nodes

Manage nodes in Redis Enterprise clusters.

## Commands

### List Nodes

```bash
redisctl enterprise node list [OPTIONS]
```

**Examples:**

```bash
# List all nodes
redisctl enterprise node list

# Table format
redisctl enterprise node list -o table

# Get node IPs and status
redisctl enterprise node list -q "[].{id:uid,addr:addr,status:status}"
```

### Get Node

```bash
redisctl enterprise node get <ID> [OPTIONS]
```

**Examples:**

```bash
# Get node details
redisctl enterprise node get 1

# Get specific fields
redisctl enterprise node get 1 -q "{addr:addr,cores:cores,memory:total_memory}"
```

### Update Node

```bash
redisctl enterprise node update <ID> --data <JSON>
```

**Examples:**

```bash
# Update node settings
redisctl enterprise node update 1 --data '{"addr": "10.0.0.2"}'

# Set rack ID
redisctl enterprise node update 1 --data '{"rack_id": "rack-1"}'
```

### Remove Node

```bash
redisctl enterprise node remove <ID> [OPTIONS]
```

Remove a node from the cluster.

**Examples:**

```bash
# Remove node
redisctl enterprise node remove 3

# Force remove (skip checks)
redisctl enterprise node remove 3 --force
```

## Node Actions

### Maintenance Mode

```bash
# Enter maintenance mode
redisctl enterprise node action <ID> maintenance-on

# Exit maintenance mode
redisctl enterprise node action <ID> maintenance-off
```

### Check Node Status

```bash
redisctl enterprise node check <ID>
```

## Node Stats

### Get Node Statistics

```bash
redisctl enterprise stats node <ID> [OPTIONS]
```

**Examples:**

```bash
# Current stats
redisctl enterprise stats node 1

# Stream continuously
redisctl enterprise stats node 1 --follow

# Get CPU and memory
redisctl enterprise stats node 1 -q "{cpu:cpu_user,memory:free_memory}"
```

## Joining Nodes

### Join Node to Cluster

```bash
redisctl enterprise cluster join --data <JSON>
```

**Example:**

```bash
redisctl enterprise cluster join --data '{
  "addr": "10.0.0.3",
  "username": "admin@cluster.local",
  "password": "password"
}'
```

## Common Patterns

### Check All Node Health

```bash
#!/bin/bash
for node in $(redisctl enterprise node list -q '[].uid' | jq -r '.[]'); do
  STATUS=$(redisctl enterprise node get $node -q 'status')
  echo "Node $node: $STATUS"
done
```

### Get Total Cluster Resources

```bash
redisctl enterprise node list -q '{
  total_memory: sum([].total_memory),
  total_cores: sum([].cores)
}'
```

### Monitor Node Resources

```bash
# Watch node stats
watch -n 5 "redisctl enterprise stats node 1 -q '{cpu:cpu_user,mem:free_memory}'"
```

### Drain Node Before Removal

```bash
# Put in maintenance mode
redisctl enterprise node action 3 maintenance-on

# Wait for shards to migrate
sleep 60

# Remove node
redisctl enterprise node remove 3
```

## Troubleshooting

### "Node not reachable"
- Check network connectivity
- Verify firewall rules allow cluster ports
- Check node services are running

### "Cannot remove node"
- Ensure no master shards on node
- Put node in maintenance mode first
- Check cluster has enough resources

### "Node out of memory"
- Add more nodes to cluster
- Reduce database memory allocations
- Check for memory leaks in applications

## API Reference

REST endpoints:
- `GET /v1/nodes` - List nodes
- `GET /v1/nodes/{id}` - Get node
- `PUT /v1/nodes/{id}` - Update node
- `DELETE /v1/nodes/{id}` - Remove node
- `POST /v1/nodes/{id}/actions/{action}` - Node action

For direct API access: `redisctl api enterprise get /v1/nodes`
