//! Signer whitelist management for TDF documents
//! Allows organizations to define trusted signers and validate signatures against them
//!
//! Security Fixes:
//! - CVE-TDF-024: Whitelist public key binding validation

use crate::error::{TdfError, TdfResult};
use ed25519_dalek::VerifyingKey;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Result of validating a signer against the whitelist
#[derive(Debug, Clone, PartialEq)]
pub enum WhitelistValidationResult {
    /// Signer is trusted and key matches (if key binding exists)
    Trusted {
        signer_name: String,
        roles: Vec<String>,
    },
    /// Signer ID found but public key doesn't match
    KeyMismatch {
        expected_key: String,
        actual_key: String,
    },
    /// Signer ID not found in whitelist
    NotFound,
    /// Signer found but no key binding to validate
    TrustedNoKeyBinding {
        signer_name: String,
        roles: Vec<String>,
    },
}

/// A whitelist of trusted signers for an organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignerWhitelist {
    /// Name of the whitelist (e.g., "ACME Corp Trusted Signers")
    pub name: String,
    /// Description of the whitelist
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// List of trusted signers
    pub trusted_signers: Vec<TrustedSigner>,
}

/// Information about a trusted signer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustedSigner {
    /// Signer ID (typically a DID, e.g., "did:web:cfo.acme.com")
    pub id: String,
    /// Human-readable name (e.g., "CFO Jane Smith")
    pub name: String,
    /// Optional hex-encoded public key for additional verification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    /// Roles this signer is authorized for (e.g., ["approver", "auditor"])
    #[serde(default)]
    pub roles: Vec<String>,
    /// Optional email for contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl SignerWhitelist {
    /// Create a new empty whitelist
    pub fn new(name: String) -> Self {
        SignerWhitelist {
            name,
            description: None,
            trusted_signers: Vec::new(),
        }
    }

    /// Check if a signer ID is in the whitelist
    pub fn is_trusted(&self, signer_id: &str) -> bool {
        self.trusted_signers.iter().any(|s| s.id == signer_id)
    }

    /// Get a trusted signer by ID
    pub fn get_signer(&self, signer_id: &str) -> Option<&TrustedSigner> {
        self.trusted_signers.iter().find(|s| s.id == signer_id)
    }

    /// Get all trusted signer IDs as a set
    pub fn trusted_ids(&self) -> HashSet<&str> {
        self.trusted_signers.iter().map(|s| s.id.as_str()).collect()
    }

    /// Add a trusted signer to the whitelist
    pub fn add_signer(&mut self, signer: TrustedSigner) {
        if !self.is_trusted(&signer.id) {
            self.trusted_signers.push(signer);
        }
    }

    /// Remove a signer from the whitelist by ID
    pub fn remove_signer(&mut self, signer_id: &str) -> bool {
        let initial_len = self.trusted_signers.len();
        self.trusted_signers.retain(|s| s.id != signer_id);
        self.trusted_signers.len() < initial_len
    }

    /// Check if a signer has a specific role
    pub fn has_role(&self, signer_id: &str, role: &str) -> bool {
        self.get_signer(signer_id)
            .map(|s| s.roles.iter().any(|r| r == role))
            .unwrap_or(false)
    }

    /// Load whitelist from JSON bytes
    pub fn from_json(data: &[u8]) -> TdfResult<Self> {
        serde_json::from_slice(data)
            .map_err(|e| TdfError::InvalidDocument(format!("Invalid whitelist JSON: {}", e)))
    }

    /// Load whitelist from JSON file
    pub fn from_json_file(path: &std::path::Path) -> TdfResult<Self> {
        let data = std::fs::read(path)?;
        Self::from_json(&data)
    }

    /// Serialize whitelist to JSON bytes
    pub fn to_json(&self) -> TdfResult<Vec<u8>> {
        serde_json::to_vec_pretty(self)
            .map_err(|e| TdfError::InvalidDocument(format!("Failed to serialize whitelist: {}", e)))
    }

    /// Validate a list of signer IDs against this whitelist
    /// Returns a tuple of (trusted_signers, untrusted_signers)
    pub fn validate_signers<'a>(&self, signer_ids: &'a [String]) -> (Vec<&'a str>, Vec<&'a str>) {
        let mut trusted = Vec::new();
        let mut untrusted = Vec::new();

        for id in signer_ids {
            if self.is_trusted(id) {
                trusted.push(id.as_str());
            } else {
                untrusted.push(id.as_str());
            }
        }

        (trusted, untrusted)
    }

    /// Validate a signer with public key binding
    ///
    /// Security Fix (CVE-TDF-024): Validates that the signer's public key matches
    /// the key binding in the whitelist (if present). This prevents an attacker
    /// from impersonating a trusted signer ID with a different key.
    pub fn validate_signer_key(
        &self,
        signer_id: &str,
        public_key: &VerifyingKey,
    ) -> WhitelistValidationResult {
        let signer = match self.get_signer(signer_id) {
            Some(s) => s,
            None => return WhitelistValidationResult::NotFound,
        };

        // If whitelist has public key binding, verify it matches
        if let Some(expected_key_hex) = &signer.public_key {
            let actual_key_hex = hex::encode(public_key.as_bytes());

            if actual_key_hex.to_lowercase() != expected_key_hex.to_lowercase() {
                return WhitelistValidationResult::KeyMismatch {
                    expected_key: expected_key_hex.clone(),
                    actual_key: actual_key_hex,
                };
            }

            // Key matches
            WhitelistValidationResult::Trusted {
                signer_name: signer.name.clone(),
                roles: signer.roles.clone(),
            }
        } else {
            // No key binding - trusted by ID only
            WhitelistValidationResult::TrustedNoKeyBinding {
                signer_name: signer.name.clone(),
                roles: signer.roles.clone(),
            }
        }
    }

    /// Validate a signer with public key binding (strict mode)
    ///
    /// Returns an error if the signer is not found, key doesn't match, or
    /// there's no key binding in the whitelist.
    pub fn validate_signer_key_strict(
        &self,
        signer_id: &str,
        public_key: &VerifyingKey,
    ) -> TdfResult<&TrustedSigner> {
        let signer = self.get_signer(signer_id)
            .ok_or_else(|| TdfError::UntrustedSigner(format!(
                "Signer {} not found in whitelist", signer_id
            )))?;

        // In strict mode, key binding is required
        let expected_key_hex = signer.public_key.as_ref()
            .ok_or_else(|| TdfError::PolicyViolation(format!(
                "Signer {} has no public key binding in whitelist (required in strict mode)",
                signer_id
            )))?;

        let actual_key_hex = hex::encode(public_key.as_bytes());

        if actual_key_hex.to_lowercase() != expected_key_hex.to_lowercase() {
            return Err(TdfError::UntrustedSigner(format!(
                "Public key mismatch for signer {}: expected {}, got {}",
                signer_id,
                &expected_key_hex[..16.min(expected_key_hex.len())],
                &actual_key_hex[..16.min(actual_key_hex.len())]
            )));
        }

        Ok(signer)
    }
}

