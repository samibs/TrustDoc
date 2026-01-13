//! End-to-end security tests for TDF format
//! Tests complete attack scenarios and multi-party signing security

use tdf_core::archive::{ArchiveBuilder, ArchiveReader};
use tdf_core::content::{ContentBlock, DocumentContent, Section};
use tdf_core::document::Document;
use tdf_core::signature::{SignatureManager, SignatureScope, VerificationResult};
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use std::fs;
use std::io::{Read, Write};
use tempfile::TempDir;
use zip::ZipArchive;
use zip::write::{FileOptions, ZipWriter};
use zip::CompressionMethod;

// CBOR helpers using ciborium (replaces unmaintained serde_cbor)
fn cbor_to_vec<T: serde::Serialize>(value: &T) -> Result<Vec<u8>, ciborium::ser::Error<std::io::Error>> {
    let mut buf = Vec::new();
    ciborium::into_writer(value, &mut buf)?;
    Ok(buf)
}

fn cbor_from_slice<T: serde::de::DeserializeOwned>(data: &[u8]) -> Result<T, ciborium::de::Error<std::io::Error>> {
    ciborium::from_reader(data)
}

// Helper: Create a realistic financial document
fn create_financial_document() -> Document {
    let content = DocumentContent {
        sections: vec![
            Section {
                id: "executive-summary".to_string(),
                title: Some("Executive Summary".to_string()),
                content: vec![
                    ContentBlock::Heading {
                        level: 1,
                        text: "Q2 2025 Financial Report".to_string(),
                        id: Some("h1".to_string()),
                    },
                    ContentBlock::Paragraph {
                        text: "Revenue increased by 15% compared to Q1.".to_string(),
                        id: Some("p1".to_string()),
                    },
                ],
            },
            Section {
                id: "financial-statements".to_string(),
                title: Some("Financial Statements".to_string()),
                content: vec![
                    ContentBlock::Table {
                        id: "tbl-revenue".to_string(),
                        caption: Some("Revenue by Region".to_string()),
                        columns: vec![],
                        rows: vec![],
                        footer: None,
                    },
                ],
            },
        ],
    };

    let mut document = Document::new(
        "Q2 2025 Financial Report".to_string(),
        "en".to_string(),
        content,
        ".heading-1 { font-size: 24pt; font-weight: bold; }".to_string(),
    );
    document.manifest.integrity.algorithm = tdf_core::document::HashAlgorithm::Sha256;
    document
}

// ============================================================================
// E2E TAMPERING ATTACK SCENARIOS
// ============================================================================

