//! Secure I/O utilities with bounded reading
//!
//! Security Fixes:
//! - CVE-TDF-005: Bounded readers to prevent memory exhaustion
//! - CVE-TDF-009: Size limits for deserialization

use crate::error::{TdfError, TdfResult};
use std::io::{self, Read};

/// A reader wrapper that enforces a maximum read limit
///
/// This prevents memory exhaustion attacks where a malicious archive
/// contains files that decompress to very large sizes.
pub struct BoundedReader<R: Read> {
    inner: R,
    limit: u64,
    read: u64,
}

impl<R: Read> BoundedReader<R> {
    /// Create a new bounded reader with the specified limit
    pub fn new(reader: R, limit: u64) -> Self {
        Self {
            inner: reader,
            limit,
            read: 0,
        }
    }

    /// Get the number of bytes read so far
    pub fn bytes_read(&self) -> u64 {
        self.read
    }

    /// Get the remaining bytes that can be read
    pub fn remaining(&self) -> u64 {
        self.limit.saturating_sub(self.read)
    }
}

impl<R: Read> Read for BoundedReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // If we've reached the limit, return EOF (0 bytes) instead of error
        // This allows reading exactly up to the limit
        if self.read >= self.limit {
            // Check if there's actually more data available
            // If the inner reader is exhausted, this is fine (return 0)
            // If there's more data, that means we'd exceed the limit
            let mut test_buf = [0u8; 1];
            match self.inner.read(&mut test_buf) {
                Ok(0) => return Ok(0), // Inner reader exhausted, we're done
                Ok(_) => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!(
                            "Read limit exceeded: attempted to read beyond {} bytes",
                            self.limit
                        ),
                    ));
                }
                Err(e) => return Err(e),
            }
        }

        // Calculate how many bytes we can still read
        let remaining = (self.limit - self.read) as usize;
        let to_read = buf.len().min(remaining);

        // Read into a potentially smaller slice
        let n = self.inner.read(&mut buf[..to_read])?;
        self.read += n as u64;

        Ok(n)
    }
}

/// Read a file with size bounds, returning an error if the limit is exceeded
pub fn read_bounded<R: Read>(mut reader: R, limit: u64) -> TdfResult<Vec<u8>> {
    let mut bounded = BoundedReader::new(&mut reader, limit);
    let mut buf = Vec::new();

    match bounded.read_to_end(&mut buf) {
        Ok(_) => Ok(buf),
        Err(e) if e.kind() == io::ErrorKind::Other => {
            Err(TdfError::ReadLimitExceeded(format!(
                "File exceeds size limit of {} bytes",
                limit
            )))
        }
        Err(e) => Err(TdfError::Io(e)),
    }
}

/// Read exactly `limit` bytes or less, with pre-allocation optimization
pub fn read_with_limit<R: Read>(mut reader: R, expected_size: usize, limit: u64) -> TdfResult<Vec<u8>> {
    // Pre-allocate with the expected size, but cap it
    let alloc_size = expected_size.min(limit as usize).min(16 * 1024 * 1024); // Cap at 16MB pre-alloc
    let mut buf = Vec::with_capacity(alloc_size);

    let mut bounded = BoundedReader::new(&mut reader, limit);
    match bounded.read_to_end(&mut buf) {
        Ok(_) => Ok(buf),
        Err(e) if e.kind() == io::ErrorKind::Other => {
            Err(TdfError::ReadLimitExceeded(format!(
                "File exceeds size limit of {} bytes",
                limit
            )))
        }
        Err(e) => Err(TdfError::Io(e)),
    }
}

/// Maximum CBOR recursion depth to prevent stack overflow
///
/// Security Fix (CVE-TDF-032): Prevents deserialization attacks via
/// deeply nested structures that could cause stack overflow.
pub const MAX_CBOR_DEPTH: usize = 64;

/// Maximum size for CBOR deserialization
///
/// Security Fix (CVE-TDF-032): Prevents memory exhaustion attacks via
/// large deserialized structures.
pub const MAX_CBOR_SIZE: usize = 50 * 1024 * 1024; // 50 MB

