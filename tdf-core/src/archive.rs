use crate::document::Document;
use crate::error::{TdfError, TdfResult};
use crate::merkle::{HashAlgorithm, MerkleTree};
use crate::signature::{SignatureBlock, SignatureManager, SignatureScope};
use crate::timestamp::{TimestampProvider, verify_timestamp_token_with_config, TimestampValidationConfig};
use crate::revocation::{RevocationList, RevocationManager};
use crate::config::SecurityConfig;
use ed25519_dalek::SigningKey;
use k256::ecdsa::SigningKey as Secp256k1SigningKey;
use serde_cbor;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use zip::write::{FileOptions, ZipWriter};
use zip::CompressionMethod;

const MANIFEST_FILE: &str = "manifest.cbor";
const CONTENT_FILE: &str = "content.cbor";
const STYLES_FILE: &str = "styles.css";
const LAYOUT_FILE: &str = "layout.cbor";
const DATA_FILE: &str = "data.json";
const HASHES_FILE: &str = "hashes.bin";
const SIGNATURES_FILE: &str = "signatures.cbor";
const REVOCATION_FILE: &str = "revocation.cbor";
const ASSETS_IMAGES_DIR: &str = "assets/images/";
const ASSETS_FONTS_DIR: &str = "assets/fonts/";

pub struct ArchiveBuilder {
    document: Document,
    assets: HashMap<String, Vec<u8>>,
    revocation_list: Option<RevocationList>,
    security_config: SecurityConfig,
}

impl ArchiveBuilder {
    pub fn new(document: Document) -> Self {
        ArchiveBuilder {
            document,
            assets: HashMap::new(),
            revocation_list: None,
            security_config: SecurityConfig::default(),
        }
    }

    pub fn with_security_config(mut self, config: SecurityConfig) -> Self {
        self.security_config = config;
        self
    }

    pub fn with_revocation_list(mut self, list: RevocationList) -> Self {
        self.revocation_list = Some(list);
        self
    }

    pub fn add_asset(&mut self, path: String, data: Vec<u8>) {
        // Check file size limit
        if let Err(e) = self.security_config.check_file_size(data.len() as u64) {
            // Log warning but don't fail - caller can handle
            eprintln!("Warning: Asset {} exceeds size limit: {}", path, e);
        }
        self.assets.insert(path, data);
    }

    pub fn build(
        &mut self,
        output_path: &Path,
        signing_key: Option<&SigningKey>,
        signer_id: Option<String>,
        signer_name: Option<String>,
    ) -> TdfResult<()> {
        self.build_with_timestamp(output_path, signing_key, None, signer_id, signer_name, None, None)
    }

