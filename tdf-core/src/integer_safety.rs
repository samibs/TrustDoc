//! Integer safety utilities to prevent overflow attacks
//!
//! Security Fixes:
//! - CVE-TDF-021: Integer overflow protection in size calculations
//! - CVE-TDF-008: Checked arithmetic for all size operations
//!
//! This module provides safe integer arithmetic operations that prevent
//! integer overflow attacks, which can lead to buffer overflows or
//! memory corruption.

use crate::error::{TdfError, TdfResult};

/// Safely add two u64 values with overflow checking
///
/// Security Fix (CVE-TDF-021): Prevents integer overflow in size calculations
///
/// # Arguments
/// * `a` - First value
/// * `b` - Second value
///
/// # Returns
/// * `Ok(sum)` if addition succeeds without overflow
/// * `Err(TdfError::IntegerOverflow)` if overflow would occur
pub fn checked_add(a: u64, b: u64) -> TdfResult<u64> {
    a.checked_add(b).ok_or_else(|| {
        TdfError::IntegerOverflow(format!(
            "Integer overflow in addition: {} + {}",
            a, b
        ))
    })
}

/// Safely multiply two u64 values with overflow checking
///
/// Security Fix (CVE-TDF-021): Prevents integer overflow in size calculations
///
/// # Arguments
/// * `a` - First value
/// * `b` - Second value
///
/// # Returns
/// * `Ok(product)` if multiplication succeeds without overflow
/// * `Err(TdfError::IntegerOverflow)` if overflow would occur
pub fn checked_mul(a: u64, b: u64) -> TdfResult<u64> {
    a.checked_mul(b).ok_or_else(|| {
        TdfError::IntegerOverflow(format!(
            "Integer overflow in multiplication: {} * {}",
            a, b
        ))
    })
}

/// Safely calculate total size from multiple components
///
/// Security Fix (CVE-TDF-021): Prevents integer overflow when summing
/// multiple size values (e.g., frame_size + mac_size + padding_size)
///
/// # Arguments
/// * `sizes` - Iterator of size values to sum
///
/// # Returns
/// * `Ok(total)` if sum succeeds without overflow
/// * `Err(TdfError::IntegerOverflow)` if overflow would occur
pub fn checked_sum<I>(sizes: I) -> TdfResult<u64>
where
    I: Iterator<Item = u64>,
{
    let mut total = 0u64;
    for size in sizes {
        total = checked_add(total, size)?;
    }
    Ok(total)
}

/// Safely calculate frame size including MAC and padding
///
/// Security Fix (CVE-TDF-021): Prevents integer overflow in frame size
/// calculations that include MAC size and padding.
///
/// # Arguments
/// * `frame_size` - Base frame size
/// * `mac_size` - MAC size (e.g., 64 for SHA-512)
/// * `padding_size` - Optional padding size
///
/// # Returns
/// * `Ok(total_size)` if calculation succeeds
/// * `Err(TdfError::IntegerOverflow)` if overflow would occur
pub fn calculate_frame_size(
    frame_size: u64,
    mac_size: u64,
    padding_size: Option<u64>,
) -> TdfResult<u64> {
    let mut total = checked_add(frame_size, mac_size)?;
    if let Some(padding) = padding_size {
        total = checked_add(total, padding)?;
    }
    Ok(total)
}

/// Safely convert usize to u64 with overflow checking
///
/// On platforms where usize is larger than u64, this prevents
/// truncation issues.
///
/// # Arguments
/// * `value` - usize value to convert
///
/// # Returns
/// * `Ok(u64_value)` if conversion succeeds
/// * `Err(TdfError::IntegerOverflow)` if value exceeds u64::MAX
pub fn usize_to_u64(value: usize) -> TdfResult<u64> {
    if value > u64::MAX as usize {
        return Err(TdfError::IntegerOverflow(format!(
            "usize value {} exceeds u64::MAX",
            value
        )));
    }
    Ok(value as u64)
}

/// Safely convert u64 to usize with overflow checking
///
/// On platforms where usize is smaller than u64, this prevents
/// truncation issues.
///
/// # Arguments
/// * `value` - u64 value to convert
///
/// # Returns
/// * `Ok(usize_value)` if conversion succeeds
/// * `Err(TdfError::IntegerOverflow)` if value exceeds usize::MAX
pub fn u64_to_usize(value: u64) -> TdfResult<usize> {
    if value > usize::MAX as u64 {
        return Err(TdfError::IntegerOverflow(format!(
            "u64 value {} exceeds usize::MAX ({})",
            value,
            usize::MAX
        )));
    }
    Ok(value as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checked_add_normal() {
        assert_eq!(checked_add(100, 200).unwrap(), 300);
        assert_eq!(checked_add(0, 0).unwrap(), 0);
    }

    #[test]
    fn test_checked_add_overflow() {
        let result = checked_add(u64::MAX, 1);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TdfError::IntegerOverflow(_)));
    }

    #[test]
    fn test_checked_mul_normal() {
        assert_eq!(checked_mul(100, 200).unwrap(), 20000);
        assert_eq!(checked_mul(0, 1000).unwrap(), 0);
    }

    #[test]
    fn test_checked_mul_overflow() {
        let result = checked_mul(u64::MAX, 2);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TdfError::IntegerOverflow(_)));
    }

    #[test]
    fn test_checked_sum_normal() {
        let sizes = vec![100u64, 200, 300];
        assert_eq!(checked_sum(sizes.iter().copied()).unwrap(), 600);
    }

    #[test]
    fn test_checked_sum_overflow() {
        let sizes = vec![u64::MAX, 1u64];
        let result = checked_sum(sizes.iter().copied());
        assert!(result.is_err());
    }

    #[test]
    fn test_calculate_frame_size() {
        // Normal case
        assert_eq!(
            calculate_frame_size(1000, 64, Some(16)).unwrap(),
            1080
        );

        // Without padding
        assert_eq!(calculate_frame_size(1000, 64, None).unwrap(), 1064);

        // Overflow case
        let result = calculate_frame_size(u64::MAX, 64, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_usize_to_u64() {
        assert_eq!(usize_to_u64(1000).unwrap(), 1000);
        assert_eq!(usize_to_u64(0).unwrap(), 0);
    }

    #[test]
    fn test_u64_to_usize() {
        assert_eq!(u64_to_usize(1000).unwrap(), 1000);
        assert_eq!(u64_to_usize(0).unwrap(), 0);
    }
}