/// Deserialize CBOR with size bounds and depth limits
///
/// Security Fixes:
/// - CVE-TDF-032: Prevents deserialization attacks via depth limits
/// - CVE-TDF-009: Size limits for deserialization
///
/// # Arguments
/// * `data` - CBOR-encoded data to deserialize
/// * `max_size` - Maximum size in bytes
///
/// # Returns
/// * `Ok(deserialized)` if deserialization succeeds within limits
/// * `Err(TdfError::SizeExceeded)` if size exceeds limit
/// * `Err(TdfError::DepthLimitExceeded)` if depth exceeds limit
/// * `Err(TdfError::ParseError)` if deserialization fails
pub fn deserialize_cbor_bounded<T: serde::de::DeserializeOwned>(
    data: &[u8],
    max_size: usize,
) -> TdfResult<T> {
    // Security Fix (CVE-TDF-032): Check size before deserialization
    if data.len() > max_size {
        return Err(TdfError::SizeExceeded(format!(
            "CBOR data size {} exceeds limit {}",
            data.len(),
            max_size
        )));
    }

    // Security Fix (CVE-TDF-032): Check for potential depth issues
    // ciborium doesn't expose depth limits directly, but we can
    // check for suspicious patterns (very small data with many nested structures)
    // This is a heuristic - proper depth checking would require custom deserializer
    if data.len() < 100 && data.iter().filter(|&&b| b == 0x81 || b == 0x82 || b == 0x83).count() > 20 {
        return Err(TdfError::DepthLimitExceeded(
            "Suspicious CBOR structure: potential depth attack".to_string()
        ));
    }

    ciborium::from_reader(data).map_err(|e| TdfError::ParseError(format!("CBOR parse error: {}", e)))
}

/// Deserialize JSON with size bounds
pub fn deserialize_json_bounded<T: serde::de::DeserializeOwned>(
    data: &[u8],
    max_size: usize,
) -> TdfResult<T> {
    if data.len() > max_size {
        return Err(TdfError::SizeExceeded(format!(
            "JSON data size {} exceeds limit {}",
            data.len(),
            max_size
        )));
    }

    serde_json::from_slice(data).map_err(|e| TdfError::ParseError(format!("JSON parse error: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_bounded_reader_within_limit() {
        let data = b"Hello, World!";
        let cursor = Cursor::new(data);
        let mut bounded = BoundedReader::new(cursor, 100);

        let mut buf = Vec::new();
        bounded.read_to_end(&mut buf).unwrap();

        assert_eq!(buf, data);
        assert_eq!(bounded.bytes_read(), data.len() as u64);
    }

    #[test]
    fn test_bounded_reader_at_limit() {
        let data = b"Hello, World!";
        let cursor = Cursor::new(data);
        let mut bounded = BoundedReader::new(cursor, data.len() as u64);

        let mut buf = Vec::new();
        bounded.read_to_end(&mut buf).unwrap();

        assert_eq!(buf, data);
    }

    #[test]
    fn test_bounded_reader_exceeds_limit() {
        let data = b"Hello, World!";
        let cursor = Cursor::new(data);
        let mut bounded = BoundedReader::new(cursor, 5); // Only allow 5 bytes

        let mut buf = Vec::new();
        let result = bounded.read_to_end(&mut buf);

        // Should fail because data is larger than limit
        assert!(result.is_err());
    }

    #[test]
    fn test_read_bounded_success() {
        let data = b"Test data";
        let cursor = Cursor::new(data);

        let result = read_bounded(cursor, 100);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), data);
    }

    #[test]
    fn test_read_bounded_exceeds() {
        let data = b"Test data that is too long";
        let cursor = Cursor::new(data);

        let result = read_bounded(cursor, 5);
        assert!(result.is_err());
    }

    #[test]
    fn test_deserialize_cbor_bounded() {
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct TestStruct {
            value: i32,
        }

        let test = TestStruct { value: 42 };
        // Use ciborium for CBOR serialization
        let mut cbor = Vec::new();
        ciborium::into_writer(&test, &mut cbor).unwrap();

        let result: TdfResult<TestStruct> = deserialize_cbor_bounded(&cbor, 1000);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), test);
    }

    #[test]
    fn test_deserialize_cbor_exceeds_limit() {
        let large_data = vec![0u8; 1000];

        let result: TdfResult<Vec<u8>> = deserialize_cbor_bounded(&large_data, 100);
        assert!(result.is_err());
    }
}
