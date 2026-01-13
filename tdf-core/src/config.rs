//! Security configuration for TDF format
//! Defines size limits, resource quotas, and security settings
//!
//! Security Fixes:
//! - CVE-TDF-010: Algorithm policy enforcement
//! - CVE-TDF-015: Decompression ratio zero divisor
//! - CVE-TDF-020: File count limits

use crate::error::{TdfError, TdfResult};
use crate::signature::SignatureAlgorithm;
use crate::merkle::HashAlgorithm;
use std::collections::HashSet;

/// Size tier definitions (from SPEC.md)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeTier {
    /// Micro: 256 KB - Invoices, receipts, simple contracts
    Micro,
    /// Standard: 5 MB - Reports, proposals, statements
    Standard,
    /// Extended: 50 MB - Annual reports, technical manuals
    Extended,
}

impl SizeTier {
    /// Maximum size in bytes for this tier
    pub fn max_size_bytes(&self) -> u64 {
        match self {
            SizeTier::Micro => 256 * 1024,      // 256 KB
            SizeTier::Standard => 5 * 1024 * 1024,  // 5 MB
            SizeTier::Extended => 50 * 1024 * 1024, // 50 MB
        }
    }

    /// Maximum decompression ratio (compressed to uncompressed)
    /// Prevents ZIP bombs
    pub fn max_decompression_ratio(&self) -> u64 {
        match self {
            SizeTier::Micro => 100,    // 100x for micro
            SizeTier::Standard => 1000,  // 1000x for standard
            SizeTier::Extended => 10000, // 10000x for extended
        }
    }

    /// Maximum individual file size in bytes
    pub fn max_file_size(&self) -> u64 {
        match self {
            SizeTier::Micro => 64 * 1024,        // 64 KB per file
            SizeTier::Standard => 1 * 1024 * 1024,  // 1 MB per file
            SizeTier::Extended => 10 * 1024 * 1024, // 10 MB per file
        }
    }
}

impl Default for SizeTier {
    fn default() -> Self {
        SizeTier::Standard
    }
}

/// Security configuration for TDF operations
#[derive(Debug, Clone)]
pub struct SecurityConfig {
    /// Maximum total archive size
    pub max_archive_size: u64,
    /// Maximum decompression ratio
    pub max_decompression_ratio: u64,
    /// Maximum individual file size
    pub max_file_size: u64,
    /// Maximum number of files in archive
    pub max_file_count: usize,
    /// Enable strict size checking
    pub strict_size_check: bool,
    // === SECURITY HARDENING (Attack Phase 1) ===
    /// Reject legacy Merkle tree format (v1 without domain separators)
    pub reject_legacy_merkle: bool,
    /// Reject legacy signature format (v1 without timestamp binding)
    pub reject_legacy_signatures: bool,
    /// Require RFC 3161 timestamps (reject manual timestamps)
    pub require_rfc3161_timestamps: bool,
}

impl SecurityConfig {
    /// Create config for a specific size tier
    pub fn for_tier(tier: SizeTier) -> Self {
        SecurityConfig {
            max_archive_size: tier.max_size_bytes(),
            max_decompression_ratio: tier.max_decompression_ratio(),
            max_file_size: tier.max_file_size(),
            max_file_count: 1000, // Reasonable limit
            strict_size_check: true,
            // Security hardening: reject legacy formats by default
            reject_legacy_merkle: true,
            reject_legacy_signatures: true,
            require_rfc3161_timestamps: false, // Not required by default (would break offline use)
        }
    }

    /// Create strict config (maximum security, recommended for production)
    pub fn strict(tier: SizeTier) -> Self {
        SecurityConfig {
            max_archive_size: tier.max_size_bytes(),
            max_decompression_ratio: tier.max_decompression_ratio(),
            max_file_size: tier.max_file_size(),
            max_file_count: 1000,
            strict_size_check: true,
            reject_legacy_merkle: true,
            reject_legacy_signatures: true,
            require_rfc3161_timestamps: true, // Strict mode requires TSA timestamps
        }
    }

