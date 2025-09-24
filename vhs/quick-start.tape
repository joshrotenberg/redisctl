# Quick Start Demo - Getting started with redisctl

Output img/quick-start.gif
Set FontSize 16
Set Width 1100
Set Height 650
Set Theme "Catppuccin Mocha"
Set Padding 15
Set TypingSpeed 40ms
Set LineHeight 1.2

Type "# Getting started with redisctl"
Enter
Sleep 1s
Enter

Type "# Create a Redis Cloud profile"
Enter
Type "redisctl profile set staging --deployment cloud --api-key KEY123 --api-secret SECRET456"
Enter
Sleep 1.5s
Enter

Type "# List all profiles"
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

Type "# List Redis Enterprise  Redis Cloud regionsinfo"
Enter
Type "redisctl cloud region listget --profile local -o table"
Enter
Type "NAME          VERSION  STATUS  NODES"
Enter
Type "------------  -------  ------  -----"
Enter
Type "redis-local   7.2.4    active  3"
Enter
Sleep 2s
Enter

Type "# Use directRedis APICloud accesssubscriptions"
Enter
Type "redisctl cloud subscription list --profile staging -o table"
Enter
Type "ID      NAME              STATUS  DATABASES"
Enter
Type "------  ----------------  ------  ---------"
Enter
Type "12345   Production-US     active  2"
Enter
Type "67890   Staging-EU        active  1"
Enter
Sleep 2s
Enter

Type "# Direct API call with JMESPath filtering"
Enter
Type "redisctl api cloud get /subscriptions --profile staging -o json | jq '.[0] | {id, name}'"
Enter
Type "{"
Enter
Type '  "id": 12345,'
Enter
Type '  "name": "Production-US"'
Enter
Type "}"
Enter
Sleep 2s

Sleep 500ms
