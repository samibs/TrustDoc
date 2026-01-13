//! Key revocation system for TDF format
//!
//! Supports Certificate Revocation Lists (CRL) and key expiration.
//!
//! # Security Features
//! - CVE-TDF-025: Cryptographic authority verification for revocation lists
//! - Signed revocation lists prevent unauthorized revocation attacks
//! - Authority identity is cryptographically bound to revocation entries

use crate::error::{TdfError, TdfResult};
use base64::{engine::general_purpose::STANDARD, Engine};
use chrono::{DateTime, Utc};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Reason code for key revocation (RFC 5280)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RevocationReason {
    Unspecified = 0,
    KeyCompromise = 1,
    CaCompromise = 2,
    AffiliationChanged = 3,
    Superseded = 4,
    CessationOfOperation = 5,
    CertificateHold = 6,
    RemoveFromCrl = 8,
    PrivilegeWithdrawn = 9,
    AaCompromise = 10,
}

impl Default for RevocationReason {
    fn default() -> Self {
        RevocationReason::Unspecified
    }
}

/// Single revocation entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevocationEntry {
    /// Signer ID (DID) that was revoked
    pub signer_id: String,
    /// When the key was revoked
    pub revoked_at: DateTime<Utc>,
    /// Reason for revocation
    pub reason: RevocationReason,
    /// Optional: When revocation was issued by authority
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<DateTime<Utc>>,
    /// Optional: Authority that issued revocation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
}

/// Revocation List (CRL-like structure)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevocationList {
    /// Version of revocation list format
    pub version: u8,
    /// When this revocation list was issued
    pub issued_at: DateTime<Utc>,
    /// When this revocation list expires (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_update: Option<DateTime<Utc>>,
    /// Authority that issued this list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// List of revoked keys
    pub revoked_keys: Vec<RevocationEntry>,
}

impl RevocationList {
    /// Create a new empty revocation list
    pub fn new() -> Self {
        RevocationList {
            version: 1,
            issued_at: Utc::now(),
            next_update: None,
            issuer: None,
            revoked_keys: Vec::new(),
        }
    }

    /// Check if a signer ID is revoked
    pub fn is_revoked(&self, signer_id: &str) -> Option<&RevocationEntry> {
        self.revoked_keys
            .iter()
            .find(|entry| entry.signer_id == signer_id)
    }

    /// Check if a signer ID is revoked at a specific time
    /// Returns the revocation entry if revoked, None if not revoked
    pub fn is_revoked_at(&self, signer_id: &str, check_time: DateTime<Utc>) -> Option<&RevocationEntry> {
        self.revoked_keys
            .iter()
            .find(|entry| {
                entry.signer_id == signer_id && entry.revoked_at <= check_time
            })
    }

    /// Add a revocation entry
    pub fn revoke(
        &mut self,
        signer_id: String,
        reason: RevocationReason,
        authority: Option<String>,
    ) {
        let entry = RevocationEntry {
            signer_id,
            revoked_at: Utc::now(),
            reason,
            issued_at: Some(Utc::now()),
            authority,
        };
        self.revoked_keys.push(entry);
        // Sort by revocation time (most recent first)
        self.revoked_keys.sort_by(|a, b| b.revoked_at.cmp(&a.revoked_at));
    }

    /// Remove a revocation (unrevoke)
    pub fn unrevoke(&mut self, signer_id: &str) -> bool {
        let initial_len = self.revoked_keys.len();
        self.revoked_keys.retain(|entry| entry.signer_id != signer_id);
        initial_len != self.revoked_keys.len()
    }

    /// Check if revocation list is expired
    pub fn is_expired(&self) -> bool {
        if let Some(next_update) = self.next_update {
            Utc::now() > next_update
        } else {
            false
        }
    }

    /// Get all revoked signer IDs as a set for fast lookup
    pub fn revoked_ids(&self) -> std::collections::HashSet<&str> {
        self.revoked_keys.iter().map(|e| e.signer_id.as_str()).collect()
    }
}

