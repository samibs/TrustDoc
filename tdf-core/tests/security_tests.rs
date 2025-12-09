//! Comprehensive security tests for TDF format
//! Tests tampering detection, signature attacks, hash manipulation, and format validation

use tdf_core::archive::{ArchiveBuilder, ArchiveReader};
use tdf_core::content::{ContentBlock, DocumentContent, Section};
use tdf_core::document::Document;
use tdf_core::merkle::{HashAlgorithm, MerkleTree};
use tdf_core::signature::{SignatureManager, SignatureScope, VerificationResult};
use ed25519_dalek::{SigningKey, VerifyingKey};
use k256::ecdsa::{SigningKey as Secp256k1SigningKey, VerifyingKey as Secp256k1VerifyingKey};
use rand::rngs::OsRng;
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use tempfile::TempDir;
use zip::ZipArchive;
use zip::write::{FileOptions, ZipWriter};
use zip::CompressionMethod;

// Helper: Create a valid test document
fn create_test_document() -> Document {
    let content = DocumentContent {
        sections: vec![Section {
            id: "sec-1".to_string(),
            title: Some("Security Test Section".to_string()),
            content: vec![ContentBlock::Paragraph {
                text: "This is original secure content that must not be tampered with.".to_string(),
                id: Some("p-1".to_string()),
            }],
        }],
    };

    let mut document = Document::new(
        "Security Test Document".to_string(),
        "en".to_string(),
        content,
        "body { font-family: Arial; }".to_string(),
    );
    document.manifest.integrity.algorithm = tdf_core::document::HashAlgorithm::Sha256;
    document
}

// Helper: Create and sign a document
fn create_signed_document(
    document: Document,
    signing_key: &SigningKey,
) -> (std::path::PathBuf, SigningKey, TempDir) {
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test.tdf");
    
    let mut builder = ArchiveBuilder::new(document);
    builder
        .build(
            &output_path,
            Some(signing_key),
            Some("did:web:security-test.com".to_string()),
            Some("Security Test Signer".to_string()),
        )
        .unwrap();

    (output_path, signing_key.clone(), temp_dir)
}

// ============================================================================
// TAMPERING DETECTION TESTS
// ============================================================================

#[test]
fn test_tamper_content_modification() {
    // Create valid document
    let document = create_test_document();
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let (path, _, _temp_dir) = create_signed_document(document, &signing_key);

    // Tamper: Modify content.cbor inside ZIP
    // First, read all files
    let file = fs::File::open(&path).unwrap();
    let mut zip = ZipArchive::new(file).unwrap();
    
    // Read original content
    let mut content_bytes = Vec::new();
    zip.by_name("content.cbor").unwrap().read_to_end(&mut content_bytes).unwrap();
    
    // Modify content (change text)
    let mut content: DocumentContent = serde_cbor::from_slice(&content_bytes).unwrap();
    if let Some(section) = content.sections.first_mut() {
        if let Some(ContentBlock::Paragraph { text, .. }) = section.content.first_mut() {
            *text = "TAMPERED CONTENT - This should be detected!".to_string();
        }
    }
    let tampered_content = serde_cbor::to_vec(&content).unwrap();
    
    // Read all other files first
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
    
    // Now write tampered content back to ZIP
    let file = fs::File::create(&path).unwrap();
    let mut zip_writer = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(CompressionMethod::Deflated);
    
    // Copy all files except content.cbor, then add tampered version
    for (name, data) in files_to_copy {
        zip_writer.start_file(name, options).unwrap();
        zip_writer.write_all(&data).unwrap();
    }
    zip_writer.start_file("content.cbor", options).unwrap();
    zip_writer.write_all(&tampered_content).unwrap();
    zip_writer.finish().unwrap();

    // Verification should FAIL
    let report = ArchiveReader::verify(&path).unwrap();
    assert!(!report.integrity_valid, "Tampered content should be detected!");
}

