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
Enter
Type "redisctl profile list"
Enter
Sleep 2.5s
Enter

Type "# Set default profile"
Enter
Type "redisctl profile default prod"
Enter
Sleep 1.5s
Enter

Type "# View profile details"
Enter
Type "redisctl profile get prod"
Enter
Sleep 2s

Sleep 500ms
