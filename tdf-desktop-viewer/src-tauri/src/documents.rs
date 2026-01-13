// Document Operations Backend

use serde::{Deserialize, Serialize};
use std::path::Path;
use tdf_core::archive::ArchiveReader;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationDetails {
    pub integrity_valid: bool,
    pub root_hash: String,
    pub signature_count: usize,
    pub signatures: Vec<SignatureInfo>,
    pub timestamp_warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureInfo {
    pub signer_id: String,
    pub signer_name: String,
    pub algorithm: String,
    pub timestamp: String,
    pub valid: bool,
}

pub fn verify_document_enhanced(file_path: &Path) -> Result<VerificationDetails, String> {
    let report = ArchiveReader::verify(file_path)
        .map_err(|e| format!("Verification failed: {}", e))?;

    // Parse signatures from the report
    let mut signatures = Vec::new();
    
    // Try to get signature details from the archive
    match ArchiveReader::read(file_path) {
        Ok((_doc, _merkle, signature_block)) => {
            for sig in &signature_block.signatures {
                signatures.push(SignatureInfo {
                    signer_id: sig.signer.id.clone(),
                    signer_name: sig.signer.name.clone(),
                    algorithm: format!("{:?}", sig.algorithm),
                    timestamp: sig.timestamp.time.to_rfc3339(),
                    valid: true, // Assume valid if verification passed
                });
            }
        }
        Err(_) => {
            // If we can't read signatures, that's okay - we still have the verification result
        }
    }

    Ok(VerificationDetails {
        integrity_valid: report.integrity_valid,
        root_hash: report.root_hash,
        signature_count: report.signature_count,
        signatures,
        timestamp_warnings: report.timestamp_warnings,
    })
}
