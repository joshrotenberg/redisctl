# Redis Enterprise Demo - Working with Redis Enterprise clusters

Output imgimg/enterprise-demo.gif
Set FontSize 16
Set Width 1200
Set Height 700
Set Theme "Dracula"
Set Padding 20
Set TypingSpeed 40ms
Set LineHeight 1.3

Type "# Redis Enterprise Management with redisctl"
Enter
Sleep 1s
Enter

Type "# Setting up environment variables"
Enter
Type "export REDIS_ENTERPRISE_URL='https://localhost:9443'"
Enter
Type "export REDIS_ENTERPRISE_USER='admin@redis.local'"
Enter
Type "export REDIS_ENTERPRISE_PASSWORD='Redis123!'"
Enter
Type "export REDIS_ENTERPRISE_INSECURE='true'"
Enter
Sleep 1s
Enter

Type "# Get cluster information"
Enter
Type "redisctl enterprise cluster get -o table"
Enter
Sleep 2.5s
Enter

Type "# List all nodes in the cluster"
Enter
Type "redisctl enterprise node list -o table"
Enter
Sleep 2s
Enter

Type "# List databases"
Enter
Type "redisctl enterprise database list -o table"
Enter
Sleep 2s
Enter

Type "# Get specific database details as JSON"
Enter
Type "redisctl enterprise database get 1 -o json | jq '.name, .memory_size'"
Enter
Sleep 2s
Enter

Type "# Create a new database"
Enter
Type 'redisctl enterprise database create \'
Enter
Type '  --data \'{"name":"demo-db","memory_size":1073741824,"type":"redis"}\' \'
Enter
Type '  --wait'
Enter
Sleep 3s
Enter

Type "# List available modules"
Enter
Type "redisctl enterprise module list -o table"
Enter
Sleep 2s
Enter

Type "# Get cluster stats"
Enter
Type "redisctl enterprise cluster stats -o json -q 'cpu_system'"
Enter
Sleep 2s
Enter

Type "# Direct API access for advanced operations"
Enter
Type "redisctl api enterprise get /v1/cluster -q 'name'"
Enter
Sleep 2s
