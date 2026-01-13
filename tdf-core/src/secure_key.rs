//! Secure key management with automatic zeroization
//!
//! Security Fixes:
//! - CVE-TDF-026: Key material zeroization after use
//! - CVE-TDF-025: Secure key storage and handling
//!
//! This module provides secure key handling that automatically zeroizes
//! sensitive key material when it goes out of scope, preventing key material
//! from remaining in memory after use.

use zeroize::{Zeroize, ZeroizeOnDrop};

/// Secure key container that automatically zeroizes on drop
///
/// Security Fix (CVE-TDF-026): Ensures key material is zeroized when
/// the key goes out of scope, preventing key material from remaining in memory.
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct SecureKey {
    key: Vec<u8>,
}

impl SecureKey {
    /// Create a new secure key from bytes
    ///
    /// # Arguments
    /// * `key_bytes` - The key material (will be zeroized on drop)
    ///
    /// # Returns
    /// * `SecureKey` instance that will automatically zeroize on drop
    pub fn new(key_bytes: Vec<u8>) -> Self {
        SecureKey { key: key_bytes }
    }

    /// Get a reference to the key bytes
    ///
    /// # Security Note
    /// The key bytes are still in memory. Use `consume_and_zeroize` if you
    /// need to extract the key and ensure it's zeroized immediately.
    pub fn as_bytes(&self) -> &[u8] {
        &self.key
    }

    /// Get the key length
    pub fn len(&self) -> usize {
        self.key.len()
    }

    /// Check if the key is empty
    pub fn is_empty(&self) -> bool {
        self.key.is_empty()
    }

    /// Consume the key and return the bytes, then zeroize
    ///
    /// # Security Note
    /// This method extracts the key bytes and immediately zeroizes the
    /// internal storage. However, the returned Vec still contains the key
    /// material until it goes out of scope. Use with caution.
    pub fn into_bytes(mut self) -> Vec<u8> {
        let bytes = std::mem::take(&mut self.key);
        // self.key is now empty, and will be zeroized on drop
        bytes
    }

    /// Explicitly zeroize the key
    ///
    /// This is called automatically on drop, but can be called manually
    /// if you need to zeroize the key before it goes out of scope.
    pub fn zeroize(&mut self) {
        self.key.zeroize();
    }
}

impl Clone for SecureKey {
    fn clone(&self) -> Self {
        // Clone the key bytes
        SecureKey::new(self.key.clone())
    }
}

/// Secure key derivation result
///
/// Security Fix (CVE-TDF-026): Wraps derived keys in SecureKey for
/// automatic zeroization.
pub struct SecureDerivedKey {
    key: SecureKey,
}

impl SecureDerivedKey {
    /// Create a new secure derived key
    pub fn new(key: SecureKey) -> Self {
        SecureDerivedKey { key }
    }

    /// Get a reference to the key bytes
    pub fn as_bytes(&self) -> &[u8] {
        self.key.as_bytes()
    }

    /// Consume and return the secure key
    ///
    /// Note: This clones the key since we can't move out of a type that implements Drop.
    /// The original key will still be zeroized when SecureDerivedKey is dropped.
    pub fn into_secure_key(self) -> SecureKey {
        self.key.clone()
    }
}

impl Drop for SecureDerivedKey {
    fn drop(&mut self) {
        // SecureKey will handle zeroization automatically
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_key_zeroization() {
        let key_bytes = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let key = SecureKey::new(key_bytes.clone());

        // Key should be accessible
        assert_eq!(key.as_bytes(), key_bytes.as_slice());

        // Note: Testing explicit zeroization is not possible with ZeroizeOnDrop
        // The key will be automatically zeroized when it goes out of scope
        // This test mainly ensures the key works correctly during its lifetime
    }

    #[test]
    fn test_secure_key_automatic_zeroization() {
        let key_bytes = vec![1u8, 2, 3, 4, 5];
        {
            let key = SecureKey::new(key_bytes.clone());
            assert_eq!(key.as_bytes(), key_bytes.as_slice());
            // Key goes out of scope here and should be zeroized
        }
        // Key is now zeroized (we can't verify this directly, but zeroize
        // ensures it happens)
    }

    #[test]
    fn test_secure_key_clone() {
        let key_bytes = vec![1u8, 2, 3, 4, 5];
        let key1 = SecureKey::new(key_bytes.clone());
        let key2 = key1.clone();

        assert_eq!(key1.as_bytes(), key_bytes.as_slice());
        assert_eq!(key2.as_bytes(), key_bytes.as_slice());
    }
}
