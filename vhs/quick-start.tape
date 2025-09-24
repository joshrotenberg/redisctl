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

Type "# Check version"
Enter
Type "redisctl --version"
Enter
Sleep 1.5s
Enter

Type "# View available commands"
Enter
Type "redisctl --help | head -20"
Enter
Sleep 2.5s
Enter

Type "# Create profiles for different environments"
Enter
Type "redisctl profile set prod --deployment cloud --api-key PROD_KEY --api-secret PROD_SECRET"
Enter
Sleep 1.5s
Enter

Type "redisctl profile set dev --deployment enterprise --url https://dev.local:9443 --username admin --password pass123 --insecure"
Enter
Sleep 1.5s
Enter

Type "# List all profiles"
configuredType "# Create profiles for different environments"
Enter
Type "redisctl profile set production --deployment cloud --api-key PROD_KEY --api-secret PROD_SECRET"
Enter
Sleep 1.5s
Enter

Type "redisctl profile set staging --deployment cloud --api-key STAGE_KEY --api-secret STAGE_SECRET"
Enter
Sleep 1.5s
Enter

Type "redisctl profile set local --deployment enterprise --url https://localhost:9443 --username admin@redis.local --password Redis123! --insecure"
Enter
Sleep 1.5s
Enter

Type "# List all profiles"
Enter
Type "redisctl profile list | grep -E '^(cloud|production|staging|local)'"
Enter
Sleep 2s
Enter

Type "# View Cloud account info"
Enter
Type "redisctl cloud account get --profile cloud -o json | jq '{id: .accountId, name: .name}'"
Enter
Sleep 2s
Enter

Type "# List Cloud regions"
Enter
Type "redisctl cloud regions list --profile cloud -o json | jq '.[0:3] | .[] | .provider + \": \" + .name'"
Enter
Sleep 2s
Enter

Type "# List subscriptions"
Enter
Type "redisctl cloud subscription list --profile cloud -o table"
Enter
Sleep 2.5s
Enter

Type "# SetDirectAPIaccesswithfilteringapi/payment-methods --profile cloud -o json | jq '.paymentMethods | length'ViewavailableEnterprisecommands"enterprisehelphead12Setdefaultprofile"default 1.5sRemoveaprofile"profile remove staging"
Enter
Sleep 1.5s
Enter

Type "# Show available commands"
Enter
Type "redisctlhelp|head15Enter
Sleep 2s

Sleep 500ms
