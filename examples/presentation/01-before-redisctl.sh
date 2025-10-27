#!/usr/bin/env bash
# Before redisctl: The painful reality of managing Redis Enterprise
# This shows what operators had to do before redisctl existed

set -e

# Configuration
REDIS_ENTERPRISE_URL="${REDIS_ENTERPRISE_URL:-https://localhost:9443}"
REDIS_ENTERPRISE_USER="${REDIS_ENTERPRISE_USER:-admin@redis.local}"
REDIS_ENTERPRISE_PASSWORD="${REDIS_ENTERPRISE_PASSWORD:-Redis123!}"

echo "=========================================="
echo "BEFORE REDISCTL: The Painful Reality"
echo "=========================================="
echo ""

# Step 1: Get cluster information
echo "Step 1: Getting cluster information..."
echo "$ curl -k -u \"admin@redis.local:Redis123!\" https://localhost:9443/v1/cluster | jq '.name'"
CLUSTER_NAME=$(curl -k -s -u "$REDIS_ENTERPRISE_USER:$REDIS_ENTERPRISE_PASSWORD" \
  "$REDIS_ENTERPRISE_URL/v1/cluster" | jq -r '.name')
echo "Cluster name: $CLUSTER_NAME"
echo ""

# Step 2: List databases with complex jq parsing
echo "Step 2: Listing databases (requires jq parsing)..."
echo "$ curl -k -u \"admin@redis.local:Redis123!\" https://localhost:9443/v1/bdbs | jq -r '.[] | \"\\(.uid): \\(.name) - \\(.status)\"'"
curl -k -s -u "$REDIS_ENTERPRISE_USER:$REDIS_ENTERPRISE_PASSWORD" \
  "$REDIS_ENTERPRISE_URL/v1/bdbs" | \
  jq -r '.[] | "\(.uid): \(.name) - \(.status)"'
echo ""

# Step 3: Get specific database details
echo "Step 3: Getting database details (ID 1)..."
echo "$ curl -k -u \"admin@redis.local:Redis123!\" https://localhost:9443/v1/bdbs/1 | jq '{name, status, memory_size, redis_version}'"
curl -k -s -u "$REDIS_ENTERPRISE_USER:$REDIS_ENTERPRISE_PASSWORD" \
  "$REDIS_ENTERPRISE_URL/v1/bdbs/1" | \
  jq '{name, status, memory_size, redis_version}'
echo ""

# Step 4: Create a database (requires complex JSON)
echo "Step 4: Creating a new database..."
echo "$ curl -k -u \"admin@redis.local:Redis123!\" -X POST https://localhost:9443/v1/bdbs \\"
echo "  -H 'Content-Type: application/json' \\"
echo "  -d '{\"name\":\"demo-db\",\"memory_size\":104857600,\"type\":\"redis\",\"port\":12100}'"
echo ""

DB_RESPONSE=$(curl -k -s -u "$REDIS_ENTERPRISE_USER:$REDIS_ENTERPRISE_PASSWORD" \
  -X POST "$REDIS_ENTERPRISE_URL/v1/bdbs" \
  -H 'Content-Type: application/json' \
  -d '{
    "name": "demo-db-before",
    "memory_size": 104857600,
    "type": "redis",
    "port": 12100
  }' 2>&1)

# Check if response contains an error
if echo "$DB_RESPONSE" | jq -e '.error_code' > /dev/null 2>&1; then
  echo "Database creation returned task:"
  echo "$DB_RESPONSE" | jq '{task_id, status}'
else
  echo "Database created:"
  echo "$DB_RESPONSE" | jq '{uid, name, status}'
fi
echo ""

# Step 5: Poll for completion (this is where it gets really painful)
echo "Step 5: Polling for operation completion..."
echo "# In reality, you'd need to:"
echo "# 1. Extract the task ID from the response"
echo "# 2. Write a loop to poll /v1/tasks/{task_id}"
echo "# 3. Parse the status field"
echo "# 4. Sleep and retry until completed or failed"
echo "# 5. Handle errors and timeouts"
echo ""
echo "while true; do"
echo "  STATUS=\$(curl -k -s -u \"admin@redis.local:Redis123!\" \\"
echo "    https://localhost:9443/v1/tasks/\$TASK_ID | jq -r '.status')"
echo "  if [ \"\$STATUS\" = \"completed\" ]; then break; fi"
echo "  echo \"Still waiting... (\$STATUS)\""
echo "  sleep 2"
echo "done"
echo ""

echo "=========================================="
echo "PROBLEMS WITH THIS APPROACH:"
echo "=========================================="
echo "1. Requires curl, jq, and bash scripting knowledge"
echo "2. No type safety - typos cause runtime failures"
echo "3. Manual JSON construction is error-prone"
echo "4. Polling loops are fragile and hard to test"
echo "5. Credentials passed on command line (security risk)"
echo "6. No progress indicators or user feedback"
echo "7. Error handling is manual and inconsistent"
echo "8. Not portable across platforms (Windows?)"
echo "9. Can't pipe to other tools without jq acrobatics"
echo "10. Every operator reinvents these scripts"
echo ""