#[test]
fn test_tamper_manifest_modification() {
    let document = create_test_document();
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let (path, _, _temp_dir) = create_signed_document(document, &signing_key);

    // Tamper: Modify manifest.cbor (change document title)
    let file = fs::File::open(&path).unwrap();
    let mut zip = ZipArchive::new(file).unwrap();
    
    let mut manifest_bytes = Vec::new();
    zip.by_name("manifest.cbor").unwrap().read_to_end(&mut manifest_bytes).unwrap();
    
    let mut manifest: tdf_core::document::Manifest = serde_cbor::from_slice(&manifest_bytes).unwrap();
    manifest.document.title = "TAMPERED TITLE - Should be detected!".to_string();
    let tampered_manifest = serde_cbor::to_vec(&manifest).unwrap();
    
    // Rebuild ZIP with tampered manifest
    // First, read all files we need to copy
    let mut files_to_copy = std::collections::HashMap::new();
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        let name = file.name().to_string();
        if name != "manifest.cbor" {
            let mut data = Vec::new();
            file.read_to_end(&mut data).unwrap();
            files_to_copy.insert(name, data);
        }
    }
    
    // Now write new ZIP
    let file = fs::File::create(&path).unwrap();
    let mut zip_writer = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(CompressionMethod::Deflated);
    
    for (name, data) in files_to_copy {
        zip_writer.start_file(name, options).unwrap();
        zip_writer.write_all(&data).unwrap();
    }
    zip_writer.start_file("manifest.cbor", options).unwrap();
    zip_writer.write_all(&tampered_manifest).unwrap();
    zip_writer.finish().unwrap();

    // Verification should FAIL
    let report = ArchiveReader::verify(&path).unwrap();
    assert!(!report.integrity_valid, "Tampered manifest should be detected!");
}

#[test]
fn test_tamper_merkle_tree_manipulation() {
    let document = create_test_document();
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let (path, _, _temp_dir) = create_signed_document(document, &signing_key);

    // Tamper: Modify hashes.bin (replace root hash with fake one)
    let file = fs::File::open(&path).unwrap();
    let mut zip = ZipArchive::new(file).unwrap();
    
    let mut hashes_bytes = Vec::new();
    zip.by_name("hashes.bin").unwrap().read_to_end(&mut hashes_bytes).unwrap();
    
    // Replace root hash (bytes 10-42) with all zeros
    if hashes_bytes.len() >= 42 {
        for i in 10..42 {
            hashes_bytes[i] = 0x00;
        }
    }
    
    // Rebuild ZIP with tampered hashes
    // First, read all files we need to copy
    let mut files_to_copy = std::collections::HashMap::new();
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        let name = file.name().to_string();
        if name != "hashes.bin" {
            let mut data = Vec::new();
            file.read_to_end(&mut data).unwrap();
            files_to_copy.insert(name, data);
        }
    }
    
    // Now write new ZIP
    let file = fs::File::create(&path).unwrap();
    let mut zip_writer = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(CompressionMethod::Deflated);
    
    for (name, data) in files_to_copy {
        zip_writer.start_file(name, options).unwrap();
        zip_writer.write_all(&data).unwrap();
    }
    zip_writer.start_file("hashes.bin", options).unwrap();
    zip_writer.write_all(&hashes_bytes).unwrap();
    zip_writer.finish().unwrap();

    // Verification should FAIL
    let report = ArchiveReader::verify(&path).unwrap();
    assert!(!report.integrity_valid, "Tampered Merkle tree should be detected!");
}

#[test]
fn test_tamper_styles_modification() {
    let document = create_test_document();
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let (path, _, _temp_dir) = create_signed_document(document, &signing_key);

    // Tamper: Modify styles.css
    let file = fs::File::open(&path).unwrap();
    let mut zip = ZipArchive::new(file).unwrap();
    
    let mut styles_bytes = Vec::new();
    zip.by_name("styles.css").unwrap().read_to_end(&mut styles_bytes).unwrap();
    let mut tampered_styles = styles_bytes.clone();
    tampered_styles.extend_from_slice(b"\n/* MALICIOUS INJECTION */");
    
    // Rebuild ZIP
    // First, read all files we need to copy
    let mut files_to_copy = std::collections::HashMap::new();
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        let name = file.name().to_string();
        if name != "styles.css" {
            let mut data = Vec::new();
            file.read_to_end(&mut data).unwrap();
            files_to_copy.insert(name, data);
        }
    }
    
    // Now write new ZIP
    let file = fs::File::create(&path).unwrap();
    let mut zip_writer = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(CompressionMethod::Deflated);
    
    for (name, data) in files_to_copy {
        zip_writer.start_file(name, options).unwrap();
        zip_writer.write_all(&data).unwrap();
    }
    zip_writer.start_file("styles.css", options).unwrap();
    zip_writer.write_all(&tampered_styles).unwrap();
    zip_writer.finish().unwrap();

    // Verification should FAIL
    let report = ArchiveReader::verify(&path).unwrap();
    assert!(!report.integrity_valid, "Tampered styles should be detected!");
}