impl Default for RevocationList {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// CRYPTOGRAPHIC REVOCATION AUTHORITY (CVE-TDF-025)
// =============================================================================

/// Authority information for signed revocation lists
///
/// Security Fix (CVE-TDF-025): Authority identity is cryptographically bound
/// to revocation entries through public key verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorityInfo {
    /// Authority identifier (e.g., DID, URL)
    pub id: String,
    /// Human-readable authority name
    pub name: String,
    /// Authority's public key (hex-encoded Ed25519)
    pub public_key: String,
    /// Optional contact URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl AuthorityInfo {
    /// Create a new authority info from an Ed25519 verifying key
    pub fn new(id: String, name: String, verifying_key: &VerifyingKey) -> Self {
        AuthorityInfo {
            id,
            name,
            public_key: hex::encode(verifying_key.as_bytes()),
            url: None,
        }
    }

    /// Get the verifying key from the stored public key
    pub fn verifying_key(&self) -> TdfResult<VerifyingKey> {
        let bytes = hex::decode(&self.public_key)
            .map_err(|e| TdfError::InvalidDocument(format!("Invalid authority public key: {}", e)))?;

        if bytes.len() != 32 {
            return Err(TdfError::InvalidDocument(
                "Authority public key must be 32 bytes".to_string()
            ));
        }

        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(&bytes);

        VerifyingKey::from_bytes(&key_bytes)
            .map_err(|e| TdfError::InvalidDocument(format!("Invalid Ed25519 public key: {}", e)))
    }
}

/// A cryptographically signed revocation list
///
/// Security Fix (CVE-TDF-025): Revocation lists are signed by a trusted authority
/// to prevent unauthorized revocation attacks. The signature covers all entries
/// and metadata.
///
/// # Example
/// ```ignore
/// use tdf_core::revocation::{SignedRevocationList, AuthorityInfo, RevocationEntry};
///
/// // Create and sign a revocation list
/// let signed_list = SignedRevocationList::new(
///     entries,
///     authority,
///     &signing_key,
/// )?;
///
/// // Verify the list
/// assert!(signed_list.verify().is_ok());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedRevocationList {
    /// Version of signed revocation list format
    pub version: u8,
    /// Revocation entries
    pub entries: Vec<RevocationEntry>,
    /// Authority that issued this list
    pub authority: AuthorityInfo,
    /// When this list was issued
    pub issued_at: DateTime<Utc>,
    /// When this list expires (for freshness checks)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_update: Option<DateTime<Utc>>,
    /// Authority's Ed25519 signature over the canonical payload (base64)
    pub signature: String,
}

/// Current version of the signed revocation list format
const SIGNED_REVOCATION_VERSION: u8 = 1;

impl SignedRevocationList {
    /// Create a new signed revocation list
    ///
    /// Security Fix (CVE-TDF-025): The authority signs all entries to prevent tampering.
    ///
    /// # Arguments
    /// * `entries` - Revocation entries to include
    /// * `authority` - Authority information (must match signing key)
    /// * `signing_key` - Authority's Ed25519 signing key
    /// * `validity_hours` - Hours until next_update (None for no expiry)
    pub fn new(
        entries: Vec<RevocationEntry>,
        authority: AuthorityInfo,
        signing_key: &SigningKey,
        validity_hours: Option<i64>,
    ) -> TdfResult<Self> {
        let issued_at = Utc::now();
        let next_update = validity_hours.map(|h| issued_at + chrono::Duration::hours(h));

        // Verify the authority's public key matches the signing key
        let expected_public_key = hex::encode(signing_key.verifying_key().as_bytes());
        if authority.public_key != expected_public_key {
            return Err(TdfError::SignatureFailure(
                "Authority public key does not match signing key".to_string()
            ));
        }

        let mut list = SignedRevocationList {
            version: SIGNED_REVOCATION_VERSION,
            entries,
            authority,
            issued_at,
            next_update,
            signature: String::new(), // Will be filled below
        };

        // Compute and add signature
        let payload = list.canonical_payload();
        let signature = signing_key.sign(&payload);
        list.signature = STANDARD.encode(signature.to_bytes());

        Ok(list)
    }

