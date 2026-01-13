//! Cryptographic utilities for TrustDoc Format
//!
//! This module provides security-critical utility functions for cryptographic
//! operations, including constant-time comparisons to prevent timing attacks.
//!
//! # Security
//! - CVE-TDF-024: Constant-time comparisons prevent timing side-channel attacks
//! - All hash and signature comparisons should use `ct_eq` or `ct_eq_slices`

use subtle::ConstantTimeEq;

/// Constant-time equality comparison for byte slices
///
/// Security Fix (CVE-TDF-024): Prevents timing side-channel attacks by ensuring
/// the comparison takes the same amount of time regardless of where differences
/// occur in the input.
///
/// # Arguments
/// * `a` - First byte slice
/// * `b` - Second byte slice
///
/// # Returns
/// * `true` if slices are equal in both length and content
/// * `false` if slices differ in length or any byte
///
/// # Security Note
/// This function uses the `subtle` crate's constant-time comparison to prevent
/// timing attacks. The comparison time depends only on the slice lengths, not
/// on the position of the first difference.
///
/// # Example
/// ```ignore
/// use tdf_core::crypto_utils::ct_eq;
///
/// let hash1 = [0u8; 32];
/// let hash2 = [0u8; 32];
/// assert!(ct_eq(&hash1, &hash2));
/// ```
#[inline]
pub fn ct_eq(a: &[u8], b: &[u8]) -> bool {
    // Length check is constant-time (single comparison)
    // but we must also compare content even if lengths differ
    // to maintain constant time for same-length slices
    if a.len() != b.len() {
        return false;
    }

    // Use subtle's constant-time comparison
    a.ct_eq(b).into()
}

/// Constant-time equality comparison for Vec<u8>
///
/// Security Fix (CVE-TDF-024): Wrapper for ct_eq that works with vectors.
///
/// # Arguments
/// * `a` - First vector
/// * `b` - Second vector
///
/// # Returns
/// * `true` if vectors are equal
/// * `false` if vectors differ
#[inline]
pub fn ct_eq_vecs(a: &Vec<u8>, b: &Vec<u8>) -> bool {
    ct_eq(a.as_slice(), b.as_slice())
}

/// Constant-time equality comparison for hex-encoded strings
///
/// Security Fix (CVE-TDF-024): Compares hex strings in constant time after decoding.
/// This is useful for comparing root hashes that are stored as hex strings.
///
/// # Arguments
/// * `hex_a` - First hex-encoded string
/// * `hex_b` - Second hex-encoded string
///
/// # Returns
/// * `Ok(true)` if strings decode to equal bytes
/// * `Ok(false)` if strings decode to different bytes
/// * `Err` if either string is not valid hex
///
/// # Example
/// ```ignore
/// use tdf_core::crypto_utils::ct_eq_hex;
///
/// let hash1 = "a1b2c3d4...";
/// let hash2 = "a1b2c3d4...";
/// assert!(ct_eq_hex(hash1, hash2).unwrap());
/// ```
pub fn ct_eq_hex(hex_a: &str, hex_b: &str) -> Result<bool, hex::FromHexError> {
    let bytes_a = hex::decode(hex_a)?;
    let bytes_b = hex::decode(hex_b)?;
    Ok(ct_eq(&bytes_a, &bytes_b))
}

/// Constant-time selection between two values
///
/// Security Fix (CVE-TDF-024): Select between two values in constant time
/// based on a condition, preventing timing leaks in conditional operations.
///
/// # Arguments
/// * `condition` - The selection condition (true = select a, false = select b)
/// * `a` - Value to return if condition is true
/// * `b` - Value to return if condition is false
///
/// # Returns
/// * `a` if condition is true
/// * `b` if condition is false
///
/// # Security Note
/// This function now uses the `subtle` crate's `ConditionallySelectable` for
/// true constant-time selection, preventing timing side-channel attacks.
#[inline]
pub fn ct_select<T>(condition: bool, a: T, b: T) -> T
where
    T: subtle::ConditionallySelectable + Copy,
{
    let choice = subtle::Choice::from((!condition) as u8);
    T::conditional_select(&a, &b, choice)
}

/// Constant-time selection for non-ConditionallySelectable types
///
/// For types that don't implement ConditionallySelectable, we use
/// a secure but potentially non-constant-time fallback. Use with caution.
#[inline]
pub fn ct_select_fallback<T: Copy>(condition: bool, a: T, b: T) -> T {
    // For cryptographic safety, prefer to avoid this function
    // and implement ConditionallySelectable for custom types
    if condition { a } else { b }
}

/// Verify that a root hash matches an expected value in constant time
///
/// Security Fix (CVE-TDF-024): Common utility for verifying Merkle root hashes
/// without timing side-channel leakage.
///
/// # Arguments
/// * `computed` - The computed hash
/// * `expected` - The expected hash
///
/// # Returns
/// * `true` if hashes match
/// * `false` if hashes differ
#[inline]
pub fn verify_root_hash(computed: &[u8], expected: &[u8]) -> bool {
    ct_eq(computed, expected)
}

