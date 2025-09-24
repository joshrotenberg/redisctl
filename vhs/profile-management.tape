# Profile Management Demo - Managing authentication profiles

Output vhs/profile-management.gif
Set FontSize 16
Set Width 1100
Set Height 650
Set Theme "Catppuccin Mocha"
Set Padding 20
Set TypingSpeed 45ms
Set LineHeight 1.3

Hide
Type "clear"
Enter
Show

Type "# Profile Management with redisctl"
Enter
Sleep 1s
Enter

Type "# List existing profiles"
Enter
Type "redisctl profile list"
Enter
Sleep 2s
Enter

Type "# Add a Redis Cloud profile with environment variables"
Enter
Type 'redisctl profile set cloud-prod \\'
Enter
Type '  --deployment-type cloud \\'
Enter
Type '  --api-key "${REDIS_CLOUD_KEY}" \\'
Enter
Type '  --api-secret "${REDIS_CLOUD_SECRET}"'
Enter
Sleep 2s
Enter

Type "# Add a Redis Enterprise profile for local testing"
Enter
Type 'redisctl profile set enterprise-local \\'
Enter
Type '  --deployment-type enterprise \\'
Enter
Type '  --url "https://localhost:9443" \\'
Enter
Type '  --username "admin@redis.local" \\'
Enter
Type '  --password "Redis123!" \\'
Enter
Type '  --insecure'
Enter
Sleep 2s
Enter

Type "# View a specific profile"
Enter
Type "redisctl profile get enterprise-local"
Enter
Sleep 2s
Enter

Type "# Set a default profile"
Enter
Type "redisctl profile default enterprise-local"
Enter
Sleep 1.5s
Enter

Type "# Test authentication"
Enter
Type "redisctl auth test --profile enterprise-local"
Enter
Sleep 2s
Enter

Type "# Remove a profile"
Enter
Type "redisctl profile remove cloud-dev"
Enter
Sleep 1.5s

Hide
Type "clear"
Enter
Show

Sleep 500ms
