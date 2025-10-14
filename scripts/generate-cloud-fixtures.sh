#!/usr/bin/env bash
# Generate test fixtures from actual Redis Cloud API responses
# This script captures real API responses to ensure our type definitions match reality

set -euo pipefail

# Configuration
REDIS_CLOUD_API_KEY="${REDIS_CLOUD_API_KEY:-}"
REDIS_CLOUD_SECRET_KEY="${REDIS_CLOUD_SECRET_KEY:-}"
REDIS_CLOUD_BASE_URL="${REDIS_CLOUD_BASE_URL:-https://api.redislabs.com/v1}"
OUTPUT_DIR="crates/redis-cloud/tests/fixtures"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check credentials
if [ -z "$REDIS_CLOUD_API_KEY" ] || [ -z "$REDIS_CLOUD_SECRET_KEY" ]; then
    echo -e "${RED}Error: REDIS_CLOUD_API_KEY and REDIS_CLOUD_SECRET_KEY must be set${NC}"
    echo "Export these environment variables with your Redis Cloud credentials"
    exit 1
fi

# Ensure output directory exists
mkdir -p "$OUTPUT_DIR"

echo "Generating Redis Cloud API fixtures..."
echo "URL: $REDIS_CLOUD_BASE_URL"
echo "Output: $OUTPUT_DIR"
echo ""
echo -e "${YELLOW}WARNING: This will make real API calls to your Cloud account${NC}"
echo -e "${YELLOW}No resources will be created, only GET requests${NC}"
echo ""

# Function to capture API response
capture_endpoint() {
    local endpoint="$1"
    local filename="$2"
    local description="$3"

    echo -n "Capturing $description... "

    if curl -s -f \
        -H "x-api-key: $REDIS_CLOUD_API_KEY" \
        -H "x-api-secret-key: $REDIS_CLOUD_SECRET_KEY" \
        "$REDIS_CLOUD_BASE_URL$endpoint" \
        -o "$OUTPUT_DIR/$filename" 2>/dev/null; then

        # Validate it's valid JSON
        if jq empty "$OUTPUT_DIR/$filename" 2>/dev/null; then
            echo -e "${GREEN}✓${NC}"
        else
            echo -e "${RED}✗ Invalid JSON${NC}"
            rm "$OUTPUT_DIR/$filename"
            return 1
        fi
    else
        echo -e "${YELLOW}⚠ Endpoint not available or no data${NC}"
        return 1
    fi
}

# Account information
capture_endpoint "/account" "account.json" "Account info"

# Subscriptions
capture_endpoint "/subscriptions" "subscriptions_list.json" "Subscriptions list"

# Databases (flexible/pro)
capture_endpoint "/subscriptions" "databases_for_subscription.json" "Databases for subscription" || true

# Cloud accounts
capture_endpoint "/cloud-accounts" "cloud_accounts_list.json" "Cloud accounts list"

# Payment methods
capture_endpoint "/payment-methods" "payment_methods_list.json" "Payment methods" || true

# Regions
capture_endpoint "/regions" "regions_list.json" "Regions list" || true

# Tasks (may be empty)
capture_endpoint "/tasks" "tasks_list.json" "Tasks list" || true

# Users
capture_endpoint "/users" "users_list.json" "Users list"

# ACLs
capture_endpoint "/acls" "acls_list.json" "ACLs list" || true

# Fixed (Essentials) subscriptions
capture_endpoint "/fixed/subscriptions" "fixed_subscriptions_list.json" "Fixed subscriptions" || true

# API Keys (careful - these are sensitive, only if needed for testing)
# capture_endpoint "/account/api-keys" "api_keys_list.json" "API keys list" || true

echo ""
echo "Fixture generation complete!"
echo "Captured fixtures in: $OUTPUT_DIR"
echo ""
echo -e "${YELLOW}IMPORTANT: Review fixtures for sensitive data before committing!${NC}"
echo "- Remove account IDs, subscription IDs if needed"
echo "- Check for any PII or secrets"
echo ""
echo "Files generated:"
ls -lh "$OUTPUT_DIR"/*.json 2>/dev/null | awk '{print "  " $9 " (" $5 ")"}'
