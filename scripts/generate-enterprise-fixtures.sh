#!/usr/bin/env bash
# Generate test fixtures from actual Redis Enterprise API responses
# This script captures real API responses to ensure our type definitions match reality

set -euo pipefail

# Configuration
REDIS_ENTERPRISE_URL="${REDIS_ENTERPRISE_URL:-https://localhost:9443}"
REDIS_ENTERPRISE_USER="${REDIS_ENTERPRISE_USER:-admin@redis.local}"
REDIS_ENTERPRISE_PASSWORD="${REDIS_ENTERPRISE_PASSWORD:-Redis123!}"
OUTPUT_DIR="crates/redis-enterprise/tests/fixtures"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Ensure output directory exists
mkdir -p "$OUTPUT_DIR"

echo "Generating Redis Enterprise API fixtures..."
echo "URL: $REDIS_ENTERPRISE_URL"
echo "Output: $OUTPUT_DIR"
echo ""

# Function to capture API response
capture_endpoint() {
    local endpoint="$1"
    local filename="$2"
    local description="$3"

    echo -n "Capturing $description... "

    if curl -k -s -f \
        -u "$REDIS_ENTERPRISE_USER:$REDIS_ENTERPRISE_PASSWORD" \
        "$REDIS_ENTERPRISE_URL$endpoint" \
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
        echo -e "${YELLOW}⚠ Endpoint not available${NC}"
        return 1
    fi
}

# Core cluster information
capture_endpoint "/v1/cluster" "cluster.json" "Cluster info"
capture_endpoint "/v1/cluster/stats" "cluster_stats.json" "Cluster stats"
capture_endpoint "/v1/cluster/alerts" "cluster_alerts.json" "Cluster alerts"

# Nodes
capture_endpoint "/v1/nodes" "nodes_list.json" "Nodes list"
capture_endpoint "/v1/nodes/1" "node_single.json" "Single node"
capture_endpoint "/v1/nodes/stats" "nodes_stats.json" "Nodes stats"

# Databases (bdbs)
capture_endpoint "/v1/bdbs" "bdbs_list.json" "Databases list"
capture_endpoint "/v1/bdbs/1" "bdb_single.json" "Single database"
capture_endpoint "/v1/bdbs/stats" "bdbs_stats.json" "Databases stats"

# Users
capture_endpoint "/v1/users" "users_list.json" "Users list"
capture_endpoint "/v1/users/1" "user_single.json" "Single user"

# Actions
capture_endpoint "/v1/actions" "actions_list.json" "Actions list"

# Modules
capture_endpoint "/v1/modules" "modules_list.json" "Modules list"

# License
capture_endpoint "/v1/license" "license.json" "License info"

# LDAP (if configured)
capture_endpoint "/v1/ldap" "ldap.json" "LDAP config" || true

# Shards
capture_endpoint "/v1/shards" "shards_list.json" "Shards list"

# Proxies
capture_endpoint "/v1/proxies" "proxies_list.json" "Proxies list"

# BDB groups
capture_endpoint "/v1/bdb_groups" "bdb_groups_list.json" "BDB groups list"

# CRDB (Active-Active)
capture_endpoint "/v1/crdbs" "crdbs_list.json" "CRDBs list" || true

# OCSP
capture_endpoint "/v1/ocsp" "ocsp.json" "OCSP config" || true

# Bootstrap status (should return 409 if already initialized)
capture_endpoint "/v1/bootstrap" "bootstrap_status.json" "Bootstrap status" || true

# Summary
echo ""
echo "Fixture generation complete!"
echo "Captured fixtures in: $OUTPUT_DIR"
echo ""
echo "Files generated:"
ls -lh "$OUTPUT_DIR"/*.json | awk '{print "  " $9 " (" $5 ")"}'