    /// Create default config (Standard tier)
    pub fn default() -> Self {
        Self::for_tier(SizeTier::Standard)
    }

    /// Create permissive config (for testing/migration ONLY - NOT for production)
    ///
    /// WARNING: This config allows legacy formats which have known vulnerabilities:
    /// - Legacy Merkle trees (v1) lack domain separators (collision attacks possible)
    /// - Legacy signatures (v1) don't bind timestamps (timestamp manipulation possible)
    pub fn permissive() -> Self {
        SecurityConfig {
            max_archive_size: 100 * 1024 * 1024, // 100 MB
            max_decompression_ratio: 100000,      // Very high
            max_file_size: 50 * 1024 * 1024,     // 50 MB
            max_file_count: 10000,
            strict_size_check: false,
            // WARNING: These settings are insecure - for testing/migration only
            reject_legacy_merkle: false,
            reject_legacy_signatures: false,
            require_rfc3161_timestamps: false,
        }
    }

    /// Check if a size is within limits
    pub fn check_size(&self, size: u64) -> TdfResult<()> {
        if size > self.max_archive_size {
            return Err(TdfError::FileSizeExceeded(format!(
                "Archive size {} exceeds limit {}",
                size, self.max_archive_size
            )));
        }
        Ok(())
    }

    /// Check if decompression ratio is acceptable
    ///
    /// Security Fix (CVE-TDF-015): Handle zero compressed size properly
    pub fn check_decompression_ratio(&self, compressed: u64, uncompressed: u64) -> TdfResult<()> {
        // Empty files are always OK
        if uncompressed == 0 {
            return Ok(());
        }

        // If compressed size is 0 but uncompressed is not, this is a stored
        // (uncompressed) file. We need to check the absolute size instead.
        if compressed == 0 {
            // For stored files, enforce the file size limit directly
            return self.check_file_size(uncompressed);
        }

        // Use checked division to avoid potential issues
        let ratio = uncompressed.checked_div(compressed).unwrap_or(u64::MAX);

        if ratio > self.max_decompression_ratio {
            return Err(TdfError::FileSizeExceeded(format!(
                "Decompression ratio {}:1 exceeds limit {}:1 (possible ZIP bomb)",
                ratio, self.max_decompression_ratio
            )));
        }
        Ok(())
    }

    /// Check file count limit
    ///
    /// Security Fix (CVE-TDF-020): Enforce max file count
    pub fn check_file_count(&self, count: usize) -> TdfResult<()> {
        if count > self.max_file_count {
            return Err(TdfError::SizeExceeded(format!(
                "Archive contains {} files, limit is {}",
                count, self.max_file_count
            )));
        }
        Ok(())
    }

    /// Check if individual file size is acceptable
    pub fn check_file_size(&self, size: u64) -> TdfResult<()> {
        if size > self.max_file_size {
            return Err(TdfError::FileSizeExceeded(format!(
                "File size {} exceeds limit {}",
                size, self.max_file_size
            )));
        }
        Ok(())
    }

    // === SECURITY HARDENING: Legacy format rejection ===

    /// Check if Merkle tree version is allowed
    ///
    /// Security Fix (Attack Phase 1): Reject legacy Merkle trees without domain separators
    pub fn check_merkle_version(&self, version: u8) -> TdfResult<()> {
        if self.reject_legacy_merkle && version < 2 {
            return Err(TdfError::PolicyViolation(format!(
                "Legacy Merkle tree version {} rejected. Version 2+ required for domain separator protection (CVE-TDF-002)",
                version
            )));
        }
        Ok(())
    }