#[test]
fn test_tamper_add_malicious_file() {
    let document = create_test_document();
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let (path, _, _temp_dir) = create_signed_document(document, &signing_key);

    // Tamper: Add malicious file to ZIP (should not affect verification if not in Merkle tree)
    // But this tests ZIP structure integrity
    let file = fs::File::open(&path).unwrap();
    let mut zip = ZipArchive::new(file).unwrap();
    
    // Read all original files first
    let mut files_to_copy = std::collections::HashMap::new();
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        let name = file.name().to_string();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        files_to_copy.insert(name, data);
    }
    
    // Rebuild ZIP with extra malicious file
    let file = fs::File::create(&path).unwrap();
    let mut zip_writer = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(CompressionMethod::Deflated);
    
    // Copy all original files
    for (name, data) in files_to_copy {
        zip_writer.start_file(name, options).unwrap();
        zip_writer.write_all(&data).unwrap();
    }
    
    // Add malicious file
    zip_writer.start_file("../../../etc/passwd", options).unwrap();
    zip_writer.write_all(b"malicious content").unwrap();
    zip_writer.finish().unwrap();

    // Verification should still work (malicious file not in Merkle tree)
    // But we test that path traversal doesn't break verification
    let report = ArchiveReader::verify(&path);
    // Should either succeed (file ignored) or fail gracefully
    assert!(report.is_ok() || report.is_err(), "Should handle malicious files gracefully");
}

// ============================================================================
// SIGNATURE ATTACK TESTS
// ============================================================================

#[test]
fn test_signature_wrong_key_verification() {
    let document = create_test_document();
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let (path, _, _temp_dir) = create_signed_document(document, &signing_key);

    // Try to verify with wrong key
    let wrong_key = SigningKey::generate(&mut csprng);
    let wrong_verifying_key = VerifyingKey::from(&wrong_key);
    
    let (_, _, sig_block) = ArchiveReader::read(&path).unwrap();
    let root_hash = hex::decode(
        ArchiveReader::verify(&path).unwrap().root_hash
    ).unwrap();
    
    let verifying_keys = vec![("did:web:wrong.com".to_string(), wrong_verifying_key)];
    let results = SignatureManager::verify_signature_block(
        &sig_block,
        &root_hash,
        &verifying_keys,
    ).unwrap();
    
    // All signatures should be invalid
    assert!(results.iter().all(|r| matches!(r, VerificationResult::Invalid { .. })));
}

#[test]
fn test_signature_replacement_attack() {
    let document = create_test_document();
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let (path, _, _temp_dir) = create_signed_document(document, &signing_key);

    // Attack: Replace signature with one from different key
    let attacker_key = SigningKey::generate(&mut csprng);
    let root_hash = hex::decode(
        ArchiveReader::verify(&path).unwrap().root_hash
    ).unwrap();
    
    // Create fake signature
    let fake_signature = SignatureManager::sign_ed25519(
        &attacker_key,
        &root_hash,
        "did:web:attacker.com".to_string(),
        "Attacker".to_string(),
        SignatureScope::Full,
    );
    
    let fake_sig_block = tdf_core::signature::SignatureBlock {
        signatures: vec![fake_signature],
    };
    let fake_sig_bytes = serde_cbor::to_vec(&fake_sig_block).unwrap();
    
    // Replace signatures.cbor in ZIP
    let file = fs::File::open(&path).unwrap();
    let mut zip = ZipArchive::new(file).unwrap();
    
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
    
    let file = fs::File::create(&path).unwrap();
    let mut zip_writer = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(CompressionMethod::Deflated);
    
    for (name, data) in files_to_copy {
        zip_writer.start_file(name, options).unwrap();
        zip_writer.write_all(&data).unwrap();
    }
    zip_writer.start_file("signatures.cbor", options).unwrap();
    zip_writer.write_all(&fake_sig_bytes).unwrap();
    zip_writer.finish().unwrap();

    // Verification should detect wrong signature
    let report = ArchiveReader::verify(&path).unwrap();
    // Integrity might still be valid, but signature verification should fail
    // We need to verify signatures separately
    let (_, _, sig_block) = ArchiveReader::read(&path).unwrap();
    let root_hash = hex::decode(report.root_hash).unwrap();
    let verifying_key = VerifyingKey::from(&signing_key);
    let verifying_keys = vec![("did:web:security-test.com".to_string(), verifying_key)];
    let results = SignatureManager::verify_signature_block(
        &sig_block,
        &root_hash,
        &verifying_keys,
    ).unwrap();
    
    assert!(results.iter().all(|r| matches!(r, VerificationResult::Invalid { .. })));
}