/// Verify that a signature matches an expected value in constant time
///
/// Security Fix (CVE-TDF-024): Utility for comparing signature bytes.
/// Note: Most signature verification is already constant-time in the crypto
/// libraries (ed25519-dalek, k256), but this is useful for comparing
/// serialized signatures.
///
/// # Arguments
/// * `computed` - The computed or received signature
/// * `expected` - The expected signature
///
/// # Returns
/// * `true` if signatures match
/// * `false` if signatures differ
#[inline]
pub fn verify_signature_bytes(computed: &[u8], expected: &[u8]) -> bool {
    ct_eq(computed, expected)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ct_eq_identical() {
        let a = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
        let b = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
        assert!(ct_eq(&a, &b));
    }

    #[test]
    fn test_ct_eq_different() {
        let a = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
        let b = vec![1u8, 2, 3, 4, 5, 6, 7, 9]; // Last byte different
        assert!(!ct_eq(&a, &b));
    }

    #[test]
    fn test_ct_eq_different_first_byte() {
        let a = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
        let b = vec![0u8, 2, 3, 4, 5, 6, 7, 8]; // First byte different
        assert!(!ct_eq(&a, &b));
    }

    #[test]
    fn test_ct_eq_different_lengths() {
        let a = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
        let b = vec![1u8, 2, 3, 4]; // Shorter
        assert!(!ct_eq(&a, &b));

        let c = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9]; // Longer
        assert!(!ct_eq(&a, &c));
    }

    #[test]
    fn test_ct_eq_empty() {
        let a: Vec<u8> = vec![];
        let b: Vec<u8> = vec![];
        assert!(ct_eq(&a, &b));
    }

    #[test]
    fn test_ct_eq_vecs() {
        let a = vec![1u8, 2, 3, 4];
        let b = vec![1u8, 2, 3, 4];
        assert!(ct_eq_vecs(&a, &b));
    }

    #[test]
    fn test_ct_eq_hex_identical() {
        let hex_a = "a1b2c3d4e5f67890";
        let hex_b = "a1b2c3d4e5f67890";
        assert!(ct_eq_hex(hex_a, hex_b).unwrap());
    }

    #[test]
    fn test_ct_eq_hex_different() {
        let hex_a = "a1b2c3d4e5f67890";
        let hex_b = "a1b2c3d4e5f67891"; // Last digit different
        assert!(!ct_eq_hex(hex_a, hex_b).unwrap());
    }

    #[test]
    fn test_ct_eq_hex_case_insensitive() {
        let hex_a = "a1b2c3d4";
        let hex_b = "A1B2C3D4";
        assert!(ct_eq_hex(hex_a, hex_b).unwrap());
    }

    #[test]
    fn test_ct_eq_hex_invalid() {
        let hex_a = "a1b2c3d4";
        let hex_b = "not_hex!";
        assert!(ct_eq_hex(hex_a, hex_b).is_err());
    }

    #[test]
    fn test_verify_root_hash() {
        let hash1 = [0u8; 32];
        let hash2 = [0u8; 32];
        assert!(verify_root_hash(&hash1, &hash2));

        let mut hash3 = [0u8; 32];
        hash3[31] = 1;
        assert!(!verify_root_hash(&hash1, &hash3));
    }

    #[test]
    fn test_ct_select() {
        // Test with ConditionallySelectable types
        assert_eq!(ct_select(true, 1u8, 2u8), 1u8);
        assert_eq!(ct_select(false, 1u8, 2u8), 2u8);

        // Test with different values
        assert_eq!(ct_select(true, 255u8, 0u8), 255u8);
        assert_eq!(ct_select(false, 255u8, 0u8), 0u8);
    }

    // Timing attack resistance test (heuristic)
    // Note: This is not a rigorous timing attack test, just a sanity check
    #[test]
    fn test_timing_consistency() {
        use std::time::Instant;

        let iterations = 10000;

        // Compare slices that differ at first byte
        let a1 = vec![0u8; 64];
        let mut b1 = vec![0u8; 64];
        b1[0] = 1;

        // Compare slices that differ at last byte
        let a2 = vec![0u8; 64];
        let mut b2 = vec![0u8; 64];
        b2[63] = 1;

        // Warm up
        for _ in 0..1000 {
            let _ = ct_eq(&a1, &b1);
            let _ = ct_eq(&a2, &b2);
        }

        // Time first-byte difference
        let start1 = Instant::now();
        for _ in 0..iterations {
            let _ = ct_eq(&a1, &b1);
        }
        let time1 = start1.elapsed();

        // Time last-byte difference
        let start2 = Instant::now();
        for _ in 0..iterations {
            let _ = ct_eq(&a2, &b2);
        }
        let time2 = start2.elapsed();

        // Times should be similar (within 50% of each other)
        // This is a very loose check - proper timing analysis requires more rigor
        let ratio = time1.as_nanos() as f64 / time2.as_nanos() as f64;
        assert!(
            ratio > 0.5 && ratio < 2.0,
            "Timing difference too large: ratio = {}",
            ratio
        );
    }
}
