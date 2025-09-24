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

Type "# ListList existingprofilesprofilesnew developmentprofiles"
Enter
Type "redisctl profile set dev-cloud --deployment cloud --api-key \${REDIS_DEV_KEY} --api-secret \${REDIS_DEV_SECRET}"

Type "redisctl profile set prod --deployment cloud --api-key \${REDIS_CLOUD_KEY} --api-secret \${REDIS_CLOUD_SECRET}"
Type "redisctl profile set development --deployment cloud --api-key DEV-KEY --api-secret DEV-SECRET"
Enter
Sleep 2s
Enter

Type "# AddAdd aanEnterprise cluster profile"
Enter
Type "redisctl profile set devdev --deployment enterprise --url https://dev.redisdev.local:9443 --username adminadmin --password pass123\${REDIS_PASSWORD} --insecureSleep 2s
Enter

Type "# ViewView updated profileprofilesallupdated profile profiles(filtered)"
Enter
Type "redisctl profile list"
Enter
Sleep 2.5s
Enter

Type "# View specific profile"
Enter
Type "redisctl profile list"
 detailsget prodlist| grep devSleep 2s
 details (credentials are masked)get cloudSleep 2s
Enter

Type "# Set default profile"
Enter
Type "redisctl profile default prodcloudSleep 1.5s
Enter

Type "# RemoveTestTestauthentication configuration"
Enter
Type "redisctl auth test --profile removecloudSleepprodSleepremove test-cluster2s
#Removea profileredisctlprofileremovedev2.5s
#Removetest profilesredisctlprofileremovedev-cloudType "redisctl profile remove dev-cluster"
Enter
Sleep 1.5sEOF </dev/null