#[test]
fn test_e2e_content_tampering_detection() {
    // Scenario: Attacker modifies financial figures in document
    let document = create_financial_document();
    let mut csprng = OsRng;
    let cfo_key = SigningKey::generate(&mut csprng);
    
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("financial-report.tdf");
    
    let mut builder = ArchiveBuilder::new(document);
    builder
        .build(
            &output_path,
            Some(&cfo_key),
            Some("did:web:cfo.acme.com".to_string()),
            Some("CFO Jane Smith".to_string()),
        )
        .unwrap();
    
    // Initial verification passes
    let report = ArchiveReader::verify(&output_path).unwrap();
    assert!(report.integrity_valid);
    
    // ATTACK: Modify revenue figures
    let file = fs::File::open(&output_path).unwrap();
    let mut zip = ZipArchive::new(file).unwrap();
    
    let mut content_bytes = Vec::new();
    zip.by_name("content.cbor").unwrap().read_to_end(&mut content_bytes).unwrap();
    
    let mut content: DocumentContent = cbor_from_slice(&content_bytes).unwrap();
    // Modify revenue text
    if let Some(section) = content.sections.first_mut() {
        if let Some(ContentBlock::Paragraph { text, .. }) = section.content.iter_mut().find(|b| {
            matches!(b, ContentBlock::Paragraph { .. })
        }) {
            *text = "Revenue increased by 50% compared to Q1.".to_string(); // Fraudulent claim
        }
    }
    let tampered_content = cbor_to_vec(&content).unwrap();
    
    // Read all files first
    let mut files_to_copy = std::collections::HashMap::new();
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        let name = file.name().to_string();
        if name != "content.cbor" {
            let mut data = Vec::new();
            file.read_to_end(&mut data).unwrap();
            files_to_copy.insert(name, data);
        }
    }
    
    // Rebuild ZIP with tampered content
    let file = fs::File::create(&output_path).unwrap();
    let mut zip_writer = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(CompressionMethod::Deflated);
    
    for (name, data) in files_to_copy {
        zip_writer.start_file(name, options).unwrap();
        zip_writer.write_all(&data).unwrap();
    }
    zip_writer.start_file("content.cbor", options).unwrap();
    zip_writer.write_all(&tampered_content).unwrap();
    zip_writer.finish().unwrap();
    
    // DETECTION: Verification should fail
    let report = ArchiveReader::verify(&output_path).unwrap();
    assert!(!report.integrity_valid, "E2E: Content tampering must be detected!");
    
    // The report.root_hash is the stored root hash from hashes.bin (original)
    // After tampering, integrity check fails because computed hash != stored hash
    // But the stored hash (and signature) still contain the original root hash
    
    // Verify signature with the original root hash (from signature) - should pass
    // This proves the signature is valid, but document is still invalid due to tampering
    let (_, _, sig_block) = ArchiveReader::read(&output_path).unwrap();
    let original_root_hash = hex::decode(&sig_block.signatures[0].root_hash).unwrap();
    
    let verifying_key = VerifyingKey::from(&cfo_key);
    let verifying_keys = vec![("did:web:cfo.acme.com".to_string(), verifying_key)];
    let results = SignatureManager::verify_signature_block(
        &sig_block,
        &original_root_hash,
        &verifying_keys,
    ).unwrap();
    
    // Signature should verify with original root hash (signature is valid)
    // But document is still invalid due to integrity failure
    assert!(results.iter().any(|r| matches!(r, VerificationResult::Valid { .. })), 
            "Signature should verify with original root hash");
    
    // The key security property: integrity check failed, so document is invalid
    // even though signature is technically valid over the original content
}

#[test]
fn test_e2e_man_in_the_middle_attack() {
    // Scenario: MITM intercepts document, modifies it, re-signs with own key
    let document = create_financial_document();
    let mut csprng = OsRng;
    let original_key = SigningKey::generate(&mut csprng);
    let attacker_key = SigningKey::generate(&mut csprng);
    
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("intercepted.tdf");
    
    // Original document signed by CFO
    let mut builder = ArchiveBuilder::new(document);
    builder
        .build(
            &output_path,
            Some(&original_key),
            Some("did:web:cfo.acme.com".to_string()),
            Some("CFO Jane Smith".to_string()),
        )
        .unwrap();
    
    // ATTACK: Intercept, modify, re-sign
    // 1. Modify content
    let file = fs::File::open(&output_path).unwrap();
    let mut zip = ZipArchive::new(file).unwrap();
    
    let mut content_bytes = Vec::new();
    zip.by_name("content.cbor").unwrap().read_to_end(&mut content_bytes).unwrap();
    let mut content: DocumentContent = cbor_from_slice(&content_bytes).unwrap();
    // Change revenue
    if let Some(section) = content.sections.first_mut() {
        if let Some(ContentBlock::Paragraph { text, .. }) = section.content.iter_mut().find(|b| {
            matches!(b, ContentBlock::Paragraph { .. })
        }) {
            *text = "Revenue DECREASED by 10%.".to_string();
        }
    }
    let _tampered_content = cbor_to_vec(&content).unwrap();
    
    // 2. Recompute Merkle tree (attacker would need to do this)
    // But they can't without knowing the original structure, so they'll fail
    // OR they rebuild the entire document
    
    // Rebuild document with attacker's signature
    // Create a fresh document with tampered content (builder will create new manifest)
    let (read_doc, _, _) = ArchiveReader::read(&output_path).unwrap();
    let fresh_document = Document::new(
        "Tampered Document".to_string(),
        "en".to_string(),
        content,
        read_doc.styles.clone(),
    );
    let mut builder = ArchiveBuilder::new(fresh_document);
    builder
        .build(
            &output_path,
            Some(&attacker_key),
            Some("did:web:attacker.com".to_string()),
            Some("Attacker".to_string()),
        )
        .unwrap();
    
    // DETECTION: Original signature is gone, new signature doesn't match original signer
    let report = ArchiveReader::verify(&output_path).unwrap();
    // Document is valid but signed by different person
    assert!(report.integrity_valid, "Rebuilt document should be valid");
    
    // Verify with original key - should fail
    let (_, _, sig_block) = ArchiveReader::read(&output_path).unwrap();
    let root_hash = hex::decode(report.root_hash).unwrap();
    let original_verifying_key = VerifyingKey::from(&original_key);
    let verifying_keys = vec![("did:web:cfo.acme.com".to_string(), original_verifying_key)];
    let results = SignatureManager::verify_signature_block(
        &sig_block,
        &root_hash,
        &verifying_keys,
    ).unwrap();
    
    // Original signature not found
    assert!(results.iter().all(|r| matches!(r, VerificationResult::Invalid { .. })));
    
    // Attacker's signature would verify with their key, but that's expected
    let attacker_verifying_key = VerifyingKey::from(&attacker_key);
    let attacker_keys = vec![("did:web:attacker.com".to_string(), attacker_verifying_key)];
    let attacker_results = SignatureManager::verify_signature_block(
        &sig_block,
        &root_hash,
        &attacker_keys,
    ).unwrap();
    
    // Attacker's signature is valid, but we detected the substitution
    assert!(attacker_results.iter().any(|r| matches!(r, VerificationResult::Valid { .. })));
}

