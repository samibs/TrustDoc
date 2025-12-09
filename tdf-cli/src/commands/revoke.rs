use tdf_core::revocation::{RevocationList, RevocationReason, RevocationManager};
use tdf_core::error::TdfResult;
use std::path::PathBuf;
use std::fs;

pub fn revoke_key(
    key_id: String,
    reason: String,
    authority: Option<String>,
    output: Option<PathBuf>,
) -> TdfResult<()> {
    let reason_enum = match reason.as_str() {
        "key-compromise" => RevocationReason::KeyCompromise,
        "ca-compromise" => RevocationReason::CaCompromise,
        "affiliation-changed" => RevocationReason::AffiliationChanged,
        "superseded" => RevocationReason::Superseded,
        "cessation-of-operation" => RevocationReason::CessationOfOperation,
        "certificate-hold" => RevocationReason::CertificateHold,
        "remove-from-crl" => RevocationReason::RemoveFromCrl,
        "privilege-withdrawn" => RevocationReason::PrivilegeWithdrawn,
        "aa-compromise" => RevocationReason::AaCompromise,
        _ => RevocationReason::Unspecified,
    };

    // Load existing revocation list or create new
    let mut revocation_list = if let Some(ref output_path) = output {
        if output_path.exists() {
            let data = fs::read(output_path)?;
            RevocationManager::from_cbor(&data)?
        } else {
            RevocationList::new()
        }
    } else {
        RevocationList::new()
    };

    // Revoke the key
    revocation_list.revoke(key_id.clone(), reason_enum, authority);

    // Save revocation list
    let output_path = output.unwrap_or_else(|| PathBuf::from("revocation.cbor"));
    let cbor_data = RevocationManager::to_cbor(&revocation_list)?;
    fs::write(&output_path, cbor_data)?;

    println!("✓ Key {} revoked successfully", key_id);
    println!("  Revocation list saved to: {}", output_path.display());

    Ok(())
}

pub fn check_revocation(
    document: PathBuf,
    revocation_list: Option<PathBuf>,
) -> TdfResult<()> {
    use tdf_core::archive::ArchiveReader;

    // Read document with revocation
    let (_, _, sig_block, doc_revocation_list) = ArchiveReader::read_with_revocation(&document)?;

    // Build revocation manager
    let mut revocation_manager = RevocationManager::new();
    if let Some(list) = doc_revocation_list {
        revocation_manager.add_list(list);
    }
    if let Some(ext_list_path) = revocation_list {
        let data = fs::read(ext_list_path)?;
        let ext_list = RevocationManager::from_cbor(&data)?;
        revocation_manager.add_list(ext_list);
    }

    // Check each signature
    println!("Checking revocation status for {} signatures...", sig_block.signatures.len());
    let mut revoked_count = 0;
    let mut valid_count = 0;

    for sig in &sig_block.signatures {
        if let Some(entry) = revocation_manager.is_revoked_at(&sig.signer.id, sig.timestamp.time) {
            println!("✗ REVOKED: {} ({})", sig.signer.name, sig.signer.id);
            println!("  Revoked at: {}", entry.revoked_at);
            println!("  Reason: {:?}", entry.reason);
            revoked_count += 1;
        } else {
            println!("✓ Valid: {} ({})", sig.signer.name, sig.signer.id);
            valid_count += 1;
        }
    }

    println!("\nSummary: {} valid, {} revoked", valid_count, revoked_count);

    if revoked_count > 0 {
        eprintln!("Warning: Document contains revoked signatures!");
        std::process::exit(1);
    }

    Ok(())
}

