# Profile Management Demo - Managing authentication profiles

Output img/profile-management.gif
Set FontSize 16
Set Width 1100
Set Height 650
Set Theme "Catppuccin Mocha"
Set Padding 20
Set TypingSpeed 45ms
Set LineHeight 1.3

Type "# Profile Management with redisctl"
Enter
Sleep 1s
Enter

Type "# List existing profiles"
Enter
Type "redisctl profile list"
Enter
Type "NAME         TYPE         DETAILS"
Enter
Type "-----------  ------------ ------------------------------"
Enter
Type "production   cloud        URL: https://api.redislabs.com/v1"
Enter
Type "staging      cloud        URL: https://api.redislabs.com/v1"
Enter
Type "local        enterprise   URL: https://localhost:9443 (insecure)"
Enter
Sleep 2s
Enter

Type "# Add a new Redis Cloud profile with environment variables"
Enter
Type 'redisctl profile set development \'
Enter
Type '  --deployment cloud \'
Enter
Type '  --api-key "${REDIS_CLOUD_KEY}" \'
Enter
Type '  --api-secret "${REDIS_CLOUD_SECRET}"'
Enter
Type "Profile 'development' saved successfully"
Enter
Sleep 2s
Enter

Type "# Add a Redis Enterprise profile for testing"
Enter
Type 'redisctl profile set test-cluster \'
Enter
Type '  --deployment enterprise \'
Enter
Type '  --url "https://test.redis.local:9443" \'
Enter
Type '  --username "admin@redis.local" \'
Enter
Type '  --password "${REDIS_PASSWORD}" \'
Enter
Type '  --insecure'
Enter
Type "Profile 'test-cluster' saved successfully"
Enter
Sleep 2s
Enter

Type "# View a specific profile"
Enter
Type "redisctl profile get development"
Enter
Type "Profile: development"
Enter
Type "Type: cloud"
Enter
Type "API URL: https://api.redislabs.com/v1"
Enter
Type "API Key: ****KEY (from environment)"
Enter
Sleep 2s
Enter

Type "# Set default profile"
Enter
Type "redisctl profile default production"
Enter
Type "Default profile set to: production"
Enter
Sleep 1.5s
Enter

Type "# Test authentication"
Enter
Type "redisctl auth test --profile production"
Enter
Type "✓ Authentication successful for profile 'production'"
Enter
Type "  Account ID: 12345"
Enter
Type "  Account Name: ACME Corp"
Enter
Sleep 2s
Enter

Type "# Remove a profile"
Enter
Type "redisctl profile remove test-cluster"
Enter
Type "Profile 'test-cluster' removed successfully"
Enter
Sleep 1.5s
