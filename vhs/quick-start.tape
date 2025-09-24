# Quick Start Demo - Getting started with redisctl

Output imgimg/quick-start.gif
Set FontSize 16
Set Width 1000
Set Height 600
Set Theme "Catppuccin Mocha"
Set Padding 15
Set TypingSpeed 40ms
Set LineHeight 1.2

Type "# Getting started with redisctl"
Enter
Sleep 1s
Enter

Type "# Check the version"
Enter
Type "redisctl --version"
Enter
Sleep 1.5s
Enter

Type "# Set up your first profile"
Enter
Type "redisctl profile set my-cloud --deployment-type cloud"
Enter
Sleep 1.5s
Enter

Type "# List configured profiles"
Enter
Type "redisctl profile list"
Enter
Sleep 2s
Enter

Type "# Get cluster information"
Enter
Type "redisctl enterprise cluster info -o table"
Enter
Sleep 2s
Enter

Type "# List databases with JSON output"
Enter
Type "redisctl enterprise database list -o json | jq '.[0].name'"
Enter
Sleep 2s

Sleep 500ms
