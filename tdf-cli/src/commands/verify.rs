//! Enhanced document verification with security hardening
//!
//! Security Features (Phase 1):
//! - Mandatory signature verification (CVE-TDF-001)
//! - Root hash binding validation (CVE-TDF-007)
//! - Strict mode by default (CVE-TDF-018)
//! - Enforced whitelist checking (CVE-TDF-012)
//! - Mandatory revocation checking (CVE-TDF-011)

use crate::utils;
use ed25519_dalek::VerifyingKey;
use std::path::PathBuf;
use tdf_core::archive::ArchiveReader;
use tdf_core::config::{SecurityConfig, SizeTier};
use tdf_core::error::{TdfError, TdfResult};
use tdf_core::revocation::RevocationManager;
use tdf_core::signature::SignatureManager;
use tdf_core::whitelist::SignerWhitelist;

/// Parse security tier string to SizeTier enum
fn parse_security_tier(tier: &str) -> TdfResult<SecurityConfig> {
    match tier.to_lowercase().as_str() {
        "micro" => Ok(SecurityConfig::for_tier(SizeTier::Micro)),
        "standard" => Ok(SecurityConfig::for_tier(SizeTier::Standard)),
        "extended" => Ok(SecurityConfig::for_tier(SizeTier::Extended)),
        "permissive" => Ok(SecurityConfig::permissive()),
        _ => Err(TdfError::InvalidDocument(format!(
            "Unknown security tier: {}. Use: micro, standard, extended, permissive",
            tier
        ))),
    }
}

/// Get tier description for display
fn tier_description(tier: &str) -> &'static str {
    match tier.to_lowercase().as_str() {
        "micro" => "256 KB max, 100:1 decompression ratio",
        "standard" => "5 MB max, 1000:1 decompression ratio",
        "extended" => "50 MB max, 10000:1 decompression ratio",
        "permissive" => "100 MB max, testing only",
        _ => "unknown",
    }
}

/// Verification configuration
#[derive(Debug)]
struct VerifyConfig {
    allow_unsigned: bool,
    strict: bool,  // Default: true (lenient = false)
    enforce_whitelist: bool,
    skip_revocation: bool,
}