    /// Compute the canonical payload for signing/verification
    ///
    /// The payload includes:
    /// - Version
    /// - Authority ID and public key
    /// - Issued timestamp
    /// - Next update timestamp (if any)
    /// - Sorted, canonicalized entries
    fn canonical_payload(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();

        // Domain separator
        hasher.update(b"TDF-REVOCATION-V1:");

        // Version
        hasher.update([self.version]);

        // Authority info
        hasher.update(self.authority.id.as_bytes());
        hasher.update(b":");
        hasher.update(self.authority.public_key.as_bytes());
        hasher.update(b":");

        // Timestamps
        hasher.update(self.issued_at.timestamp().to_be_bytes());
        if let Some(next) = self.next_update {
            hasher.update(next.timestamp().to_be_bytes());
        }

        // Entries (sorted by signer_id for determinism)
        let mut sorted_entries = self.entries.clone();
        sorted_entries.sort_by(|a, b| a.signer_id.cmp(&b.signer_id));

        for entry in &sorted_entries {
            hasher.update(entry.signer_id.as_bytes());
            hasher.update(b":");
            hasher.update(entry.revoked_at.timestamp().to_be_bytes());
            hasher.update(b":");
            hasher.update((entry.reason as u8).to_be_bytes());
            hasher.update(b";");
        }

        hasher.finalize().to_vec()
    }

    /// Verify the signature on this revocation list
    ///
    /// Security Fix (CVE-TDF-025): Verifies that the list was signed by the
    /// claimed authority and hasn't been tampered with.
    ///
    /// # Returns
    /// * `Ok(true)` if signature is valid
    /// * `Err` if signature is invalid or verification fails
    pub fn verify(&self) -> TdfResult<bool> {
        // Get the authority's verifying key
        let verifying_key = self.authority.verifying_key()?;

        // Decode signature
        let sig_bytes = STANDARD.decode(&self.signature)
            .map_err(|e| TdfError::SignatureFailure(format!("Invalid signature base64: {}", e)))?;

        if sig_bytes.len() != 64 {
            return Err(TdfError::SignatureFailure(
                format!("Invalid signature length: expected 64, got {}", sig_bytes.len())
            ));
        }

        let mut sig_array = [0u8; 64];
        sig_array.copy_from_slice(&sig_bytes);
        let signature = Signature::from_bytes(&sig_array);

        // Compute payload and verify
        let payload = self.canonical_payload();
        verifying_key.verify(&payload, &signature)
            .map_err(|e| TdfError::SignatureFailure(format!("Signature verification failed: {}", e)))?;

        Ok(true)
    }

    /// Verify the signature using an external verifying key
    ///
    /// This is useful when you have the authority's key from a trusted source
    /// (e.g., embedded in the application) rather than trusting the key in the list.
    pub fn verify_with_key(&self, verifying_key: &VerifyingKey) -> TdfResult<bool> {
        // Optionally verify the embedded key matches
        let embedded_key = self.authority.verifying_key()?;
        if embedded_key.as_bytes() != verifying_key.as_bytes() {
            return Err(TdfError::SignatureFailure(
                "Provided key does not match authority's embedded key".to_string()
            ));
        }

        self.verify()
    }

    /// Check if this revocation list has expired
    pub fn is_expired(&self) -> bool {
        if let Some(next_update) = self.next_update {
            Utc::now() > next_update
        } else {
            false
        }
    }

    /// Check if a signer ID is revoked in this list
    pub fn is_revoked(&self, signer_id: &str) -> Option<&RevocationEntry> {
        self.entries.iter().find(|e| e.signer_id == signer_id)
    }

