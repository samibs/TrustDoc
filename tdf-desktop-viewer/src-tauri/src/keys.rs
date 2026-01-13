// Key Management Backend

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tdf_core::error::TdfResult;
use ed25519_dalek::{SigningKey, VerifyingKey, SECRET_KEY_LENGTH, PUBLIC_KEY_LENGTH};
use k256::ecdsa::SigningKey as Secp256k1SigningKey;
use rand_core::OsRng;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyInfo {
    pub id: String,
    pub name: String,
    pub algorithm: String,
    pub created: String,
    pub signer_id: Option<String>,
    pub fingerprint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct KeyMetadata {
    name: String,
    algorithm: String,
    created: String,
    signer_id: Option<String>,
}

pub fn get_keys_directory() -> TdfResult<PathBuf> {
    let mut keys_dir = dirs::home_dir()
        .ok_or_else(|| tdf_core::error::TdfError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Home directory not found"
        )))?;
    
    keys_dir.push(".tdf");
    keys_dir.push("keys");
    
    // Create directory if it doesn't exist
    if !keys_dir.exists() {
        fs::create_dir_all(&keys_dir)?;
    }
    
    Ok(keys_dir)
}

pub fn list_keys() -> TdfResult<Vec<KeyInfo>> {
    let keys_dir = get_keys_directory()?;
    let mut keys = Vec::new();

    if !keys_dir.exists() {
        return Ok(keys);
    }

    let entries = fs::read_dir(&keys_dir)?;
    
    let mut key_ids = std::collections::HashSet::new();
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            if file_name.ends_with(".meta.json") {
                if let Some(key_id) = file_name.strip_suffix(".meta.json") {
                    key_ids.insert(key_id.to_string());
                }
            }
        }
    }

    for key_id in key_ids {
        if let Ok(metadata) = load_key_metadata(&key_id) {
            keys.push(KeyInfo {
                id: key_id.clone(),
                name: metadata.name,
                algorithm: metadata.algorithm,
                created: metadata.created,
                signer_id: metadata.signer_id,
                fingerprint: Some(compute_fingerprint(&key_id)?),
            });
        }
    }

    // Sort by created date (newest first)
    keys.sort_by(|a, b| b.created.cmp(&a.created));

    Ok(keys)
}

pub fn generate_key(name: String, algorithm: String, signer_id: Option<String>) -> TdfResult<KeyInfo> {
    let keys_dir = get_keys_directory()?;
    
    // Generate unique key ID
    let key_id = format!("{}-{}", 
        name.to_lowercase().replace(" ", "-"),
        chrono::Utc::now().format("%Y%m%d%H%M%S")
    );

    let signing_key_path = keys_dir.join(format!("{}.signing", key_id));
    let verifying_key_path = keys_dir.join(format!("{}.verifying", key_id));
    let metadata_path = keys_dir.join(format!("{}.meta.json", key_id));

    match algorithm.as_str() {
        "ed25519" => {
            let signing_key = SigningKey::generate(&mut OsRng);
            let verifying_key = signing_key.verifying_key();

            // Write signing key (private)
            fs::write(&signing_key_path, signing_key.to_bytes())?;

            // Write verifying key (public)
            fs::write(&verifying_key_path, verifying_key.to_bytes())?;
        }
        "secp256k1" => {
            use k256::elliptic_curve::SecretKey;
            use k256::Secp256k1;
            let signing_key = SecretKey::<Secp256k1>::random(&mut OsRng);
            let verifying_key = signing_key.public_key();

            // Write signing key (private) - 32 bytes
            fs::write(&signing_key_path, signing_key.to_bytes().as_slice())?;

            // Write verifying key (public) - SEC1 format
            fs::write(&verifying_key_path, verifying_key.to_sec1_bytes())?;
        }
        _ => {
            return Err(tdf_core::error::TdfError::UnsupportedSignatureAlgorithm(algorithm));
        }
    }

    // Write metadata
    let metadata = KeyMetadata {
        name: name.clone(),
        algorithm: algorithm.clone(),
        created: chrono::Utc::now().to_rfc3339(),
        signer_id,
    };

    fs::write(&metadata_path, serde_json::to_string_pretty(&metadata)?)?;

    Ok(KeyInfo {
        id: key_id.clone(),
        name,
        algorithm,
        created: metadata.created,
        signer_id: metadata.signer_id,
        fingerprint: Some(compute_fingerprint(&key_id)?),
    })
}

