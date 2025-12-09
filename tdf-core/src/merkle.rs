use crate::error::{TdfError, TdfResult};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

const MERKLE_MAGIC: &[u8] = b"TDFH";
const MERKLE_VERSION: u8 = 0x01;
const ALGORITHM_SHA256: u8 = 0x01;
const ALGORITHM_BLAKE3: u8 = 0x02;

#[derive(Debug, Clone, PartialEq)]
pub enum HashAlgorithm {
    Sha256,
    Blake3,
}

impl HashAlgorithm {
    fn to_u8(&self) -> u8 {
        match self {
            HashAlgorithm::Sha256 => ALGORITHM_SHA256,
            HashAlgorithm::Blake3 => ALGORITHM_BLAKE3,
        }
    }

    fn from_u8(byte: u8) -> TdfResult<Self> {
        match byte {
            ALGORITHM_SHA256 => Ok(HashAlgorithm::Sha256),
            ALGORITHM_BLAKE3 => Ok(HashAlgorithm::Blake3),
            _ => Err(TdfError::UnsupportedHashAlgorithm(format!("0x{:02x}", byte))),
        }
    }
}

pub struct MerkleTree {
    algorithm: HashAlgorithm,
    root_hash: Vec<u8>,
    leaf_hashes: Vec<Vec<u8>>,
}

impl MerkleTree {
    pub fn new(algorithm: HashAlgorithm) -> Self {
        MerkleTree {
            algorithm,
            root_hash: Vec::new(),
            leaf_hashes: Vec::new(),
        }
    }

    pub fn compute_root(&mut self, components: &HashMap<String, Vec<u8>>) -> TdfResult<Vec<u8>> {
        let mut hashes: Vec<Vec<u8>> = Vec::new();

        // Hash each component
        for (_name, data) in components {
            let hash = match self.algorithm {
                HashAlgorithm::Sha256 => {
                    let mut hasher = Sha256::new();
                    hasher.update(data);
                    hasher.finalize().to_vec()
                }
                HashAlgorithm::Blake3 => {
                    blake3::hash(data).as_bytes().to_vec()
                }
            };
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
                let combined = if chunk.len() == 2 {
                    let mut combined = chunk[0].clone();
                    combined.extend_from_slice(&chunk[1]);
                    combined
                } else {
                    chunk[0].clone()
                };

                let hash = match self.algorithm {
                    HashAlgorithm::Sha256 => {
                        let mut hasher = Sha256::new();
                        hasher.update(&combined);
                        hasher.finalize().to_vec()
                    }
                    HashAlgorithm::Blake3 => {
                        blake3::hash(&combined).as_bytes().to_vec()
                    }
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

    pub fn verify(&self, components: &HashMap<String, Vec<u8>>) -> TdfResult<bool> {
        let mut tree = MerkleTree::new(self.algorithm.clone());
        let computed_root = tree.compute_root(components)?;
        Ok(computed_root == self.root_hash)
    }

    pub fn to_binary(&self) -> TdfResult<Vec<u8>> {
        let mut buf = Vec::new();

        // Magic header
        buf.extend_from_slice(MERKLE_MAGIC);
        // Version
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
        if version != MERKLE_VERSION {
            return Err(TdfError::InvalidDocument(format!(
                "Unsupported Merkle tree version: {}",
                version
            )));
        }

        let algorithm = HashAlgorithm::from_u8(data[5])?;
        let count = u32::from_be_bytes([data[6], data[7], data[8], data[9]]) as usize;

        if data.len() < 10 + 32 + (count * 32) {
            return Err(TdfError::InvalidDocument(
                "Merkle tree binary incomplete for declared count".to_string(),
            ));
        }

        let root_hash = data[10..42].to_vec();
        let mut leaf_hashes = Vec::new();

        let mut offset = 42;
        for _ in 0..count {
            if offset + 32 > data.len() {
                return Err(TdfError::InvalidDocument(
                    "Merkle tree binary truncated".to_string(),
                ));
            }
            leaf_hashes.push(data[offset..offset + 32].to_vec());
            offset += 32;
        }

        Ok(MerkleTree {
            algorithm,
            root_hash,
            leaf_hashes,
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
}

