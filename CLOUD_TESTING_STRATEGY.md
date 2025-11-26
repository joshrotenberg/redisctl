# Cloud Testing Strategy

## Overview

Testing against live Redis Cloud API requires careful resource management to avoid:
- Leaving test resources running (costs money)
- Creating orphaned resources
- Hitting rate limits
- Interfering with production resources

## Pre-Testing Setup

### 1. Credentials
```bash
export REDIS_CLOUD_API_KEY="your-api-key"
export REDIS_CLOUD_SECRET_KEY="your-secret-key"
```

### 2. Test Profile
```bash
redisctl profile set cloud-test \
  --deployment cloud \
  --api-key "$REDIS_CLOUD_API_KEY" \
  --api-secret "$REDIS_CLOUD_SECRET_KEY"
```

### 3. Baseline Inventory
Before starting, record current resources:
```bash
redisctl --profile cloud-test cloud subscription list -o json > pre-test-subscriptions.json
redisctl --profile cloud-test cloud database list -o json > pre-test-databases.json
```

## Testing Workflow

### Phase 1: Read-Only Operations (Safe)
Test all GET operations first - no resource creation:
```bash
# Account operations
redisctl --profile cloud-test cloud account get
redisctl --profile cloud-test cloud account regions list
redisctl --profile cloud-test cloud account modules list

# List operations
redisctl --profile cloud-test cloud subscription list
redisctl --profile cloud-test cloud database list
redisctl --profile cloud-test cloud user list
redisctl --profile cloud-test cloud acl list
```

### Phase 2: Create → Verify → Delete (Controlled)

**IMPORTANT**: For each create operation, immediately note the resource ID for cleanup.

#### Database Testing Pattern
```bash
# 1. Create with minimal config
SUBSCRIPTION_ID="<existing-sub-id>"
cat > test-db.json <<EOF
{
  "name": "test-db-$(date +%s)",
  "protocol": "redis",
  "memoryLimitInGb": 1.0,
  "replication": false,
  "dataEvictionPolicy": "allkeys-lru"
}
EOF

# 2. Create and capture ID
DB_RESULT=$(redisctl --profile cloud-test cloud database create \
  --subscription "$SUBSCRIPTION_ID" \
  test-db.json \
  --wait \
  -o json)

DB_ID=$(echo "$DB_RESULT" | jq -r '.databaseId')
echo "Created database: $DB_ID"

# 3. Verify
redisctl --profile cloud-test cloud database get \
  --subscription "$SUBSCRIPTION_ID" \
  --database "$DB_ID"

# 4. Test operations on the database
redisctl --profile cloud-test cloud database update \
  --subscription "$SUBSCRIPTION_ID" \
  --database "$DB_ID" \
  '{"memoryLimitInGb": 2.0}'

# 5. CLEANUP - Delete immediately
redisctl --profile cloud-test cloud database delete \
  --subscription "$SUBSCRIPTION_ID" \
  --database "$DB_ID" \
  --wait

echo "Deleted database: $DB_ID"
```

#### User/ACL Testing Pattern
```bash
# Create
USER_RESULT=$(redisctl --profile cloud-test cloud user create \
  '{"name":"test-user-'$(date +%s)'","role":"manager"}' \
  -o json)
USER_ID=$(echo "$USER_RESULT" | jq -r '.userId')

# Test
redisctl --profile cloud-test cloud user get "$USER_ID"

# CLEANUP
redisctl --profile cloud-test cloud user delete "$USER_ID" --wait
```

### Phase 3: Async Operations Testing

Test `--wait` functionality:
```bash
# Create with --wait
redisctl --profile cloud-test cloud database create \
  --subscription "$SUBSCRIPTION_ID" \
  test-db.json \
  --wait \
  --wait-timeout 300 \
  --wait-interval 5

# Verify task tracking works
```

### Phase 4: Error Handling

Test error scenarios:
```bash
# Invalid database config
redisctl --profile cloud-test cloud database create \
  --subscription "$SUBSCRIPTION_ID" \
  '{"invalid": "config"}' 2>&1

# Non-existent resource
redisctl --profile cloud-test cloud database get \
  --subscription "$SUBSCRIPTION_ID" \
  --database 999999 2>&1

# Invalid subscription ID
redisctl --profile cloud-test cloud database list \
  --subscription 999999 2>&1
```

## Cleanup Procedure

### During Testing
Maintain a cleanup script as you test:
```bash
#!/bin/bash
# cleanup.sh - Run this if testing is interrupted

set -x

# Add each created resource
redisctl --profile cloud-test cloud database delete --subscription 12345 --database 67890 --wait
redisctl --profile cloud-test cloud user delete 111 --wait
# etc.
```

### After Testing
```bash
# Compare current vs baseline
redisctl --profile cloud-test cloud subscription list -o json > post-test-subscriptions.json
redisctl --profile cloud-test cloud database list -o json > post-test-databases.json

# Identify any orphaned resources
diff <(jq -r '.[].id' pre-test-subscriptions.json) \
     <(jq -r '.[].id' post-test-subscriptions.json)
```

### Emergency Cleanup
If you lose track of test resources:
```bash
# List all databases and manually review
redisctl --profile cloud-test cloud database list -o json | \
  jq -r '.[] | select(.name | startswith("test-")) | "\(.subscriptionId) \(.databaseId) \(.name)"'

# Delete any with "test-" prefix (DANGEROUS - verify first!)
```

## Resource Naming Convention

Always prefix test resources for easy identification:
- Databases: `test-db-<timestamp>`
- Users: `test-user-<timestamp>`
- ACL Rules: `test-acl-<timestamp>`
- Subscriptions: **DO NOT CREATE** - use existing only

## Rate Limiting

Redis Cloud API limits:
- 400 requests per minute per account
- Add delays if testing many operations:
  ```bash
  sleep 0.2  # 200ms between requests = ~300 req/min
  ```

## Safety Checklist

Before running tests:
- [ ] Using test profile (not production)
- [ ] Baseline inventory captured
- [ ] Cleanup script ready
- [ ] Resource naming convention clear
- [ ] Testing on non-critical subscription

During tests:
- [ ] Note each created resource ID
- [ ] Delete resources immediately after testing
- [ ] Monitor for failures
- [ ] Check rate limit warnings

After tests:
- [ ] All test resources deleted
- [ ] Baseline comparison clean
- [ ] No unexpected costs
- [ ] Cleanup script cleared

## What NOT to Test

**DO NOT** test these operations on live account:
- ❌ Subscription creation (costs money immediately)
- ❌ Subscription deletion (cannot undo)
- ❌ Payment method changes
- ❌ Account-level settings changes
- ❌ VPC/Peering/TGW (complex setup, hard to clean up)

Use mocked tests (wiremock) for these instead.

## Test Coverage Goals

Focus Cloud testing on:
1. ✅ Read operations (all handlers)
2. ✅ Database CRUD (critical path)
3. ✅ User/ACL CRUD (security critical)
4. ✅ Async operation handling (`--wait`)
5. ✅ Output format validation (JSON, YAML, table)
6. ✅ Error handling and messages
7. ⏸️ Complex operations (use mocks instead)

## Emergency Contact

If things go wrong:
1. Run cleanup script
2. Check Redis Cloud console
3. Delete resources manually via UI if needed
4. Note any orphaned resources for billing review