    /// Check if a signer ID was revoked at a specific time
    pub fn is_revoked_at(&self, signer_id: &str, check_time: DateTime<Utc>) -> Option<&RevocationEntry> {
        self.entries.iter().find(|e| e.signer_id == signer_id && e.revoked_at <= check_time)
    }

    /// Add a new revocation entry (re-signs the list)
    ///
    /// # Arguments
    /// * `entry` - New revocation entry to add
    /// * `signing_key` - Authority's signing key to re-sign the list
    pub fn add_entry(&mut self, entry: RevocationEntry, signing_key: &SigningKey) -> TdfResult<()> {
        // Verify authority matches signing key
        let expected_public_key = hex::encode(signing_key.verifying_key().as_bytes());
        if self.authority.public_key != expected_public_key {
            return Err(TdfError::SignatureFailure(
                "Signing key does not match authority".to_string()
            ));
        }

        // Check for duplicate
        if self.entries.iter().any(|e| e.signer_id == entry.signer_id) {
            return Err(TdfError::InvalidDocument(
                format!("Signer {} is already revoked", entry.signer_id)
            ));
        }

        // Add entry and re-sign
        self.entries.push(entry);
        self.issued_at = Utc::now();

        let payload = self.canonical_payload();
        let signature = signing_key.sign(&payload);
        self.signature = STANDARD.encode(signature.to_bytes());

        Ok(())
    }

    /// Convert to unsigned RevocationList (for compatibility)
    pub fn to_unsigned(&self) -> RevocationList {
        RevocationList {
            version: self.version,
            issued_at: self.issued_at,
            next_update: self.next_update,
            issuer: Some(self.authority.id.clone()),
            revoked_keys: self.entries.clone(),
        }
    }

    /// Serialize to CBOR bytes
    pub fn to_cbor(&self) -> TdfResult<Vec<u8>> {
        let mut buf = Vec::new();
        ciborium::into_writer(self, &mut buf)
            .map_err(|e| TdfError::InvalidDocument(format!("Failed to serialize signed revocation list: {}", e)))?;
        Ok(buf)
    }

    /// Deserialize from CBOR bytes
    pub fn from_cbor(data: &[u8]) -> TdfResult<Self> {
        ciborium::from_reader(data)
            .map_err(|e| TdfError::InvalidDocument(format!("Invalid signed revocation list CBOR: {}", e)))
    }

    /// Deserialize from CBOR and verify signature
    ///
    /// Security Fix (CVE-TDF-025): Always verify signatures when loading revocation lists.
    pub fn from_cbor_verified(data: &[u8]) -> TdfResult<Self> {
        let list = Self::from_cbor(data)?;
        list.verify()?;
        Ok(list)
    }

    /// Deserialize from CBOR and verify with an external key
    pub fn from_cbor_verified_with_key(data: &[u8], verifying_key: &VerifyingKey) -> TdfResult<Self> {
        let list = Self::from_cbor(data)?;
        list.verify_with_key(verifying_key)?;
        Ok(list)
    }
}

/// Revocation manager for checking multiple revocation lists
///
/// Supports both unsigned (legacy) and signed (CVE-TDF-025) revocation lists.
pub struct RevocationManager {
    lists: Vec<RevocationList>,
    signed_lists: Vec<SignedRevocationList>,
    /// Trusted authority keys for signed list verification
    trusted_authorities: Vec<(String, VerifyingKey)>,
}

impl RevocationManager {
    /// Create a new revocation manager
    pub fn new() -> Self {
        RevocationManager {
            lists: Vec::new(),
            signed_lists: Vec::new(),
            trusted_authorities: Vec::new(),
        }
    }

    /// Add a trusted authority for signed revocation list verification
    ///
    /// Security Fix (CVE-TDF-025): Only signed lists from trusted authorities
    /// will be accepted when using add_signed_list_verified.
    pub fn add_trusted_authority(&mut self, authority_id: String, verifying_key: VerifyingKey) {
        self.trusted_authorities.push((authority_id, verifying_key));
    }