#[test]
fn test_e2e_multi_party_signature_attack() {
    // Scenario: Multiple signers, one key compromised
    let document = create_financial_document();
    let mut csprng = OsRng;
    let cfo_key = SigningKey::generate(&mut csprng);
    let ceo_key = SigningKey::generate(&mut csprng);
    let attacker_key = SigningKey::generate(&mut csprng);
    
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("multiparty.tdf");
    
    // Create document signed by CFO
    let mut builder = ArchiveBuilder::new(document);
    builder
        .build(
            &output_path,
            Some(&cfo_key),
            Some("did:web:cfo.acme.com".to_string()),
            Some("CFO Jane Smith".to_string()),
        )
        .unwrap();
    
    // Add CEO signature (multi-party)
    let (_, _, mut sig_block) = ArchiveReader::read(&output_path).unwrap();
    let root_hash = hex::decode(
        ArchiveReader::verify(&output_path).unwrap().root_hash
    ).unwrap();
    
    let ceo_sig = SignatureManager::sign_ed25519(
        &ceo_key,
        &root_hash,
        "did:web:ceo.acme.com".to_string(),
        "CEO Bob Lee".to_string(),
        SignatureScope::Full,
    );
    sig_block.signatures.push(ceo_sig);
    
    // Write back with both signatures
    // First, read all files we need to copy
    let file_read = fs::File::open(&output_path).unwrap();
    let mut zip_read = ZipArchive::new(file_read).unwrap();
    let mut files_to_copy = std::collections::HashMap::new();
    for i in 0..zip_read.len() {
        let mut file = zip_read.by_index(i).unwrap();
        let name = file.name().to_string();
        if name != "signatures.cbor" {
            let mut data = Vec::new();
            file.read_to_end(&mut data).unwrap();
            files_to_copy.insert(name, data);
        }
    }
    
    // Now write new ZIP
    let file = fs::File::create(&output_path).unwrap();
    let mut zip_writer = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(CompressionMethod::Deflated);
    
    for (name, data) in files_to_copy {
        zip_writer.start_file(name, options).unwrap();
        zip_writer.write_all(&data).unwrap();
    }
    let sig_bytes = cbor_to_vec(&sig_block).unwrap();
    zip_writer.start_file("signatures.cbor", options).unwrap();
    zip_writer.write_all(&sig_bytes).unwrap();
    zip_writer.finish().unwrap();
    
    // ATTACK: Attacker tries to add their signature
    let (_, _, mut sig_block_attacked) = ArchiveReader::read(&output_path).unwrap();
    let root_hash_attacked = hex::decode(
        ArchiveReader::verify(&output_path).unwrap().root_hash
    ).unwrap();
    
    let attacker_sig = SignatureManager::sign_ed25519(
        &attacker_key,
        &root_hash_attacked,
        "did:web:attacker.com".to_string(),
        "Attacker".to_string(),
        SignatureScope::Full,
    );
    sig_block_attacked.signatures.push(attacker_sig);
    
    // Write back
    // First, read all files we need to copy
    let file_read = fs::File::open(&output_path).unwrap();
    let mut zip_read = ZipArchive::new(file_read).unwrap();
    let mut files_to_copy = std::collections::HashMap::new();
    for i in 0..zip_read.len() {
        let mut file = zip_read.by_index(i).unwrap();
        let name = file.name().to_string();
        if name != "signatures.cbor" {
            let mut data = Vec::new();
            file.read_to_end(&mut data).unwrap();
            files_to_copy.insert(name, data);
        }
    }
    
    // Now write new ZIP
    let file = fs::File::create(&output_path).unwrap();
    let mut zip_writer = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(CompressionMethod::Deflated);
    
    for (name, data) in files_to_copy {
        zip_writer.start_file(name, options).unwrap();
        zip_writer.write_all(&data).unwrap();
    }
    let sig_bytes = cbor_to_vec(&sig_block_attacked).unwrap();
    zip_writer.start_file("signatures.cbor", options).unwrap();
    zip_writer.write_all(&sig_bytes).unwrap();
    zip_writer.finish().unwrap();
    
    // DETECTION: All signatures should verify, but attacker's signature is present
    let report = ArchiveReader::verify(&output_path).unwrap();
    assert!(report.integrity_valid);
    assert_eq!(report.signature_count, 3); // CFO + CEO + Attacker
    
    // Verify with legitimate keys
    let (_, _, sig_block_final) = ArchiveReader::read(&output_path).unwrap();
    let root_hash_final = hex::decode(report.root_hash).unwrap();
    let cfo_verifying_key = VerifyingKey::from(&cfo_key);
    let ceo_verifying_key = VerifyingKey::from(&ceo_key);
    let verifying_keys = vec![
        ("did:web:cfo.acme.com".to_string(), cfo_verifying_key),
        ("did:web:ceo.acme.com".to_string(), ceo_verifying_key),
    ];
    let results = SignatureManager::verify_signature_block(
        &sig_block_final,
        &root_hash_final,
        &verifying_keys,
    ).unwrap();
    
    // CFO and CEO signatures should be valid
    let valid_count = results.iter().filter(|r| matches!(r, VerificationResult::Valid { .. })).count();
    assert_eq!(valid_count, 2, "CFO and CEO signatures should be valid");
    
    // Attacker's signature would also verify with their key, but that's detectable
    // In real scenario, you'd check signer IDs against whitelist
}

