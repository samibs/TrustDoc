//! Key revocation system for TDF format
//! Supports Certificate Revocation Lists (CRL) and key expiration

use crate::error::{TdfError, TdfResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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

/// Revocation manager for checking multiple revocation lists
pub struct RevocationManager {
    lists: Vec<RevocationList>,
}

impl RevocationManager {
    /// Create a new revocation manager
    pub fn new() -> Self {
        RevocationManager {
            lists: Vec::new(),
        }
    }

    /// Add a revocation list
    pub fn add_list(&mut self, list: RevocationList) {
        self.lists.push(list);
    }

    /// Check if a signer ID is revoked in any list
    pub fn is_revoked(&self, signer_id: &str) -> Option<&RevocationEntry> {
        for list in &self.lists {
            if let Some(entry) = list.is_revoked(signer_id) {
                return Some(entry);
            }
        }
        None
    }

    /// Check if a signer ID is revoked at a specific time
    pub fn is_revoked_at(&self, signer_id: &str, check_time: DateTime<Utc>) -> Option<&RevocationEntry> {
        for list in &self.lists {
            if let Some(entry) = list.is_revoked_at(signer_id, check_time) {
                return Some(entry);
            }
        }
        None
    }

    /// Load revocation list from CBOR bytes
    pub fn from_cbor(data: &[u8]) -> TdfResult<RevocationList> {
        serde_cbor::from_slice(data)
            .map_err(|e| TdfError::InvalidDocument(format!("Invalid revocation list CBOR: {}", e)))
    }

    /// Serialize revocation list to CBOR bytes
    pub fn to_cbor(list: &RevocationList) -> TdfResult<Vec<u8>> {
        serde_cbor::to_vec(list)
            .map_err(|e| TdfError::InvalidDocument(format!("Failed to serialize revocation list: {}", e)))
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
}

