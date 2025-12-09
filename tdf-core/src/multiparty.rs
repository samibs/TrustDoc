use crate::error::{TdfError, TdfResult};
use crate::signature::{DocumentSignature, SignatureBlock, SignatureManager};
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

        // Verify signature
        // Note: This requires the verifying key, which should be passed separately
        // For now, we just validate structure

        self.signatures.push(signature);
        Ok(())
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

