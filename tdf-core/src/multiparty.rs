use crate::error::{TdfError, TdfResult};
use crate::signature::{DocumentSignature, SignatureBlock, SignatureManager, VerificationResult};
use crate::revocation::RevocationManager;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SigningOrder {
    /// Signatures can be added in any order
    Unordered,
    /// Signatures must be added in specified order
    Ordered(Vec<String>), // List of signer IDs in required order
    /// All signers must sign simultaneously (not yet implemented)
    Simultaneous,
}

#[derive(Debug, Clone)]
pub struct MultiPartySigningSession {
    pub root_hash: Vec<u8>,
    pub order: SigningOrder,
    pub signatures: Vec<DocumentSignature>,
    pub required_signers: Vec<String>, // Signer IDs
    pub created: DateTime<Utc>,
}

impl MultiPartySigningSession {
    pub fn new(
        root_hash: Vec<u8>,
        order: SigningOrder,
        required_signers: Vec<String>,
    ) -> Self {
        MultiPartySigningSession {
            root_hash,
            order,
            signatures: Vec::new(),
            required_signers,
            created: Utc::now(),
        }
    }

    pub fn add_signature(&mut self, signature: DocumentSignature) -> TdfResult<()> {
        // Validate signer is in required list
        if !self.required_signers.contains(&signature.signer.id) {
            return Err(TdfError::SignatureFailure(format!(
                "Signer {} not in required signers list",
                signature.signer.id
            )));
        }

        // Check if already signed
        if self.signatures.iter().any(|s| s.signer.id == signature.signer.id) {
            return Err(TdfError::SignatureFailure(format!(
                "Signer {} has already signed",
                signature.signer.id
            )));
        }

        // Validate order if required
        if let SigningOrder::Ordered(ref order) = self.order {
            let expected_index = order
                .iter()
                .position(|id| *id == signature.signer.id)
                .ok_or_else(|| {
                    TdfError::SignatureFailure(format!(
                        "Signer {} not in ordered list",
                        signature.signer.id
                    ))
                })?;

            let current_count = self.signatures.len();
            if expected_index != current_count {
                return Err(TdfError::SignatureFailure(format!(
                    "Signer {} must sign in position {}, but {} signatures already present",
                    signature.signer.id, expected_index, current_count
                )));
            }
        }

        // Note: Signature verification requires keys - use add_signature_verified for full security
        self.signatures.push(signature);
        Ok(())
    }

    /// Add a signature with full verification of existing signatures
    ///
    /// Security Fix (CVE-TDF-016): Verifies all existing signatures are valid
    /// before adding a new signature. This prevents attackers from adding
    /// signatures to tampered documents.
    ///
    /// # Arguments
    /// * `signature` - The new signature to add
    /// * `new_key` - Verifying key for the new signature
    /// * `existing_keys` - Verifying keys for existing signatures
    /// * `revocation_manager` - Optional revocation manager to check key validity
    ///
    /// # Returns
    /// * `Ok(())` if signature was added successfully
    /// * `Err` if validation fails or existing signatures are invalid
    pub fn add_signature_verified(
        &mut self,
        signature: DocumentSignature,
        new_key: &ed25519_dalek::VerifyingKey,
        existing_keys: &[(String, ed25519_dalek::VerifyingKey)],
        revocation_manager: Option<&RevocationManager>,
    ) -> TdfResult<()> {
        // First, verify all existing signatures are valid
        // Security Fix (CVE-TDF-016): Prevent adding signatures to tampered documents
        if !self.signatures.is_empty() {
            let existing_results = SignatureManager::verify_signature_block_with_revocation(
                &self.to_signature_block(),
                &self.root_hash,
                existing_keys,
                revocation_manager,
            )?;

            // Check for any invalid or revoked signatures
            for result in &existing_results {
                match result {
                    VerificationResult::Valid { .. } => {
                        // OK
                    }
                    VerificationResult::Invalid { signer, reason } => {
                        return Err(TdfError::SignatureFailure(format!(
                            "Cannot add signature: existing signature from '{}' is invalid: {}",
                            signer, reason
                        )));
                    }
                    VerificationResult::Revoked { signer, reason, .. } => {
                        return Err(TdfError::RevokedKey(format!(
                            "Cannot add signature: existing signer '{}' key is revoked: {}",
                            signer, reason
                        )));
                    }
                    VerificationResult::Unsupported { signer, algorithm } => {
                        return Err(TdfError::UnsupportedSignatureAlgorithm(format!(
                            "Cannot verify signature from '{}': unsupported algorithm {}",
                            signer, algorithm
                        )));
                    }
                }
            }
        }

        // Verify the new signature itself
        let verify_result = SignatureManager::verify_ed25519(
            &signature,
            &self.root_hash,
            new_key,
        )?;

        if !verify_result {
            return Err(TdfError::SignatureFailure(format!(
                "New signature from '{}' is invalid",
                signature.signer.id
            )));
        }

        // Now call the basic add_signature which validates order and requirements
        self.add_signature(signature)
    }

