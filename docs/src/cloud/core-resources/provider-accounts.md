# Cloud Provider Accounts

Cloud provider accounts link your AWS, GCP, or Azure accounts to Redis Cloud for features like VPC peering and BYOK encryption.

## Commands Overview

```bash
redisctl cloud provider-account --help
```

## List Provider Accounts

```bash
# List all linked cloud provider accounts
redisctl cloud provider-account list

# Output as JSON
redisctl cloud provider-account list -o json
```

## Get Provider Account Details

```bash
# Get details for a specific provider account
redisctl cloud provider-account get <provider_account_id>
```

## Create Provider Account

Link a new cloud provider account to Redis Cloud:

```bash
# Create an AWS provider account
redisctl cloud provider-account create --data '{
  "name": "production-aws",
  "provider": "AWS",
  "accessKeyId": "AKIAIOSFODNN7EXAMPLE",
  "accessSecretKey": "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"
}'

# Create a GCP provider account
redisctl cloud provider-account create --data '{
  "name": "production-gcp",
  "provider": "GCP",
  "credentials": "<service-account-json>"
}'
```

## Update Provider Account

```bash
# Update provider account credentials or settings
redisctl cloud provider-account update <provider_account_id> --data '{
  "name": "production-aws-renamed"
}'
```

## Delete Provider Account

```bash
# Remove a provider account link
redisctl cloud provider-account delete <provider_account_id>
```

## Use Cases

### VPC Peering

Provider accounts are required to set up VPC peering between your cloud infrastructure and Redis Cloud:

```bash
# 1. First, link your AWS account
redisctl cloud provider-account create --data '{...}'

# 2. Then create VPC peering
redisctl cloud connectivity vpc-peering create <subscription_id> --data '{...}'
```

### BYOK Encryption

Bring Your Own Key (BYOK) encryption requires a linked cloud provider account to access your KMS keys.

## JSON Output

```bash
# List all provider accounts with their providers
redisctl cloud provider-account list -o json | jq '.[] | {name, provider, id}'
```