#[test]
fn test_e2e_key_compromise_scenario() {
    // Scenario: Signing key is compromised, attacker creates fraudulent document
    let document = create_financial_document();
    let mut csprng = OsRng;
    let compromised_key = SigningKey::generate(&mut csprng);
    
    // Attacker uses compromised key to sign fraudulent document
    let fraudulent_doc = create_financial_document();
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("fraudulent.tdf");
    
    let mut builder = ArchiveBuilder::new(fraudulent_doc);
    builder
        .build(
            &output_path,
            Some(&compromised_key),
            Some("did:web:cfo.acme.com".to_string()),
            Some("CFO Jane Smith".to_string()),
        )
        .unwrap();
    
    // Document is technically valid and signature verifies
    let report = ArchiveReader::verify(&output_path).unwrap();
    assert!(report.integrity_valid);
    
    let (_, _, sig_block) = ArchiveReader::read(&output_path).unwrap();
    let root_hash = hex::decode(report.root_hash).unwrap();
    let verifying_key = VerifyingKey::from(&compromised_key);
    let verifying_keys = vec![("did:web:cfo.acme.com".to_string(), verifying_key)];
    let results = SignatureManager::verify_signature_block(
        &sig_block,
        &root_hash,
        &verifying_keys,
    ).unwrap();
    
    // Signature verifies (key was compromised)
    assert!(results.iter().any(|r| matches!(r, VerificationResult::Valid { .. })));
    
    // MITIGATION: In real scenario, you'd check:
    // 1. Timestamp (was key revoked before this timestamp?)
    // 2. Certificate revocation list
    // 3. Signer ID whitelist
    // 4. Document content audit trail
}