    /// Add a signature with verification, supporting mixed algorithm types
    ///
    /// Security Fix (CVE-TDF-016): Full verification with mixed Ed25519 and secp256k1 keys
    pub fn add_signature_verified_mixed(
        &mut self,
        signature: DocumentSignature,
        ed25519_keys: &[(String, ed25519_dalek::VerifyingKey)],
        secp256k1_keys: &[(String, k256::ecdsa::VerifyingKey)],
        revocation_manager: Option<&RevocationManager>,
    ) -> TdfResult<()> {
        // Verify all existing signatures first
        if !self.signatures.is_empty() {
            let existing_results = SignatureManager::verify_signature_block_mixed(
                &self.to_signature_block(),
                &self.root_hash,
                ed25519_keys,
                secp256k1_keys,
                revocation_manager,
            )?;

            // Check for any invalid or revoked signatures
            for result in &existing_results {
                match result {
                    VerificationResult::Valid { .. } => {}
                    VerificationResult::Invalid { signer, reason } => {
                        return Err(TdfError::SignatureFailure(format!(
                            "Cannot add signature: existing signature from '{}' is invalid: {}",
                            signer, reason
                        )));
                    }
                    VerificationResult::Revoked { signer, reason, .. } => {
                        return Err(TdfError::RevokedKey(format!(
                            "Cannot add signature: existing signer '{}' key is revoked: {}",
                            signer, reason
                        )));
                    }
                    VerificationResult::Unsupported { signer, algorithm } => {
                        return Err(TdfError::UnsupportedSignatureAlgorithm(format!(
                            "Cannot verify signature from '{}': unsupported algorithm {}",
                            signer, algorithm
                        )));
                    }
                }
            }
        }

        // Verify the new signature based on its algorithm
        let is_valid = match signature.algorithm {
            crate::signature::SignatureAlgorithm::Ed25519 => {
                let key = ed25519_keys
                    .iter()
                    .find(|(id, _)| *id == signature.signer.id)
                    .map(|(_, k)| k)
                    .ok_or_else(|| {
                        TdfError::SignatureFailure(format!(
                            "No Ed25519 key found for signer '{}'",
                            signature.signer.id
                        ))
                    })?;
                SignatureManager::verify_ed25519(&signature, &self.root_hash, key)?
            }
            crate::signature::SignatureAlgorithm::Secp256k1 => {
                let key = secp256k1_keys
                    .iter()
                    .find(|(id, _)| *id == signature.signer.id)
                    .map(|(_, k)| k)
                    .ok_or_else(|| {
                        TdfError::SignatureFailure(format!(
                            "No secp256k1 key found for signer '{}'",
                            signature.signer.id
                        ))
                    })?;
                SignatureManager::verify_secp256k1(&signature, &self.root_hash, key)?
            }
            crate::signature::SignatureAlgorithm::RsaPss => {
                return Err(TdfError::UnsupportedSignatureAlgorithm(
                    "RSA-PSS not yet supported".to_string()
                ));
            }
        };

        if !is_valid {
            return Err(TdfError::SignatureFailure(format!(
                "New signature from '{}' is invalid",
                signature.signer.id
            )));
        }

        // Now call the basic add_signature
        self.add_signature(signature)
    }

    pub fn is_complete(&self) -> bool {
        self.signatures.len() == self.required_signers.len()
    }