    pub fn build_with_timestamp(
        &mut self,
        output_path: &Path,
        ed25519_key: Option<&SigningKey>,
        secp256k1_key: Option<&Secp256k1SigningKey>,
        signer_id: Option<String>,
        signer_name: Option<String>,
        signature_algorithm: Option<crate::signature::SignatureAlgorithm>,
        timestamp_provider: Option<&dyn TimestampProvider>,
    ) -> TdfResult<()> {

        // Validate document
        self.document.validate()?;

        // Serialize components
        let manifest_bytes = serde_cbor::to_vec(&self.document.manifest)?;
        let content_bytes = serde_cbor::to_vec(&self.document.content)?;
        let styles_bytes = self.document.styles.as_bytes().to_vec();

        let layout_bytes = if let Some(ref layout) = self.document.layout {
            Some(serde_cbor::to_vec(layout)?)
        } else {
            None
        };

        let data_bytes = if let Some(ref data) = self.document.data {
            Some(serde_json::to_vec(data)?)
        } else {
            None
        };

        // Build component map for Merkle tree (before updating manifest with root hash)
        let mut components = HashMap::new();
        // Use original manifest bytes (without root hash)
        components.insert("manifest".to_string(), manifest_bytes.clone());
        components.insert("content".to_string(), content_bytes.clone());
        components.insert("styles".to_string(), styles_bytes.clone());

        if let Some(ref layout) = layout_bytes {
            components.insert("layout".to_string(), layout.clone());
        }

        if let Some(ref data) = data_bytes {
            components.insert("data".to_string(), data.clone());
        }

        // Add asset hashes
        for (path, data) in &self.assets {
            components.insert(format!("asset:{}", path), data.clone());
        }

        // Compute Merkle tree
        let algorithm = match self.document.manifest.integrity.algorithm {
            crate::document::HashAlgorithm::Sha256 => HashAlgorithm::Sha256,
            crate::document::HashAlgorithm::Blake3 => HashAlgorithm::Blake3,
        };

        let mut merkle_tree = MerkleTree::new(algorithm);
        let root_hash = merkle_tree.compute_root(&components)?;

        // Update manifest with root hash (this will be written to archive)
        self.document.manifest.integrity.root_hash = merkle_tree.root_hash_hex();

        // Create signatures
        let mut signatures = Vec::new();
        let algo = signature_algorithm.unwrap_or_else(|| {
            if secp256k1_key.is_some() {
                crate::signature::SignatureAlgorithm::Secp256k1
            } else {
                crate::signature::SignatureAlgorithm::Ed25519
            }
        });

        if let (Some(id), Some(name)) = (signer_id, signer_name) {
            let signature = match algo {
                crate::signature::SignatureAlgorithm::Ed25519 => {
                    if let Some(key) = ed25519_key {
                        Some(SignatureManager::sign_ed25519_with_timestamp(
                            key,
                            &root_hash,
                            id.clone(),
                            name.clone(),
                            SignatureScope::Full,
                            timestamp_provider,
                        ))
                    } else {
                        None
                    }
                }
                crate::signature::SignatureAlgorithm::Secp256k1 => {
                    if let Some(key) = secp256k1_key {
                        // secp256k1 signing with timestamp (similar pattern)
                        Some(SignatureManager::sign_secp256k1(
                            key,
                            &root_hash,
                            id.clone(),
                            name.clone(),
                            SignatureScope::Full,
                        ))
                    } else {
                        None
                    }
                }
                _ => None,
            };
            if let Some(sig) = signature {
                signatures.push(sig);
            }
        }

        let signature_block = SignatureBlock { signatures };
        let signatures_bytes = serde_cbor::to_vec(&signature_block)?;
        let hashes_binary = merkle_tree.to_binary()?;

        // Estimate total size and check limits
        let estimated_size = manifest_bytes.len()
            + content_bytes.len()
            + styles_bytes.len()
            + layout_bytes.as_ref().map(|b| b.len()).unwrap_or(0)
            + data_bytes.as_ref().map(|b| b.len()).unwrap_or(0)
            + self.assets.values().map(|v| v.len()).sum::<usize>()
            + hashes_binary.len()
            + signatures_bytes.len()
            + self.revocation_list.as_ref().map(|r| RevocationManager::to_cbor(r).unwrap().len()).unwrap_or(0);
        
        // Check size limits
        self.security_config.check_size(estimated_size as u64)?;

        // Write ZIP archive
        let file = File::create(output_path)?;
        let mut zip = ZipWriter::new(file);
        let options = FileOptions::default()
            .compression_method(CompressionMethod::Deflated)
            .unix_permissions(0o644);

        // Write manifest (update with root hash)
        let updated_manifest_bytes = serde_cbor::to_vec(&self.document.manifest)?;
        zip.start_file(MANIFEST_FILE, options)?;
        zip.write_all(&updated_manifest_bytes)?;

        // Write content
        zip.start_file(CONTENT_FILE, options)?;
        zip.write_all(&content_bytes)?;

        // Write styles
        zip.start_file(STYLES_FILE, options)?;
        zip.write_all(&styles_bytes)?;

        // Write layout (if present)
        if let Some(ref layout) = layout_bytes {
            zip.start_file(LAYOUT_FILE, options)?;
            zip.write_all(layout)?;
        }

        // Write data (if present)
        if let Some(ref data) = data_bytes {
            zip.start_file(DATA_FILE, options)?;
            zip.write_all(data)?;
        }

        // Write hashes
        zip.start_file(HASHES_FILE, options)?;
        zip.write_all(&hashes_binary)?;

        // Write signatures
        zip.start_file(SIGNATURES_FILE, options)?;
        zip.write_all(&signatures_bytes)?;

        // Write revocation list (if present)
        if let Some(ref revocation_list) = self.revocation_list {
            let revocation_bytes = RevocationManager::to_cbor(revocation_list)?;
            zip.start_file(REVOCATION_FILE, options)?;
            zip.write_all(&revocation_bytes)?;
        }

        // Write assets
        for (path, data) in &self.assets {
            let full_path = if path.starts_with("assets/images/") {
                path.clone()
            } else if path.starts_with("assets/fonts/") {
                path.clone()
            } else if path.ends_with(".webp") || path.ends_with(".avif") || path.ends_with(".png") {
                format!("{}{}", ASSETS_IMAGES_DIR, path)
            } else if path.ends_with(".woff2") {
                format!("{}{}", ASSETS_FONTS_DIR, path)
            } else {
                format!("assets/{}", path)
            };

            zip.start_file(&full_path, options)?;
            zip.write_all(data)?;
        }

        zip.finish()?;

        Ok(())
    }
}