#[test]
fn test_e2e_timestamp_manipulation_attack() {
    // Scenario: Attacker tries to backdate document
    let document = create_financial_document();
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("backdated.tdf");
    
    let mut builder = ArchiveBuilder::new(document);
    builder
        .build(
            &output_path,
            Some(&signing_key),
            Some("did:web:test.com".to_string()),
            Some("Test Signer".to_string()),
        )
        .unwrap();
    
    // ATTACK: Modify timestamp in signature
    let file = fs::File::open(&output_path).unwrap();
    let mut zip = ZipArchive::new(file).unwrap();
    
    let mut sig_bytes = Vec::new();
    zip.by_name("signatures.cbor").unwrap().read_to_end(&mut sig_bytes).unwrap();
    
    let mut sig_block: tdf_core::signature::SignatureBlock = cbor_from_slice(&sig_bytes).unwrap();
    // Try to backdate (this would require re-signing, but test the structure)
    if let Some(sig) = sig_block.signatures.first_mut() {
        // Modify timestamp (but signature won't match)
        use chrono::{TimeZone, Utc};
        sig.timestamp.time = Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap();
    }
    
    // Read all files first
    let mut files_to_copy = std::collections::HashMap::new();
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        let name = file.name().to_string();
        if name != "signatures.cbor" {
            let mut data = Vec::new();
            file.read_to_end(&mut data).unwrap();
            files_to_copy.insert(name, data);
        }
    }
    
    // Rebuild ZIP
    let file = fs::File::create(&output_path).unwrap();
    let mut zip_writer = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(CompressionMethod::Deflated);
    
    for (name, data) in files_to_copy {
        zip_writer.start_file(name, options).unwrap();
        zip_writer.write_all(&data).unwrap();
    }
    let tampered_sig_bytes = cbor_to_vec(&sig_block).unwrap();
    zip_writer.start_file("signatures.cbor", options).unwrap();
    zip_writer.write_all(&tampered_sig_bytes).unwrap();
    zip_writer.finish().unwrap();
    
    // DETECTION: Signature verification should fail (signature doesn't match modified timestamp)
    // Security Fix (CVE-TDF-003): Timestamp is NOW part of signed data in v2 signatures
    let report = ArchiveReader::verify(&output_path).unwrap();
    assert!(report.integrity_valid); // Integrity still valid (Merkle tree intact)

    // Verify that timestamp manipulation is detected
    let (_, _, sig_block_final) = ArchiveReader::read(&output_path).unwrap();
    let root_hash = hex::decode(report.root_hash).unwrap();
    let verifying_key = VerifyingKey::from(&signing_key);
    let verifying_keys = vec![("did:web:test.com".to_string(), verifying_key)];
    let results = SignatureManager::verify_signature_block(
        &sig_block_final,
        &root_hash,
        &verifying_keys,
    ).unwrap();

    // Security Fix (CVE-TDF-003): Timestamp is bound to signature in v2
    // Signature verification should FAIL because timestamp was manipulated
    assert!(
        results.iter().any(|r| matches!(r, VerificationResult::Invalid { .. })),
        "Timestamp manipulation should be detected and invalidate the signature"
    );
}

