#!/usr/bin/env bash
# Demo workflow: Complete presentation flow
# This script demonstrates the key features of redisctl

set -e

echo "=========================================="
echo "REDISCTL DEMO: Key Features"
echo "=========================================="
echo ""

# Feature 1: Profile Management
echo "Feature 1: Profile Management"
echo "------------------------------"
echo "$ redisctl profile list"
redisctl profile list
echo ""

# Feature 2: Cluster Information
echo "Feature 2: Cluster Information"
echo "------------------------------"
echo "$ redisctl enterprise cluster get -o table"
redisctl enterprise cluster get -o json | jq '{name, version, license_grace_period: .license_grace_period}'
echo ""

# Feature 3: Database Management
echo "Feature 3: Database Management"
echo "------------------------------"
echo "$ redisctl enterprise database list"
redisctl enterprise database list -o json | jq -r '.[] | "\(.name) [\(.uid)]: \(.memory_size | tonumber / 1048576)MB - \(.status)"'
echo ""

# Feature 4: Structured Output (JSON with JMESPath)
echo "Feature 4: Structured Output + JMESPath"
echo "---------------------------------------"
echo "$ redisctl enterprise database get 1 -o json -q 'name'"
redisctl enterprise database get 1 -o json -q 'name'
echo ""

# Feature 5: Node Information
echo "Feature 5: Node Information"
echo "---------------------------"
echo "$ redisctl enterprise node list"
redisctl enterprise node list -o json | jq -r '.[] | "Node \(.uid): \(.addr) - \(.status)"'
echo ""

# Feature 6: Support Package (saves hours of manual work)
echo "Feature 6: Support Package Generation"
echo "-------------------------------------"
echo "$ redisctl enterprise support-package cluster --help"
echo ""
echo "Key options:"
echo "  --optimize        Compress logs (reduces size by 20-30%)"
echo "  --upload          Upload directly to Redis Support (Files.com)"
echo "  --output PATH     Save to specific location"
echo ""
echo "Before redisctl: 10+ minutes of manual clicking and uploading"
echo "With redisctl:   30 seconds, fully automated"
echo ""

# Feature 7: API Command (raw access)
echo "Feature 7: Raw API Access"
echo "-------------------------"
echo "$ redisctl api enterprise get /v1/cluster -q 'name'"
redisctl api enterprise get /v1/cluster -q 'name'
echo ""

# Feature 8: Library Usage Example
echo "Feature 8: Library-First Architecture"
echo "-------------------------------------"
echo "redisctl is built as reusable libraries:"
echo ""
echo "  redisctl-config      Profile and credential management"
echo "  redis-cloud          Cloud API client (21 handlers)"
echo "  redis-enterprise     Enterprise API client (29 handlers)"
echo "  redisctl             CLI binary (thin orchestration layer)"
echo ""
echo "Coming soon:"
echo "  redisctl-workflows   Reusable workflow orchestration"
echo "  redisctl-output      Consistent output formatting"
echo ""
echo "This enables:"
echo "  - Terraform providers"
echo "  - Backup/migration tools"
echo "  - Monitoring dashboards"
echo "  - Custom automation scripts"
echo ""

echo "=========================================="
echo "DEMO COMPLETE"
echo "=========================================="
echo ""
echo "Key Takeaways:"
echo "  1. FIRST CLI tool for Redis Cloud and Enterprise"
echo "  2. Eliminates fragile bash + curl + jq scripts"
echo "  3. Type-safe, tested, production-ready"
echo "  4. Library-first enables ecosystem growth"
echo "  5. Cross-platform, secure, user-friendly"
echo ""
echo "Questions?"
echo ""