pub struct ArchiveReader;

impl ArchiveReader {
    pub fn read(path: &Path) -> TdfResult<(Document, MerkleTree, SignatureBlock)> {
        let file = File::open(path)?;
        let mut zip = zip::ZipArchive::new(file)?;

        // Read manifest
        let manifest_bytes = {
            let mut manifest_file = zip.by_name(MANIFEST_FILE)
                .map_err(|_| TdfError::MissingFile(MANIFEST_FILE.to_string()))?;
            let mut bytes = Vec::new();
            manifest_file.read_to_end(&mut bytes)?;
            bytes
        };
        let manifest: crate::document::Manifest = serde_cbor::from_slice(&manifest_bytes)?;

        // Read content
        let content_bytes = {
            let mut content_file = zip.by_name(CONTENT_FILE)
                .map_err(|_| TdfError::MissingFile(CONTENT_FILE.to_string()))?;
            let mut bytes = Vec::new();
            content_file.read_to_end(&mut bytes)?;
            bytes
        };
        let content: crate::content::DocumentContent = serde_cbor::from_slice(&content_bytes)?;

        // Read styles
        let styles = {
            let mut styles_file = zip.by_name(STYLES_FILE)
                .map_err(|_| TdfError::MissingFile(STYLES_FILE.to_string()))?;
            let mut s = String::new();
            styles_file.read_to_string(&mut s)?;
            s
        };

        // Read layout (optional)
        let layout = {
            if let Ok(mut layout_file) = zip.by_name(LAYOUT_FILE) {
                let mut layout_bytes = Vec::new();
                layout_file.read_to_end(&mut layout_bytes)?;
                Some(serde_cbor::from_slice(&layout_bytes)?)
            } else {
                None
            }
        };

        // Read data (optional)
        let data = {
            if let Ok(mut data_file) = zip.by_name(DATA_FILE) {
                let mut data_bytes = Vec::new();
                data_file.read_to_end(&mut data_bytes)?;
                Some(serde_json::from_slice(&data_bytes)?)
            } else {
                None
            }
        };

        // Read hashes
        let hashes_bytes = {
            let mut hashes_file = zip.by_name(HASHES_FILE)
                .map_err(|_| TdfError::MissingFile(HASHES_FILE.to_string()))?;
            let mut bytes = Vec::new();
            hashes_file.read_to_end(&mut bytes)?;
            bytes
        };
        let merkle_tree = MerkleTree::from_binary(&hashes_bytes)?;

        // Read signatures
        let signatures_bytes = {
            let mut signatures_file = zip.by_name(SIGNATURES_FILE)
                .map_err(|_| TdfError::MissingFile(SIGNATURES_FILE.to_string()))?;
            let mut bytes = Vec::new();
            signatures_file.read_to_end(&mut bytes)?;
            bytes
        };
        let signature_block: SignatureBlock = serde_cbor::from_slice(&signatures_bytes)?;

        let document = Document {
            manifest,
            content,
            styles,
            layout,
            data,
        };

        Ok((document, merkle_tree, signature_block))
    }

