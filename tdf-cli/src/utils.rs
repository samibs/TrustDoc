use ed25519_dalek::{SigningKey, VerifyingKey};
use std::fs;
use std::path::Path;
use tdf_core::error::{TdfError, TdfResult};

pub fn load_signing_key(path: &Path) -> TdfResult<SigningKey> {
    let key_bytes = fs::read(path)?;
    if key_bytes.len() != 32 {
        return Err(TdfError::InvalidDocument(
            "Signing key must be 32 bytes".to_string(),
        ));
    }
    let mut key_array = [0u8; 32];
    key_array.copy_from_slice(&key_bytes);
    Ok(SigningKey::from_bytes(&key_array))
}

pub fn load_verifying_key(path: &Path) -> TdfResult<VerifyingKey> {
    let key_bytes = fs::read(path)?;
    if key_bytes.len() != 32 {
        return Err(TdfError::InvalidDocument(
            "Verifying key must be 32 bytes".to_string(),
        ));
    }
    let mut key_array = [0u8; 32];
    key_array.copy_from_slice(&key_bytes);
    VerifyingKey::from_bytes(&key_array)
        .map_err(|e| TdfError::InvalidDocument(format!("Invalid verifying key: {}", e)))
}

pub fn generate_keypair() -> (SigningKey, VerifyingKey) {
    use rand::rngs::OsRng;
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();
    (signing_key, verifying_key)
}

