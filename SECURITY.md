# Security Policy

## Supported Versions

We release patches for security vulnerabilities for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| Latest  | :white_check_mark: |
| < Latest| :x:                |

We recommend always using the latest version to ensure you have the most recent security updates.

## Reporting a Vulnerability

If you discover a security vulnerability in redisctl, please report it privately to help us address it before public disclosure.

**Please do NOT report security vulnerabilities through public GitHub issues.**

### How to Report

1. **Email**: Send details to the project maintainers via GitHub
2. **Include**:
   - Description of the vulnerability
   - Steps to reproduce the issue
   - Potential impact
   - Any suggested fixes (if available)

### What to Expect

- **Acknowledgment**: We will acknowledge receipt of your report within 48 hours
- **Updates**: We will provide regular updates on our progress
- **Timeline**: We aim to release a fix within 90 days of the initial report
- **Credit**: We will credit you in the security advisory (unless you prefer to remain anonymous)

### Security Best Practices

When using redisctl:

1. **Credentials**: Use the `--use-keyring` flag with `secure-storage` feature to store credentials securely
2. **Permissions**: Store configuration files with restrictive permissions (e.g., `chmod 600 ~/.config/redisctl/config.toml`)
3. **TLS**: Always use HTTPS endpoints; only use `REDIS_ENTERPRISE_INSECURE=true` for testing
4. **Environment Variables**: Be cautious when using environment variables for credentials in shared environments
5. **Updates**: Keep redisctl updated to the latest version

## Known Security Considerations

- Credentials stored in plain text configuration files are readable by any process with access to the file
- The `--insecure` flag disables TLS certificate verification and should only be used in development
- API keys and secrets in command-line arguments may be visible in process listings

For enhanced security, we recommend:
- Using the `secure-storage` feature for credential management
- Configuring proper file system permissions
- Using environment variables or profiles instead of command-line flags for sensitive data
