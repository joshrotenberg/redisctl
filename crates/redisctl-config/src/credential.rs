//! Credential storage abstraction with optional keyring support
//!
//! This module provides a unified interface for storing and retrieving credentials,
//! with support for:
//! - OS keyring (when feature enabled)
//! - Plaintext storage (fallback)
//! - Environment variable override

use crate::error::{ConfigError, Result};
use std::env;

/// Prefix that indicates a value should be retrieved from the keyring
const KEYRING_PREFIX: &str = "keyring:";

/// Service name for keyring entries
#[cfg(feature = "secure-storage")]
const SERVICE_NAME: &str = "redisctl";

/// Storage backend for credentials
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum CredentialStorage {
    /// Store in OS keyring
    #[cfg(feature = "secure-storage")]
    Keyring,
    /// Store as plaintext
    Plaintext,
}

/// Credential store abstraction
pub struct CredentialStore {
    #[allow(dead_code)]
    storage: CredentialStorage,
}

impl Default for CredentialStore {
    fn default() -> Self {
        Self::new()
    }
}

impl CredentialStore {
    /// Create a new credential store with automatic backend selection
    pub fn new() -> Self {
        #[cfg(feature = "secure-storage")]
        {
            // Try to use keyring if available
            if Self::is_keyring_available() {
                Self {
                    storage: CredentialStorage::Keyring,
                }
            } else {
                Self {
                    storage: CredentialStorage::Plaintext,
                }
            }
        }
        #[cfg(not(feature = "secure-storage"))]
        {
            Self {
                storage: CredentialStorage::Plaintext,
            }
        }
    }

    /// Check if keyring is available on this system
    #[cfg(feature = "secure-storage")]
    fn is_keyring_available() -> bool {
        // Try to create a test entry to see if keyring works
        match keyring::Entry::new(SERVICE_NAME, "__test__") {
            Ok(entry) => {
                // Try to get a non-existent password (should fail gracefully)
                let _ = entry.get_password();
                true
            }
            Err(_) => false,
        }
    }

    /// Store a credential value
    #[allow(dead_code)]
    pub fn store_credential(&self, key: &str, value: &str) -> Result<String> {
        #[cfg(feature = "secure-storage")]
        {
            match self.storage {
                CredentialStorage::Keyring => {
                    let entry = keyring::Entry::new(SERVICE_NAME, key)
                        .map_err(|e| ConfigError::KeyringError(e.to_string()))?;
                    entry.set_password(value).map_err(|e| {
                        ConfigError::KeyringError(format!(
                            "Failed to store credential in keyring: {}",
                            e
                        ))
                    })?;
                    // Return the reference string that will be stored in config
                    Ok(format!("{}{}", KEYRING_PREFIX, key))
                }
                CredentialStorage::Plaintext => Ok(value.to_string()),
            }
        }
        #[cfg(not(feature = "secure-storage"))]
        {
            // Without secure-storage feature, always use plaintext
            let _ = key; // Not used without secure-storage
            Ok(value.to_string())
        }
    }

    /// Retrieve a credential value
    ///
    /// Resolution order:
    /// 1. Check environment variable (if env_var provided)
    /// 2. If value starts with "keyring:", retrieve from keyring
    /// 3. Otherwise, return the value as-is (plaintext)
    pub fn get_credential(&self, value: &str, env_var: Option<&str>) -> Result<String> {
        // First check environment variable if provided
        if let Some(var) = env_var
            && let Ok(env_value) = env::var(var)
        {
            return Ok(env_value);
        }

        // Check if this is a keyring reference
        if value.starts_with(KEYRING_PREFIX) {
            #[cfg(feature = "secure-storage")]
            {
                let key = value.trim_start_matches(KEYRING_PREFIX);
                let entry = keyring::Entry::new(SERVICE_NAME, key)
                    .map_err(|e| ConfigError::KeyringError(e.to_string()))?;
                entry.get_password().map_err(|e| {
                    ConfigError::KeyringError(format!(
                        "Failed to retrieve credential '{}' from keyring: {}",
                        key, e
                    ))
                })
            }
            #[cfg(not(feature = "secure-storage"))]
            {
                Err(ConfigError::CredentialError(
                    "Credential references keyring but secure-storage feature is not enabled"
                        .to_string(),
                ))
            }
        } else {
            // Plain text value
            Ok(value.to_string())
        }
    }

    /// Delete a credential from storage
    #[allow(dead_code)]
    pub fn delete_credential(&self, key: &str) -> Result<()> {
        #[cfg(feature = "secure-storage")]
        {
            match self.storage {
                CredentialStorage::Keyring => {
                    let entry = keyring::Entry::new(SERVICE_NAME, key)
                        .map_err(|e| ConfigError::KeyringError(e.to_string()))?;
                    match entry.delete_credential() {
                        Ok(()) => Ok(()),
                        Err(keyring::Error::NoEntry) => Ok(()), // Already deleted
                        Err(e) => Err(ConfigError::KeyringError(format!(
                            "Failed to delete credential from keyring: {}",
                            e
                        ))),
                    }
                }
                CredentialStorage::Plaintext => Ok(()), // Nothing to delete for plaintext
            }
        }
        #[cfg(not(feature = "secure-storage"))]
        {
            let _ = key; // Not used without secure-storage
            Ok(()) // Nothing to delete for plaintext
        }
    }

    /// Check if a value is a keyring reference
    #[allow(dead_code)]
    pub fn is_keyring_reference(value: &str) -> bool {
        value.starts_with(KEYRING_PREFIX)
    }

    /// Get the current storage backend
    #[allow(dead_code)]
    pub fn storage_backend(&self) -> &str {
        #[cfg(feature = "secure-storage")]
        {
            match self.storage {
                CredentialStorage::Keyring => "keyring",
                CredentialStorage::Plaintext => "plaintext",
            }
        }
        #[cfg(not(feature = "secure-storage"))]
        {
            "plaintext"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plaintext_storage() {
        let store = CredentialStore::new();

        // Plaintext values should be returned as-is
        let result = store.get_credential("my-api-key", None).unwrap();
        assert_eq!(result, "my-api-key");
    }

    #[test]
    fn test_env_var_override() {
        unsafe {
            env::set_var("TEST_CREDENTIAL", "env-value");
        }

        let store = CredentialStore::new();
        let result = store
            .get_credential("config-value", Some("TEST_CREDENTIAL"))
            .unwrap();
        assert_eq!(result, "env-value");

        unsafe {
            env::remove_var("TEST_CREDENTIAL");
        }
    }

    #[test]
    fn test_keyring_reference_detection() {
        assert!(CredentialStore::is_keyring_reference("keyring:my-key"));
        assert!(!CredentialStore::is_keyring_reference("my-key"));
        assert!(!CredentialStore::is_keyring_reference(""));
    }

    #[cfg(feature = "secure-storage")]
    #[test]
    #[ignore = "Requires keyring service to be available"]
    fn test_keyring_storage() {
        let store = CredentialStore::new();

        // Store a credential
        let key = "test-credential";
        let value = "test-value";
        let reference = store.store_credential(key, value).unwrap();

        // Should return a keyring reference
        assert!(reference.starts_with(KEYRING_PREFIX));

        // Retrieve it back
        let retrieved = store.get_credential(&reference, None).unwrap();
        assert_eq!(retrieved, value);

        // Clean up
        let _ = store.delete_credential(key);
    }
}