    /// Check if an authority is trusted
    pub fn is_authority_trusted(&self, authority_id: &str) -> Option<&VerifyingKey> {
        self.trusted_authorities
            .iter()
            .find(|(id, _)| id == authority_id)
            .map(|(_, key)| key)
    }

    /// Add an unsigned revocation list (legacy)
    pub fn add_list(&mut self, list: RevocationList) {
        self.lists.push(list);
    }

    /// Add a signed revocation list (verified)
    ///
    /// Security Fix (CVE-TDF-025): Verifies the list signature before adding.
    pub fn add_signed_list(&mut self, list: SignedRevocationList) -> TdfResult<()> {
        list.verify()?;
        self.signed_lists.push(list);
        Ok(())
    }

    /// Add a signed revocation list with trusted authority verification
    ///
    /// Security Fix (CVE-TDF-025): Only accepts lists from pre-registered
    /// trusted authorities.
    pub fn add_signed_list_verified(&mut self, list: SignedRevocationList) -> TdfResult<()> {
        // Verify the signature
        list.verify()?;

        // Check if authority is trusted
        let trusted_key = self.is_authority_trusted(&list.authority.id)
            .ok_or_else(|| TdfError::UntrustedSigner(format!(
                "Authority '{}' is not in the trusted authorities list",
                list.authority.id
            )))?;

        // Verify the embedded key matches the trusted key
        let embedded_key = list.authority.verifying_key()?;
        if embedded_key.as_bytes() != trusted_key.as_bytes() {
            return Err(TdfError::SignatureFailure(
                "Authority public key does not match trusted key".to_string()
            ));
        }

        self.signed_lists.push(list);
        Ok(())
    }

    /// Check if a signer ID is revoked in any list
    pub fn is_revoked(&self, signer_id: &str) -> Option<&RevocationEntry> {
        // Check unsigned lists first
        for list in &self.lists {
            if let Some(entry) = list.is_revoked(signer_id) {
                return Some(entry);
            }
        }
        // Check signed lists
        for list in &self.signed_lists {
            if let Some(entry) = list.is_revoked(signer_id) {
                return Some(entry);
            }
        }
        None
    }

    /// Check if a signer ID is revoked at a specific time
    pub fn is_revoked_at(&self, signer_id: &str, check_time: DateTime<Utc>) -> Option<&RevocationEntry> {
        // Check unsigned lists first
        for list in &self.lists {
            if let Some(entry) = list.is_revoked_at(signer_id, check_time) {
                return Some(entry);
            }
        }
        // Check signed lists
        for list in &self.signed_lists {
            if let Some(entry) = list.is_revoked_at(signer_id, check_time) {
                return Some(entry);
            }
        }
        None
    }

    /// Get count of signed lists
    pub fn signed_list_count(&self) -> usize {
        self.signed_lists.len()
    }

    /// Get count of unsigned lists
    pub fn unsigned_list_count(&self) -> usize {
        self.lists.len()
    }

    /// Load revocation list from CBOR bytes
    pub fn from_cbor(data: &[u8]) -> TdfResult<RevocationList> {
        ciborium::from_reader(data)
            .map_err(|e| TdfError::InvalidDocument(format!("Invalid revocation list CBOR: {}", e)))
    }

    /// Serialize revocation list to CBOR bytes
    pub fn to_cbor(list: &RevocationList) -> TdfResult<Vec<u8>> {
        let mut buf = Vec::new();
        ciborium::into_writer(list, &mut buf)
            .map_err(|e| TdfError::InvalidDocument(format!("Failed to serialize revocation list: {}", e)))?;
        Ok(buf)
    }
}

