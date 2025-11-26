#!/bin/bash
# Cloud API Testing Script
# Run comprehensive tests against live Redis Cloud API with automatic cleanup

set -e  # Exit on error
set -u  # Exit on undefined variable

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
PROFILE="cloud-test"
CLEANUP_FILE="$(mktemp)"
TIMESTAMP=$(date +%s)

# Trap to ensure cleanup on exit
trap cleanup EXIT INT TERM

cleanup() {
    echo -e "\n${YELLOW}Running cleanup...${NC}"
    if [ -f "$CLEANUP_FILE" ]; then
        while IFS= read -r cmd; do
            echo -e "${YELLOW}Cleanup: $cmd${NC}"
            eval "$cmd" || echo -e "${RED}Cleanup failed: $cmd${NC}"
        done < "$CLEANUP_FILE"
        rm -f "$CLEANUP_FILE"
    fi
    echo -e "${GREEN}Cleanup complete${NC}"
}

add_cleanup() {
    echo "$1" >> "$CLEANUP_FILE"
}

log_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

log_error() {
    echo -e "${RED}✗ $1${NC}"
}

log_info() {
    echo -e "${YELLOW}→ $1${NC}"
}

# Verify profile exists
if ! cargo run -- profile list | grep -q "$PROFILE"; then
    echo -e "${RED}Error: Profile '$PROFILE' not found${NC}"
    echo "Create it with:"
    echo "  cargo run -- profile set $PROFILE --deployment cloud --api-key \$REDIS_CLOUD_API_KEY --api-secret \$REDIS_CLOUD_SECRET_KEY"
    exit 1
fi

echo -e "${GREEN}=== Redis Cloud API Testing ===${NC}"
echo "Profile: $PROFILE"
echo "Timestamp: $TIMESTAMP"
echo

# Phase 1: Read-Only Operations
echo -e "${GREEN}=== Phase 1: Read-Only Operations ===${NC}"

log_info "Testing account operations..."
cargo run -- --profile "$PROFILE" cloud account get > /dev/null
log_success "Account get"

cargo run -- --profile "$PROFILE" cloud account regions list > /dev/null
log_success "Regions list"

cargo run -- --profile "$PROFILE" cloud account modules list > /dev/null
log_success "Modules list"

log_info "Testing list operations..."
SUBSCRIPTIONS=$(cargo run -- --profile "$PROFILE" cloud subscription list -o json)
log_success "Subscriptions list"

SUBSCRIPTION_ID=$(echo "$SUBSCRIPTIONS" | jq -r '.[0].id // empty')
if [ -z "$SUBSCRIPTION_ID" ]; then
    log_error "No subscriptions found - cannot continue database tests"
    exit 1
fi
log_info "Using subscription ID: $SUBSCRIPTION_ID"

cargo run -- --profile "$PROFILE" cloud database list -o json > /dev/null
log_success "Databases list"

cargo run -- --profile "$PROFILE" cloud user list > /dev/null
log_success "Users list"

echo

# Phase 2: Database CRUD
echo -e "${GREEN}=== Phase 2: Database CRUD ===${NC}"

log_info "Creating test database..."
cat > /tmp/test-db-$TIMESTAMP.json <<EOF
{
  "name": "test-db-$TIMESTAMP",
  "protocol": "redis",
  "memoryLimitInGb": 1.0,
  "replication": false,
  "dataEvictionPolicy": "allkeys-lru",
  "throughputMeasurement": {
    "by": "operations-per-second",
    "value": 1000
  }
}
EOF

DB_RESULT=$(cargo run -- --profile "$PROFILE" cloud database create \
  --subscription "$SUBSCRIPTION_ID" \
  /tmp/test-db-$TIMESTAMP.json \
  --wait \
  -o json)

DB_ID=$(echo "$DB_RESULT" | jq -r '.databaseId')
log_success "Database created: $DB_ID"

# Schedule cleanup
add_cleanup "cargo run -- --profile $PROFILE cloud database delete --subscription $SUBSCRIPTION_ID --database $DB_ID --wait"

log_info "Getting database details..."
cargo run -- --profile "$PROFILE" cloud database get \
  --subscription "$SUBSCRIPTION_ID" \
  --database "$DB_ID" \
  -o json > /dev/null
log_success "Database get"

log_info "Testing output formats..."
cargo run -- --profile "$PROFILE" cloud database get \
  --subscription "$SUBSCRIPTION_ID" \
  --database "$DB_ID" \
  -o yaml > /dev/null
log_success "YAML output"

cargo run -- --profile "$PROFILE" cloud database get \
  --subscription "$SUBSCRIPTION_ID" \
  --database "$DB_ID" \
  -o table > /dev/null
log_success "Table output"

log_info "Testing JMESPath query..."
cargo run -- --profile "$PROFILE" cloud database get \
  --subscription "$SUBSCRIPTION_ID" \
  --database "$DB_ID" \
  -o json \
  -q 'name' > /dev/null
log_success "JMESPath query"

log_info "Deleting test database..."
cargo run -- --profile "$PROFILE" cloud database delete \
  --subscription "$SUBSCRIPTION_ID" \
  --database "$DB_ID" \
  --wait
log_success "Database deleted"

# Remove from cleanup list since we already deleted it
sed -i.bak "/$DB_ID/d" "$CLEANUP_FILE"

echo

# Phase 3: User CRUD
echo -e "${GREEN}=== Phase 3: User CRUD ===${NC}"

log_info "Creating test user..."
USER_RESULT=$(cargo run -- --profile "$PROFILE" cloud user create \
  "{\"name\":\"test-user-$TIMESTAMP\",\"role\":\"viewer\"}" \
  -o json)

USER_ID=$(echo "$USER_RESULT" | jq -r '.id // .userId')
if [ -n "$USER_ID" ] && [ "$USER_ID" != "null" ]; then
    log_success "User created: $USER_ID"

    # Schedule cleanup
    add_cleanup "cargo run -- --profile $PROFILE cloud user delete $USER_ID --wait"

    log_info "Getting user details..."
    cargo run -- --profile "$PROFILE" cloud user get "$USER_ID" -o json > /dev/null
    log_success "User get"

    log_info "Deleting test user..."
    cargo run -- --profile "$PROFILE" cloud user delete "$USER_ID" --wait
    log_success "User deleted"

    # Remove from cleanup
    sed -i.bak "/$USER_ID/d" "$CLEANUP_FILE"
else
    log_error "User creation failed or returned invalid ID"
fi

echo

# Phase 4: Error Handling
echo -e "${GREEN}=== Phase 4: Error Handling ===${NC}"

log_info "Testing invalid database ID..."
if cargo run -- --profile "$PROFILE" cloud database get \
    --subscription "$SUBSCRIPTION_ID" \
    --database 999999 2>&1 | grep -q "error\|not found"; then
    log_success "Error handling for invalid database ID"
else
    log_error "Did not get expected error"
fi

log_info "Testing invalid subscription ID..."
if cargo run -- --profile "$PROFILE" cloud database list \
    --subscription 999999 2>&1 | grep -q "error\|not found"; then
    log_success "Error handling for invalid subscription ID"
else
    log_error "Did not get expected error"
fi

echo
echo -e "${GREEN}=== All Tests Complete ===${NC}"
echo
echo -e "${YELLOW}Verify no test resources remain:${NC}"
echo "  cargo run -- --profile $PROFILE cloud database list | grep test-"
echo "  cargo run -- --profile $PROFILE cloud user list | grep test-"
