# Configuration & Authentication Audit

## Current State Analysis

### Authentication Methods Priority Order

1. **Command-line flags** (highest priority)
   - `--profile <name>` - Use specific profile
   - `--deployment <cloud|enterprise>` - Force deployment type

2. **Environment Variables** (second priority)
   - `REDISCTL_PROFILE` - Default profile name
   - Cloud-specific:
     - `REDIS_CLOUD_API_KEY` - API key
     - `REDIS_CLOUD_API_SECRET` - API secret  
     - `REDIS_CLOUD_API_URL` - Custom API URL (optional)
   - Enterprise-specific:
     - `REDIS_ENTERPRISE_URL` - Cluster URL
     - `REDIS_ENTERPRISE_USER` - Username
     - `REDIS_ENTERPRISE_PASSWORD` - Password
     - `REDIS_ENTERPRISE_INSECURE` - Allow insecure TLS (true/false)

3. **Profile Configuration** (third priority)
   - Location varies by OS:
     - Linux: `~/.config/redisctl/config.toml`
     - macOS: `~/Library/Application Support/com.redis.redisctl/config.toml`
     - Windows: `%APPDATA%\redis\redisctl\config.toml`
   - TOML format with profiles section

4. **Default Profile** (lowest priority)
   - Set via `redisctl profile default <name>`
   - Stored in config file

### Current Pain Points

1. **Discovery Issues**
   - Users don't know where config file is stored
   - No way to print config file location
   - No command to show current active configuration

2. **Setup Complexity**
   - No guided setup for first-time users
   - Manual profile creation requires knowing all fields
   - No validation during setup

3. **Error Messages**
   - Generic "authentication failed" doesn't guide to solution
   - No suggestion to run setup wizard
   - Doesn't indicate which auth method was attempted

4. **Security Concerns**
   - Passwords stored in plain text in config
   - No support for credential helpers or keychains
   - Environment variables expose secrets in process list

5. **Testing & Validation**
   - No way to test credentials without running actual command
   - No dry-run mode to show what would be executed
   - Can't verify connection before saving profile

## Proposed Improvements

### 1. Interactive Setup Wizard

```bash
# First-time setup
$ redisctl setup
Welcome to redisctl! Let's configure your Redis connection.

? What type of Redis deployment? (Use arrow keys)
❯ Redis Cloud
  Redis Enterprise
  
? Enter your Redis Cloud API credentials:
  API Key: ********
  API Secret: ********
  
? Test connection? (Y/n) Y
✓ Successfully connected to Redis Cloud!

? Save as profile? (Y/n) Y
? Profile name: (production) 
? Set as default profile? (Y/n) Y

Configuration saved! You can now use: redisctl database list
```

### 2. Authentication Testing Command

```bash
# Test current configuration
$ redisctl auth test
Testing authentication...
✓ Profile: production (default)
✓ Type: Redis Cloud
✓ API URL: https://api.redislabs.com/v1
✓ Credentials: Valid
✓ Permissions: Read/Write
✓ Account: acme-corp (ID: 12345)

# Test specific profile
$ redisctl auth test --profile staging
Testing authentication...
✗ Profile: staging
✗ Type: Redis Enterprise
✗ URL: https://cluster.example.com:9443
✗ Error: Connection refused
  
  Suggestions:
  - Check if the cluster URL is correct
  - Verify the cluster is accessible from your network
  - Try with --insecure flag if using self-signed certificates
```

### 3. Configuration Management Commands

```bash
# Show configuration details
$ redisctl config show
Active Configuration:
  Source: Environment Variables
  Type: Redis Enterprise
  URL: https://localhost:9443
  User: admin@redis.local
  
Available Profiles:
  - production (Cloud) [default]
  - staging (Enterprise)
  - local (Enterprise)

# Show config file location
$ redisctl config path
/Users/alice/Library/Application Support/com.redis.redisctl/config.toml

# Edit config file
$ redisctl config edit
# Opens in $EDITOR

# Export configuration (without secrets)
$ redisctl config export
profiles:
  production:
    type: cloud
    api_url: https://api.redislabs.com/v1
    # Credentials hidden - set via environment or prompt
```