#[test]
fn test_e2e_zip_bomb_attack() {
    // Scenario: Attacker creates ZIP bomb (decompression bomb)
    // TDF uses ZIP, so we test handling of malicious ZIP structures
    let document = create_financial_document();
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("zipbomb.tdf");
    
    let mut builder = ArchiveBuilder::new(document);
    builder
        .build(
            &output_path,
            Some(&signing_key),
            Some("did:web:test.com".to_string()),
            Some("Test Signer".to_string()),
        )
        .unwrap();
    
    // ATTACK: Add extremely large file to ZIP
    let file = fs::File::open(&output_path).unwrap();
    let mut zip = ZipArchive::new(file).unwrap();
    
    // Create large file (simulated - in real attack, would be compressed)
    let large_data = vec![0x00; 10_000_000]; // 10MB
    
    // Read all files first
    let mut files_to_copy = std::collections::HashMap::new();
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        let name = file.name().to_string();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        files_to_copy.insert(name, data);
    }
    
    // Rebuild ZIP with bomb
    let file = fs::File::create(&output_path).unwrap();
    let mut zip_writer = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(CompressionMethod::Deflated);
    
    for (name, data) in files_to_copy {
        zip_writer.start_file(name, options).unwrap();
        zip_writer.write_all(&data).unwrap();
    }
    
    // Add bomb file
    zip_writer.start_file("assets/images/bomb.bin", options).unwrap();
    zip_writer.write_all(&large_data).unwrap();
    zip_writer.finish().unwrap();
    
    // DETECTION: Verification should handle gracefully
    // Either fail (file too large) or succeed (but hash mismatch)
    let result = ArchiveReader::verify(&output_path);
    
    // Should either fail or detect hash mismatch
    match result {
        Ok(report) => {
            // If it succeeds, hash should mismatch (bomb not in original Merkle tree)
            assert!(!report.integrity_valid, "ZIP bomb should cause hash mismatch");
        }
        Err(_) => {
            // Or fail to process (size limit, etc.)
            assert!(true, "ZIP bomb should be rejected");
        }
    }
}

#[test]
fn test_e2e_path_traversal_attack() {
    // Scenario: Attacker tries path traversal in ZIP
    let document = create_financial_document();
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("traversal.tdf");
    
    let mut builder = ArchiveBuilder::new(document);
    builder
        .build(
            &output_path,
            Some(&signing_key),
            Some("did:web:test.com".to_string()),
            Some("Test Signer".to_string()),
        )
        .unwrap();
    
    // ATTACK: Add file with path traversal
    let file = fs::File::open(&output_path).unwrap();
    let mut zip = ZipArchive::new(file).unwrap();
    
    let malicious_content = b"malicious";
    
    // Read all files first
    let mut files_to_copy = std::collections::HashMap::new();
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        let name = file.name().to_string();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        files_to_copy.insert(name, data);
    }
    
    // Rebuild ZIP with traversal path
    let file = fs::File::create(&output_path).unwrap();
    let mut zip_writer = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(CompressionMethod::Deflated);
    
    for (name, data) in files_to_copy {
        zip_writer.start_file(name, options).unwrap();
        zip_writer.write_all(&data).unwrap();
    }
    
    // Add traversal file
    zip_writer.start_file("../../../../etc/passwd", options).unwrap();
    zip_writer.write_all(malicious_content).unwrap();
    zip_writer.finish().unwrap();
    
    // DETECTION: Should handle gracefully
    // Path traversal in ZIP is handled by zip crate, but we test it doesn't break verification
    // The malicious file is not in assets/ so it's not included in Merkle tree
    let result = ArchiveReader::verify(&output_path);
    
    // File outside Merkle tree doesn't affect integrity
    // The zip crate handles path traversal safely (doesn't extract outside target)
    match result {
        Ok(report) => {
            // Integrity is still valid because malicious file is not in Merkle tree
            // This is actually correct behavior - extra files don't break integrity
            // But in production, you'd want to validate file paths during extraction
            assert!(report.integrity_valid, "Extra file not in Merkle tree doesn't affect integrity");
        }
        Err(_) => {
            // Or fail to process (size limit, etc.)
            assert!(true, "Path traversal should be handled safely");
        }
    }
}

