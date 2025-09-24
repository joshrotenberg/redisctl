# Redis Cloud Demo - Managing Redis Cloud resources

Output img/cloud-demo.gif
Set FontSize 16
Set Width 1200
Set Height 700
Set Theme "Tokyo Night"
Set Padding 20
Set TypingSpeed 40ms
Set LineHeight 1.3

Hide
Type "clear"
Enter
Type "# Using mock API credentials for demo"
Enter
Type "export REDIS_CLOUD_API_KEY='demo-key-123'"
Enter
Type "export REDIS_CLOUD_SECRET_KEY='demo-secret-456'"
Enter
Type "clear"
Enter
Show

Type "# Redis Cloud Management with redisctl"
Enter
Sleep 1s
Enter

Type "# List available cloud regions"
Enter
Type "redisctl cloud region list -o table | head -15"
Enter
Sleep 2.5s
Enter

Type "# List subscriptions"
Enter
Type "redisctl cloud subscription list -o table"
Enter
Sleep 2s
Enter

Type "# Get subscription details with JMESPath filtering"
Enter
Type "redisctl cloud subscription get 12345 -o json -q '{name:name, status:status}'"
Enter
Sleep 2s
Enter

Type "# List databases in a subscription"
Enter
Type "redisctl cloud database list --subscription-id 12345 -o table"
Enter
Sleep 2s
Enter

Type "# Create a new database with async tracking"
Enter
Type 'redisctl cloud database create --subscription-id 12345 \\'
Enter
Type '  --data \'{"name":"api-cache","memoryLimitInGb":1,"datasetSizeInGb":1}\' \\'
Enter
Type '  --wait --wait-timeout 300'
Enter
Sleep 3s
Enter

Type "# List cloud accounts"
Enter
Type "redisctl cloud account list -o table"
Enter
Sleep 2s
Enter

Type "# Get payment methods"
Enter
Type "redisctl cloud payment-method list -o json -q '[0].cardType'"
Enter
Sleep 2s
Enter

Type "# Direct API access for custom queries"
Enter
Type "redisctl api cloud get /subscriptions -q 'subscriptions[0].name'"
Enter
Sleep 2s

Hide
Type "clear"
Enter
Show

Sleep 500ms