    pub fn get_missing_signers(&self) -> Vec<String> {
        let signed_ids: Vec<String> = self.signatures.iter().map(|s| s.signer.id.clone()).collect();
        self.required_signers
            .iter()
            .filter(|id| !signed_ids.contains(id))
            .cloned()
            .collect()
    }

    pub fn to_signature_block(&self) -> SignatureBlock {
        SignatureBlock {
            signatures: self.signatures.clone(),
        }
    }

    /// Validate that signatures are in chronological timestamp order
    ///
    /// Security Fix (CVE-TDF-023): Enforces that signatures in a multiparty session
    /// are ordered chronologically by their timestamps. This prevents replay attacks
    /// and ensures temporal consistency in signature chains.
    ///
    /// # Returns
    /// * `Ok(())` if all signatures are in chronological order (or empty/single signature)
    /// * `Err(TimestampError)` if any signature has a timestamp before the previous one
    ///
    /// # Example
    /// ```ignore
    /// let session = MultiPartySigningSession::new(...);
    /// // Add signatures...
    /// session.validate_signature_order()?; // Ensures chronological order
    /// ```
    pub fn validate_signature_order(&self) -> TdfResult<()> {
        let mut prev_time: Option<DateTime<Utc>> = None;
        let mut prev_signer: Option<&str> = None;

        for sig in &self.signatures {
            if let Some(prev) = prev_time {
                if sig.timestamp.time < prev {
                    return Err(TdfError::TimestampError(format!(
                        "Signature timestamp {} from '{}' is before previous timestamp {} from '{}'. \
                         Signatures must be in chronological order.",
                        sig.timestamp.time.format("%Y-%m-%dT%H:%M:%SZ"),
                        sig.signer.id,
                        prev.format("%Y-%m-%dT%H:%M:%SZ"),
                        prev_signer.unwrap_or("unknown")
                    )));
                }
            }
            prev_time = Some(sig.timestamp.time);
            prev_signer = Some(&sig.signer.id);
        }
        Ok(())
    }

    /// Validate signature order with a maximum allowed time gap between signatures
    ///
    /// Security Fix (CVE-TDF-023): Enhanced version that also detects suspicious
    /// time gaps that may indicate a stale session attack.
    ///
    /// # Arguments
    /// * `max_gap` - Maximum allowed time difference between consecutive signatures
    ///
    /// # Returns
    /// * `Ok(())` if all timestamps are valid and within acceptable gaps
    /// * `Err(TimestampError)` if ordering is violated or gaps are too large
    pub fn validate_signature_order_with_max_gap(
        &self,
        max_gap: chrono::Duration,
    ) -> TdfResult<()> {
        let mut prev_time: Option<DateTime<Utc>> = None;
        let mut prev_signer: Option<&str> = None;

        for sig in &self.signatures {
            if let Some(prev) = prev_time {
                // Check chronological order
                if sig.timestamp.time < prev {
                    return Err(TdfError::TimestampError(format!(
                        "Signature timestamp {} from '{}' is before previous timestamp {} from '{}'. \
                         Signatures must be in chronological order.",
                        sig.timestamp.time.format("%Y-%m-%dT%H:%M:%SZ"),
                        sig.signer.id,
                        prev.format("%Y-%m-%dT%H:%M:%SZ"),
                        prev_signer.unwrap_or("unknown")
                    )));
                }

                // Check time gap isn't too large (stale session detection)
                let gap = sig.timestamp.time - prev;
                if gap > max_gap {
                    return Err(TdfError::TimestampError(format!(
                        "Time gap between signatures ({} seconds) exceeds maximum allowed ({} seconds). \
                         Signature from '{}' may be part of a stale session attack.",
                        gap.num_seconds(),
                        max_gap.num_seconds(),
                        sig.signer.id
                    )));
                }
            }
            prev_time = Some(sig.timestamp.time);
            prev_signer = Some(&sig.signer.id);
        }
        Ok(())
    }

