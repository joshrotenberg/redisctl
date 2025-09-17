# OCSP Certificate Validation Commands

Manage Online Certificate Status Protocol (OCSP) for certificate validation in Redis Enterprise.

## Overview

OCSP commands configure and manage certificate revocation checking for enhanced security in Redis Enterprise clusters. OCSP provides real-time certificate validation without requiring Certificate Revocation Lists (CRLs).

## Available Commands

### Get OCSP Configuration

```bash
redisctl enterprise ocsp get
```

Retrieves current OCSP settings including:
- OCSP functionality status (enabled/disabled)
- Responder URL
- Query frequency
- Recovery settings
- Response timeout

### Update OCSP Configuration

```bash
redisctl enterprise ocsp update --data '{
  "ocsp_functionality": true,
  "responder_url": "http://ocsp.company.com",
  "query_frequency": 3600,
  "response_timeout": 5,
  "recovery_frequency": 60,
  "recovery_max_tries": 5
}'
```

### Get OCSP Status

```bash
redisctl enterprise ocsp status
```

Shows the current operational status of OCSP validation.

### Test OCSP Validation

```bash
redisctl enterprise ocsp test
```

Tests OCSP configuration and certificate validation.

### Enable OCSP

```bash
redisctl enterprise ocsp enable
```

Quick command to enable OCSP validation with current settings.

### Disable OCSP

```bash
redisctl enterprise ocsp disable
```

Quick command to disable OCSP validation.

## Configuration Examples

### Basic OCSP Setup

```json
{
  "ocsp_functionality": true,
  "responder_url": "http://ocsp.digicert.com",
  "query_frequency": 3600,
  "response_timeout": 5
}
```

### High-Security Configuration

```json
{
  "ocsp_functionality": true,
  "responder_url": "https://ocsp.internal.company.com",
  "query_frequency": 900,
  "response_timeout": 3,
  "recovery_frequency": 30,
  "recovery_max_tries": 10,
  "require_ocsp_response": true,
  "cache_response": true,
  "cache_duration": 3600
}
```

## Common Use Cases

### Initial OCSP Setup

```bash
# 1. Check current configuration
redisctl enterprise ocsp get

# 2. Configure OCSP responder
redisctl enterprise ocsp update --data '{
  "responder_url": "http://ocsp.company.com",
  "query_frequency": 3600
}'

# 3. Test configuration
redisctl enterprise ocsp test

# 4. Enable OCSP
redisctl enterprise ocsp enable

# 5. Verify status
redisctl enterprise ocsp status
```

### Troubleshooting Certificate Issues

```bash
# Check if OCSP is causing connection issues
redisctl enterprise ocsp status

# Temporarily disable for testing
redisctl enterprise ocsp disable

# Test certificates manually
openssl ocsp -issuer issuer.crt \
  -cert server.crt \
  -url http://ocsp.company.com \
  -resp_text

# Re-enable after fixing
redisctl enterprise ocsp enable
```

### Monitoring OCSP Health

```bash
#!/bin/bash
# Monitor OCSP status and alert on failures

while true; do
  STATUS=$(redisctl enterprise ocsp status -q 'validation_status')
  
  if [[ "$STATUS" != "healthy" ]]; then
    echo "OCSP validation unhealthy: $STATUS"
    # Send alert
  fi
  
  sleep 300
done
```

## Configuration Parameters

| Parameter | Description | Default |
|-----------|-------------|---------|
| `ocsp_functionality` | Enable/disable OCSP | false |
| `responder_url` | OCSP responder endpoint | "" |
| `query_frequency` | Seconds between OCSP queries | 3600 |
| `response_timeout` | Timeout for OCSP responses | 1 |
| `recovery_frequency` | Retry interval on failure | 60 |
| `recovery_max_tries` | Maximum retry attempts | 5 |

## Security Best Practices

1. **Use HTTPS for OCSP Responder**
   ```bash
   redisctl enterprise ocsp update --data '{
     "responder_url": "https://ocsp.company.com"
   }'
   ```

2. **Configure Appropriate Timeouts**
   - Balance between security and availability
   - Consider network latency to responder

3. **Monitor OCSP Health**
   - Set up alerts for OCSP failures
   - Track response times and success rates

4. **Test Before Production**
   - Verify responder connectivity
   - Test with actual certificates
   - Check failover behavior

## Troubleshooting

### OCSP Responder Unreachable

```bash
# Check network connectivity
curl -I http://ocsp.company.com

# Verify DNS resolution
nslookup ocsp.company.com

# Test with OpenSSL
openssl ocsp -url http://ocsp.company.com -timeout 5
```

### Certificate Validation Failures

```bash
# Get detailed status
redisctl enterprise ocsp status -o json

# Check logs for OCSP errors
redisctl enterprise logs list --filter "OCSP"

# Test specific certificate
redisctl enterprise ocsp test --data '{
  "certificate": "-----BEGIN CERTIFICATE-----..."
}'
```

### Performance Impact

```bash
# Monitor query times
redisctl enterprise ocsp status -q 'average_response_time'

# Adjust query frequency if needed
redisctl enterprise ocsp update --data '{
  "query_frequency": 7200
}'
```

## Output Examples

### Configuration Output

```json
{
  "ocsp_functionality": true,
  "responder_url": "http://ocsp.company.com",
  "query_frequency": 3600,
  "response_timeout": 5,
  "recovery_frequency": 60,
  "recovery_max_tries": 5,
  "last_check": "2025-09-15T10:30:00Z",
  "next_check": "2025-09-15T11:30:00Z"
}
```

### Status Output

```json
{
  "enabled": true,
  "validation_status": "healthy",
  "certificates_checked": 12,
  "certificates_valid": 12,
  "certificates_revoked": 0,
  "last_success": "2025-09-15T10:30:00Z",
  "failures_count": 0
}
```

## Related Commands

- [Cluster Commands](cluster.md) - Cluster security settings
- [Auth Commands](auth.md) - Authentication configuration
- [Certificate Commands](cluster.md#certificates) - Certificate management