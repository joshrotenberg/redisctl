# Redis Cloud API Fixtures

## Current Status

The Cloud API fixtures directory currently only contains the OpenAPI specification.

## Why No Real Fixtures Yet?

Unlike Enterprise API (which uses Docker for testing), Cloud API fixtures require:
1. A real Cloud account with active resources
2. Billable subscriptions and databases
3. Careful sanitization of account data before committing

## Generating Cloud Fixtures

When you have a Cloud account with test resources, you can generate fixtures:

```bash
export REDIS_CLOUD_API_KEY="your-key"
export REDIS_CLOUD_SECRET_KEY="your-secret"
./scripts/generate-cloud-fixtures.sh
```

**Important**: Review all generated fixtures for sensitive data before committing!

## Current Testing Approach

Cloud API tests currently use wiremock with inline JSON mocks. This approach:
- ✅ Works well for testing
- ✅ No infrastructure required
- ✅ No costs
- ⚠️  Doesn't catch type mismatches from real API responses

## Future Work

To get the full benefits of fixture-based testing for Cloud:
1. Use a test Cloud account with minimal resources
2. Generate fixtures from real API responses
3. Sanitize account/subscription IDs
4. Add validation tests like Enterprise has
