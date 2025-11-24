# Security Best Practices

This guide covers security best practices for using redisctl in production environments.

## Credential Storage

### Storage Methods Comparison

| Method | Security Level | Use Case | Pros | Cons |
|--------|---------------|----------|------|------|
| OS Keyring | ⭐⭐⭐⭐⭐ High | Production | Encrypted by OS, Most secure | Requires `secure-storage` feature |
| Environment Variables | ⭐⭐⭐⭐ Good | CI/CD, Containers | No file storage, Easy rotation | Must be set each session |
| Config File (Plaintext) | ⭐⭐ Low | Development only | Simple setup | Credentials visible in file |

### Using OS Keyring (Recommended for Production)

The most secure way to store credentials is using your operating system's keyring:

```bash
# Install with secure storage support
cargo install redisctl --features secure-storage

# Create secure profile
redisctl profile set production \
  --deployment cloud \
  --api-key "your-api-key" \
  --api-secret "your-api-secret" \
  --use-keyring
```

#### Platform Support
- **macOS**: Uses Keychain (automatic)
- **Windows**: Uses Credential Manager (automatic)
- **Linux**: Uses Secret Service (requires GNOME Keyring or KWallet)

#### How Keyring Storage Works

1. **Initial Setup**: When you use `--use-keyring`, credentials are stored in the OS keyring
2. **Config Reference**: The config file stores references like `keyring:production-api-key`
3. **Automatic Retrieval**: redisctl automatically retrieves credentials from keyring when needed
4. **Secure Updates**: Credentials can be updated without exposing them in files

Example config with keyring references:
```toml
[profiles.production]
deployment_type = "cloud"
api_key = "keyring:production-api-key"      # Actual value in keyring
api_secret = "keyring:production-api-secret" # Actual value in keyring
api_url = "https://api.redislabs.com/v1"    # Non-sensitive, plaintext
```

### Environment Variables (CI/CD)

For automated environments, use environment variables:

```bash
# Set credentials
export REDIS_CLOUD_API_KEY="your-key"
export REDIS_CLOUD_API_SECRET="your-secret"

# Use in commands (overrides config)
redisctl cloud database list

# Or reference in config
cat > config.toml <<EOF
[profiles.ci]
deployment_type = "cloud"
api_key = "\${REDIS_CLOUD_API_KEY}"
api_secret = "\${REDIS_CLOUD_API_SECRET}"
EOF
```

#### GitHub Actions Example
```yaml
- name: Deploy Database
  env:
    REDIS_CLOUD_API_KEY: ${{ secrets.REDIS_API_KEY }}
    REDIS_CLOUD_API_SECRET: ${{ secrets.REDIS_API_SECRET }}
  run: |
    redisctl cloud database create \
      --subscription 12345 \
      --data @database.json \
      --wait
```

## File Permissions

Protect configuration files containing credentials:

```bash
# Restrict to owner only
chmod 600 ~/.config/redisctl/config.toml

# Verify permissions
ls -la ~/.config/redisctl/config.toml
# -rw------- 1 user user 1234 Jan 15 10:00 config.toml
```

## Credential Rotation

### Regular Rotation Schedule

1. **Generate new credentials** in Redis Cloud/Enterprise console
2. **Update keyring** with new credentials:
   ```bash
   redisctl profile set production \
     --api-key "new-key" \
     --api-secret "new-secret" \
     --use-keyring
   ```
3. **Test access** with new credentials
4. **Revoke old credentials** in console

### Automated Rotation Script
```bash
#!/bin/bash
# rotate-credentials.sh

PROFILE="production"
NEW_KEY=$(generate-api-key)  # Your key generation method
NEW_SECRET=$(generate-api-secret)

# Update credentials
redisctl profile set "$PROFILE" \
  --api-key "$NEW_KEY" \
  --api-secret "$NEW_SECRET" \
  --use-keyring

# Test new credentials
if redisctl --profile "$PROFILE" cloud subscription list > /dev/null; then
  echo "Credential rotation successful"
  # Notify old credentials can be revoked
else
  echo "Credential rotation failed"
  exit 1
fi
```

## Secure Development Practices

### Never Commit Credentials

Add to `.gitignore`:
```gitignore
# Redis configuration
~/.config/redisctl/config.toml
.redisctl/
*.secret
*_credentials.toml
```

### Use Git Hooks

Pre-commit hook to detect credentials:
```bash
#!/bin/bash
# .git/hooks/pre-commit

# Check for API keys
if git diff --cached | grep -E "api_key|api_secret|password" | grep -v "keyring:"; then
  echo "ERROR: Potential credentials detected in commit"
  echo "Use --use-keyring or environment variables instead"
  exit 1
fi
```

### Separate Development and Production

Use different profiles for each environment:
```toml
# Development (with keyring for safety)
[profiles.dev]
deployment_type = "cloud"
api_key = "keyring:dev-api-key"
api_secret = "keyring:dev-api-secret"

# Staging
[profiles.staging]
deployment_type = "cloud"
api_key = "keyring:staging-api-key"
api_secret = "keyring:staging-api-secret"

# Production
[profiles.production]
deployment_type = "cloud"
api_key = "keyring:production-api-key"
api_secret = "keyring:production-api-secret"
```

## Audit and Monitoring

### Profile Usage Audit

Monitor which profiles are being used:
```bash
# Enable debug logging
export RUST_LOG=debug

# Commands will log profile usage
redisctl --profile production cloud database list
# [DEBUG] Using Redis Cloud profile: production
```

### Access Logging

Create wrapper script for audit logging:
```bash
#!/bin/bash
# /usr/local/bin/redisctl-audit

# Log command execution
echo "[$(date)] User: $USER, Command: redisctl $*" >> /var/log/redisctl-audit.log

# Execute actual command
exec /usr/local/bin/redisctl "$@"
```

### Credential Access Monitoring

Monitor keyring access (macOS example):
```bash
# View keychain access logs
log show --predicate 'subsystem == "com.apple.securityd"' --last 1h
```

## Network Security

### TLS/SSL Verification

Always verify SSL certificates in production:
```toml
[profiles.production]
deployment_type = "enterprise"
url = "https://cluster.example.com:9443"
username = "admin@example.com"
password = "keyring:production-password"
insecure = false  # Never true in production
```

### IP Whitelisting

Configure API access from specific IPs only:
1. In Redis Cloud console, set IP whitelist
2. In Redis Enterprise, configure firewall rules
3. Document allowed IPs in team runbook

## Incident Response

### Compromised Credentials

If credentials are compromised:

1. **Immediately revoke** compromised credentials in console
2. **Generate new credentials**
3. **Update all systems** using the credentials:
   ```bash
   # Update all profiles using compromised credentials
   for profile in $(redisctl profile list | grep production); do
     redisctl profile set "$profile" \
       --api-key "new-key" \
       --api-secret "new-secret" \
       --use-keyring
   done
   ```
4. **Audit access logs** for unauthorized usage
5. **Document incident** and update security procedures

### Security Checklist

- [ ] Using OS keyring for production credentials
- [ ] Config files have restricted permissions (600)
- [ ] Credentials not committed to version control
- [ ] Environment variables used in CI/CD
- [ ] Regular credential rotation scheduled
- [ ] Audit logging enabled
- [ ] SSL verification enabled
- [ ] IP whitelisting configured
- [ ] Incident response plan documented
- [ ] Team trained on security procedures

## Additional Resources

- [Redis Security Documentation](https://redis.io/docs/manual/security/)
- [Redis Cloud Security](https://redis.com/redis-enterprise-cloud/security/)
- [Redis Enterprise Security](https://redis.com/redis-enterprise/security/)