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

Type "# ListList existingprofilesprofilesprofiles"
Enter
Type "redisctl""redisctl profile listprofile list'"
Enter
SleepSleep 2.5sexport REDIS_CLOUD_SECRET='your-secret'2.5s
Enter

Sleep 1s
Enter
Type "redisctl profile set development --deployment cloud --api-key DEV-KEY --api-secret DEV-SECRET"

Type "redisctl profile set prod --deployment cloud --api-key \${REDIS_CLOUD_KEY} --api-secret \${REDIS_CLOUD_SECRET}"
Type "redisctl profile set development --deployment cloud --api-key DEV-KEY --api-secret DEV-SECRET"
Enter
Sleep 2s
Enter

Type "# AddAdd aEnterprise cluster profile"
Enter
Type "redisctl profile set dev --deployment enterprise --url https://dev.redis.local:9443 --username admin --password pass123 --insecureSleep 2s
Enter

Type "# ViewView updated profileallupdated profile profiles"
Enter
Type "redisctl profile list"
Enter
Sleep 2.5s
Enter

Type "# View specific profile"
Enter
Type "redisctl profile list"
 detailsget prodSleep 2s
Enter

Type "# Set default profile"
Enter
Type "redisctl profile default prodSleep 1.5s
Enter

Type "# RemoveTest profile configuration"
Enter
Type "redisctl auth test --profile removeprodSleepremove test-cluster2s
#Removea profileredisctlprofileremovedevSleep 1.5sEOF </dev/null