### 4. Improved Error Messages

```bash
$ redisctl database list
Error: Authentication failed for Redis Cloud

The API returned: 401 Unauthorized
Current configuration source: Environment variables

Possible solutions:
1. Check your API credentials:
   - REDIS_CLOUD_API_KEY is set but may be incorrect
   - REDIS_CLOUD_API_SECRET is set but may be incorrect

2. Run setup wizard:
   redisctl setup

3. Test your credentials:
   redisctl auth test

4. Use a different profile:
   redisctl database list --profile <name>

For more help: redisctl help auth
```

### 5. Secure Credential Storage

```bash
# Use system keychain (macOS)
$ redisctl config set production --use-keychain
Password will be stored in macOS Keychain

# Use credential helper
$ redisctl config set production --credential-helper "pass show redis/production"

# Use 1Password CLI
$ redisctl config set production --credential-helper "op read op://Redis/production/password"

# Prompt for password
$ redisctl config set production --prompt-password
Password will be requested when needed
```

### 6. Environment Detection

```bash
# Auto-detect Redis Enterprise in Docker
$ redisctl detect
Detected Redis Enterprise at https://localhost:9443
Would you like to configure a profile for this cluster? (Y/n)

# Auto-detect from kubectl context
$ redisctl detect --k8s
Detected Redis Enterprise Operator in namespace 'redis'
Found cluster: prod-cluster-redis-enterprise
Would you like to configure a profile? (Y/n)
```

## Implementation Plan

### Phase 1: Core Improvements (Priority: High)
1. Add `auth test` command for credential validation
2. Implement `config show` and `config path` commands
3. Improve error messages with actionable suggestions
4. Add `--dry-run` flag to show what would be executed

### Phase 2: Setup Wizard (Priority: High)
1. Create interactive setup wizard using `dialoguer` or `inquire`
2. Add connection testing during setup
3. Support profile creation and editing
4. Add migration from existing .env files

### Phase 3: Security Enhancements (Priority: Medium)
1. Integrate with system keychains (keyring-rs)
2. Support credential helpers
3. Add password prompting option
4. Implement secure token refresh for OAuth

### Phase 4: Advanced Features (Priority: Low)
1. Auto-detection of local Redis instances
2. Kubernetes integration for operator detection
3. Import from existing redis-cli configurations
4. Export to other formats (env, docker-compose)

## Benefits

1. **Reduced Setup Time**
   - From 10+ minutes reading docs to 1 minute wizard
   - No need to find correct environment variable names

2. **Fewer Support Issues**
   - Clear error messages reduce confusion
   - Built-in testing prevents misconfiguration
   - Guided setup reduces errors

3. **Better Security**
   - Passwords not stored in plain text
   - Integration with existing credential stores
   - Reduced exposure in environment variables

4. **Improved Developer Experience**
   - Similar to `aws configure` or `gcloud init`
   - Familiar patterns from other CLI tools
   - Progressive disclosure of complexity

## Success Metrics

- Setup time for new users < 2 minutes
- Authentication error resolution < 1 minute
- 50% reduction in auth-related support issues
- 90% of users successfully connect on first attempt

## Comparison with Curl

### Current curl approach:
```bash
# Users need to:
# 1. Find API endpoint documentation
# 2. Figure out authentication headers
# 3. Construct complex curl commands
# 4. Parse JSON responses manually

curl -X GET https://api.redislabs.com/v1/subscriptions \
  -H "x-api-key: $REDIS_CLOUD_API_KEY" \
  -H "x-api-secret-key: $REDIS_CLOUD_API_SECRET" \
  -H "Content-Type: application/json" | jq '.'
```

### With improved redisctl:
```bash
# One-time setup
redisctl setup

# Then just:
redisctl cloud subscription list
```

The difference is dramatic - from error-prone manual API calls to simple, validated commands.