impl TrustedSigner {
    /// Create a new trusted signer with minimal information
    pub fn new(id: String, name: String) -> Self {
        TrustedSigner {
            id,
            name,
            public_key: None,
            roles: Vec::new(),
            email: None,
        }
    }

    /// Create a trusted signer with roles
    pub fn with_roles(id: String, name: String, roles: Vec<String>) -> Self {
        TrustedSigner {
            id,
            name,
            public_key: None,
            roles,
            email: None,
        }
    }

    /// Create a trusted signer with public key binding
    ///
    /// Security Fix (CVE-TDF-024): Binds a public key to the signer ID
    pub fn with_key(id: String, name: String, public_key: &VerifyingKey) -> Self {
        TrustedSigner {
            id,
            name,
            public_key: Some(hex::encode(public_key.as_bytes())),
            roles: Vec::new(),
            email: None,
        }
    }

    /// Create a trusted signer with public key binding and roles
    pub fn with_key_and_roles(
        id: String,
        name: String,
        public_key: &VerifyingKey,
        roles: Vec<String>,
    ) -> Self {
        TrustedSigner {
            id,
            name,
            public_key: Some(hex::encode(public_key.as_bytes())),
            roles,
            email: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_whitelist() -> SignerWhitelist {
        let mut whitelist = SignerWhitelist::new("Test Whitelist".to_string());
        whitelist.add_signer(TrustedSigner::with_roles(
            "did:web:cfo.acme.com".to_string(),
            "CFO Jane Smith".to_string(),
            vec!["financial-approver".to_string(), "auditor".to_string()],
        ));
        whitelist.add_signer(TrustedSigner::with_roles(
            "did:web:ceo.acme.com".to_string(),
            "CEO Bob Lee".to_string(),
            vec!["executive-approver".to_string()],
        ));
        whitelist
    }

    #[test]
    fn test_is_trusted() {
        let whitelist = create_test_whitelist();
        assert!(whitelist.is_trusted("did:web:cfo.acme.com"));
        assert!(whitelist.is_trusted("did:web:ceo.acme.com"));
        assert!(!whitelist.is_trusted("did:web:attacker.com"));
    }

    #[test]
    fn test_get_signer() {
        let whitelist = create_test_whitelist();
        let signer = whitelist.get_signer("did:web:cfo.acme.com").unwrap();
        assert_eq!(signer.name, "CFO Jane Smith");
        assert!(whitelist.get_signer("did:web:unknown.com").is_none());
    }

    #[test]
    fn test_has_role() {
        let whitelist = create_test_whitelist();
        assert!(whitelist.has_role("did:web:cfo.acme.com", "financial-approver"));
        assert!(whitelist.has_role("did:web:cfo.acme.com", "auditor"));
        assert!(!whitelist.has_role("did:web:cfo.acme.com", "executive-approver"));
        assert!(whitelist.has_role("did:web:ceo.acme.com", "executive-approver"));
    }

    #[test]
    fn test_validate_signers() {
        let whitelist = create_test_whitelist();
        let signer_ids = vec![
            "did:web:cfo.acme.com".to_string(),
            "did:web:attacker.com".to_string(),
            "did:web:ceo.acme.com".to_string(),
        ];

        let (trusted, untrusted) = whitelist.validate_signers(&signer_ids);
        assert_eq!(trusted.len(), 2);
        assert_eq!(untrusted.len(), 1);
        assert!(trusted.contains(&"did:web:cfo.acme.com"));
        assert!(untrusted.contains(&"did:web:attacker.com"));
    }

    #[test]
    fn test_json_serialization() {
        let whitelist = create_test_whitelist();
        let json = whitelist.to_json().unwrap();
        let restored = SignerWhitelist::from_json(&json).unwrap();

        assert_eq!(restored.name, whitelist.name);
        assert_eq!(restored.trusted_signers.len(), whitelist.trusted_signers.len());
        assert!(restored.is_trusted("did:web:cfo.acme.com"));
    }

    #[test]
    fn test_add_remove_signer() {
        let mut whitelist = SignerWhitelist::new("Test".to_string());
        assert!(!whitelist.is_trusted("did:web:test.com"));

        whitelist.add_signer(TrustedSigner::new(
            "did:web:test.com".to_string(),
            "Test User".to_string(),
        ));
        assert!(whitelist.is_trusted("did:web:test.com"));

        // Adding same signer again should not duplicate
        whitelist.add_signer(TrustedSigner::new(
            "did:web:test.com".to_string(),
            "Test User".to_string(),
        ));
        assert_eq!(whitelist.trusted_signers.len(), 1);

        // Remove signer
        assert!(whitelist.remove_signer("did:web:test.com"));
        assert!(!whitelist.is_trusted("did:web:test.com"));
        assert!(!whitelist.remove_signer("did:web:test.com")); // Already removed
    }

    #[test]
    fn test_validate_signer_key_matching() {
        use ed25519_dalek::SigningKey;
        use rand::rngs::OsRng;

        // Generate a key pair
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();

        // Create whitelist with key binding
        let mut whitelist = SignerWhitelist::new("Test".to_string());
        whitelist.add_signer(TrustedSigner::with_key(
            "did:web:test.com".to_string(),
            "Test Signer".to_string(),
            &verifying_key,
        ));

        // Validate with correct key
        let result = whitelist.validate_signer_key("did:web:test.com", &verifying_key);
        assert!(matches!(result, WhitelistValidationResult::Trusted { .. }));
    }

    #[test]
    fn test_validate_signer_key_mismatch() {
        use ed25519_dalek::SigningKey;
        use rand::rngs::OsRng;

        // Generate two different key pairs
        let signing_key1 = SigningKey::generate(&mut OsRng);
        let verifying_key1 = signing_key1.verifying_key();
        let signing_key2 = SigningKey::generate(&mut OsRng);
        let verifying_key2 = signing_key2.verifying_key();

        // Create whitelist with key1 binding
        let mut whitelist = SignerWhitelist::new("Test".to_string());
        whitelist.add_signer(TrustedSigner::with_key(
            "did:web:test.com".to_string(),
            "Test Signer".to_string(),
            &verifying_key1,
        ));

        // Validate with key2 (wrong key) - should detect mismatch
        let result = whitelist.validate_signer_key("did:web:test.com", &verifying_key2);
        assert!(matches!(result, WhitelistValidationResult::KeyMismatch { .. }));
    }

    #[test]
    fn test_validate_signer_key_no_binding() {
        use ed25519_dalek::SigningKey;
        use rand::rngs::OsRng;

        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();

        // Create whitelist without key binding
        let mut whitelist = SignerWhitelist::new("Test".to_string());
        whitelist.add_signer(TrustedSigner::new(
            "did:web:test.com".to_string(),
            "Test Signer".to_string(),
        ));

        // Validate with any key - should return TrustedNoKeyBinding
        let result = whitelist.validate_signer_key("did:web:test.com", &verifying_key);
        assert!(matches!(result, WhitelistValidationResult::TrustedNoKeyBinding { .. }));
    }

    #[test]
    fn test_validate_signer_key_not_found() {
        use ed25519_dalek::SigningKey;
        use rand::rngs::OsRng;

        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();

        let whitelist = SignerWhitelist::new("Test".to_string());

        // Validate unknown signer
        let result = whitelist.validate_signer_key("did:web:unknown.com", &verifying_key);
        assert!(matches!(result, WhitelistValidationResult::NotFound));
    }

    #[test]
    fn test_validate_signer_key_strict_requires_binding() {
        use ed25519_dalek::SigningKey;
        use rand::rngs::OsRng;

        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();

        // Create whitelist without key binding
        let mut whitelist = SignerWhitelist::new("Test".to_string());
        whitelist.add_signer(TrustedSigner::new(
            "did:web:test.com".to_string(),
            "Test Signer".to_string(),
        ));

        // Strict mode should fail because no key binding
        let result = whitelist.validate_signer_key_strict("did:web:test.com", &verifying_key);
        assert!(result.is_err());
    }
}
