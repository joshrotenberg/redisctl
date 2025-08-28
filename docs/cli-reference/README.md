# redisctl Command Reference

```text
Unified Redis CLI for Cloud and Enterprise

Usage: redisctl [OPTIONS] <COMMAND>

Commands:
  cloud       Redis Cloud commands
  enterprise  Redis Enterprise commands
  profile     Profile management
  database    Database operations (smart routing)
  cluster     Cluster operations (smart routing)
  user        User operations (smart routing)
  account     Account operations (smart routing to Cloud subscriptions)
  auth        Authentication testing and management
  config      Configuration management
  help        Print this message or the help of the given subcommand(s)

Options:
  -o, --output <OUTPUT>          Output format [default: json] [possible values: json, yaml, table]
  -q, --query <QUERY>            JMESPath query to filter output
  -p, --profile <PROFILE>        Profile name to use (overrides REDISCTL_PROFILE env var)
  -d, --deployment <DEPLOYMENT>  Deployment type (auto-detected from profile if not specified) [possible values: cloud, enterprise]
  -v, --verbose...               Verbose logging
  -h, --help                     Print help
  -V, --version                  Print version
```text
