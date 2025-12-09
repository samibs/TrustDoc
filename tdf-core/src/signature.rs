use crate::error::{TdfError, TdfResult};
use crate::timestamp::{create_timestamp_token, TimestampToken, TimestampProvider};
use crate::revocation::RevocationManager;
use chrono::{DateTime, Utc};
use ed25519_dalek::{Signature, Signer as Ed25519Signer, SigningKey, Verifier as Ed25519Verifier, VerifyingKey};
use k256::ecdsa::{SigningKey as Secp256k1SigningKey, VerifyingKey as Secp256k1VerifyingKey, Signature as Secp256k1Signature};
use serde::{Deserialize, Serialize};
use base64::{engine::general_purpose::STANDARD, Engine};

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum SignatureScope {
    Full,
    ContentOnly,
    Sections(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SignatureAlgorithm {
    Ed25519,
    Secp256k1,
    RsaPss,
}

pub struct SignatureManager;

impl SignatureManager {
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

    pub fn sign_ed25519_with_timestamp(
        signing_key: &SigningKey,
        root_hash: &[u8],
        signer_id: String,
        signer_name: String,
        scope: SignatureScope,
        timestamp_provider: Option<&dyn TimestampProvider>,
    ) -> DocumentSignature {
        let signature: Signature = signing_key.sign(root_hash);
        let signature_b64 = STANDARD.encode(signature.to_bytes());

        // Get timestamp
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

        DocumentSignature {
            version: 1,
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

        verifying_key
            .verify(root_hash, &sig)
            .map_err(|e| TdfError::SignatureFailure(format!("Signature verification failed: {}", e)))?;

        Ok(true)
    }

    pub fn sign_secp256k1(
        signing_key: &Secp256k1SigningKey,
        root_hash: &[u8],
        signer_id: String,
        signer_name: String,
        scope: SignatureScope,
    ) -> DocumentSignature {
        use k256::ecdsa::signature::Signer;
        let signature: Secp256k1Signature = signing_key.sign(root_hash);
        // k256 signatures are DER-encoded, use to_der() to get bytes
        let signature_b64 = STANDARD.encode(signature.to_der().as_bytes());

        DocumentSignature {
            version: 1,
            signer: SignerInfo {
                id: signer_id,
                name: signer_name,
                certificate: None,
            },
            timestamp: TimestampInfo {
                time: Utc::now(),
                authority: None,
                proof: None,
            },
            scope,
            algorithm: SignatureAlgorithm::Secp256k1,
            root_hash: hex::encode(root_hash),
            signature: signature_b64,
        }
    }

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

        verifying_key
            .verify(root_hash, &sig)
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