    /// Check if signature version is allowed
    ///
    /// Security Fix (Attack Phase 1): Reject legacy signatures without timestamp binding
    pub fn check_signature_version(&self, version: u8) -> TdfResult<()> {
        if self.reject_legacy_signatures && version < 2 {
            return Err(TdfError::PolicyViolation(format!(
                "Legacy signature version {} rejected. Version 2+ required for timestamp binding (CVE-TDF-003)",
                version
            )));
        }
        Ok(())
    }

    /// Check if timestamp source is acceptable
    ///
    /// Security Fix (Attack Phase 4): Optionally require RFC 3161 TSA timestamps
    pub fn check_timestamp_source(&self, has_rfc3161_proof: bool) -> TdfResult<()> {
        if self.require_rfc3161_timestamps && !has_rfc3161_proof {
            return Err(TdfError::PolicyViolation(
                "RFC 3161 timestamp proof required but not present. Manual timestamps not allowed in strict mode.".to_string()
            ));
        }
        Ok(())
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self::for_tier(SizeTier::Standard)
    }
}

/// Algorithm policy for signature and hash algorithm enforcement
///
/// Security Fix (CVE-TDF-010): Prevents algorithm downgrade attacks
#[derive(Debug, Clone)]
pub struct AlgorithmPolicy {
    /// Allowed signature algorithms
    pub allowed_signature_algorithms: HashSet<SignatureAlgorithm>,
    /// Allowed hash algorithms
    pub allowed_hash_algorithms: HashSet<HashAlgorithm>,
    /// Minimum key size in bits
    pub minimum_key_size: usize,
    /// Reject legacy v1 signatures (without timestamp binding)
    pub reject_legacy_signatures: bool,
}

impl AlgorithmPolicy {
    /// Create a strict policy (recommended for production)
    pub fn strict() -> Self {
        let mut sig_algos = HashSet::new();
        sig_algos.insert(SignatureAlgorithm::Ed25519);
        sig_algos.insert(SignatureAlgorithm::Secp256k1);

        let mut hash_algos = HashSet::new();
        hash_algos.insert(HashAlgorithm::Sha256);
        hash_algos.insert(HashAlgorithm::Sha3_256);
        hash_algos.insert(HashAlgorithm::Sha3_512);
        hash_algos.insert(HashAlgorithm::Blake3);

        Self {
            allowed_signature_algorithms: sig_algos,
            allowed_hash_algorithms: hash_algos,
            minimum_key_size: 256,
            reject_legacy_signatures: true,
        }
    }

    /// Create a permissive policy (for testing/migration)
    pub fn permissive() -> Self {
        let mut sig_algos = HashSet::new();
        sig_algos.insert(SignatureAlgorithm::Ed25519);
        sig_algos.insert(SignatureAlgorithm::Secp256k1);
        sig_algos.insert(SignatureAlgorithm::RsaPss);

        let mut hash_algos = HashSet::new();
        hash_algos.insert(HashAlgorithm::Sha256);
        hash_algos.insert(HashAlgorithm::Sha3_256);
        hash_algos.insert(HashAlgorithm::Sha3_512);
        hash_algos.insert(HashAlgorithm::Blake3);

        Self {
            allowed_signature_algorithms: sig_algos,
            allowed_hash_algorithms: hash_algos,
            minimum_key_size: 128,
            reject_legacy_signatures: false,
        }
    }

    /// Check if a signature algorithm is allowed
    pub fn check_signature_algorithm(&self, algorithm: &SignatureAlgorithm) -> TdfResult<()> {
        if !self.allowed_signature_algorithms.contains(algorithm) {
            return Err(TdfError::PolicyViolation(format!(
                "Signature algorithm {:?} not allowed by policy. Allowed: {:?}",
                algorithm, self.allowed_signature_algorithms
            )));
        }
        Ok(())
    }

