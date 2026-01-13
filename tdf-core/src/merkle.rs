//! Merkle Tree Implementation with Security Hardening
//!
//! Security Fixes:
//! - CVE-TDF-002: Added domain separators to prevent collision attacks
//! - CVE-TDF-008: Fixed integer overflow in from_binary deserialization

use crate::error::{TdfError, TdfResult};
use sha2::{Digest, Sha256};
use sha3::{Sha3_256, Sha3_512, Digest as Sha3Digest};
use hmac::{Hmac, Mac};
use std::collections::HashMap;

type HmacSha256 = Hmac<Sha256>;

const MERKLE_MAGIC: &[u8] = b"TDFH";
const MERKLE_VERSION: u8 = 0x02;  // Version bump for domain separator change
const MERKLE_VERSION_LEGACY: u8 = 0x01;  // Support reading legacy format
const ALGORITHM_SHA256: u8 = 0x01;
const ALGORITHM_BLAKE3: u8 = 0x02;

// === SECURITY FIX (CVE-TDF-002): Domain separators ===
// These prevent second-preimage attacks where internal nodes could be
// confused with leaf nodes.
const LEAF_DOMAIN_SEPARATOR: u8 = 0x00;
const INTERNAL_DOMAIN_SEPARATOR: u8 = 0x01;

// Maximum number of leaf hashes to prevent DoS
const MAX_LEAF_COUNT: usize = 1_000_000;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HashAlgorithm {
    Sha256,      // Legacy support
    Sha3_256,    // NIST standard - quantum resistant
    Sha3_512,    // Higher security level
    Blake3,      // High-performance alternative
}

const ALGORITHM_SHA3_256: u8 = 0x03;
const ALGORITHM_SHA3_512: u8 = 0x04;

impl HashAlgorithm {
    fn to_u8(&self) -> u8 {
        match self {
            HashAlgorithm::Sha256 => ALGORITHM_SHA256,
            HashAlgorithm::Sha3_256 => ALGORITHM_SHA3_256,
            HashAlgorithm::Sha3_512 => ALGORITHM_SHA3_512,
            HashAlgorithm::Blake3 => ALGORITHM_BLAKE3,
        }
    }

    fn from_u8(byte: u8) -> TdfResult<Self> {
        match byte {
            ALGORITHM_SHA256 => Ok(HashAlgorithm::Sha256),
            ALGORITHM_SHA3_256 => Ok(HashAlgorithm::Sha3_256),
            ALGORITHM_SHA3_512 => Ok(HashAlgorithm::Sha3_512),
            ALGORITHM_BLAKE3 => Ok(HashAlgorithm::Blake3),
            _ => Err(TdfError::UnsupportedHashAlgorithm(format!("0x{:02x}", byte))),
        }
    }
}

pub struct MerkleTree {
    algorithm: HashAlgorithm,
    root_hash: Vec<u8>,
    leaf_hashes: Vec<Vec<u8>>,
    use_domain_separators: bool,  // True for v2+, false for legacy
}

impl MerkleTree {
    pub fn new(algorithm: HashAlgorithm) -> Self {
        MerkleTree {
            algorithm,
            root_hash: Vec::new(),
            leaf_hashes: Vec::new(),
            use_domain_separators: true,  // Always use domain separators for new trees
        }
    }