pub fn verify_document(
    document: PathBuf,
    key: Option<PathBuf>,
    security_tier: String,
    revocation_list: Option<PathBuf>,
    trusted_signers: Option<PathBuf>,
    allow_unsigned: bool,
    lenient: bool,
    enforce_whitelist: bool,
    skip_revocation: bool,
) -> TdfResult<()> {
    let config = VerifyConfig {
        allow_unsigned,
        strict: !lenient,  // Default is strict (lenient = false)
        enforce_whitelist,
        skip_revocation,
    };

    let mut warnings: Vec<String> = Vec::new();
    let mut errors: Vec<String> = Vec::new();

    // Parse security configuration
    let security_config = parse_security_tier(&security_tier)?;

    // Load external revocation list if provided
    let mut revocation_manager = RevocationManager::new();
    if let Some(revocation_path) = &revocation_list {
        let revocation_data = std::fs::read(revocation_path)?;
        let list = RevocationManager::from_cbor(&revocation_data)?;
        revocation_manager.add_list(list);
        println!("Loaded external revocation list: {}", revocation_path.display());
    }

    // Load trusted signers whitelist if provided
    let whitelist = if let Some(whitelist_path) = &trusted_signers {
        Some(SignerWhitelist::from_json_file(whitelist_path)?)
    } else {
        None
    };

    // Print header
    println!();
    println!("TDF Verification Report");
    println!("=======================");
    println!("Document: {}", document.display());
    println!("Security Tier: {} ({})", security_tier.to_uppercase(), tier_description(&security_tier));
    println!("Mode: {}", if config.strict { "STRICT (default)" } else { "LENIENT" });
    println!();

    // Perform verification with security config
    let report = ArchiveReader::verify_with_config(&document, security_config, Some(&revocation_manager))?;

    // Print integrity status
    println!("INTEGRITY: {}", if report.integrity_valid { "✓ VALID" } else { "✗ INVALID" });
    println!("  Root Hash: {}", &report.root_hash[..32.min(report.root_hash.len())]);
    println!("  Algorithm: SHA-256");
    println!();

    if !report.integrity_valid {
        eprintln!("✗ INTEGRITY FAILURE");
        eprintln!("The document has been modified or corrupted.");
        return Err(TdfError::IntegrityFailure(
            "Document integrity check failed - content has been modified".to_string()
        ));
    }

    // Print timestamp warnings from report
    if !report.timestamp_warnings.is_empty() {
        for warning in &report.timestamp_warnings {
            warnings.push(format!("Timestamp: {}", warning));
        }
    }

    // === SECURITY FIX: Check signature requirements (CVE-TDF-001) ===
    println!("SIGNATURES: {} found", report.signature_count);

    // Read document to check signatures
    let (doc, _, sig_block) = ArchiveReader::read(&document)?;
    let has_signatures = !sig_block.signatures.is_empty();

    // Mandatory signature check
    if has_signatures && key.is_none() {
        errors.push("Document has signatures but no verification key provided".to_string());
        println!();
        println!("  ✗ ERROR: Document is signed but --key not provided");
        println!("    Signatures MUST be verified. Use --key <path> to provide verifying key.");
        println!("    If you want to skip signature verification, use --allow-unsigned");

        if !config.allow_unsigned {
            return Err(TdfError::SignatureRequired(
                "Document has signatures but no verification key provided. \
                 Use --key to verify or --allow-unsigned to skip.".to_string()
            ));
        }
    }

    if !has_signatures && !config.allow_unsigned {
        errors.push("Document has no signatures".to_string());
        println!();
        println!("  ✗ ERROR: Document has no signatures");
        println!("    TDF documents should be signed for authenticity.");
        println!("    Use --allow-unsigned to verify integrity only.");

        return Err(TdfError::SignatureRequired(
            "Document has no signatures. Use --allow-unsigned to verify integrity only.".to_string()
        ));
    }

    // Get root hash for signature verification
    let root_hash_hex = &doc.manifest.integrity.root_hash;
    let root_hash = hex::decode(root_hash_hex)
        .map_err(|e| TdfError::InvalidDocument(format!("Invalid root hash hex: {}", e)))?;

    // Build verification key
    let verifying_key = if let Some(key_path) = &key {
        Some(utils::load_verifying_key(key_path)?)
    } else {
        None
    };

    // === VERIFY SIGNATURES ===
    if has_signatures && verifying_key.is_some() {
        let vk = verifying_key.as_ref().unwrap();

        // Build key list for signature verification
        let mut keys: Vec<(String, VerifyingKey)> = Vec::new();
        for sig in &sig_block.signatures {
            keys.push((sig.signer.id.clone(), vk.clone()));
        }

        // Verify signatures with revocation checking
        let revocation_ref = if config.skip_revocation {
            println!("  ⚠ Revocation checking DISABLED (--skip-revocation)");
            warnings.push("Revocation checking disabled".to_string());
            None
        } else {
            Some(&revocation_manager)
        };

        let results = SignatureManager::verify_signature_block_with_revocation(
            &sig_block,
            &root_hash,
            &keys,
            revocation_ref,
        )?;

        // Print signature details
        for (i, sig) in sig_block.signatures.iter().enumerate() {
            let result = results.get(i);

            // === SECURITY FIX: Validate signature root hash matches computed (CVE-TDF-007) ===
            if sig.root_hash != report.root_hash {
                errors.push(format!(
                    "Signature root hash mismatch for {}: expected {}, got {}",
                    sig.signer.id, &report.root_hash[..16], &sig.root_hash[..16.min(sig.root_hash.len())]
                ));
                println!();
                println!("  ✗ {} ({})", sig.signer.name, sig.signer.id);
                println!("    ROOT HASH MISMATCH - Signature may be from different document!");
                continue;
            }

            // Determine signature status
            let (status_icon, status_text, is_error) = match result {
                Some(tdf_core::signature::VerificationResult::Valid { .. }) => ("✓", "VALID", false),
                Some(tdf_core::signature::VerificationResult::Invalid { reason, .. }) => {
                    errors.push(format!("Signature invalid: {}", reason));
                    ("✗", "INVALID", true)
                }
                Some(tdf_core::signature::VerificationResult::Revoked { revoked_at, reason, .. }) => {
                    let msg = format!("Key revoked at {}: {}", revoked_at, reason);
                    if config.strict {
                        errors.push(msg);
                    } else {
                        warnings.push(msg);
                    }
                    ("⚠", "REVOKED", config.strict)
                }
                Some(tdf_core::signature::VerificationResult::Unsupported { algorithm, .. }) => {
                    errors.push(format!("Unsupported algorithm: {}", algorithm));
                    ("✗", "UNSUPPORTED", true)
                }
                None => {
                    errors.push("Signature not verified".to_string());
                    ("✗", "NOT VERIFIED", true)
                }
            };

            println!();
            println!("  {} {} ({})", status_icon, sig.signer.name, sig.signer.id);
            println!("    Algorithm: {:?}", sig.algorithm);
            println!("    Timestamp: {}", sig.timestamp.time);
            println!("    Status: {}", status_text);
            println!("    Root Hash Binding: ✓ VALID");

            // === SECURITY FIX: Enforce whitelist (CVE-TDF-012) ===
            if let Some(wl) = &whitelist {
                if wl.is_trusted(&sig.signer.id) {
                    let signer_info = wl.get_signer(&sig.signer.id);
                    let roles = signer_info
                        .map(|s| s.roles.join(", "))
                        .unwrap_or_default();
                    if roles.is_empty() {
                        println!("    Trusted: ✓ (in whitelist)");
                    } else {
                        println!("    Trusted: ✓ (in whitelist, roles: {})", roles);
                    }
                } else {
                    println!("    Trusted: ✗ NOT IN WHITELIST");
                    let msg = format!("Signer not in whitelist: {}", sig.signer.id);

                    if config.enforce_whitelist {
                        errors.push(msg);
                    } else {
                        warnings.push(msg);
                    }
                }
            }

            // Check revocation status explicitly
            if !config.skip_revocation {
                if let Some(entry) = revocation_manager.is_revoked(&sig.signer.id) {
                    println!("    Revoked: ✓ (at {}, reason: {:?})", entry.revoked_at, entry.reason);
                } else {
                    println!("    Revoked: ✗");
                }
            }

            if is_error {
                // Signature verification failed - this is always a hard error
                continue;
            }
        }
    } else if has_signatures && verifying_key.is_none() {
        println!("  Signatures present but verification skipped (--allow-unsigned)");
        warnings.push("Signatures not verified".to_string());
    } else {
        println!("  No signatures (--allow-unsigned mode)");
    }

    // Print warnings summary
    println!();
    if warnings.is_empty() {
        println!("WARNINGS: 0");
    } else {
        println!("WARNINGS: {}", warnings.len());
        for warning in &warnings {
            println!("  ⚠ {}", warning);
        }
    }

    // Print errors summary
    if !errors.is_empty() {
        println!();
        println!("ERRORS: {}", errors.len());
        for error in &errors {
            println!("  ✗ {}", error);
        }
    }

    // Final result
    println!();

    // Determine final status
    let has_signature_errors = errors.iter().any(|e|
        e.contains("invalid") ||
        e.contains("UNSUPPORTED") ||
        e.contains("mismatch") ||
        e.contains("not verified")
    );

    if !errors.is_empty() || (config.strict && !warnings.is_empty()) {
        println!("RESULT: ✗ VERIFICATION FAILED");

        if has_signature_errors {
            return Err(TdfError::SignatureFailure(
                format!("Signature verification failed: {}", errors.join("; "))
            ));
        }

        if config.enforce_whitelist && errors.iter().any(|e| e.contains("whitelist")) {
            return Err(TdfError::UntrustedSigner(
                "Document signed by untrusted signer (not in whitelist)".to_string()
            ));
        }

        if config.strict && !warnings.is_empty() {
            return Err(TdfError::VerificationFailed(
                format!("Strict mode: verification has {} warning(s)", warnings.len())
            ));
        }

        return Err(TdfError::VerificationFailed(
            format!("Verification failed with {} error(s)", errors.len())
        ));
    }

    if !warnings.is_empty() {
        println!("RESULT: ⚠ DOCUMENT VERIFIED WITH WARNINGS");
        println!("        (Use strict mode to treat warnings as errors)");
    } else {
        println!("RESULT: ✓ DOCUMENT VERIFIED");
    }

    Ok(())
}
