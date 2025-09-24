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

Type "# List existing profiles"
Enter
Type "redisctl profile list"
Enter
Sleep 2.5s
Enter

Type "# Create a new Cloud profile"
Enter
Type "redisctl profile set development --deployment cloud --api-key DEV-KEY --api-secret DEV-SECRET"
Enter
Sleep 2s
Enter

Type "# View profile details"
Enter
Type "redisctl profile get production"
Enter
Sleep 2s
Enter

Type "# Get Redis Cloud regions (first 3)"
Enter
Type 'redisctl cloud region list --profile production -o json | jq ".[0:3] | .[].name"'
Enter
Sleep 2s
Enter

Type "# Direct API with JMESPath filtering"
Enter
Type 'redisctl api cloud get /payment-methods --profile production -o json -q "paymentMethods[0].cardType"'
Enter
Sleep 2s

Sleep 500ms