    /// Hash a leaf node with cryptographic protection (Vuln #45, #49 fix)
    /// Uses HMAC-SHA256/SHA3 to prevent length extension attacks
    fn hash_leaf(&self, data: &[u8]) -> Vec<u8> {
        match self.algorithm {
            HashAlgorithm::Sha256 => {
                // Use HMAC-SHA256 for length extension protection
                let mut mac = HmacSha256::new_from_slice(b"TDF-MERKLE-LEAF-KEY-2026")
                    .expect("HMAC key is valid length");
                if self.use_domain_separators {
                    mac.update(&[LEAF_DOMAIN_SEPARATOR]);
                }
                mac.update(data);
                mac.finalize().into_bytes().to_vec()
            }
            HashAlgorithm::Sha3_256 => {
                // SHA-3 is resistant to length extension
                let mut hasher = Sha3_256::new();
                if self.use_domain_separators {
                    hasher.update(&[LEAF_DOMAIN_SEPARATOR]);
                }
                hasher.update(data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::Sha3_512 => {
                // SHA-3-512 for maximum security
                let mut hasher = Sha3_512::new();
                if self.use_domain_separators {
                    hasher.update(&[LEAF_DOMAIN_SEPARATOR]);
                }
                hasher.update(data);
                // Truncate to 256 bits for consistency
                let result = hasher.finalize();
                result[..32].to_vec()
            }
            HashAlgorithm::Blake3 => {
                // BLAKE3 is resistant to length extension by design
                let mut data_with_prefix = Vec::new();
                if self.use_domain_separators {
                    data_with_prefix.push(LEAF_DOMAIN_SEPARATOR);
                }
                data_with_prefix.extend_from_slice(data);
                blake3::hash(&data_with_prefix).as_bytes().to_vec()
            }
        }
    }

    /// Hash an internal node with cryptographic protection (Vuln #45, #49 fix)
    /// Uses HMAC-SHA256/SHA3 to prevent length extension attacks
    fn hash_internal(&self, left: &[u8], right: &[u8]) -> Vec<u8> {
        match self.algorithm {
            HashAlgorithm::Sha256 => {
                // Use HMAC-SHA256 for length extension protection
                let mut mac = HmacSha256::new_from_slice(b"TDF-MERKLE-INTERNAL-KEY-2026")
                    .expect("HMAC key is valid length");
                if self.use_domain_separators {
                    mac.update(&[INTERNAL_DOMAIN_SEPARATOR]);
                }
                mac.update(left);
                mac.update(right);
                mac.finalize().into_bytes().to_vec()
            }
            HashAlgorithm::Sha3_256 => {
                // SHA-3 is resistant to length extension
                let mut hasher = Sha3_256::new();
                if self.use_domain_separators {
                    hasher.update(&[INTERNAL_DOMAIN_SEPARATOR]);
                }
                hasher.update(left);
                hasher.update(right);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::Sha3_512 => {
                // SHA-3-512 for maximum security
                let mut hasher = Sha3_512::new();
                if self.use_domain_separators {
                    hasher.update(&[INTERNAL_DOMAIN_SEPARATOR]);
                }
                hasher.update(left);
                hasher.update(right);
                // Truncate to 256 bits for consistency
                let result = hasher.finalize();
                result[..32].to_vec()
            }
            HashAlgorithm::Blake3 => {
                // BLAKE3 is resistant to length extension by design
                let mut combined = Vec::new();
                if self.use_domain_separators {
                    combined.push(INTERNAL_DOMAIN_SEPARATOR);
                }
                combined.extend_from_slice(left);
                combined.extend_from_slice(right);
                blake3::hash(&combined).as_bytes().to_vec()
            }
        }
    }

    /// Hash a single node (for odd-length levels)
    fn hash_single(&self, data: &[u8]) -> Vec<u8> {
        match self.algorithm {
            HashAlgorithm::Sha256 => {
                // Use HMAC-SHA256 for length extension protection
                let mut mac = HmacSha256::new_from_slice(b"TDF-MERKLE-SINGLE-KEY-2026")
                    .expect("HMAC key is valid length");
                if self.use_domain_separators {
                    mac.update(&[INTERNAL_DOMAIN_SEPARATOR]);
                }
                mac.update(data);
                mac.finalize().into_bytes().to_vec()
            }
            HashAlgorithm::Sha3_256 => {
                // SHA-3 is resistant to length extension
                let mut hasher = Sha3_256::new();
                if self.use_domain_separators {
                    hasher.update(&[INTERNAL_DOMAIN_SEPARATOR]);
                }
                hasher.update(data);
                hasher.finalize().to_vec()
            }
            HashAlgorithm::Sha3_512 => {
                // SHA-3-512 for maximum security
                let mut hasher = Sha3_512::new();
                if self.use_domain_separators {
                    hasher.update(&[INTERNAL_DOMAIN_SEPARATOR]);
                }
                hasher.update(data);
                // Truncate to 256 bits for consistency
                let result = hasher.finalize();
                result[..32].to_vec()
            }
            HashAlgorithm::Blake3 => {
                let mut combined = Vec::new();
                if self.use_domain_separators {
                    combined.push(INTERNAL_DOMAIN_SEPARATOR);
                }
                combined.extend_from_slice(data);
                blake3::hash(&combined).as_bytes().to_vec()
            }
        }
    }

    pub fn compute_root(&mut self, components: &HashMap<String, Vec<u8>>) -> TdfResult<Vec<u8>> {
        let mut hashes: Vec<Vec<u8>> = Vec::new();

        // Hash each component as a leaf node with domain separator
        for (_name, data) in components {
            let hash = self.hash_leaf(data);
            hashes.push(hash);
        }

        // Sort hashes for deterministic ordering
        hashes.sort();

        // Build Merkle tree
        self.leaf_hashes = hashes.clone();
        self.root_hash = self.build_tree(hashes)?;

        Ok(self.root_hash.clone())
    }

    fn build_tree(&self, mut hashes: Vec<Vec<u8>>) -> TdfResult<Vec<u8>> {
        if hashes.is_empty() {
            return Err(TdfError::InvalidDocument(
                "Cannot build Merkle tree from empty hashes".to_string(),
            ));
        }

        while hashes.len() > 1 {
            let mut next_level = Vec::new();

            for chunk in hashes.chunks(2) {
                let hash = if chunk.len() == 2 {
                    // Hash two nodes together with internal domain separator
                    self.hash_internal(&chunk[0], &chunk[1])
                } else {
                    // Single node - promote with internal hash
                    self.hash_single(&chunk[0])
                };
                next_level.push(hash);
            }

            hashes = next_level;
        }

        Ok(hashes[0].clone())
    }

    pub fn root_hash(&self) -> &[u8] {
        &self.root_hash
    }

    pub fn root_hash_hex(&self) -> String {
        hex::encode(&self.root_hash)
    }

    /// Returns the format version of this Merkle tree
    ///
    /// Version 1: Legacy format without domain separators (vulnerable to CVE-TDF-002)
    /// Version 2: Current format with domain separators (secure)
    pub fn version(&self) -> u8 {
        if self.use_domain_separators {
            2
        } else {
            1
        }
    }

    /// Verify that computed root hash matches stored root hash
    ///
    /// Security Fix (CVE-TDF-024): Uses constant-time comparison to prevent
    /// timing side-channel attacks. An attacker cannot determine which byte
    /// of the hash differs by measuring verification time.
    pub fn verify(&self, components: &HashMap<String, Vec<u8>>) -> TdfResult<bool> {
        let mut tree = MerkleTree::new(self.algorithm.clone());
        tree.use_domain_separators = self.use_domain_separators;
        let computed_root = tree.compute_root(components)?;

        // Security Fix (CVE-TDF-024): Use constant-time comparison
        Ok(crate::crypto_utils::ct_eq(&computed_root, &self.root_hash))
    }

    pub fn to_binary(&self) -> TdfResult<Vec<u8>> {
        let mut buf = Vec::new();

        // Magic header
        buf.extend_from_slice(MERKLE_MAGIC);
        // Version (v2 = domain separators)
        buf.push(MERKLE_VERSION);
        // Algorithm
        buf.push(self.algorithm.to_u8());
        // Node count (leaf hashes)
        let count = self.leaf_hashes.len() as u32;
        buf.extend_from_slice(&count.to_be_bytes());
        // Root hash
        buf.extend_from_slice(&self.root_hash);
        // Leaf hashes
        for hash in &self.leaf_hashes {
            buf.extend_from_slice(hash);
        }

        Ok(buf)
    }

    pub fn from_binary(data: &[u8]) -> TdfResult<Self> {
        if data.len() < 4 {
            return Err(TdfError::InvalidDocument(
                "Merkle tree binary too short".to_string(),
            ));
        }

        // Check magic
        if &data[0..4] != MERKLE_MAGIC {
            return Err(TdfError::InvalidDocument(
                "Invalid Merkle tree magic header".to_string(),
            ));
        }

        if data.len() < 42 {
            return Err(TdfError::InvalidDocument(
                "Merkle tree binary incomplete".to_string(),
            ));
        }

        let version = data[4];
        let use_domain_separators = match version {
            MERKLE_VERSION_LEGACY => {
                eprintln!("WARNING: Loading legacy Merkle tree format (v1) without domain separators");
                false
            }
            MERKLE_VERSION => true,
            _ => {
                return Err(TdfError::InvalidDocument(format!(
                    "Unsupported Merkle tree version: {}",
                    version
                )));
            }
        };

        let algorithm = HashAlgorithm::from_u8(data[5])?;
        let count = u32::from_be_bytes([data[6], data[7], data[8], data[9]]) as usize;

        // === SECURITY FIX (CVE-TDF-008): Integer overflow protection ===
        // Use checked arithmetic to prevent integer overflow attacks
        if count > MAX_LEAF_COUNT {
            return Err(TdfError::SizeExceeded(format!(
                "Merkle tree leaf count {} exceeds maximum {}",
                count, MAX_LEAF_COUNT
            )));
        }

        // Calculate required size with overflow protection
        let hash_size: usize = 32;
        let header_size: usize = 10;
        let root_hash_size: usize = 32;

        let leaves_size = count.checked_mul(hash_size).ok_or_else(|| {
            TdfError::IntegerOverflow(format!(
                "Merkle tree size calculation overflow: {} * {}",
                count, hash_size
            ))
        })?;

        let required_size = header_size
            .checked_add(root_hash_size)
            .and_then(|s| s.checked_add(leaves_size))
            .ok_or_else(|| {
                TdfError::IntegerOverflow(format!(
                    "Merkle tree total size calculation overflow: {} + {} + {}",
                    header_size, root_hash_size, leaves_size
                ))
            })?;

        if data.len() < required_size {
            return Err(TdfError::InvalidDocument(format!(
                "Merkle tree binary incomplete: expected {} bytes, got {}",
                required_size,
                data.len()
            )));
        }

        let root_hash = data[10..42].to_vec();
        let mut leaf_hashes = Vec::with_capacity(count);

        let mut offset = 42;
        for _ in 0..count {
            // This check is now redundant due to required_size check above,
            // but kept for defense in depth
            if offset + hash_size > data.len() {
                return Err(TdfError::InvalidDocument(
                    "Merkle tree binary truncated".to_string(),
                ));
            }
            leaf_hashes.push(data[offset..offset + hash_size].to_vec());
            offset += hash_size;
        }

        Ok(MerkleTree {
            algorithm,
            root_hash,
            leaf_hashes,
            use_domain_separators,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_tree_computation() {
        let mut components = HashMap::new();
        components.insert("manifest".to_string(), b"test data 1".to_vec());
        components.insert("content".to_string(), b"test data 2".to_vec());

        let mut tree = MerkleTree::new(HashAlgorithm::Sha256);
        let root = tree.compute_root(&components).unwrap();
        assert_eq!(root.len(), 32);
    }

    #[test]
    fn test_merkle_tree_verification() {
        let mut components = HashMap::new();
        components.insert("manifest".to_string(), b"test data 1".to_vec());
        components.insert("content".to_string(), b"test data 2".to_vec());

        let mut tree = MerkleTree::new(HashAlgorithm::Sha256);
        let root = tree.compute_root(&components).unwrap();

        assert!(tree.verify(&components).unwrap());

        // Tamper with data
        components.insert("content".to_string(), b"tampered data".to_vec());
        assert!(!tree.verify(&components).unwrap());
    }

    #[test]
    fn test_merkle_tree_serialization() {
        let mut components = HashMap::new();
        components.insert("manifest".to_string(), b"test data".to_vec());

        let mut tree = MerkleTree::new(HashAlgorithm::Sha256);
        tree.compute_root(&components).unwrap();

        let binary = tree.to_binary().unwrap();
        let restored = MerkleTree::from_binary(&binary).unwrap();

        assert_eq!(tree.root_hash, restored.root_hash);
        assert_eq!(tree.leaf_hashes, restored.leaf_hashes);
    }

    #[test]
    fn test_domain_separators_prevent_collision() {
        // Test that leaf and internal node hashes are different even with same content
        let mut tree = MerkleTree::new(HashAlgorithm::Sha256);

        let data = b"test data";
        let leaf_hash = tree.hash_leaf(data);
        let internal_hash = tree.hash_internal(data, &[]);

        // These should be different due to domain separators
        assert_ne!(leaf_hash, internal_hash);
    }

    #[test]
    fn test_integer_overflow_protection() {
        // Create malicious binary with huge count
        let mut malicious = Vec::new();
        malicious.extend_from_slice(MERKLE_MAGIC);
        malicious.push(MERKLE_VERSION);
        malicious.push(ALGORITHM_SHA256);
        // Huge count that would cause overflow: 0xFFFFFFFF
        malicious.extend_from_slice(&[0xFF, 0xFF, 0xFF, 0xFF]);
        // Add minimal data
        malicious.extend_from_slice(&[0u8; 32]); // root hash

        let result = MerkleTree::from_binary(&malicious);
        assert!(result.is_err());

        // Should fail with size exceeded, not crash
        if let Err(e) = result {
            let msg = format!("{}", e);
            assert!(msg.contains("exceeds maximum") || msg.contains("overflow"));
        }
    }

    #[test]
    fn test_max_leaf_count() {
        // Test that we reject trees with too many leaves
        let mut data = Vec::new();
        data.extend_from_slice(MERKLE_MAGIC);
        data.push(MERKLE_VERSION);
        data.push(ALGORITHM_SHA256);
        // Count just over the limit
        let over_limit = (MAX_LEAF_COUNT + 1) as u32;
        data.extend_from_slice(&over_limit.to_be_bytes());
        data.extend_from_slice(&[0u8; 32]); // root hash

        let result = MerkleTree::from_binary(&data);
        assert!(result.is_err());
    }
}
