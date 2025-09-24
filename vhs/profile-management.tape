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

Type "# ListList existingexisting profilesprofiles"
Enter
Type "redisctl"redisctl profile listprofile list"
Enter
SleepSleep 2.5s2.5s
Enter

Type "## Add a development profileAdd a development profile"
Enter
Type "redisctl profile set development --deployment cloud --api-key DEV-KEY --api-secret DEV-SECRET"
Type "redisctl profile set development --deployment cloud --api-key DEV-KEY --api-secret DEV-SECRET"
Enter
Sleep 2s
Enter

Type "# AddAdd aa test cluster profile"
Enter
Type "redisctl profile set test-cluster --deployment enterprise --url https://test.local:9443 --username test@redis.local --password TestPass123 --insecure"
Enter
Sleep 2s
Enter

Type "# ViewView updated profileupdated profile list"
Enter
Type "redisctl profile list"
Enter
Sleep 2.5s
Enter

Type "# View specific profile"
Enter
Type "redisctl profile get development"
Enter
Sleep 2s
Enter

Type "# Set default profile"
Enter
Type "redisctl profile default production"
Enter
Sleep 1.5s
Enter

Type "# RemoveRemove test profile"
Enter
Type "redisctl profile removeremove test-clusterSleep 1.5sEOF </dev/null
