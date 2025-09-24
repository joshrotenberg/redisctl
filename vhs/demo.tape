# redisctl Demo
# This creates an animated GIF showcasing redisctl's key features

# Terminal configuration
Output demo.gif
Set FontSize 14
Set Width 1200
Set Height 800
Set Theme "Dracula"
Set Padding 20
Set TypingSpeed 50ms

# Start with a clean slate
Type "# redisctl - Unified CLI for Redis Cloud & Enterprise"
Enter
Sleep 1s
Enter

# Show help
Type "# Let's explore redisctl's capabilities"
Enter
Sleep 500ms
Type "redisctl --help"
Enter
Sleep 2s
Enter

# Profile management
Type "# First, let's set up our profiles"
Enter
Sleep 500ms
Type "redisctl profile list"
Enter
Sleep 1.5s
Enter

Type "# Add a Redis Cloud profile"
Enter
Sleep 500ms
Type "redisctl profile set cloud-dev --deployment-type cloud --api-key YOUR_KEY --api-secret YOUR_SECRET"
Enter
Sleep 1.5s
Enter

Type "# Add a Redis Enterprise profile"
Enter
Sleep 500ms
Type "redisctl profile set enterprise-local --deployment-type enterprise --url https://localhost:9443 --username admin@redis.local --password Redis123! --insecure"
Enter
Sleep 1.5s
Enter

# Cloud operations
Type "# Working with Redis Cloud"
Enter
Sleep 500ms
Type "redisctl cloud subscription list -o table"
Enter
Sleep 2s
Enter

Type "# Get subscription details with JSON output and JMESPath filtering"
Enter
Sleep 500ms
Type "redisctl cloud subscription get 12345 -o json -q 'name'"
Enter
Sleep 2s
Enter

Type "# Create a new database with async operation tracking"
Enter
Sleep 500ms
Type 'redisctl cloud database create --subscription-id 12345 --data \'{"name":"my-database","memoryLimitInGb":1}\' --wait'
Enter
Sleep 3s
Enter

# Enterprise operations
Type "# Working with Redis Enterprise"
Enter
Sleep 500ms
Type "redisctl enterprise cluster info -o table"
Enter
Sleep 2s
Enter

Type "# List all databases"
Enter
Sleep 500ms
Type "redisctl enterprise database list -o table"
Enter
Sleep 2s
Enter

Type "# Get specific database details"
Enter
Sleep 500ms
Type "redisctl enterprise database get 1 -o yaml"
Enter
Sleep 2s
Enter

# Raw API access
Type "# Direct API access for advanced users"
Enter
Sleep 500ms
Type "redisctl api cloud get /subscriptions -q '[0].name'"
Enter
Sleep 2s
Enter

Type "# POST request with data"
Enter
Sleep 500ms
Type 'redisctl api enterprise post /v1/bdbs --data \'{"name":"test-db","memory_size":1073741824}\''
Enter
Sleep 2s
Enter

# Module management
Type "# Module management in Redis Enterprise"
Enter
Sleep 500ms
Type "redisctl enterprise module list -o table"
Enter
Sleep 2s
Enter

# Output formats
Type "# Multiple output formats available"
Enter
Sleep 500ms
Type "# JSON format"
Enter
Type "redisctl cloud region list -o json | head -5"
Enter
Sleep 1.5s
Enter

Type "# YAML format"
Enter
Type "redisctl cloud region list -o yaml | head -10"
Enter
Sleep 1.5s
Enter

Type "# Table format (default for interactive terminals)"
Enter
Type "redisctl cloud region list -o table | head -10"
Enter
Sleep 2s
Enter

# Async operations
Type "# Async operation support with progress tracking"
Enter
Sleep 500ms
Type "# All create/update/delete operations support --wait flag"
Enter
Type "redisctl cloud database delete 12345:67890 --wait --wait-timeout 300"
Enter
Sleep 2s
Enter

# Final message
Type "# Learn more at: https://github.com/joshrotenberg/redisctl"
Enter
Sleep 1s
Type "# Thank you for using redisctl! 🚀"
Enter
Sleep 3s
