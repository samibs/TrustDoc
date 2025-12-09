//! Security configuration for TDF format
//! Defines size limits, resource quotas, and security settings

use crate::error::{TdfError, TdfResult};

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
        }
    }

    /// Create default config (Standard tier)
    pub fn default() -> Self {
        Self::for_tier(SizeTier::Standard)
    }

    /// Create permissive config (for testing)
    pub fn permissive() -> Self {
        SecurityConfig {
            max_archive_size: 100 * 1024 * 1024, // 100 MB
            max_decompression_ratio: 100000,      // Very high
            max_file_size: 50 * 1024 * 1024,     // 50 MB
            max_file_count: 10000,
            strict_size_check: false,
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
    pub fn check_decompression_ratio(&self, compressed: u64, uncompressed: u64) -> TdfResult<()> {
        if compressed == 0 {
            return Ok(()); // No compression
        }
        let ratio = uncompressed / compressed;
        if ratio > self.max_decompression_ratio {
            return Err(TdfError::FileSizeExceeded(format!(
                "Decompression ratio {}:1 exceeds limit {}:1 (possible ZIP bomb)",
                ratio, self.max_decompression_ratio
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
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self::for_tier(SizeTier::Standard)
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
}