impl Default for RevocationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_revocation_list_creation() {
        let list = RevocationList::new();
        assert_eq!(list.version, 1);
        assert_eq!(list.revoked_keys.len(), 0);
    }

    #[test]
    fn test_revoke_key() {
        let mut list = RevocationList::new();
        list.revoke(
            "did:web:test.com".to_string(),
            RevocationReason::KeyCompromise,
            Some("admin".to_string()),
        );

        assert!(list.is_revoked("did:web:test.com").is_some());
        assert!(list.is_revoked("did:web:other.com").is_none());
    }

    #[test]
    fn test_unrevoke_key() {
        let mut list = RevocationList::new();
        list.revoke(
            "did:web:test.com".to_string(),
            RevocationReason::KeyCompromise,
            None,
        );

        assert!(list.is_revoked("did:web:test.com").is_some());
        assert!(list.unrevoke("did:web:test.com"));
        assert!(list.is_revoked("did:web:test.com").is_none());
    }

    #[test]
    fn test_revocation_serialization() {
        let mut list = RevocationList::new();
        list.revoke(
            "did:web:test.com".to_string(),
            RevocationReason::KeyCompromise,
            None,
        );

        let bytes = RevocationManager::to_cbor(&list).unwrap();
        let deserialized = RevocationManager::from_cbor(&bytes).unwrap();

        assert_eq!(deserialized.revoked_keys.len(), 1);
        assert_eq!(deserialized.revoked_keys[0].signer_id, "did:web:test.com");
    }

    #[test]
    fn test_revocation_manager() {
        let mut manager = RevocationManager::new();
        let mut list1 = RevocationList::new();
        list1.revoke("did:web:test1.com".to_string(), RevocationReason::KeyCompromise, None);
        manager.add_list(list1);

        let mut list2 = RevocationList::new();
        list2.revoke("did:web:test2.com".to_string(), RevocationReason::Superseded, None);
        manager.add_list(list2);

        assert!(manager.is_revoked("did:web:test1.com").is_some());
        assert!(manager.is_revoked("did:web:test2.com").is_some());
        assert!(manager.is_revoked("did:web:test3.com").is_none());
    }

    // === CVE-TDF-025: Signed Revocation List Tests ===

    use rand::rngs::OsRng;

    fn create_test_authority(id: &str, name: &str) -> (AuthorityInfo, SigningKey) {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();
        let authority = AuthorityInfo::new(id.to_string(), name.to_string(), &verifying_key);
        (authority, signing_key)
    }

    fn create_test_entry(signer_id: &str, reason: RevocationReason) -> RevocationEntry {
        RevocationEntry {
            signer_id: signer_id.to_string(),
            revoked_at: Utc::now(),
            reason,
            issued_at: Some(Utc::now()),
            authority: None,
        }
    }

    #[test]
    fn test_signed_revocation_list_creation() {
        let (authority, signing_key) = create_test_authority(
            "did:web:authority.example.com",
            "Test Authority"
        );

        let entries = vec![
            create_test_entry("did:web:revoked1.com", RevocationReason::KeyCompromise),
            create_test_entry("did:web:revoked2.com", RevocationReason::Superseded),
        ];

        let signed_list = SignedRevocationList::new(
            entries,
            authority,
            &signing_key,
            Some(24), // 24 hours validity
        ).unwrap();

        assert_eq!(signed_list.entries.len(), 2);
        assert!(!signed_list.signature.is_empty());
        assert!(signed_list.next_update.is_some());
    }

    #[test]
    fn test_signed_revocation_list_verification() {
        let (authority, signing_key) = create_test_authority(
            "did:web:authority.example.com",
            "Test Authority"
        );

        let entries = vec![
            create_test_entry("did:web:revoked.com", RevocationReason::KeyCompromise),
        ];

        let signed_list = SignedRevocationList::new(
            entries,
            authority,
            &signing_key,
            None,
        ).unwrap();

        // Verification should succeed
        assert!(signed_list.verify().is_ok());
    }

    #[test]
    fn test_signed_revocation_list_tamper_detection() {
        let (authority, signing_key) = create_test_authority(
            "did:web:authority.example.com",
            "Test Authority"
        );

        let entries = vec![
            create_test_entry("did:web:revoked.com", RevocationReason::KeyCompromise),
        ];

        let mut signed_list = SignedRevocationList::new(
            entries,
            authority,
            &signing_key,
            None,
        ).unwrap();

        // Tamper with the entries
        signed_list.entries[0].signer_id = "did:web:attacker-injected.com".to_string();

        // Verification should fail
        let result = signed_list.verify();
        assert!(result.is_err());
        if let Err(TdfError::SignatureFailure(_)) = result {
            // Expected
        } else {
            panic!("Expected SignatureFailure error");
        }
    }

    #[test]
    fn test_signed_revocation_list_wrong_authority() {
        let (authority, signing_key) = create_test_authority(
            "did:web:authority.example.com",
            "Test Authority"
        );

        // Create a different authority's key
        let (_, attacker_key) = create_test_authority(
            "did:web:attacker.com",
            "Attacker"
        );

        let entries = vec![
            create_test_entry("did:web:target.com", RevocationReason::KeyCompromise),
        ];

        // Try to create signed list with mismatched authority and key
        let result = SignedRevocationList::new(
            entries,
            authority,  // Authority claims to be "authority.example.com"
            &attacker_key,  // But signing with attacker's key
            None,
        );

        // Should fail because authority's public key doesn't match signing key
        assert!(result.is_err());
    }

    #[test]
    fn test_signed_revocation_list_add_entry() {
        let (authority, signing_key) = create_test_authority(
            "did:web:authority.example.com",
            "Test Authority"
        );

        let entries = vec![
            create_test_entry("did:web:revoked1.com", RevocationReason::KeyCompromise),
        ];

        let mut signed_list = SignedRevocationList::new(
            entries,
            authority,
            &signing_key,
            None,
        ).unwrap();

        // Add another entry
        let new_entry = create_test_entry("did:web:revoked2.com", RevocationReason::Superseded);
        signed_list.add_entry(new_entry, &signing_key).unwrap();

        // Should have 2 entries now
        assert_eq!(signed_list.entries.len(), 2);

        // Verification should still succeed
        assert!(signed_list.verify().is_ok());
    }

    #[test]
    fn test_signed_revocation_list_add_entry_wrong_key() {
        let (authority, signing_key) = create_test_authority(
            "did:web:authority.example.com",
            "Test Authority"
        );

        let (_, wrong_key) = create_test_authority(
            "did:web:other.com",
            "Other Authority"
        );

        let entries = vec![
            create_test_entry("did:web:revoked1.com", RevocationReason::KeyCompromise),
        ];

        let mut signed_list = SignedRevocationList::new(
            entries,
            authority,
            &signing_key,
            None,
        ).unwrap();

        // Try to add entry with wrong key
        let new_entry = create_test_entry("did:web:revoked2.com", RevocationReason::Superseded);
        let result = signed_list.add_entry(new_entry, &wrong_key);

        // Should fail
        assert!(result.is_err());
    }

    #[test]
    fn test_signed_revocation_list_cbor_roundtrip() {
        let (authority, signing_key) = create_test_authority(
            "did:web:authority.example.com",
            "Test Authority"
        );

        let entries = vec![
            create_test_entry("did:web:revoked.com", RevocationReason::KeyCompromise),
        ];

        let signed_list = SignedRevocationList::new(
            entries,
            authority,
            &signing_key,
            None,
        ).unwrap();

        // Serialize and deserialize
        let cbor = signed_list.to_cbor().unwrap();
        let restored = SignedRevocationList::from_cbor_verified(&cbor).unwrap();

        assert_eq!(restored.entries.len(), 1);
        assert_eq!(restored.entries[0].signer_id, "did:web:revoked.com");
    }

    #[test]
    fn test_signed_revocation_list_is_revoked() {
        let (authority, signing_key) = create_test_authority(
            "did:web:authority.example.com",
            "Test Authority"
        );

        let entries = vec![
            create_test_entry("did:web:revoked.com", RevocationReason::KeyCompromise),
        ];

        let signed_list = SignedRevocationList::new(
            entries,
            authority,
            &signing_key,
            None,
        ).unwrap();

        assert!(signed_list.is_revoked("did:web:revoked.com").is_some());
        assert!(signed_list.is_revoked("did:web:not-revoked.com").is_none());
    }

    #[test]
    fn test_revocation_manager_with_signed_lists() {
        let mut manager = RevocationManager::new();

        // Add unsigned list
        let mut unsigned_list = RevocationList::new();
        unsigned_list.revoke("did:web:unsigned-revoked.com".to_string(), RevocationReason::KeyCompromise, None);
        manager.add_list(unsigned_list);

        // Add signed list
        let (authority, signing_key) = create_test_authority(
            "did:web:authority.example.com",
            "Test Authority"
        );

        let entries = vec![
            create_test_entry("did:web:signed-revoked.com", RevocationReason::KeyCompromise),
        ];

        let signed_list = SignedRevocationList::new(
            entries,
            authority,
            &signing_key,
            None,
        ).unwrap();

        manager.add_signed_list(signed_list).unwrap();

        // Both should be found
        assert!(manager.is_revoked("did:web:unsigned-revoked.com").is_some());
        assert!(manager.is_revoked("did:web:signed-revoked.com").is_some());
        assert!(manager.is_revoked("did:web:not-revoked.com").is_none());

        assert_eq!(manager.unsigned_list_count(), 1);
        assert_eq!(manager.signed_list_count(), 1);
    }

    #[test]
    fn test_revocation_manager_trusted_authorities() {
        let (authority, signing_key) = create_test_authority(
            "did:web:trusted-authority.com",
            "Trusted Authority"
        );

        let entries = vec![
            create_test_entry("did:web:revoked.com", RevocationReason::KeyCompromise),
        ];

        let signed_list = SignedRevocationList::new(
            entries,
            authority.clone(),
            &signing_key,
            None,
        ).unwrap();

        // Create manager without trusted authority
        let mut manager = RevocationManager::new();
        let result = manager.add_signed_list_verified(signed_list.clone());
        assert!(result.is_err()); // Should fail - authority not trusted

        // Add trusted authority
        manager.add_trusted_authority(
            "did:web:trusted-authority.com".to_string(),
            signing_key.verifying_key()
        );

        // Now it should work
        let result = manager.add_signed_list_verified(signed_list);
        assert!(result.is_ok());

        assert!(manager.is_revoked("did:web:revoked.com").is_some());
    }

    #[test]
    fn test_revocation_manager_rejects_untrusted_authority() {
        let (authority, signing_key) = create_test_authority(
            "did:web:untrusted.com",
            "Untrusted Authority"
        );

        let (_, trusted_key) = create_test_authority(
            "did:web:trusted.com",
            "Trusted Authority"
        );

        let entries = vec![
            create_test_entry("did:web:target.com", RevocationReason::KeyCompromise),
        ];

        let signed_list = SignedRevocationList::new(
            entries,
            authority,
            &signing_key,
            None,
        ).unwrap();

        // Create manager with different trusted authority
        let mut manager = RevocationManager::new();
        manager.add_trusted_authority(
            "did:web:trusted.com".to_string(),
            trusted_key.verifying_key()
        );

        // Should fail - authority not in trusted list
        let result = manager.add_signed_list_verified(signed_list);
        assert!(result.is_err());

        if let Err(TdfError::UntrustedSigner(msg)) = result {
            assert!(msg.contains("untrusted.com"));
        } else {
            panic!("Expected UntrustedSigner error");
        }
    }

    #[test]
    fn test_authority_info_from_key() {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();

        let authority = AuthorityInfo::new(
            "did:web:test.com".to_string(),
            "Test Authority".to_string(),
            &verifying_key
        );

        // Should be able to get the key back
        let recovered_key = authority.verifying_key().unwrap();
        assert_eq!(recovered_key.as_bytes(), verifying_key.as_bytes());
    }
}