    /// Read document with revocation list
    pub fn read_with_revocation(path: &Path) -> TdfResult<(Document, MerkleTree, SignatureBlock, Option<RevocationList>)> {
        let file = File::open(path)?;
        let mut zip = zip::ZipArchive::new(file)?;

        // Read manifest
        let manifest_bytes = {
            let mut manifest_file = zip.by_name(MANIFEST_FILE)
                .map_err(|_| TdfError::MissingFile(MANIFEST_FILE.to_string()))?;
            let mut bytes = Vec::new();
            manifest_file.read_to_end(&mut bytes)?;
            bytes
        };
        let manifest: crate::document::Manifest = serde_cbor::from_slice(&manifest_bytes)?;

        // Read content
        let content_bytes = {
            let mut content_file = zip.by_name(CONTENT_FILE)
                .map_err(|_| TdfError::MissingFile(CONTENT_FILE.to_string()))?;
            let mut bytes = Vec::new();
            content_file.read_to_end(&mut bytes)?;
            bytes
        };
        let content: crate::content::DocumentContent = serde_cbor::from_slice(&content_bytes)?;

        // Read styles
        let styles = {
            let mut styles_file = zip.by_name(STYLES_FILE)
                .map_err(|_| TdfError::MissingFile(STYLES_FILE.to_string()))?;
            let mut s = String::new();
            styles_file.read_to_string(&mut s)?;
            s
        };

        // Read layout (optional)
        let layout = {
            if let Ok(mut layout_file) = zip.by_name(LAYOUT_FILE) {
                let mut layout_bytes = Vec::new();
                layout_file.read_to_end(&mut layout_bytes)?;
                Some(serde_cbor::from_slice(&layout_bytes)?)
            } else {
                None
            }
        };

        // Read data (optional)
        let data = {
            if let Ok(mut data_file) = zip.by_name(DATA_FILE) {
                let mut data_bytes = Vec::new();
                data_file.read_to_end(&mut data_bytes)?;
                Some(serde_json::from_slice(&data_bytes)?)
            } else {
                None
            }
        };

        // Read hashes
        let hashes_bytes = {
            let mut hashes_file = zip.by_name(HASHES_FILE)
                .map_err(|_| TdfError::MissingFile(HASHES_FILE.to_string()))?;
            let mut bytes = Vec::new();
            hashes_file.read_to_end(&mut bytes)?;
            bytes
        };
        let merkle_tree = MerkleTree::from_binary(&hashes_bytes)?;

        // Read signatures
        let signatures_bytes = {
            let mut signatures_file = zip.by_name(SIGNATURES_FILE)
                .map_err(|_| TdfError::MissingFile(SIGNATURES_FILE.to_string()))?;
            let mut bytes = Vec::new();
            signatures_file.read_to_end(&mut bytes)?;
            bytes
        };
        let signature_block: SignatureBlock = serde_cbor::from_slice(&signatures_bytes)?;

        // Read revocation list (optional)
        let revocation_list = {
            if let Ok(mut revocation_file) = zip.by_name(REVOCATION_FILE) {
                let mut revocation_bytes = Vec::new();
                revocation_file.read_to_end(&mut revocation_bytes)?;
                Some(RevocationManager::from_cbor(&revocation_bytes)?)
            } else {
                None
            }
        };

        let document = Document {
            manifest,
            content,
            styles,
            layout,
            data,
        };

        Ok((document, merkle_tree, signature_block, revocation_list))
    }

    pub fn verify(path: &Path) -> TdfResult<VerificationReport> {
        Self::verify_with_config(path, SecurityConfig::default(), None)
    }

    pub fn verify_with_revocation(path: &Path, revocation_manager: Option<&RevocationManager>) -> TdfResult<VerificationReport> {
        Self::verify_with_config(path, SecurityConfig::default(), revocation_manager)
    }