    /// Check if a hash algorithm is allowed
    pub fn check_hash_algorithm(&self, algorithm: &HashAlgorithm) -> TdfResult<()> {
        if !self.allowed_hash_algorithms.contains(algorithm) {
            return Err(TdfError::PolicyViolation(format!(
                "Hash algorithm {:?} not allowed by policy. Allowed: {:?}",
                algorithm, self.allowed_hash_algorithms
            )));
        }
        Ok(())
    }

    /// Check if signature version is allowed
    pub fn check_signature_version(&self, version: u8) -> TdfResult<()> {
        if self.reject_legacy_signatures && version < 2 {
            return Err(TdfError::PolicyViolation(format!(
                "Legacy signature version {} not allowed. Minimum version: 2",
                version
            )));
        }
        Ok(())
    }
}

impl Default for AlgorithmPolicy {
    fn default() -> Self {
        Self::strict()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_tier_limits() {
        assert_eq!(SizeTier::Micro.max_size_bytes(), 256 * 1024);
        assert_eq!(SizeTier::Standard.max_size_bytes(), 5 * 1024 * 1024);
        assert_eq!(SizeTier::Extended.max_size_bytes(), 50 * 1024 * 1024);
    }

    #[test]
    fn test_security_config_size_check() {
        let config = SecurityConfig::for_tier(SizeTier::Micro);
        assert!(config.check_size(100 * 1024).is_ok());
        assert!(config.check_size(300 * 1024).is_err());
    }

    #[test]
    fn test_decompression_ratio_check() {
        let config = SecurityConfig::for_tier(SizeTier::Standard);
        // 1KB compressed -> 500KB uncompressed = 500:1 ratio (OK)
        assert!(config.check_decompression_ratio(1024, 500 * 1024).is_ok());
        // 1KB compressed -> 2MB uncompressed = 2000:1 ratio (too high)
        assert!(config.check_decompression_ratio(1024, 2 * 1024 * 1024).is_err());
    }

    #[test]
    fn test_decompression_ratio_zero_divisor() {
        // CVE-TDF-015: Test that zero compressed size doesn't bypass checks
        let config = SecurityConfig::for_tier(SizeTier::Standard);

        // Empty file - should be OK
        assert!(config.check_decompression_ratio(0, 0).is_ok());

        // Stored file within limit - should be OK
        assert!(config.check_decompression_ratio(0, 500 * 1024).is_ok());

        // Stored file exceeding limit - should fail
        assert!(config.check_decompression_ratio(0, 10 * 1024 * 1024).is_err());
    }

    #[test]
    fn test_file_count_check() {
        let config = SecurityConfig::for_tier(SizeTier::Standard);
        assert!(config.check_file_count(100).is_ok());
        assert!(config.check_file_count(1000).is_ok());
        assert!(config.check_file_count(1001).is_err());
    }

    #[test]
    fn test_algorithm_policy_strict() {
        let policy = AlgorithmPolicy::strict();

        // Ed25519 and Secp256k1 should be allowed
        assert!(policy.check_signature_algorithm(&SignatureAlgorithm::Ed25519).is_ok());
        assert!(policy.check_signature_algorithm(&SignatureAlgorithm::Secp256k1).is_ok());

        // RSA-PSS should be rejected in strict mode
        assert!(policy.check_signature_algorithm(&SignatureAlgorithm::RsaPss).is_err());

        // All hash algorithms should be allowed in strict mode
        assert!(policy.check_hash_algorithm(&HashAlgorithm::Sha256).is_ok());
        assert!(policy.check_hash_algorithm(&HashAlgorithm::Sha3_256).is_ok());
        assert!(policy.check_hash_algorithm(&HashAlgorithm::Sha3_512).is_ok());
        assert!(policy.check_hash_algorithm(&HashAlgorithm::Blake3).is_ok());
    }

    #[test]
    fn test_algorithm_policy_permissive() {
        let policy = AlgorithmPolicy::permissive();

        // All algorithms should be allowed in permissive mode
        assert!(policy.check_signature_algorithm(&SignatureAlgorithm::Ed25519).is_ok());
        assert!(policy.check_signature_algorithm(&SignatureAlgorithm::Secp256k1).is_ok());
        assert!(policy.check_signature_algorithm(&SignatureAlgorithm::RsaPss).is_ok());
    }

    #[test]
    fn test_algorithm_policy_version_check() {
        let strict = AlgorithmPolicy::strict();
        let permissive = AlgorithmPolicy::permissive();

        // Strict rejects legacy v1 signatures
        assert!(strict.check_signature_version(1).is_err());
        assert!(strict.check_signature_version(2).is_ok());

        // Permissive allows legacy
        assert!(permissive.check_signature_version(1).is_ok());
        assert!(permissive.check_signature_version(2).is_ok());
    }

    // === ATTACK PHASE 1: Security Hardening Tests ===

    #[test]
    fn test_security_config_merkle_version_check() {
        // Default config rejects legacy Merkle trees
        let default_config = SecurityConfig::for_tier(SizeTier::Standard);
        assert!(default_config.check_merkle_version(1).is_err());
        assert!(default_config.check_merkle_version(2).is_ok());

        // Strict config also rejects legacy
        let strict_config = SecurityConfig::strict(SizeTier::Standard);
        assert!(strict_config.check_merkle_version(1).is_err());
        assert!(strict_config.check_merkle_version(2).is_ok());

        // Permissive config allows legacy (for migration only)
        let permissive_config = SecurityConfig::permissive();
        assert!(permissive_config.check_merkle_version(1).is_ok());
        assert!(permissive_config.check_merkle_version(2).is_ok());
    }

    #[test]
    fn test_security_config_signature_version_check() {
        // Default config rejects legacy signatures
        let default_config = SecurityConfig::for_tier(SizeTier::Standard);
        assert!(default_config.check_signature_version(1).is_err());
        assert!(default_config.check_signature_version(2).is_ok());

        // Strict config also rejects legacy
        let strict_config = SecurityConfig::strict(SizeTier::Standard);
        assert!(strict_config.check_signature_version(1).is_err());
        assert!(strict_config.check_signature_version(2).is_ok());

        // Permissive config allows legacy (for migration only)
        let permissive_config = SecurityConfig::permissive();
        assert!(permissive_config.check_signature_version(1).is_ok());
        assert!(permissive_config.check_signature_version(2).is_ok());
    }

    #[test]
    fn test_security_config_timestamp_source_check() {
        // Default config allows manual timestamps (for offline use)
        let default_config = SecurityConfig::for_tier(SizeTier::Standard);
        assert!(default_config.check_timestamp_source(false).is_ok());
        assert!(default_config.check_timestamp_source(true).is_ok());

        // Strict config requires RFC 3161 timestamps
        let strict_config = SecurityConfig::strict(SizeTier::Standard);
        assert!(strict_config.check_timestamp_source(false).is_err());
        assert!(strict_config.check_timestamp_source(true).is_ok());

        // Permissive config allows manual timestamps
        let permissive_config = SecurityConfig::permissive();
        assert!(permissive_config.check_timestamp_source(false).is_ok());
        assert!(permissive_config.check_timestamp_source(true).is_ok());
    }

    #[test]
    fn test_strict_vs_default_security() {
        let default_config = SecurityConfig::for_tier(SizeTier::Standard);
        let strict_config = SecurityConfig::strict(SizeTier::Standard);

        // Both reject legacy Merkle
        assert!(default_config.reject_legacy_merkle);
        assert!(strict_config.reject_legacy_merkle);

        // Both reject legacy signatures
        assert!(default_config.reject_legacy_signatures);
        assert!(strict_config.reject_legacy_signatures);

        // Only strict requires RFC 3161
        assert!(!default_config.require_rfc3161_timestamps);
        assert!(strict_config.require_rfc3161_timestamps);
    }
}