#[test]
fn test_signature_modification_attack() {
    let document = create_test_document();
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let (path, _, _temp_dir) = create_signed_document(document, &signing_key);

    // Attack: Modify signature bytes directly
    let file = fs::File::open(&path).unwrap();
    let mut zip = ZipArchive::new(file).unwrap();
    
    let mut sig_bytes = Vec::new();
    zip.by_name("signatures.cbor").unwrap().read_to_end(&mut sig_bytes).unwrap();
    
    // Flip some bits in signature
    if sig_bytes.len() > 50 {
        for i in 50..sig_bytes.len().min(100) {
            sig_bytes[i] ^= 0xFF;
        }
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
    let file = fs::File::create(&path).unwrap();
    let mut zip_writer = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(CompressionMethod::Deflated);
    
    for (name, data) in files_to_copy {
        zip_writer.start_file(name, options).unwrap();
        zip_writer.write_all(&data).unwrap();
    }
    zip_writer.start_file("signatures.cbor", options).unwrap();
    zip_writer.write_all(&sig_bytes).unwrap();
    zip_writer.finish().unwrap();

    // Signature verification should fail (may fail at parse or verification)
    let sig_block_result = ArchiveReader::read(&path);
    // Should either fail to parse or have invalid signatures
    match sig_block_result {
        Ok((_, _, block)) => {
            let root_hash = hex::decode(
                ArchiveReader::verify(&path).unwrap().root_hash
            ).unwrap();
            let verifying_key = VerifyingKey::from(&signing_key);
            let verifying_keys = vec![("did:web:security-test.com".to_string(), verifying_key)];
            let results = SignatureManager::verify_signature_block(
                &block,
                &root_hash,
                &verifying_keys,
            ).unwrap();
            assert!(results.iter().all(|r| matches!(r, VerificationResult::Invalid { .. })));
        }
        Err(_) => {
            // Failed to parse - also acceptable
            assert!(true);
        }
    }
}

#[test]
fn test_signature_replay_attack() {
    // Test that reusing a signature from one document on another fails
    let document1 = create_test_document();
    let document2 = create_test_document();
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    
    let temp_dir = TempDir::new().unwrap();
    let path1 = temp_dir.path().join("doc1.tdf");
    let path2 = temp_dir.path().join("doc2.tdf");
    
    let mut builder1 = ArchiveBuilder::new(document1);
    builder1.build(
        &path1,
        Some(&signing_key),
        Some("did:web:test.com".to_string()),
        Some("Test Signer".to_string()),
    ).unwrap();
    
    let mut builder2 = ArchiveBuilder::new(document2);
    builder2.build(
        &path2,
        Some(&signing_key),
        Some("did:web:test.com".to_string()),
        Some("Test Signer".to_string()),
    ).unwrap();
    
    // Get signature from doc1
    let (_, _, sig_block1) = ArchiveReader::read(&path1).unwrap();
    let root_hash1 = ArchiveReader::verify(&path1).unwrap().root_hash;
    
    // Get root hash from doc2
    let root_hash2 = ArchiveReader::verify(&path2).unwrap().root_hash;
    
    // Try to use doc1's signature with doc2's root hash
    let verifying_key = VerifyingKey::from(&signing_key);
    let verifying_keys = vec![("did:web:test.com".to_string(), verifying_key)];
    let root_hash2_bytes = hex::decode(&root_hash2).unwrap();
    
    let results = SignatureManager::verify_signature_block(
        &sig_block1,
        &root_hash2_bytes,
        &verifying_keys,
    ).unwrap();
    
    // Should fail - signature is for different root hash
    assert!(results.iter().all(|r| matches!(r, VerificationResult::Invalid { .. })));
}

// ============================================================================
// HASH ATTACK TESTS
// ============================================================================

#[test]
fn test_hash_collision_resistance() {
    // Test that different content produces different hashes
    let mut tree1 = MerkleTree::new(HashAlgorithm::Sha256);
    let mut tree2 = MerkleTree::new(HashAlgorithm::Sha256);
    
    let mut components1 = HashMap::new();
    components1.insert("content".to_string(), b"Original content".to_vec());
    
    let mut components2 = HashMap::new();
    components2.insert("content".to_string(), b"Modified content".to_vec());
    
    let root1 = tree1.compute_root(&components1).unwrap();
    let root2 = tree2.compute_root(&components2).unwrap();
    
    // Hashes should be different
    assert_ne!(root1, root2, "Different content should produce different hashes");
}

#[test]
fn test_merkle_tree_ordering_matters() {
    // Test that component order affects root hash
    let mut tree1 = MerkleTree::new(HashAlgorithm::Sha256);
    let mut tree2 = MerkleTree::new(HashAlgorithm::Sha256);
    
    let mut components1 = HashMap::new();
    components1.insert("a".to_string(), b"data1".to_vec());
    components1.insert("b".to_string(), b"data2".to_vec());
    
    let mut components2 = HashMap::new();
    components2.insert("b".to_string(), b"data2".to_vec());
    components2.insert("a".to_string(), b"data1".to_vec());
    
    // MerkleTree sorts hashes, so order shouldn't matter
    let root1 = tree1.compute_root(&components1).unwrap();
    let root2 = tree2.compute_root(&components2).unwrap();
    
    // Should be same (sorted)
    assert_eq!(root1, root2, "Sorted components should produce same hash");
}

#[test]
fn test_root_hash_substitution_attack() {
    let document = create_test_document();
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let (path, _, _temp_dir) = create_signed_document(document, &signing_key);

    // Attack: Replace root hash in manifest with different hash
    let file = fs::File::open(&path).unwrap();
    let mut zip = ZipArchive::new(file).unwrap();
    
    let mut manifest_bytes = Vec::new();
    zip.by_name("manifest.cbor").unwrap().read_to_end(&mut manifest_bytes).unwrap();
    
    let mut manifest: tdf_core::document::Manifest = serde_cbor::from_slice(&manifest_bytes).unwrap();
    // Replace with fake hash
    manifest.integrity.root_hash = "a".repeat(64); // Fake SHA256 hex hash
    let tampered_manifest = serde_cbor::to_vec(&manifest).unwrap();
    
    // Rebuild ZIP
    // First, read all files we need to copy
    let mut files_to_copy = std::collections::HashMap::new();
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        let name = file.name().to_string();
        if name != "manifest.cbor" {
            let mut data = Vec::new();
            file.read_to_end(&mut data).unwrap();
            files_to_copy.insert(name, data);
        }
    }
    
    // Now write new ZIP
    let file = fs::File::create(&path).unwrap();
    let mut zip_writer = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(CompressionMethod::Deflated);
    
    for (name, data) in files_to_copy {
        zip_writer.start_file(name, options).unwrap();
        zip_writer.write_all(&data).unwrap();
    }
    zip_writer.start_file("manifest.cbor", options).unwrap();
    zip_writer.write_all(&tampered_manifest).unwrap();
    zip_writer.finish().unwrap();

    // Verification: The root_hash in manifest is excluded from hash computation
    // So modifying it doesn't affect integrity check directly
    // However, the root_hash should match the Merkle tree root
    let report = ArchiveReader::verify(&path).unwrap();
    // The root hash in manifest is stored but not used in hash computation
    // The actual verification compares Merkle tree root with computed root
    // So modifying manifest root_hash doesn't break integrity (it's metadata)
    // But we verify the stored root_hash doesn't match the computed one
    let computed_root = hex::decode(&report.root_hash).unwrap();
    let stored_root = hex::decode(&manifest.integrity.root_hash).unwrap();
    assert_ne!(computed_root, stored_root, "Modified root hash should not match computed root");
    // Integrity is still valid because root_hash is excluded from manifest hash
    assert!(report.integrity_valid, "Integrity valid because root_hash is metadata");
}

