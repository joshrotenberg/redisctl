# LDAP Integration Commands

Configure and manage LDAP authentication for Redis Enterprise.

## Overview

LDAP commands enable integration with Active Directory and other LDAP services for centralized authentication and authorization in Redis Enterprise clusters.

## LDAP Configuration Commands

### Get LDAP Configuration

```bash
redisctl enterprise ldap get
```

Retrieves the current LDAP configuration for the cluster.

### Update LDAP Configuration

```bash
redisctl enterprise ldap update --data '{
  "server_url": "ldaps://ldap.company.com:636",
  "bind_dn": "CN=redis,OU=ServiceAccounts,DC=company,DC=com",
  "bind_password": "password",
  "user_search_base": "OU=Users,DC=company,DC=com",
  "user_search_filter": "(sAMAccountName={0})",
  "group_search_base": "OU=Groups,DC=company,DC=com",
  "group_search_filter": "(member={0})"
}'
```

### Delete LDAP Configuration

```bash
redisctl enterprise ldap delete
```

Removes LDAP configuration, reverting to local authentication only.

### Test LDAP Connection

```bash
redisctl enterprise ldap test
```

Tests the LDAP connection and configuration.

## LDAP Mappings Commands

LDAP mappings define how LDAP groups map to Redis Enterprise roles.

### List LDAP Mappings

```bash
redisctl enterprise ldap-mappings list
```

Shows all configured LDAP group-to-role mappings.

### Get Specific Mapping

```bash
redisctl enterprise ldap-mappings get <uid>
```

### Create LDAP Mapping

```bash
redisctl enterprise ldap-mappings create --data '{
  "ldap_group": "CN=Redis-Admins,OU=Groups,DC=company,DC=com",
  "role": "admin",
  "email": "redis-admins@company.com"
}'
```

Maps an LDAP group to a Redis Enterprise role.

### Update LDAP Mapping

```bash
redisctl enterprise ldap-mappings update <uid> --data '{
  "role": "db_admin"
}'
```

### Delete LDAP Mapping

```bash
redisctl enterprise ldap-mappings delete <uid>
```

## Common Use Cases

### Setting Up Active Directory Integration

```bash
# 1. Configure LDAP connection
redisctl enterprise ldap update --data '{
  "server_url": "ldaps://dc01.company.com:636",
  "bind_dn": "CN=Redis Service,OU=Services,DC=company,DC=com",
  "bind_password": "${LDAP_BIND_PASSWORD}",
  "user_search_base": "OU=Users,DC=company,DC=com",
  "user_search_filter": "(sAMAccountName={0})",
  "certificate": "-----BEGIN CERTIFICATE-----..."
}'

# 2. Test the connection
redisctl enterprise ldap test

# 3. Create role mappings
redisctl enterprise ldap-mappings create --data '{
  "ldap_group": "CN=Redis-Admins,OU=Groups,DC=company,DC=com",
  "role": "admin"
}'

redisctl enterprise ldap-mappings create --data '{
  "ldap_group": "CN=Redis-Users,OU=Groups,DC=company,DC=com",
  "role": "db_viewer"
}'
```

### Troubleshooting LDAP Authentication

```bash
# Check current configuration
redisctl enterprise ldap get

# Test with specific user (requires additional test data)
redisctl enterprise ldap test --data '{
  "username": "testuser",
  "password": "testpass"
}'

# View all mappings
redisctl enterprise ldap-mappings list -o table
```

### Migrating from Local to LDAP Authentication

```bash
# 1. Keep local admin account active
redisctl enterprise user update admin@redis.local --data '{
  "auth_method": "local"
}'

# 2. Configure LDAP
redisctl enterprise ldap update --data @ldap-config.json

# 3. Create mappings for existing roles
redisctl enterprise role list -q '[].name' | while read role; do
  echo "Map LDAP group for role: $role"
done

# 4. Test LDAP authentication before disabling local auth
redisctl enterprise ldap test
```

## Configuration Examples

### Basic Active Directory

```json
{
  "server_url": "ldaps://ad.company.com:636",
  "bind_dn": "redis-service@company.com",
  "bind_password": "password",
  "user_search_base": "DC=company,DC=com",
  "user_search_filter": "(sAMAccountName={0})",
  "group_search_base": "DC=company,DC=com",
  "group_search_filter": "(member={0})"
}
```

### OpenLDAP

```json
{
  "server_url": "ldap://openldap.company.com:389",
  "bind_dn": "cn=admin,dc=company,dc=com",
  "bind_password": "password",
  "user_search_base": "ou=people,dc=company,dc=com",
  "user_search_filter": "(uid={0})",
  "group_search_base": "ou=groups,dc=company,dc=com",
  "group_search_filter": "(memberUid={0})"
}
```

## Security Considerations

- Always use LDAPS (LDAP over SSL) for production
- Store bind passwords in environment variables or secrets management
- Use service accounts with minimal privileges for bind DN
- Regularly rotate bind account passwords
- Test configuration changes in non-production first

## Troubleshooting

### Connection Issues

```bash
# Check network connectivity
nc -zv ldap.company.com 636

# Verify certificate
openssl s_client -connect ldap.company.com:636 -showcerts

# Test with ldapsearch
ldapsearch -H ldaps://ldap.company.com:636 \
  -D "CN=redis,OU=Services,DC=company,DC=com" \
  -w password \
  -b "DC=company,DC=com" \
  "(sAMAccountName=testuser)"
```

### Authentication Failures

1. Verify bind DN and password
2. Check user search base and filter
3. Ensure group memberships are correct
4. Review Redis Enterprise logs
5. Test with `ldap test` command

## Related Commands

- User Commands - Manage local users
- Role Commands - Configure roles and permissions
- Auth Commands - Authentication settings