    /// Check if a new signature's timestamp would maintain chronological order
    ///
    /// Security Fix (CVE-TDF-023): Pre-validation before adding a signature
    /// to ensure timestamp order is preserved.
    ///
    /// # Arguments
    /// * `new_timestamp` - The timestamp of the signature to be added
    ///
    /// # Returns
    /// * `true` if the timestamp would maintain chronological order
    /// * `false` if the timestamp would violate ordering
    pub fn would_maintain_order(&self, new_timestamp: DateTime<Utc>) -> bool {
        if let Some(last_sig) = self.signatures.last() {
            new_timestamp >= last_sig.timestamp.time
        } else {
            true // Empty session - any timestamp is valid
        }
    }

    pub fn verify_all_signatures(
        &self,
        ed25519_keys: &[(String, ed25519_dalek::VerifyingKey)],
        secp256k1_keys: &[(String, k256::ecdsa::VerifyingKey)],
    ) -> TdfResult<Vec<crate::signature::VerificationResult>> {
        SignatureManager::verify_signature_block_mixed(
            &self.to_signature_block(),
            &self.root_hash,
            ed25519_keys,
            secp256k1_keys,
            None, // No revocation manager in multiparty context
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigningWorkflow {
    pub id: String,
    pub document_id: String,
    pub order: SigningOrder,
    pub required_signers: Vec<SignerRequirement>,
    pub status: WorkflowStatus,
    pub created: DateTime<Utc>,
    pub completed: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignerRequirement {
    pub signer_id: String,
    pub signer_name: String,
    pub role: Option<String>,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WorkflowStatus {
    Pending,
    InProgress { signed_count: usize, total: usize },
    Completed,
    Rejected { reason: String },
}

impl SigningWorkflow {
    pub fn new(
        document_id: String,
        order: SigningOrder,
        required_signers: Vec<SignerRequirement>,
    ) -> Self {
        let total = required_signers.len();
        SigningWorkflow {
            id: uuid::Uuid::new_v4().to_string(),
            document_id,
            order,
            required_signers,
            status: WorkflowStatus::InProgress {
                signed_count: 0,
                total,
            },
            created: Utc::now(),
            completed: None,
        }
    }

    pub fn add_signature(&mut self, signature: &DocumentSignature) -> TdfResult<()> {
        // Find signer requirement (validate signer is in workflow)
        let _signer_req = self
            .required_signers
            .iter()
            .find(|r| r.signer_id == signature.signer.id)
            .ok_or_else(|| {
                TdfError::SignatureFailure(format!(
                    "Signer {} not in workflow requirements",
                    signature.signer.id
                ))
            })?;

        // Update status
        match &self.status {
            WorkflowStatus::InProgress { signed_count, total } => {
                let new_count = signed_count + 1;
                if new_count >= *total {
                    self.status = WorkflowStatus::Completed;
                    self.completed = Some(Utc::now());
                } else {
                    self.status = WorkflowStatus::InProgress {
                        signed_count: new_count,
                        total: *total,
                    };
                }
            }
            _ => {
                return Err(TdfError::SignatureFailure(
                    "Cannot add signature to completed or rejected workflow".to_string(),
                ));
            }
        }

        Ok(())
    }

    pub fn get_next_signer(&self) -> Option<&SignerRequirement> {
        match &self.order {
            SigningOrder::Ordered(order) => {
                // Find first unsigned signer in order
                for signer_id in order {
                    if let Some(req) = self.required_signers.iter().find(|r| r.signer_id == *signer_id) {
                        // Check if already signed (would need to track this separately)
                        return Some(req);
                    }
                }
                None
            }
            SigningOrder::Unordered => {
                // Return first required signer
                self.required_signers.iter().find(|r| r.required)
            }
            SigningOrder::Simultaneous => {
                // All must sign, return first
                self.required_signers.first()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signature::{SignatureManager, SignatureScope};
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;

    fn create_test_signature(
        signing_key: &SigningKey,
        root_hash: &[u8],
        signer_id: &str,
        signer_name: &str,
    ) -> DocumentSignature {
        SignatureManager::sign_ed25519(
            signing_key,
            root_hash,
            signer_id.to_string(),
            signer_name.to_string(),
            SignatureScope::Full,
        )
    }

    #[test]
    fn test_multiparty_basic_signing() {
        let root_hash = b"test_root_hash_for_multiparty_00".to_vec();
        let required_signers = vec![
            "signer-1".to_string(),
            "signer-2".to_string(),
        ];

        let mut session = MultiPartySigningSession::new(
            root_hash.clone(),
            SigningOrder::Unordered,
            required_signers,
        );

        let signing_key1 = SigningKey::generate(&mut OsRng);
        let sig1 = create_test_signature(&signing_key1, &root_hash, "signer-1", "Signer One");

        assert!(session.add_signature(sig1).is_ok());
        assert!(!session.is_complete());
        assert_eq!(session.get_missing_signers(), vec!["signer-2".to_string()]);
    }

    #[test]
    fn test_multiparty_duplicate_signer_rejected() {
        let root_hash = b"test_root_hash_for_multiparty_00".to_vec();
        let required_signers = vec!["signer-1".to_string()];

        let mut session = MultiPartySigningSession::new(
            root_hash.clone(),
            SigningOrder::Unordered,
            required_signers,
        );

        let signing_key1 = SigningKey::generate(&mut OsRng);
        let sig1 = create_test_signature(&signing_key1, &root_hash, "signer-1", "Signer One");
        let sig1_dup = create_test_signature(&signing_key1, &root_hash, "signer-1", "Signer One");

        assert!(session.add_signature(sig1).is_ok());
        assert!(session.add_signature(sig1_dup).is_err()); // Duplicate rejected
    }

    #[test]
    fn test_multiparty_unauthorized_signer_rejected() {
        let root_hash = b"test_root_hash_for_multiparty_00".to_vec();
        let required_signers = vec!["signer-1".to_string()];

        let mut session = MultiPartySigningSession::new(
            root_hash.clone(),
            SigningOrder::Unordered,
            required_signers,
        );

        let signing_key = SigningKey::generate(&mut OsRng);
        let sig = create_test_signature(&signing_key, &root_hash, "attacker", "Attacker");

        assert!(session.add_signature(sig).is_err()); // Unauthorized rejected
    }

    // === CVE-TDF-016: Multiparty Signature Verification Tests ===

    #[test]
    fn test_add_signature_verified_validates_existing() {
        let root_hash = b"test_root_hash_for_multiparty_00".to_vec();
        let required_signers = vec![
            "signer-1".to_string(),
            "signer-2".to_string(),
        ];

        let mut session = MultiPartySigningSession::new(
            root_hash.clone(),
            SigningOrder::Unordered,
            required_signers,
        );

        // Create first signature
        let signing_key1 = SigningKey::generate(&mut OsRng);
        let verifying_key1 = signing_key1.verifying_key();
        let sig1 = create_test_signature(&signing_key1, &root_hash, "signer-1", "Signer One");

        // Add first signature without verification (for setup)
        assert!(session.add_signature(sig1).is_ok());

        // Create second signature
        let signing_key2 = SigningKey::generate(&mut OsRng);
        let verifying_key2 = signing_key2.verifying_key();
        let sig2 = create_test_signature(&signing_key2, &root_hash, "signer-2", "Signer Two");

        // Add second signature with verification
        let existing_keys = vec![
            ("signer-1".to_string(), verifying_key1),
        ];

        let result = session.add_signature_verified(
            sig2,
            &verifying_key2,
            &existing_keys,
            None,
        );

        assert!(result.is_ok(), "Should succeed when existing signatures are valid");
        assert!(session.is_complete());
    }

    #[test]
    fn test_add_signature_verified_rejects_tampered_document() {
        let root_hash = b"test_root_hash_for_multiparty_00".to_vec();
        let tampered_hash = b"tampered_root_hash_for_attack_00".to_vec();
        let required_signers = vec![
            "signer-1".to_string(),
            "signer-2".to_string(),
        ];

        let mut session = MultiPartySigningSession::new(
            root_hash.clone(),
            SigningOrder::Unordered,
            required_signers,
        );

        // Create first signature
        let signing_key1 = SigningKey::generate(&mut OsRng);
        let verifying_key1 = signing_key1.verifying_key();
        let sig1 = create_test_signature(&signing_key1, &root_hash, "signer-1", "Signer One");

        // Add first signature
        assert!(session.add_signature(sig1).is_ok());

        // Simulate document tampering by changing the root hash
        session.root_hash = tampered_hash.clone();

        // Create second signature (against tampered document)
        let signing_key2 = SigningKey::generate(&mut OsRng);
        let verifying_key2 = signing_key2.verifying_key();
        let sig2 = create_test_signature(&signing_key2, &tampered_hash, "signer-2", "Signer Two");

        // Try to add second signature with verification
        // This should FAIL because existing signature was for original hash
        let existing_keys = vec![
            ("signer-1".to_string(), verifying_key1),
        ];

        let result = session.add_signature_verified(
            sig2,
            &verifying_key2,
            &existing_keys,
            None,
        );

        assert!(result.is_err(), "Should fail when existing signatures don't match document");
    }

    #[test]
    fn test_add_signature_verified_first_signature() {
        let root_hash = b"test_root_hash_for_multiparty_00".to_vec();
        let required_signers = vec!["signer-1".to_string()];

        let mut session = MultiPartySigningSession::new(
            root_hash.clone(),
            SigningOrder::Unordered,
            required_signers,
        );

        // Create first signature
        let signing_key1 = SigningKey::generate(&mut OsRng);
        let verifying_key1 = signing_key1.verifying_key();
        let sig1 = create_test_signature(&signing_key1, &root_hash, "signer-1", "Signer One");

        // First signature should work even with empty existing keys
        let result = session.add_signature_verified(
            sig1,
            &verifying_key1,
            &[],
            None,
        );

        assert!(result.is_ok(), "First signature should succeed");
        assert!(session.is_complete());
    }

    #[test]
    fn test_add_signature_verified_invalid_new_signature() {
        let root_hash = b"test_root_hash_for_multiparty_00".to_vec();
        let wrong_hash = b"wrong_root_hash_for_test_attack_".to_vec();
        let required_signers = vec!["signer-1".to_string()];

        let mut session = MultiPartySigningSession::new(
            root_hash.clone(),
            SigningOrder::Unordered,
            required_signers,
        );

        // Create signature against wrong hash
        let signing_key1 = SigningKey::generate(&mut OsRng);
        let verifying_key1 = signing_key1.verifying_key();
        let sig1 = create_test_signature(&signing_key1, &wrong_hash, "signer-1", "Signer One");

        // Should fail because signature doesn't match session's root hash
        let result = session.add_signature_verified(
            sig1,
            &verifying_key1,
            &[],
            None,
        );

        assert!(result.is_err(), "Should fail when new signature doesn't match");
    }

    #[test]
    fn test_ordered_signing_enforced() {
        let root_hash = b"test_root_hash_for_multiparty_00".to_vec();
        let required_signers = vec![
            "signer-1".to_string(),
            "signer-2".to_string(),
        ];
        let order = SigningOrder::Ordered(vec![
            "signer-1".to_string(),
            "signer-2".to_string(),
        ]);

        let mut session = MultiPartySigningSession::new(
            root_hash.clone(),
            order,
            required_signers,
        );

        // Try to sign out of order (signer-2 before signer-1)
        let signing_key2 = SigningKey::generate(&mut OsRng);
        let sig2 = create_test_signature(&signing_key2, &root_hash, "signer-2", "Signer Two");

        let result = session.add_signature(sig2);
        assert!(result.is_err(), "Out of order signing should fail");
    }

    // === CVE-TDF-023: Timestamp Ordering Enforcement Tests ===

    fn create_test_signature_with_timestamp(
        signing_key: &SigningKey,
        root_hash: &[u8],
        signer_id: &str,
        signer_name: &str,
        timestamp: DateTime<Utc>,
    ) -> DocumentSignature {
        use crate::signature::{SignatureAlgorithm, SignerInfo, TimestampInfo};
        use ed25519_dalek::Signer as _;

        let signature = signing_key.sign(root_hash);
        DocumentSignature {
            version: 1,
            signer: SignerInfo {
                id: signer_id.to_string(),
                name: signer_name.to_string(),
                certificate: None,
            },
            algorithm: SignatureAlgorithm::Ed25519,
            timestamp: TimestampInfo {
                time: timestamp,
                authority: Some("test".to_string()),
                proof: None,
            },
            signature: hex::encode(signature.to_bytes()),
            scope: SignatureScope::Full,
            root_hash: hex::encode(root_hash),
        }
    }

    #[test]
    fn test_timestamp_order_validation_valid() {
        use chrono::Duration;

        let root_hash = b"test_root_hash_for_multiparty_00".to_vec();
        let required_signers = vec![
            "signer-1".to_string(),
            "signer-2".to_string(),
            "signer-3".to_string(),
        ];

        let mut session = MultiPartySigningSession::new(
            root_hash.clone(),
            SigningOrder::Unordered,
            required_signers,
        );

        let base_time = Utc::now();

        // Add signatures in chronological order
        let signing_key1 = SigningKey::generate(&mut OsRng);
        let sig1 = create_test_signature_with_timestamp(
            &signing_key1, &root_hash, "signer-1", "Signer One", base_time
        );
        session.add_signature(sig1).unwrap();

        let signing_key2 = SigningKey::generate(&mut OsRng);
        let sig2 = create_test_signature_with_timestamp(
            &signing_key2, &root_hash, "signer-2", "Signer Two", base_time + Duration::minutes(5)
        );
        session.add_signature(sig2).unwrap();

        let signing_key3 = SigningKey::generate(&mut OsRng);
        let sig3 = create_test_signature_with_timestamp(
            &signing_key3, &root_hash, "signer-3", "Signer Three", base_time + Duration::minutes(10)
        );
        session.add_signature(sig3).unwrap();

        // Validation should pass
        let result = session.validate_signature_order();
        assert!(result.is_ok(), "Chronologically ordered signatures should pass validation");
    }

    #[test]
    fn test_timestamp_order_validation_invalid() {
        use chrono::Duration;

        let root_hash = b"test_root_hash_for_multiparty_00".to_vec();
        let required_signers = vec![
            "signer-1".to_string(),
            "signer-2".to_string(),
        ];

        let mut session = MultiPartySigningSession::new(
            root_hash.clone(),
            SigningOrder::Unordered,
            required_signers,
        );

        let base_time = Utc::now();

        // Add first signature with later timestamp
        let signing_key1 = SigningKey::generate(&mut OsRng);
        let sig1 = create_test_signature_with_timestamp(
            &signing_key1, &root_hash, "signer-1", "Signer One", base_time + Duration::hours(1)
        );
        session.add_signature(sig1).unwrap();

        // Add second signature with EARLIER timestamp (violates order)
        let signing_key2 = SigningKey::generate(&mut OsRng);
        let sig2 = create_test_signature_with_timestamp(
            &signing_key2, &root_hash, "signer-2", "Signer Two", base_time
        );
        session.add_signature(sig2).unwrap();

        // Validation should FAIL
        let result = session.validate_signature_order();
        assert!(result.is_err(), "Out-of-order timestamps should fail validation");

        // Check error message
        if let Err(TdfError::TimestampError(msg)) = result {
            assert!(msg.contains("before previous timestamp"), "Error should mention timestamp ordering: {}", msg);
        } else {
            panic!("Expected TimestampError");
        }
    }

    #[test]
    fn test_timestamp_order_empty_session() {
        let root_hash = b"test_root_hash_for_multiparty_00".to_vec();
        let session = MultiPartySigningSession::new(
            root_hash,
            SigningOrder::Unordered,
            vec!["signer-1".to_string()],
        );

        // Empty session should pass validation
        assert!(session.validate_signature_order().is_ok());
    }

    #[test]
    fn test_timestamp_order_single_signature() {
        let root_hash = b"test_root_hash_for_multiparty_00".to_vec();
        let mut session = MultiPartySigningSession::new(
            root_hash.clone(),
            SigningOrder::Unordered,
            vec!["signer-1".to_string()],
        );

        let signing_key = SigningKey::generate(&mut OsRng);
        let sig = create_test_signature(&signing_key, &root_hash, "signer-1", "Signer One");
        session.add_signature(sig).unwrap();

        // Single signature should pass validation
        assert!(session.validate_signature_order().is_ok());
    }

    #[test]
    fn test_timestamp_order_with_max_gap_valid() {
        use chrono::Duration;

        let root_hash = b"test_root_hash_for_multiparty_00".to_vec();
        let required_signers = vec![
            "signer-1".to_string(),
            "signer-2".to_string(),
        ];

        let mut session = MultiPartySigningSession::new(
            root_hash.clone(),
            SigningOrder::Unordered,
            required_signers,
        );

        let base_time = Utc::now();

        // Add signatures within acceptable gap
        let signing_key1 = SigningKey::generate(&mut OsRng);
        let sig1 = create_test_signature_with_timestamp(
            &signing_key1, &root_hash, "signer-1", "Signer One", base_time
        );
        session.add_signature(sig1).unwrap();

        let signing_key2 = SigningKey::generate(&mut OsRng);
        let sig2 = create_test_signature_with_timestamp(
            &signing_key2, &root_hash, "signer-2", "Signer Two", base_time + Duration::hours(1)
        );
        session.add_signature(sig2).unwrap();

        // Validation with 2 hour max gap should pass
        let result = session.validate_signature_order_with_max_gap(Duration::hours(2));
        assert!(result.is_ok(), "Signatures within max gap should pass");
    }

    #[test]
    fn test_timestamp_order_with_max_gap_exceeded() {
        use chrono::Duration;

        let root_hash = b"test_root_hash_for_multiparty_00".to_vec();
        let required_signers = vec![
            "signer-1".to_string(),
            "signer-2".to_string(),
        ];

        let mut session = MultiPartySigningSession::new(
            root_hash.clone(),
            SigningOrder::Unordered,
            required_signers,
        );

        let base_time = Utc::now();

        // Add signatures with large gap (stale session simulation)
        let signing_key1 = SigningKey::generate(&mut OsRng);
        let sig1 = create_test_signature_with_timestamp(
            &signing_key1, &root_hash, "signer-1", "Signer One", base_time
        );
        session.add_signature(sig1).unwrap();

        let signing_key2 = SigningKey::generate(&mut OsRng);
        let sig2 = create_test_signature_with_timestamp(
            &signing_key2, &root_hash, "signer-2", "Signer Two", base_time + Duration::days(7)
        );
        session.add_signature(sig2).unwrap();

        // Validation with 1 hour max gap should FAIL
        let result = session.validate_signature_order_with_max_gap(Duration::hours(1));
        assert!(result.is_err(), "Signatures exceeding max gap should fail");

        // Check error message
        if let Err(TdfError::TimestampError(msg)) = result {
            assert!(msg.contains("exceeds maximum allowed"), "Error should mention gap: {}", msg);
            assert!(msg.contains("stale session"), "Error should mention stale session: {}", msg);
        } else {
            panic!("Expected TimestampError");
        }
    }

    #[test]
    fn test_would_maintain_order() {
        use chrono::Duration;

        let root_hash = b"test_root_hash_for_multiparty_00".to_vec();
        let required_signers = vec![
            "signer-1".to_string(),
            "signer-2".to_string(),
        ];

        let mut session = MultiPartySigningSession::new(
            root_hash.clone(),
            SigningOrder::Unordered,
            required_signers,
        );

        let base_time = Utc::now();

        // Add first signature
        let signing_key1 = SigningKey::generate(&mut OsRng);
        let sig1 = create_test_signature_with_timestamp(
            &signing_key1, &root_hash, "signer-1", "Signer One", base_time
        );
        session.add_signature(sig1).unwrap();

        // Check that later timestamp would maintain order
        assert!(session.would_maintain_order(base_time + Duration::hours(1)));

        // Check that earlier timestamp would NOT maintain order
        assert!(!session.would_maintain_order(base_time - Duration::hours(1)));

        // Check that equal timestamp is acceptable
        assert!(session.would_maintain_order(base_time));
    }

    #[test]
    fn test_would_maintain_order_empty_session() {
        let root_hash = b"test_root_hash_for_multiparty_00".to_vec();
        let session = MultiPartySigningSession::new(
            root_hash,
            SigningOrder::Unordered,
            vec!["signer-1".to_string()],
        );

        // Any timestamp should work for empty session
        assert!(session.would_maintain_order(Utc::now()));
        assert!(session.would_maintain_order(Utc::now() - chrono::Duration::days(365)));
    }
}