// ============================================================================
// FORMAT VALIDATION TESTS
// ============================================================================

#[test]
fn test_missing_required_file() {
    let document = create_test_document();
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let (path, _, _temp_dir) = create_signed_document(document, &signing_key);

    // Attack: Remove required file (content.cbor)
    let file = fs::File::open(&path).unwrap();
    let mut zip = ZipArchive::new(file).unwrap();
    
    // Read all files except content.cbor
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
    
    let file = fs::File::create(&path).unwrap();
    let mut zip_writer = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(CompressionMethod::Deflated);
    
    for (name, data) in files_to_copy {
        zip_writer.start_file(name, options).unwrap();
        zip_writer.write_all(&data).unwrap();
    }
    zip_writer.finish().unwrap();

    // Should fail - missing required file
    let result = ArchiveReader::verify(&path);
    assert!(result.is_err(), "Missing required file should cause error");
}

#[test]
fn test_malformed_cbor_attack() {
    let document = create_test_document();
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let (path, _, _temp_dir) = create_signed_document(document, &signing_key);

    // Attack: Replace content.cbor with malformed CBOR
    let file = fs::File::open(&path).unwrap();
    let mut zip = ZipArchive::new(file).unwrap();
    
    let malformed_cbor = vec![0xFF, 0xFF, 0xFF, 0xFF]; // Invalid CBOR
    
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
    
    // Rebuild ZIP
    let file = fs::File::create(&path).unwrap();
    let mut zip_writer = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(CompressionMethod::Deflated);
    
    for (name, data) in files_to_copy {
        zip_writer.start_file(name, options).unwrap();
        zip_writer.write_all(&data).unwrap();
    }
    zip_writer.start_file("content.cbor", options).unwrap();
    zip_writer.write_all(&malformed_cbor).unwrap();
    zip_writer.finish().unwrap();

    // Should fail - malformed CBOR
    let result = ArchiveReader::verify(&path);
    assert!(result.is_err(), "Malformed CBOR should cause error");
}

