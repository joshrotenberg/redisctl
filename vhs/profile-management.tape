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

Type "# ListListList existingexistingprofilesprofilesprofilesnew developmentprofiles"
Enter
Type "redisctl profile set dev-cloud --deployment cloud --api-key \${REDIS_DEV_KEY} --api-secret \${REDIS_DEV_SECRET}"
Type "redisctl profile list"

Type "redisctl profile set prod --deployment cloud --api-key \${REDIS_CLOUD_KEY} --api-secret \${REDIS_CLOUD_SECRET}"
Type "redisctl profile set development --deployment cloud --api-key DEV-KEY --api-secret DEV-SECRET"
Enter
Sleep 2s
Enter

Type "# AddAddAdd aastaging cluster profile"
Enter
Type "redisctl profile set devstagingdev --deployment cloud --apikeySTAGE_KEY--api-secret STAGE_SECRETSleep 2s
Enter

Type "# ViewAddView updated profileaprofilesallupdated profile profileslocal dev profile(filtered)"
Enter
Type "redisctl profile list"
Enter
Sleep 2.5s
Enter

Type "# View specific profile"
Enter
Type "redisctl profile list"
 detailsget prodsetlocal-dev --deployment enterprise --url https://dev.local:9443 --username dev@redis.local --password DevPass123 --insecureSleep2s
#View updated profile listredisctlprofile detailsget staginglist| grep devSleep 2s
 details (credentials are masked)get cloudSleep 2s
Enter

Type "# Set default profile"
Enter
Type "redisctl profile default prodstagingcloudSleep 1.5s
Enter

Type "# RemoveRemoveTestTestauthenticationa configuration"
Enter
Type "redisctl auth test --profile removeremove local-dev2.5s
#Removetest profilesredisctlprofileremovedev-cloudType "redisctl profile remove dev-cluster"
Enter
Sleep 1.5sEOF </dev/null
