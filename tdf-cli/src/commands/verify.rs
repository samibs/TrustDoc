use crate::utils;
use ed25519_dalek::VerifyingKey;
use std::path::PathBuf;
use tdf_core::archive::ArchiveReader;
use tdf_core::error::TdfResult;
use tdf_core::signature::SignatureManager;

pub fn verify_document(document: PathBuf, key: Option<PathBuf>) -> TdfResult<()> {
    let report = ArchiveReader::verify(&document)?;

    println!("Verification Report");
    println!("==================");
    println!("Document: {}", document.display());
    println!("Integrity: {}", if report.integrity_valid { "✓ VALID" } else { "✗ INVALID" });
    println!("Root Hash: {}", report.root_hash);
    println!("Signatures: {}", report.signature_count);

    if !report.integrity_valid {
        eprintln!("\n✗ INTEGRITY FAILURE");
        eprintln!("The document has been modified or corrupted.");
        return Ok(());
    }

    // If key provided, verify signatures
    if let Some(key_path) = key {
        let verifying_key = utils::load_verifying_key(&key_path)?;
        let (doc, _, sig_block) = ArchiveReader::read(&document)?;

        // Get root hash from document
        let root_hash_hex = &doc.manifest.integrity.root_hash;
        let root_hash = hex::decode(root_hash_hex)
            .map_err(|e| tdf_core::error::TdfError::InvalidDocument(format!("Invalid root hash hex: {}", e)))?;

        let mut keys = Vec::new();
        for sig in &sig_block.signatures {
            keys.push((sig.signer.id.clone(), verifying_key.clone()));
        }

        let results = SignatureManager::verify_signature_block(&sig_block, &root_hash, &keys)?;

        println!("\nSignature Verification:");
        for result in results {
            match result {
                tdf_core::signature::VerificationResult::Valid { signer, timestamp } => {
                    println!("  ✓ {} (signed at {})", signer, timestamp);
                }
                tdf_core::signature::VerificationResult::Invalid { signer, reason } => {
                    println!("  ✗ {}: {}", signer, reason);
                }
                tdf_core::signature::VerificationResult::Revoked { signer, revoked_at, reason } => {
                    println!("  ⚠ REVOKED: {} (revoked at {}, reason: {})", signer, revoked_at, reason);
                }
                tdf_core::signature::VerificationResult::Unsupported { signer, algorithm } => {
                    println!("  ? {}: {} (unsupported)", signer, algorithm);
                }
            }
        }
    } else if report.signature_count > 0 {
        println!("\nNote: {} signature(s) present but not verified (use --key to verify)", report.signature_count);
    }

    Ok(())
}