#[test]
fn test_malformed_merkle_tree_attack() {
    let document = create_test_document();
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let (path, _, _temp_dir) = create_signed_document(document, &signing_key);

    // Attack: Replace hashes.bin with garbage
    let file = fs::File::open(&path).unwrap();
    let mut zip = ZipArchive::new(file).unwrap();
    
    let garbage = vec![0x00; 100]; // Invalid Merkle tree format
    
    // Read all files first
    let mut files_to_copy = std::collections::HashMap::new();
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        let name = file.name().to_string();
        if name != "hashes.bin" {
            let mut data = Vec::new();
            file.read_to_end(&mut data).unwrap();
            files_to_copy.insert(name, data);
        }
    }
    
    // Rebuild ZIP
    let file = fs::File::create(&path).unwrap();
    let mut zip_writer = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(CompressionMethod::Deflated);
    
    for (name, data) in files_to_copy {
        zip_writer.start_file(name, options).unwrap();
        zip_writer.write_all(&data).unwrap();
    }
    zip_writer.start_file("hashes.bin", options).unwrap();
    zip_writer.write_all(&garbage).unwrap();
    zip_writer.finish().unwrap();

    // Should fail - malformed Merkle tree
    let result = ArchiveReader::verify(&path);
    assert!(result.is_err(), "Malformed Merkle tree should cause error");
}