pub fn import_key(path: &Path, name: String, _password: Option<String>) -> TdfResult<KeyInfo> {
    let keys_dir = get_keys_directory()?;
    
    // Determine if it's a signing or verifying key
    let is_signing = path.file_name()
        .and_then(|n| n.to_str())
        .map(|n| n.ends_with(".signing"))
        .unwrap_or(false);

    // Read key data
    let key_data = fs::read(path)?;

    // Determine algorithm based on key size
    let algorithm = if key_data.len() == SECRET_KEY_LENGTH || key_data.len() == PUBLIC_KEY_LENGTH {
        "ed25519"
    } else if key_data.len() == 32 || key_data.len() == 33 || key_data.len() == 65 {
        "secp256k1"
    } else {
        return Err(tdf_core::error::TdfError::SignatureFailure(
            format!("Unknown key format: {} bytes", key_data.len())
        ));
    };

    // Generate unique key ID
    let key_id = format!("{}-{}", 
        name.to_lowercase().replace(" ", "-"),
        chrono::Utc::now().format("%Y%m%d%H%M%S")
    );

    let signing_key_path = keys_dir.join(format!("{}.signing", key_id));
    let verifying_key_path = keys_dir.join(format!("{}.verifying", key_id));
    let metadata_path = keys_dir.join(format!("{}.meta.json", key_id));

    if is_signing {
        // Import signing key
        fs::write(&signing_key_path, &key_data)?;
        
        // Derive verifying key
        match algorithm {
            "ed25519" => {
                if key_data.len() != SECRET_KEY_LENGTH {
                    return Err(tdf_core::error::TdfError::SignatureFailure(
                        format!("Invalid Ed25519 key length: expected {}, got {}", SECRET_KEY_LENGTH, key_data.len())
                    ));
                }
                let key_bytes: [u8; SECRET_KEY_LENGTH] = key_data.as_slice()
                    .try_into()
                    .map_err(|_| tdf_core::error::TdfError::SignatureFailure("Invalid Ed25519 key length".to_string()))?;
                let signing_key = SigningKey::from_bytes(&key_bytes);
                let verifying_key = signing_key.verifying_key();
                fs::write(&verifying_key_path, verifying_key.to_bytes())?;
            }
            "secp256k1" => {
                if key_data.len() != 32 {
                    return Err(tdf_core::error::TdfError::SignatureFailure(
                        format!("Invalid secp256k1 key length: expected 32, got {}", key_data.len())
                    ));
                }
                use k256::elliptic_curve::SecretKey;
                use k256::Secp256k1;
                let key_bytes: &[u8; 32] = key_data.as_slice()
                    .try_into()
                    .map_err(|_| tdf_core::error::TdfError::SignatureFailure("Invalid secp256k1 key length".to_string()))?;
                let signing_key = SecretKey::<Secp256k1>::from_bytes(key_bytes.into())
                    .map_err(|e| tdf_core::error::TdfError::SignatureFailure(format!("Invalid secp256k1 key: {}", e)))?;
                let verifying_key = signing_key.public_key();
                fs::write(&verifying_key_path, verifying_key.to_sec1_bytes())?;
            }
            _ => unreachable!()
        }
    } else {
        // Import verifying key only
        fs::write(&verifying_key_path, &key_data)?;
    }

    // Write metadata
    let metadata = KeyMetadata {
        name: name.clone(),
        algorithm: algorithm.to_string(),
        created: chrono::Utc::now().to_rfc3339(),
        signer_id: None,
    };

    fs::write(&metadata_path, serde_json::to_string_pretty(&metadata)?)?;

    Ok(KeyInfo {
        id: key_id.clone(),
        name,
        algorithm: algorithm.to_string(),
        created: metadata.created,
        signer_id: None,
        fingerprint: Some(compute_fingerprint(&key_id)?),
    })
}

pub fn export_key(key_id: &str, path: &Path) -> TdfResult<()> {
    let keys_dir = get_keys_directory()?;
    let signing_key_path = keys_dir.join(format!("{}.signing", key_id));

    if !signing_key_path.exists() {
        return Err(tdf_core::error::TdfError::MissingFile(
            format!("Key {} not found", key_id)
        ));
    }

    let key_data = fs::read(&signing_key_path)?;
    fs::write(path, key_data)?;

    Ok(())
}

pub fn delete_key(key_id: &str) -> TdfResult<()> {
    let keys_dir = get_keys_directory()?;
    
    let signing_key_path = keys_dir.join(format!("{}.signing", key_id));
    let verifying_key_path = keys_dir.join(format!("{}.verifying", key_id));
    let metadata_path = keys_dir.join(format!("{}.meta.json", key_id));

    // Delete all key files
    if signing_key_path.exists() {
        fs::remove_file(&signing_key_path)?;
    }
    if verifying_key_path.exists() {
        fs::remove_file(&verifying_key_path)?;
    }
    if metadata_path.exists() {
        fs::remove_file(&metadata_path)?;
    }

    Ok(())
}

pub fn get_key_details(key_id: &str) -> TdfResult<KeyInfo> {
    let metadata = load_key_metadata(key_id)?;
    
    Ok(KeyInfo {
        id: key_id.to_string(),
        name: metadata.name,
        algorithm: metadata.algorithm,
        created: metadata.created,
        signer_id: metadata.signer_id,
        fingerprint: Some(compute_fingerprint(key_id)?),
    })
}

fn load_key_metadata(key_id: &str) -> TdfResult<KeyMetadata> {
    let keys_dir = get_keys_directory()?;
    let metadata_path = keys_dir.join(format!("{}.meta.json", key_id));

    let metadata_json = fs::read_to_string(&metadata_path)?;
    let metadata: KeyMetadata = serde_json::from_str(&metadata_json)?;

    Ok(metadata)
}

fn compute_fingerprint(key_id: &str) -> TdfResult<String> {
    use sha2::{Sha256, Digest};
    
    let keys_dir = get_keys_directory()?;
    let verifying_key_path = keys_dir.join(format!("{}.verifying", key_id));

    if !verifying_key_path.exists() {
        return Ok("N/A".to_string());
    }

    let key_data = fs::read(&verifying_key_path)?;
    let hash = Sha256::digest(&key_data);
    
    // Return first 16 bytes as hex
    let fingerprint: Vec<String> = hash[..16].iter().map(|b| format!("{:02x}", b)).collect();
    Ok(fingerprint.join(""))
}
