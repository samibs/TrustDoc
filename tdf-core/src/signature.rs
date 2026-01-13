//! Digital signature management with timestamp binding
//!
//! Security Fixes:
//! - CVE-TDF-003: Timestamp bound to signature payload
//! - CVE-TDF-006: Sign-then-timestamp with cryptographic binding
//! - CVE-TDF-013: Timestamp validation in signature verification

use crate::error::{TdfError, TdfResult};
use crate::timestamp::{create_timestamp_token, TimestampToken, TimestampProvider};
use crate::revocation::RevocationManager;
use chrono::{DateTime, Utc};
use ed25519_dalek::{Signature, Signer as Ed25519Signer, SigningKey, Verifier as Ed25519Verifier, VerifyingKey};
use k256::ecdsa::{SigningKey as Secp256k1SigningKey, VerifyingKey as Secp256k1VerifyingKey, Signature as Secp256k1Signature};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use base64::{engine::general_purpose::STANDARD, Engine};

/// Current signature format version with timestamp binding
pub const SIGNATURE_VERSION_CURRENT: u8 = 2;
/// Legacy signature format (root_hash only, no timestamp binding)
pub const SIGNATURE_VERSION_LEGACY: u8 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureBlock {
    pub signatures: Vec<DocumentSignature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentSignature {
    pub version: u8,
    pub signer: SignerInfo,
    pub timestamp: TimestampInfo,
    pub scope: SignatureScope,
    pub algorithm: SignatureAlgorithm,
    pub root_hash: String,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignerInfo {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimestampInfo {
    pub time: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authority: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<String>,
}

impl From<TimestampToken> for TimestampInfo {
    fn from(token: TimestampToken) -> Self {
        TimestampInfo {
            time: token.time,
            authority: Some(token.authority),
            proof: if token.proof.is_empty() { None } else { Some(token.proof) },
        }
    }
}

impl From<&TimestampInfo> for TimestampToken {
    fn from(info: &TimestampInfo) -> Self {
        create_timestamp_token(info.time, info.authority.clone(), info.proof.clone())
    }
}

/// Compute the canonical signing payload for signature v2+
///
/// Security Fix (CVE-TDF-003, CVE-TDF-006): Bind timestamp to signature
/// The signed payload includes: root_hash || timestamp_iso || signer_id || scope
/// This prevents timestamp manipulation attacks.
pub fn compute_signing_payload(
    root_hash: &[u8],
    timestamp: &DateTime<Utc>,
    signer_id: &str,
    scope: &SignatureScope,
) -> Vec<u8> {
    let mut hasher = Sha256::new();

    // Domain separator for v2 signature payload
    hasher.update(b"TDF-SIGNATURE-V2:");

    // Root hash (raw bytes)
    hasher.update(root_hash);

    // Timestamp in RFC 3339 format (deterministic string representation)
    let timestamp_str = timestamp.to_rfc3339();
    hasher.update(timestamp_str.as_bytes());

    // Signer ID
    hasher.update(signer_id.as_bytes());

    // Scope (canonical representation)
    let scope_str = match scope {
        SignatureScope::Full => "full".to_string(),
        SignatureScope::ContentOnly => "content-only".to_string(),
        SignatureScope::Sections(sections) => {
            let mut sorted = sections.clone();
            sorted.sort();
            format!("sections:{}", sorted.join(","))
        }
    };
    hasher.update(scope_str.as_bytes());

    hasher.finalize().to_vec()
}

/// Compute the legacy signing payload (v1, root_hash only)
///
/// WARNING: Legacy format is vulnerable to timestamp manipulation.
/// Only used for backward compatibility during verification.
pub fn compute_signing_payload_legacy(root_hash: &[u8]) -> Vec<u8> {
    root_hash.to_vec()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum SignatureScope {
    Full,
    ContentOnly,
    Sections(Vec<String>),
}

impl SignatureScope {
    /// Validate that the scope is valid for the given document sections
    ///
    /// Security Fix (CVE-TDF-019): Prevents invalid scope attacks where a signature
    /// claims to cover sections that don't exist in the document.
    ///
    /// # Arguments
    /// * `available_section_ids` - The section IDs that exist in the document
    ///
    /// # Returns
    /// * `Ok(())` if the scope is valid
    /// * `Err(TdfError::InvalidDocument)` if scope references non-existent sections
    pub fn validate(&self, available_section_ids: &[&str]) -> TdfResult<()> {
        match self {
            // Full and ContentOnly are always valid scopes
            SignatureScope::Full => Ok(()),
            SignatureScope::ContentOnly => Ok(()),
            SignatureScope::Sections(section_ids) => {
                // Validate that all referenced sections exist
                if section_ids.is_empty() {
                    return Err(TdfError::InvalidDocument(
                        "Sections scope cannot be empty - use Full or ContentOnly instead".to_string()
                    ));
                }

                let mut missing_sections = Vec::new();
                for section_id in section_ids {
                    if !available_section_ids.contains(&section_id.as_str()) {
                        missing_sections.push(section_id.clone());
                    }
                }

                if !missing_sections.is_empty() {
                    return Err(TdfError::InvalidDocument(format!(
                        "Signature scope references non-existent sections: {}. Available sections: {}",
                        missing_sections.join(", "),
                        if available_section_ids.is_empty() {
                            "(none)".to_string()
                        } else {
                            available_section_ids.join(", ")
                        }
                    )));
                }

                Ok(())
            }
        }
    }

    /// Check if this scope covers a specific section
    ///
    /// # Returns
    /// * `true` if this scope covers the given section
    /// * `false` otherwise
    pub fn covers_section(&self, section_id: &str) -> bool {
        match self {
            SignatureScope::Full => true,
            SignatureScope::ContentOnly => true,
            SignatureScope::Sections(sections) => sections.iter().any(|s| s == section_id),
        }
    }

    /// Get the list of section IDs covered by this scope
    ///
    /// # Arguments
    /// * `all_section_ids` - All section IDs in the document (needed for Full/ContentOnly)
    ///
    /// # Returns
    /// * The list of section IDs that this scope covers
    pub fn covered_sections<'a>(&'a self, all_section_ids: &'a [&'a str]) -> Vec<&'a str> {
        match self {
            SignatureScope::Full => all_section_ids.to_vec(),
            SignatureScope::ContentOnly => all_section_ids.to_vec(),
            SignatureScope::Sections(sections) => {
                sections.iter().map(|s| s.as_str()).collect()
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum SignatureAlgorithm {
    Ed25519,
    Secp256k1,
    RsaPss,
}

pub struct SignatureManager;

impl SignatureManager {
    /// Sign a document using Ed25519 with timestamp binding (v2 format)
    ///
    /// Security Fix (CVE-TDF-003): Timestamp is now cryptographically bound
    /// to the signature payload, preventing timestamp manipulation attacks.
    pub fn sign_ed25519(
        signing_key: &SigningKey,
        root_hash: &[u8],
        signer_id: String,
        signer_name: String,
        scope: SignatureScope,
    ) -> DocumentSignature {
        Self::sign_ed25519_with_timestamp(
            signing_key,
            root_hash,
            signer_id,
            signer_name,
            scope,
            None,
        )
    }

    /// Sign a document using Ed25519 with timestamp provider and timestamp binding
    ///
    /// Security Fixes:
    /// - CVE-TDF-003: Timestamp bound to signature payload
    /// - CVE-TDF-006: Sign-then-timestamp with cryptographic binding
    pub fn sign_ed25519_with_timestamp(
        signing_key: &SigningKey,
        root_hash: &[u8],
        signer_id: String,
        signer_name: String,
        scope: SignatureScope,
        timestamp_provider: Option<&dyn TimestampProvider>,
    ) -> DocumentSignature {
        // Get timestamp FIRST (so it's bound to the signature)
        let timestamp = if let Some(provider) = timestamp_provider {
            provider.get_timestamp(root_hash)
                .map(|t| t.into())
                .unwrap_or_else(|_| TimestampInfo {
                    time: Utc::now(),
                    authority: None,
                    proof: None,
                })
        } else {
            TimestampInfo {
                time: Utc::now(),
                authority: None,
                proof: None,
            }
        };

        // Compute signing payload with timestamp binding (v2 format)
        let signing_payload = compute_signing_payload(
            root_hash,
            &timestamp.time,
            &signer_id,
            &scope,
        );

        // Sign the full payload (includes timestamp)
        let signature: Signature = signing_key.sign(&signing_payload);
        let signature_b64 = STANDARD.encode(signature.to_bytes());

        DocumentSignature {
            version: SIGNATURE_VERSION_CURRENT,  // v2 with timestamp binding
            signer: SignerInfo {
                id: signer_id,
                name: signer_name,
                certificate: None,
            },
            timestamp,
            scope,
            algorithm: SignatureAlgorithm::Ed25519,
            root_hash: hex::encode(root_hash),
            signature: signature_b64,
        }
    }

    /// Verify an Ed25519 signature with version-aware payload construction
    ///
    /// Security Fixes:
    /// - CVE-TDF-003: Verifies timestamp binding for v2 signatures
    /// - CVE-TDF-013: Validates timestamp is included in verification
    pub fn verify_ed25519(
        signature: &DocumentSignature,
        root_hash: &[u8],
        verifying_key: &VerifyingKey,
    ) -> TdfResult<bool> {
        if signature.algorithm != SignatureAlgorithm::Ed25519 {
            return Err(TdfError::UnsupportedSignatureAlgorithm(format!(
                "{:?}",
                signature.algorithm
            )));
        }

        let signature_bytes = STANDARD
            .decode(&signature.signature)
            .map_err(|e| TdfError::SignatureFailure(format!("Invalid base64 signature: {}", e)))?;

        if signature_bytes.len() != 64 {
            return Err(TdfError::SignatureFailure(
                format!("Invalid signature length: expected 64, got {}", signature_bytes.len())
            ));
        }

        let mut sig_bytes = [0u8; 64];
        sig_bytes.copy_from_slice(&signature_bytes);
        let sig = Signature::from_bytes(&sig_bytes);

        // === SECURITY FIX (CVE-TDF-003, CVE-TDF-013): Version-aware verification ===
        // v2 signatures include timestamp in the signed payload
        // v1 signatures only signed the root_hash (vulnerable to timestamp manipulation)
        let verification_payload = if signature.version >= SIGNATURE_VERSION_CURRENT {
            // v2+: Verify with timestamp binding
            compute_signing_payload(
                root_hash,
                &signature.timestamp.time,
                &signature.signer.id,
                &signature.scope,
            )
        } else {
            // v1 (legacy): Only root_hash was signed
            // WARNING: This is vulnerable to timestamp manipulation
            eprintln!(
                "WARNING: Verifying legacy v1 signature for {}. \
                 Timestamp is NOT cryptographically bound.",
                signature.signer.id
            );
            compute_signing_payload_legacy(root_hash)
        };

        verifying_key
            .verify(&verification_payload, &sig)
            .map_err(|e| TdfError::SignatureFailure(format!("Signature verification failed: {}", e)))?;

        Ok(true)
    }

    /// Sign a document using secp256k1 with timestamp binding (v2 format)
    ///
    /// Security Fix (CVE-TDF-003): Timestamp bound to signature payload
    pub fn sign_secp256k1(
        signing_key: &Secp256k1SigningKey,
        root_hash: &[u8],
        signer_id: String,
        signer_name: String,
        scope: SignatureScope,
    ) -> DocumentSignature {
        use k256::ecdsa::signature::Signer;

        // Get timestamp FIRST (so it's bound to the signature)
        let timestamp = TimestampInfo {
            time: Utc::now(),
            authority: None,
            proof: None,
        };

        // Compute signing payload with timestamp binding (v2 format)
        let signing_payload = compute_signing_payload(
            root_hash,
            &timestamp.time,
            &signer_id,
            &scope,
        );

        // Sign the full payload (includes timestamp)
        let signature: Secp256k1Signature = signing_key.sign(&signing_payload);
        // k256 signatures are DER-encoded, use to_der() to get bytes
        let signature_b64 = STANDARD.encode(signature.to_der().as_bytes());

        DocumentSignature {
            version: SIGNATURE_VERSION_CURRENT,  // v2 with timestamp binding
            signer: SignerInfo {
                id: signer_id,
                name: signer_name,
                certificate: None,
            },
            timestamp,
            scope,
            algorithm: SignatureAlgorithm::Secp256k1,
            root_hash: hex::encode(root_hash),
            signature: signature_b64,
        }
    }

    /// Verify a secp256k1 signature with version-aware payload construction
    ///
    /// Security Fixes:
    /// - CVE-TDF-003: Verifies timestamp binding for v2 signatures
    /// - CVE-TDF-013: Validates timestamp is included in verification
    pub fn verify_secp256k1(
        signature: &DocumentSignature,
        root_hash: &[u8],
        verifying_key: &Secp256k1VerifyingKey,
    ) -> TdfResult<bool> {
        if signature.algorithm != SignatureAlgorithm::Secp256k1 {
            return Err(TdfError::UnsupportedSignatureAlgorithm(format!(
                "{:?}",
                signature.algorithm
            )));
        }

        let signature_bytes = STANDARD
            .decode(&signature.signature)
            .map_err(|e| TdfError::SignatureFailure(format!("Invalid base64 signature: {}", e)))?;

        use k256::ecdsa::signature::Verifier;

        // k256 signatures are DER-encoded, use from_der
        let sig = Secp256k1Signature::from_der(&signature_bytes)
            .map_err(|e| TdfError::SignatureFailure(format!("Invalid DER signature: {}", e)))?;

        // === SECURITY FIX (CVE-TDF-003, CVE-TDF-013): Version-aware verification ===
        let verification_payload = if signature.version >= SIGNATURE_VERSION_CURRENT {
            // v2+: Verify with timestamp binding
            compute_signing_payload(
                root_hash,
                &signature.timestamp.time,
                &signature.signer.id,
                &signature.scope,
            )
        } else {
            // v1 (legacy): Only root_hash was signed
            eprintln!(
                "WARNING: Verifying legacy v1 signature for {}. \
                 Timestamp is NOT cryptographically bound.",
                signature.signer.id
            );
            compute_signing_payload_legacy(root_hash)
        };

        verifying_key
            .verify(&verification_payload, &sig)
            .map_err(|e| TdfError::SignatureFailure(format!("Signature verification failed: {}", e)))?;

        Ok(true)
    }

    pub fn verify_signature_block(
        block: &SignatureBlock,
        root_hash: &[u8],
        verifying_keys: &[(String, VerifyingKey)],
    ) -> TdfResult<Vec<VerificationResult>> {
        Self::verify_signature_block_mixed(block, root_hash, verifying_keys, &[], None)
    }

    pub fn verify_signature_block_with_revocation(
        block: &SignatureBlock,
        root_hash: &[u8],
        verifying_keys: &[(String, VerifyingKey)],
        revocation_manager: Option<&RevocationManager>,
    ) -> TdfResult<Vec<VerificationResult>> {
        Self::verify_signature_block_mixed(block, root_hash, verifying_keys, &[], revocation_manager)
    }

    pub fn verify_signature_block_mixed(
        block: &SignatureBlock,
        root_hash: &[u8],
        ed25519_keys: &[(String, VerifyingKey)],
        secp256k1_keys: &[(String, Secp256k1VerifyingKey)],
        revocation_manager: Option<&RevocationManager>,
    ) -> TdfResult<Vec<VerificationResult>> {
        let mut results = Vec::new();

        for sig in &block.signatures {
            // Check revocation first (before signature verification)
            if let Some(manager) = revocation_manager {
                if let Some(revocation_entry) = manager.is_revoked_at(&sig.signer.id, sig.timestamp.time) {
                    results.push(VerificationResult::Revoked {
                        signer: sig.signer.name.clone(),
                        revoked_at: revocation_entry.revoked_at,
                        reason: format!("Key revoked: {:?}", revocation_entry.reason),
                    });
                    continue; // Skip signature verification for revoked keys
                }
            }

            let result = match sig.algorithm {
                SignatureAlgorithm::Ed25519 => {
                    // Find matching Ed25519 verifying key
                    let key_opt = ed25519_keys
                        .iter()
                        .find(|(id, _)| *id == sig.signer.id)
                        .map(|(_, key)| key);

                    match key_opt {
                        Some(key) => {
                            match Self::verify_ed25519(sig, root_hash, key) {
                                Ok(true) => VerificationResult::Valid {
                                    signer: sig.signer.name.clone(),
                                    timestamp: sig.timestamp.time,
                                },
                                Ok(false) => VerificationResult::Invalid {
                                    signer: sig.signer.name.clone(),
                                    reason: "Signature verification returned false".to_string(),
                                },
                                Err(e) => VerificationResult::Invalid {
                                    signer: sig.signer.name.clone(),
                                    reason: format!("{}", e),
                                },
                            }
                        }
                        None => VerificationResult::Invalid {
                            signer: sig.signer.name.clone(),
                            reason: format!("No Ed25519 verifying key found for signer: {}", sig.signer.id),
                        },
                    }
                }
                SignatureAlgorithm::Secp256k1 => {
                    // Find matching secp256k1 verifying key
                    let key_opt = secp256k1_keys
                        .iter()
                        .find(|(id, _)| *id == sig.signer.id)
                        .map(|(_, key)| key);

                    match key_opt {
                        Some(key) => {
                            match Self::verify_secp256k1(sig, root_hash, key) {
                                Ok(true) => VerificationResult::Valid {
                                    signer: sig.signer.name.clone(),
                                    timestamp: sig.timestamp.time,
                                },
                                Ok(false) => VerificationResult::Invalid {
                                    signer: sig.signer.name.clone(),
                                    reason: "Signature verification returned false".to_string(),
                                },
                                Err(e) => VerificationResult::Invalid {
                                    signer: sig.signer.name.clone(),
                                    reason: format!("{}", e),
                                },
                            }
                        }
                        None => VerificationResult::Invalid {
                            signer: sig.signer.name.clone(),
                            reason: format!("No secp256k1 verifying key found for signer: {}", sig.signer.id),
                        },
                    }
                }
                SignatureAlgorithm::RsaPss => {
                    VerificationResult::Unsupported {
                        signer: sig.signer.name.clone(),
                        algorithm: "RSA-PSS".to_string(),
                    }
                }
            };

            results.push(result);
        }

        Ok(results)
    }
}

#[derive(Debug, Clone)]
pub enum VerificationResult {
    Valid {
        signer: String,
        timestamp: DateTime<Utc>,
    },
    Invalid {
        signer: String,
        reason: String,
    },
    Revoked {
        signer: String,
        revoked_at: DateTime<Utc>,
        reason: String,
    },
    Unsupported {
        signer: String,
        algorithm: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;
    use rand::rngs::OsRng;
    use chrono::Duration;

    #[test]
    fn test_signature_v2_with_timestamp_binding() {
        // Generate a signing key
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();

        let root_hash = b"test_root_hash_for_signing_12345";

        // Sign with v2 (timestamp binding)
        let signature = SignatureManager::sign_ed25519(
            &signing_key,
            root_hash,
            "test-signer".to_string(),
            "Test Signer".to_string(),
            SignatureScope::Full,
        );

        // Verify signature is v2
        assert_eq!(signature.version, SIGNATURE_VERSION_CURRENT);

        // Verify signature is valid
        let result = SignatureManager::verify_ed25519(&signature, root_hash, &verifying_key);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_timestamp_manipulation_detected() {
        // Generate a signing key
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();

        let root_hash = b"test_root_hash_for_signing_12345";

        // Sign with v2 (timestamp binding)
        let mut signature = SignatureManager::sign_ed25519(
            &signing_key,
            root_hash,
            "test-signer".to_string(),
            "Test Signer".to_string(),
            SignatureScope::Full,
        );

        // Manipulate the timestamp (attack attempt)
        signature.timestamp.time = signature.timestamp.time + Duration::days(365);

        // Verification should FAIL because timestamp is bound to signature
        let result = SignatureManager::verify_ed25519(&signature, root_hash, &verifying_key);
        assert!(result.is_err(), "Timestamp manipulation should be detected");
    }

    #[test]
    fn test_signer_id_manipulation_detected() {
        // Generate a signing key
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();

        let root_hash = b"test_root_hash_for_signing_12345";

        // Sign with v2 (timestamp binding)
        let mut signature = SignatureManager::sign_ed25519(
            &signing_key,
            root_hash,
            "original-signer".to_string(),
            "Original Signer".to_string(),
            SignatureScope::Full,
        );

        // Manipulate the signer ID (attack attempt)
        signature.signer.id = "attacker-signer".to_string();

        // Verification should FAIL because signer ID is bound to signature
        let result = SignatureManager::verify_ed25519(&signature, root_hash, &verifying_key);
        assert!(result.is_err(), "Signer ID manipulation should be detected");
    }

    #[test]
    fn test_scope_manipulation_detected() {
        // Generate a signing key
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();

        let root_hash = b"test_root_hash_for_signing_12345";

        // Sign with v2 (timestamp binding) - Full scope
        let mut signature = SignatureManager::sign_ed25519(
            &signing_key,
            root_hash,
            "test-signer".to_string(),
            "Test Signer".to_string(),
            SignatureScope::Full,
        );

        // Manipulate the scope (attack attempt - try to claim only content was signed)
        signature.scope = SignatureScope::ContentOnly;

        // Verification should FAIL because scope is bound to signature
        let result = SignatureManager::verify_ed25519(&signature, root_hash, &verifying_key);
        assert!(result.is_err(), "Scope manipulation should be detected");
    }

    #[test]
    fn test_signing_payload_deterministic() {
        let root_hash = b"test_hash";
        let timestamp = Utc::now();
        let signer_id = "test-signer";
        let scope = SignatureScope::Full;

        // Compute payload twice
        let payload1 = compute_signing_payload(root_hash, &timestamp, signer_id, &scope);
        let payload2 = compute_signing_payload(root_hash, &timestamp, signer_id, &scope);

        // Should be identical
        assert_eq!(payload1, payload2);
    }

    #[test]
    fn test_signing_payload_different_inputs() {
        let root_hash = b"test_hash";
        let timestamp = Utc::now();
        let signer_id = "test-signer";
        let scope = SignatureScope::Full;

        let payload1 = compute_signing_payload(root_hash, &timestamp, signer_id, &scope);

        // Different root hash
        let payload2 = compute_signing_payload(b"different", &timestamp, signer_id, &scope);
        assert_ne!(payload1, payload2);

        // Different timestamp
        let payload3 = compute_signing_payload(root_hash, &(timestamp + Duration::seconds(1)), signer_id, &scope);
        assert_ne!(payload1, payload3);

        // Different signer
        let payload4 = compute_signing_payload(root_hash, &timestamp, "other-signer", &scope);
        assert_ne!(payload1, payload4);

        // Different scope
        let payload5 = compute_signing_payload(root_hash, &timestamp, signer_id, &SignatureScope::ContentOnly);
        assert_ne!(payload1, payload5);
    }

    #[test]
    fn test_sections_scope_sorted() {
        let root_hash = b"test_hash";
        let timestamp = Utc::now();
        let signer_id = "test-signer";

        // Different order, same sections
        let scope1 = SignatureScope::Sections(vec!["section-a".to_string(), "section-b".to_string()]);
        let scope2 = SignatureScope::Sections(vec!["section-b".to_string(), "section-a".to_string()]);

        let payload1 = compute_signing_payload(root_hash, &timestamp, signer_id, &scope1);
        let payload2 = compute_signing_payload(root_hash, &timestamp, signer_id, &scope2);

        // Should be identical because sections are sorted
        assert_eq!(payload1, payload2);
    }

    // === CVE-TDF-019: Scope validation tests ===

    #[test]
    fn test_scope_validation_full_always_valid() {
        let scope = SignatureScope::Full;

        // Full scope is always valid, even with no sections
        assert!(scope.validate(&[]).is_ok());
        assert!(scope.validate(&["section-1", "section-2"]).is_ok());
    }

    #[test]
    fn test_scope_validation_content_only_always_valid() {
        let scope = SignatureScope::ContentOnly;

        // ContentOnly scope is always valid
        assert!(scope.validate(&[]).is_ok());
        assert!(scope.validate(&["section-1"]).is_ok());
    }

    #[test]
    fn test_scope_validation_sections_valid() {
        let available = ["section-1", "section-2", "section-3"];

        // Valid: all sections exist
        let scope1 = SignatureScope::Sections(vec!["section-1".to_string()]);
        assert!(scope1.validate(&available).is_ok());

        let scope2 = SignatureScope::Sections(vec![
            "section-1".to_string(),
            "section-2".to_string(),
        ]);
        assert!(scope2.validate(&available).is_ok());

        let scope3 = SignatureScope::Sections(vec![
            "section-1".to_string(),
            "section-2".to_string(),
            "section-3".to_string(),
        ]);
        assert!(scope3.validate(&available).is_ok());
    }

    #[test]
    fn test_scope_validation_sections_missing() {
        let available = ["section-1", "section-2"];

        // Invalid: section-3 doesn't exist
        let scope = SignatureScope::Sections(vec![
            "section-1".to_string(),
            "section-3".to_string(),  // doesn't exist
        ]);
        let result = scope.validate(&available);
        assert!(result.is_err());

        let err = result.unwrap_err();
        let msg = format!("{}", err);
        assert!(msg.contains("section-3"));
        assert!(msg.contains("non-existent"));
    }

    #[test]
    fn test_scope_validation_sections_empty() {
        let available = ["section-1", "section-2"];

        // Invalid: empty sections list
        let scope = SignatureScope::Sections(vec![]);
        let result = scope.validate(&available);
        assert!(result.is_err());

        let err = result.unwrap_err();
        let msg = format!("{}", err);
        assert!(msg.contains("empty"));
    }

    #[test]
    fn test_scope_validation_no_available_sections() {
        // Invalid: referencing sections when none exist
        let scope = SignatureScope::Sections(vec!["section-1".to_string()]);
        let result = scope.validate(&[]);
        assert!(result.is_err());

        let err = result.unwrap_err();
        let msg = format!("{}", err);
        assert!(msg.contains("(none)"));
    }

    #[test]
    fn test_scope_covers_section() {
        // Full covers everything
        let full = SignatureScope::Full;
        assert!(full.covers_section("any-section"));
        assert!(full.covers_section("another-section"));

        // ContentOnly covers everything
        let content = SignatureScope::ContentOnly;
        assert!(content.covers_section("any-section"));

        // Sections only covers listed sections
        let sections = SignatureScope::Sections(vec![
            "section-1".to_string(),
            "section-2".to_string(),
        ]);
        assert!(sections.covers_section("section-1"));
        assert!(sections.covers_section("section-2"));
        assert!(!sections.covers_section("section-3"));
        assert!(!sections.covers_section("other"));
    }

    #[test]
    fn test_scope_covered_sections() {
        let all_sections = ["section-1", "section-2", "section-3"];

        // Full returns all sections
        let full = SignatureScope::Full;
        let covered = full.covered_sections(&all_sections);
        assert_eq!(covered.len(), 3);

        // ContentOnly returns all sections
        let content = SignatureScope::ContentOnly;
        let covered = content.covered_sections(&all_sections);
        assert_eq!(covered.len(), 3);

        // Sections returns only listed sections
        let sections = SignatureScope::Sections(vec![
            "section-1".to_string(),
            "section-3".to_string(),
        ]);
        let covered = sections.covered_sections(&all_sections);
        assert_eq!(covered.len(), 2);
        assert!(covered.contains(&"section-1"));
        assert!(covered.contains(&"section-3"));
        assert!(!covered.contains(&"section-2"));
    }
}