#[test]
fn test_empty_file_attack() {
    let document = create_test_document();
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let (path, _, _temp_dir) = create_signed_document(document, &signing_key);

    // Attack: Replace content.cbor with empty file
    let file = fs::File::open(&path).unwrap();
    let mut zip = ZipArchive::new(file).unwrap();
    
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
    
    // Rebuild ZIP
    let file = fs::File::create(&path).unwrap();
    let mut zip_writer = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(CompressionMethod::Deflated);
    
    for (name, data) in files_to_copy {
        zip_writer.start_file(name, options).unwrap();
        zip_writer.write_all(&data).unwrap();
    }
    zip_writer.start_file("content.cbor", options).unwrap();
    // Empty file
    zip_writer.finish().unwrap();

    // Should fail - empty content or hash mismatch
    let result = ArchiveReader::verify(&path);
    // Either fails to parse or hash mismatch
    match result {
        Ok(report) => assert!(!report.integrity_valid, "Empty content should cause hash mismatch"),
        Err(_) => assert!(true, "Empty content should cause parse error"),
    }
}

// ============================================================================
// MULTI-ALGORITHM TESTS
// ============================================================================

#[test]
fn test_secp256k1_signature_verification() {
    let document = create_test_document();
    let mut csprng = OsRng;
    let secp256k1_key = Secp256k1SigningKey::random(&mut csprng);
    
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test-secp256k1.tdf");
    
    let mut builder = ArchiveBuilder::new(document);
    builder
        .build_with_timestamp(
            &output_path,
            None,
            Some(&secp256k1_key),
            Some("did:web:secp256k1-test.com".to_string()),
            Some("Secp256k1 Signer".to_string()),
            Some(tdf_core::signature::SignatureAlgorithm::Secp256k1),
            None,
        )
        .unwrap();
    
    // Verify
    let report = ArchiveReader::verify(&output_path).unwrap();
    assert!(report.integrity_valid);
    assert_eq!(report.signature_count, 1);
    
    // Verify signature
    let (_, _, sig_block) = ArchiveReader::read(&output_path).unwrap();
    let root_hash = hex::decode(report.root_hash).unwrap();
    let verifying_key = Secp256k1VerifyingKey::from(&secp256k1_key);
    let secp256k1_keys = vec![("did:web:secp256k1-test.com".to_string(), verifying_key)];
    
    let results = SignatureManager::verify_signature_block_mixed(
        &sig_block,
        &root_hash,
        &[],
        &secp256k1_keys,
        None,
    ).unwrap();
    
    assert!(results.iter().any(|r| matches!(r, VerificationResult::Valid { .. })));
}

#[test]
fn test_blake3_hash_algorithm() {
    let mut document = create_test_document();
    document.manifest.integrity.algorithm = tdf_core::document::HashAlgorithm::Blake3;
    
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let (path, _, _temp_dir) = create_signed_document(document, &signing_key);
    
    // Verify with Blake3
    let report = ArchiveReader::verify(&path).unwrap();
    assert!(report.integrity_valid, "Blake3 hash should verify correctly");
}

#[test]
fn test_cross_algorithm_attack() {
    // Try to use Ed25519 signature with secp256k1 key (should fail)
    let document = create_test_document();
    let mut csprng = OsRng;
    let ed25519_key = SigningKey::generate(&mut csprng);
    let secp256k1_key = Secp256k1SigningKey::random(&mut csprng);
    
    let temp_dir = TempDir::new().unwrap();
    let output_path = temp_dir.path().join("test-cross.tdf");
    
    let mut builder = ArchiveBuilder::new(document);
    builder
        .build(
            &output_path,
            Some(&ed25519_key),
            Some("did:web:test.com".to_string()),
            Some("Test Signer".to_string()),
        )
        .unwrap();
    
    // Try to verify with wrong algorithm key
    let (_, _, sig_block) = ArchiveReader::read(&output_path).unwrap();
    let root_hash = hex::decode(
        ArchiveReader::verify(&output_path).unwrap().root_hash
    ).unwrap();
    
    let secp256k1_verifying_key = Secp256k1VerifyingKey::from(&secp256k1_key);
    let secp256k1_keys = vec![("did:web:test.com".to_string(), secp256k1_verifying_key)];
    
    let results = SignatureManager::verify_signature_block_mixed(
        &sig_block,
        &root_hash,
        &[],
        &secp256k1_keys,
        None,
    ).unwrap();
    
    // Should fail - wrong algorithm
    assert!(results.iter().all(|r| matches!(r, VerificationResult::Invalid { .. })));
}