#[test]
fn test_e2e_complete_workflow_under_attack() {
    // Scenario: Complete document lifecycle with multiple attack attempts
    let document = create_financial_document();
    let mut csprng = OsRng;
    let cfo_key = SigningKey::generate(&mut csprng);
    let ceo_key = SigningKey::generate(&mut csprng);
    
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("workflow.tdf");
    
    // 1. Create document
    let mut builder = ArchiveBuilder::new(document);
    builder
        .build(
            &output_path,
            Some(&cfo_key),
            Some("did:web:cfo.acme.com".to_string()),
            Some("CFO Jane Smith".to_string()),
        )
        .unwrap();
    
    // 2. Initial verification
    let report1 = ArchiveReader::verify(&output_path).unwrap();
    assert!(report1.integrity_valid);
    
    // 3. Add CEO signature
    let (_, _, mut sig_block) = ArchiveReader::read(&output_path).unwrap();
    let root_hash = hex::decode(report1.root_hash).unwrap();
    
    let ceo_sig = SignatureManager::sign_ed25519(
        &ceo_key,
        &root_hash,
        "did:web:ceo.acme.com".to_string(),
        "CEO Bob Lee".to_string(),
        SignatureScope::Full,
    );
    sig_block.signatures.push(ceo_sig);
    
    // Write back
    // First, read all files we need to copy
    let file_read = fs::File::open(&output_path).unwrap();
    let mut zip_read = ZipArchive::new(file_read).unwrap();
    let mut files_to_copy = std::collections::HashMap::new();
    for i in 0..zip_read.len() {
        let mut file = zip_read.by_index(i).unwrap();
        let name = file.name().to_string();
        if name != "signatures.cbor" {
            let mut data = Vec::new();
            file.read_to_end(&mut data).unwrap();
            files_to_copy.insert(name, data);
        }
    }
    
    // Now write new ZIP
    let file = fs::File::create(&output_path).unwrap();
    let mut zip_writer = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(CompressionMethod::Deflated);
    
    for (name, data) in files_to_copy {
        zip_writer.start_file(name, options).unwrap();
        zip_writer.write_all(&data).unwrap();
    }
    let sig_bytes = cbor_to_vec(&sig_block).unwrap();
    zip_writer.start_file("signatures.cbor", options).unwrap();
    zip_writer.write_all(&sig_bytes).unwrap();
    zip_writer.finish().unwrap();
    
    // 4. Verify both signatures
    let report2 = ArchiveReader::verify(&output_path).unwrap();
    assert!(report2.integrity_valid);
    assert_eq!(report2.signature_count, 2);
    
    let (_, _, sig_block_final) = ArchiveReader::read(&output_path).unwrap();
    let root_hash_final = hex::decode(report2.root_hash).unwrap();
    let cfo_verifying_key = VerifyingKey::from(&cfo_key);
    let ceo_verifying_key = VerifyingKey::from(&ceo_key);
    let verifying_keys = vec![
        ("did:web:cfo.acme.com".to_string(), cfo_verifying_key),
        ("did:web:ceo.acme.com".to_string(), ceo_verifying_key),
    ];
    let results = SignatureManager::verify_signature_block(
        &sig_block_final,
        &root_hash_final,
        &verifying_keys,
    ).unwrap();
    
    let valid_count = results.iter().filter(|r| matches!(r, VerificationResult::Valid { .. })).count();
    assert_eq!(valid_count, 2, "Both signatures should be valid");
    
    // 5. ATTACK: Try to modify after signing
    let file = fs::File::open(&output_path).unwrap();
    let mut zip = ZipArchive::new(file).unwrap();
    
    let mut content_bytes = Vec::new();
    zip.by_name("content.cbor").unwrap().read_to_end(&mut content_bytes).unwrap();
    let mut content: DocumentContent = cbor_from_slice(&content_bytes).unwrap();
    // Modify
    if let Some(section) = content.sections.first_mut() {
        if let Some(ContentBlock::Paragraph { text, .. }) = section.content.iter_mut().find(|b| {
            matches!(b, ContentBlock::Paragraph { .. })
        }) {
            *text = "TAMPERED AFTER SIGNING".to_string();
        }
    }
    let tampered_content = cbor_to_vec(&content).unwrap();
    
    // Read all files first
    let mut files_to_copy = std::collections::HashMap::new();
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        let name = file.name().to_string();
        if name != "content.cbor" {
            let mut data = Vec::new();
            file.read_to_end(&mut data).unwrap();
            files_to_copy.insert(name, data);
        }
    }
    
    // Rebuild
    let file = fs::File::create(&output_path).unwrap();
    let mut zip_writer = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(CompressionMethod::Deflated);
    
    for (name, data) in files_to_copy {
        zip_writer.start_file(name, options).unwrap();
        zip_writer.write_all(&data).unwrap();
    }
    zip_writer.start_file("content.cbor", options).unwrap();
    zip_writer.write_all(&tampered_content).unwrap();
    zip_writer.finish().unwrap();
    
    // 6. DETECTION: Should fail
    let report3 = ArchiveReader::verify(&output_path).unwrap();
    assert!(!report3.integrity_valid, "Post-signing tampering must be detected!");
}

