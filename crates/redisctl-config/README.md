# redisctl-config

Configuration and profile management for Redis CLI tools.

## Overview

This library provides a reusable configuration and profile system for managing
credentials and settings for Redis Cloud and Redis Enterprise deployments.

## Features

- Multiple named profiles for different Redis deployments
- Secure credential storage using OS keyring (optional)
- Environment variable expansion in config files
- Platform-specific config file locations
- Support for both Redis Cloud and Redis Enterprise

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
redisctl-config = "0.1"

# Optional: Enable secure credential storage
redisctl-config = { version = "0.1", features = ["secure-storage"] }
```

## Usage

### Basic Configuration

```rust
use redisctl_config::{Config, Profile, DeploymentType, ProfileCredentials};

// Load configuration from standard location
let config = Config::load()?;

// Create a new Cloud profile
let profile = Profile {
    deployment_type: DeploymentType::Cloud,
    credentials: ProfileCredentials::Cloud {
        api_key: "your-api-key".to_string(),
        api_secret: "your-secret".to_string(),
        api_url: "https://api.redislabs.com/v1".to_string(),
    },
    files_api_key: None,
};

// Add profile to config
let mut config = Config::default();
config.set_profile("production".to_string(), profile);
config.save()?;
```

### Profile Resolution

```rust
// Resolve which profile to use for Cloud operations
let profile_name = config.resolve_cloud_profile(None)?;
let profile = config.profiles.get(&profile_name).unwrap();

// Get credentials with keyring support
if let Some((api_key, api_secret, api_url)) = profile.resolve_cloud_credentials()? {
    println!("API URL: {}", api_url);
}
```

### Credential Storage

```rust
use redisctl_config::CredentialStore;

let store = CredentialStore::new();

// Store credential in keyring (requires secure-storage feature)
#[cfg(feature = "secure-storage")]
let reference = store.store_credential("my-key", "secret-value")?;

// Retrieve credential (with environment variable fallback)
let value = store.get_credential("${MY_VAR}", Some("MY_VAR"))?;
```

## Environment Variables

Credentials can reference environment variables in the config file:

```toml
[profiles.production]
deployment_type = "cloud"
api_key = "${REDIS_CLOUD_API_KEY}"
api_secret = "${REDIS_CLOUD_SECRET_KEY}"
api_url = "${REDIS_CLOUD_API_URL:-https://api.redislabs.com/v1}"
```

## Config File Location

The config file is automatically placed in platform-specific locations:

- **Linux/macOS**: `~/.config/redisctl/config.toml`
- **macOS** (fallback): `~/Library/Application Support/com.redis.redisctl/config.toml`
- **Windows**: `%APPDATA%\redis\redisctl\config.toml`

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