    pub fn verify_with_config(
        path: &Path,
        security_config: SecurityConfig,
        revocation_manager: Option<&RevocationManager>,
    ) -> TdfResult<VerificationReport> {
        // Check file size before opening
        let metadata = std::fs::metadata(path)?;
        security_config.check_size(metadata.len())?;

        // Read raw bytes from archive to get exact data that was hashed
        let file = File::open(path)?;
        let mut zip = zip::ZipArchive::new(file)?;

        // Check decompression ratio protection (ZIP bomb protection)
        let compressed_size = metadata.len();
        let mut total_uncompressed = 0u64;
        for i in 0..zip.len() {
            let file = zip.by_index(i)?;
            total_uncompressed += file.size();
            // Check individual file size
            security_config.check_file_size(file.size())?;
        }
        security_config.check_decompression_ratio(compressed_size, total_uncompressed)?;
        
        let mut components = HashMap::new();

        // Read manifest bytes directly
        let manifest_bytes = {
            let mut manifest_file = zip.by_name(MANIFEST_FILE)
                .map_err(|_| TdfError::MissingFile(MANIFEST_FILE.to_string()))?;
            let mut bytes = Vec::new();
            manifest_file.read_to_end(&mut bytes)?;
            bytes
        };
        
        // Parse manifest to remove root_hash for hashing
        let mut manifest: crate::document::Manifest = serde_cbor::from_slice(&manifest_bytes)?;
        let _stored_root_hash = manifest.integrity.root_hash.clone();
        manifest.integrity.root_hash = String::new();
        let manifest_bytes_for_hash = serde_cbor::to_vec(&manifest)?;
        
        // Read content bytes
        let content_bytes = {
            let mut content_file = zip.by_name(CONTENT_FILE)
                .map_err(|_| TdfError::MissingFile(CONTENT_FILE.to_string()))?;
            let mut bytes = Vec::new();
            content_file.read_to_end(&mut bytes)?;
            bytes
        };
        
        // Read styles bytes
        let styles_bytes = {
            let mut styles_file = zip.by_name(STYLES_FILE)
                .map_err(|_| TdfError::MissingFile(STYLES_FILE.to_string()))?;
            let mut bytes = Vec::new();
            styles_file.read_to_end(&mut bytes)?;
            bytes
        };

        components.insert("manifest".to_string(), manifest_bytes_for_hash);
        components.insert("content".to_string(), content_bytes);
        components.insert("styles".to_string(), styles_bytes);

        // Read layout (if present)
        {
            if let Ok(mut layout_file) = zip.by_name(LAYOUT_FILE) {
                let mut layout_bytes = Vec::new();
                layout_file.read_to_end(&mut layout_bytes)?;
                components.insert("layout".to_string(), layout_bytes);
            }
        }

        // Read data (if present)
        {
            if let Ok(mut data_file) = zip.by_name(DATA_FILE) {
                let mut data_bytes = Vec::new();
                data_file.read_to_end(&mut data_bytes)?;
                components.insert("data".to_string(), data_bytes);
            }
        }

        // Read assets from archive
        {
            for i in 0..zip.len() {
                let mut file = zip.by_index(i)?;
                let name = file.name().to_string();
                if name.starts_with("assets/") {
                    let mut data = Vec::new();
                    file.read_to_end(&mut data)?;
                    components.insert(format!("asset:{}", name), data);
                }
            }
        }

        // Read Merkle tree
        let hashes_bytes = {
            let mut hashes_file = zip.by_name(HASHES_FILE)
                .map_err(|_| TdfError::MissingFile(HASHES_FILE.to_string()))?;
            let mut bytes = Vec::new();
            hashes_file.read_to_end(&mut bytes)?;
            bytes
        };
        let merkle_tree = MerkleTree::from_binary(&hashes_bytes)?;

        // Read signatures
        let signatures_bytes = {
            let mut signatures_file = zip.by_name(SIGNATURES_FILE)
                .map_err(|_| TdfError::MissingFile(SIGNATURES_FILE.to_string()))?;
            let mut bytes = Vec::new();
            signatures_file.read_to_end(&mut bytes)?;
            bytes
        };
        let signature_block: SignatureBlock = serde_cbor::from_slice(&signatures_bytes)?;

        // Read revocation list (optional) and combine with external
        let mut combined_revocation_manager = RevocationManager::new();
        let has_internal_revocation = if let Ok(mut revocation_file) = zip.by_name(REVOCATION_FILE) {
            let mut revocation_bytes = Vec::new();
            revocation_file.read_to_end(&mut revocation_bytes)?;
            let list = RevocationManager::from_cbor(&revocation_bytes)?;
            combined_revocation_manager.add_list(list);
            true
        } else {
            false
        };
        // Use external revocation manager if provided, otherwise use internal if present
        // Note: We can't easily combine them without cloning, so prioritize external
        let _final_revocation_manager = if let Some(ext_manager) = revocation_manager {
            Some(ext_manager)
        } else if has_internal_revocation {
            // We'd need to return this, but for now just note it's available
            None // TODO: Return revocation manager in VerificationReport
        } else {
            None
        };

        // Verify Merkle tree
        let integrity_valid = merkle_tree.verify(&components)?;
        let root_hash = merkle_tree.root_hash().to_vec();
        
        // Validate timestamps in signatures
        let timestamp_config = TimestampValidationConfig::default();
        let mut timestamp_warnings = Vec::new();
        for sig in &signature_block.signatures {
            // Convert TimestampInfo to TimestampToken for validation
            let token = crate::timestamp::TimestampToken {
                time: sig.timestamp.time,
                authority: sig.timestamp.authority.clone().unwrap_or_else(|| "manual".to_string()),
                proof: sig.timestamp.proof.clone().unwrap_or_default(),
                algorithm: if sig.timestamp.proof.is_some() {
                    "rfc3161".to_string()
                } else {
                    "manual".to_string()
                },
            };
            if let Err(e) = verify_timestamp_token_with_config(&token, &root_hash, timestamp_config.clone()) {
                timestamp_warnings.push(format!("Signature {}: {}", sig.signer.id, e));
            }
        }
        
        // Reconstruct document for report
        let document = Document {
            manifest,
            content: serde_cbor::from_slice(&components["content"])?,
            styles: String::from_utf8_lossy(&components["styles"]).to_string(),
            layout: None,
            data: None,
        };

        Ok(VerificationReport {
            integrity_valid,
            root_hash: hex::encode(&root_hash),
            signature_count: signature_block.signatures.len(),
            document,
            timestamp_warnings,
        })
    }
}

#[derive(Debug)]
pub struct VerificationReport {
    pub integrity_valid: bool,
    pub root_hash: String,
    pub signature_count: usize,
    pub document: Document,
    pub timestamp_warnings: Vec<String>,
}

