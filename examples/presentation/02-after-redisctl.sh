#!/usr/bin/env bash
# After redisctl: The elegant solution
# This shows how redisctl simplifies Redis Enterprise management

set -e

echo "=========================================="
echo "AFTER REDISCTL: The Elegant Solution"
echo "=========================================="
echo ""

# Step 1: Get cluster information (simple, readable)
echo "Step 1: Getting cluster information..."
echo "$ redisctl enterprise cluster get -o json -q 'name'"
redisctl enterprise cluster get -o json -q 'name'
echo ""

# Step 2: List databases (clean output)
echo "Step 2: Listing databases..."
echo "$ redisctl enterprise database list -o json | jq -r '.[] | \"\\(.uid): \\(.name) - \\(.status)\"'"
redisctl enterprise database list -o json | jq -r '.[] | "\(.uid): \(.name) - \(.status)"'
echo ""

# Step 3: Get specific database details (with JMESPath)
echo "Step 3: Getting database details (ID 1)..."
echo "$ redisctl enterprise database get 1 -o json -q '{name, status, memory_size, redis_version}'"
redisctl enterprise database get 1 -o json -q '{name: name, status: status, memory_size: memory_size, redis_version: redis_version}'
echo ""

# Step 4: Create a database (type-safe, human-friendly)
echo "Step 4: Creating a new database with automatic polling..."
echo "$ redisctl enterprise database create \\"
echo "  --name demo-db-after \\"
echo "  --memory-size 100MB \\"
echo "  --type redis \\"
echo "  --port 12101"
echo ""

# Note: Commented out actual creation to avoid side effects in demo
echo "# (Creation skipped in demo - database already exists)"
echo "# In reality, this would:"
echo "# - Create the database"
echo "# - Show progress indicator"
echo "# - Return JSON with full database details"
echo ""

# Step 5: No polling needed! (the --wait flag handles it automatically)
echo "Step 5: Polling for completion..."
echo "# NOT NEEDED! redisctl handles async operations automatically"
echo "# Just add --wait flag to any create/update/delete operation"
echo ""
echo "$ redisctl enterprise database create --name mydb --wait"
echo "  ✓ Creates database"
echo "  ✓ Polls automatically"
echo "  ✓ Shows progress"
echo "  ✓ Returns when complete"
echo ""

# Bonus: Profile management
echo "=========================================="
echo "BONUS: Profile Management"
echo "=========================================="
echo ""
echo "$ redisctl profile set production \\"
echo "  --deployment-type enterprise \\"
echo "  --url https://prod-cluster.example.com:9443 \\"
echo "  --username admin@example.com \\"
echo "  --use-keyring  # Secure credential storage!"
echo ""
echo "$ redisctl profile list"
echo "# Manages multiple clusters easily"
echo "# Credentials stored securely in OS keyring"
echo "# Override with environment variables or flags"
echo ""

# Bonus: Structured output for automation
echo "=========================================="
echo "BONUS: CI/CD Integration"
echo "=========================================="
echo ""
echo "$ DB_STATUS=\$(redisctl enterprise database get 1 -o json -q 'status')"
echo "$ if [ \"\$DB_STATUS\" != \"active\" ]; then exit 1; fi"
echo ""
echo "# Perfect for:"
echo "# - Monitoring scripts"
echo "# - CI/CD pipelines"
echo "# - Automation tools"
echo "# - Terraform providers (coming soon!)"
echo ""

echo "=========================================="
echo "ADVANTAGES OF REDISCTL:"
echo "=========================================="
echo "✓ Type-safe API clients (catch errors at compile time)"
echo "✓ Automatic async operation handling (no manual polling)"
echo "✓ Secure credential storage (OS keyring integration)"
echo "✓ Structured output (JSON, YAML, Table)"
echo "✓ JMESPath query support (filter output easily)"
echo "✓ Progress indicators and user feedback"
echo "✓ Consistent error messages with suggestions"
echo "✓ Cross-platform (macOS, Linux, Windows)"
echo "✓ Library-first architecture (reusable components)"
echo "✓ Battle-tested (85%+ test coverage)"
echo ""
echo "🎯 ONE COMMAND vs. 50 LINES OF BASH"
echo ""
