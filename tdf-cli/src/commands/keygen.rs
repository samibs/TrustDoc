use crate::utils;
use ed25519_dalek::{SigningKey, VerifyingKey};
use k256::ecdsa::{SigningKey as Secp256k1SigningKey, VerifyingKey as Secp256k1VerifyingKey};
use std::fs;
use std::path::{Path, PathBuf};
use tdf_core::error::TdfResult;

pub fn generate_keypair(output: Option<PathBuf>, name: String) -> TdfResult<()> {
    generate_keypair_ed25519(output, name)
}

pub fn generate_keypair_ed25519(output: Option<PathBuf>, name: String) -> TdfResult<()> {
    let output_dir = output.unwrap_or_else(|| PathBuf::from("."));
    
    // Ensure output directory exists
    fs::create_dir_all(&output_dir)?;

    // Generate keypair
    let (signing_key, verifying_key) = utils::generate_keypair();

    // Write signing key (private key)
    let signing_key_path = output_dir.join(format!("{}.signing", name));
    fs::write(&signing_key_path, signing_key.to_bytes())?;
    println!("Signing key (private) written to: {}", signing_key_path.display());
    println!("  ⚠️  Keep this file secure and never share it!");

    // Write verifying key (public key)
    let verifying_key_path = output_dir.join(format!("{}.verifying", name));
    fs::write(&verifying_key_path, verifying_key.to_bytes())?;
    println!("Verifying key (public) written to: {}", verifying_key_path.display());
    println!("  ✓  This file can be shared publicly");

    // Show key info
    println!("\nKey Information:");
    println!("  Signing key size: {} bytes", signing_key.to_bytes().len());
    println!("  Verifying key size: {} bytes", verifying_key.to_bytes().len());
    println!("\nUsage:");
    println!("  Create document: tdf create input.json --key {}.signing --signer-id \"did:web:example.com\" --signer-name \"Your Name\"", name);
    println!("  Verify document: tdf verify document.tdf --key {}.verifying", name);

    Ok(())
}

pub fn generate_keypair_secp256k1(output: Option<PathBuf>, name: String) -> TdfResult<()> {
    let output_dir = output.unwrap_or_else(|| PathBuf::from("."));
    
    fs::create_dir_all(&output_dir)?;

    // Generate secp256k1 keypair
    use rand::rngs::OsRng;
    let mut csprng = OsRng;
    let signing_key = Secp256k1SigningKey::random(&mut csprng);
    let verifying_key = Secp256k1VerifyingKey::from(&signing_key);

    // Write signing key (private key) - raw bytes
    let signing_key_path = output_dir.join(format!("{}.secp256k1.signing", name));
    let signing_key_bytes = signing_key.to_bytes();
    fs::write(&signing_key_path, signing_key_bytes.as_slice())?;
    println!("Signing key (private) written to: {}", signing_key_path.display());
    println!("  ⚠️  Keep this file secure and never share it!");

    // Write verifying key (public key) - SEC1 format
    let verifying_key_path = output_dir.join(format!("{}.secp256k1.verifying", name));
    let pubkey_bytes = verifying_key.to_sec1_bytes();
    fs::write(&verifying_key_path, &*pubkey_bytes)?;
    println!("Verifying key (public) written to: {}", verifying_key_path.display());
    println!("  ✓  This file can be shared publicly");

    println!("\nKey Information:");
    println!("  Algorithm: secp256k1 (Web3 compatible)");
    println!("  Signing key size: {} bytes", signing_key_bytes.as_slice().len());
    println!("  Verifying key size: {} bytes", pubkey_bytes.len());

    Ok(())
}

