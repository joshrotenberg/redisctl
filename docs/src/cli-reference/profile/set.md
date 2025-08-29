# Profile Set Command

```
Create or update a profile

Usage: redisctl profile set [OPTIONS] <NAME> <DEPLOYMENT_TYPE>

Arguments:
  <NAME>             Profile name
  <DEPLOYMENT_TYPE>  Deployment type [possible values: cloud, enterprise]

Options:
      --url <URL>                Connection URL (Enterprise) or API URL (Cloud)
      --username <USERNAME>      Username (Enterprise only)
      --password <PASSWORD>      Password (Enterprise only)
      --api-key <API_KEY>        API Key (Cloud only)
      --api-secret <API_SECRET>  API Secret (Cloud only)
      --insecure                 Allow insecure TLS (Enterprise only)
  -h, --help                     Print help
```

