//! Error message sanitization utilities
//!
//! Security Fixes:
//! - Vulnerability #11: Sandbox crash information leakage
//! - Vulnerability #12: Social engineering via protocol error messages
//!
//! This module provides utilities to sanitize error messages, preventing
//! information leakage that could be used for attacks or social engineering.

use crate::error::TdfError;

/// Sanitize error messages to prevent information leakage
///
/// Security Fix (Vuln #11, #12): Removes sensitive information from
/// error messages that could be used for:
/// - Social engineering attacks
/// - Information gathering for exploits
/// - Timing analysis
///
/// # Arguments
/// * `error` - The error to sanitize
///
/// # Returns
/// * Generic error message without sensitive details
pub fn sanitize_error(error: &TdfError) -> String {
    match error {
        // Generic errors - no sensitive info
        TdfError::Io(_) => "I/O operation failed".to_string(),
        TdfError::CborSer(_) => "Serialization failed".to_string(),
        TdfError::CborDe(_) => "Deserialization failed".to_string(),
        TdfError::Json(_) => "JSON parsing failed".to_string(),
        TdfError::Zip(_) => "Archive operation failed".to_string(),
        
        // Document errors - sanitize paths and details
        TdfError::InvalidDocument(msg) => {
            // Remove file paths and sensitive details
            sanitize_message(msg)
        }
        TdfError::IntegrityFailure(_) => "Integrity verification failed".to_string(),
        TdfError::SignatureFailure(_) => "Signature verification failed".to_string(),
        
        // Algorithm errors - generic message
        TdfError::UnsupportedHashAlgorithm(_) => "Unsupported hash algorithm".to_string(),
        TdfError::UnsupportedSignatureAlgorithm(_) => "Unsupported signature algorithm".to_string(),
        
        // File errors - sanitize paths
        TdfError::MissingFile(_) => "Required file not found".to_string(),
        TdfError::FileSizeExceeded(_) => "File size limit exceeded".to_string(),
        TdfError::InvalidContentType(_) => "Invalid content type".to_string(),
        
        // Security errors - generic messages
        TdfError::VerificationFailed(_) => "Verification failed".to_string(),
        TdfError::SignatureRequired(_) => "Signature required".to_string(),
        TdfError::UntrustedSigner(_) => "Untrusted signer".to_string(),
        TdfError::RevokedKey(_) => "Key has been revoked".to_string(),
        TdfError::SizeExceeded(_) => "Size limit exceeded".to_string(),
        TdfError::ReadLimitExceeded(_) => "Read limit exceeded".to_string(),
        TdfError::InvalidPath(_) => "Invalid path".to_string(),
        TdfError::PolicyViolation(_) => "Security policy violation".to_string(),
        TdfError::RootHashMismatch(_) => "Root hash mismatch".to_string(),
        TdfError::TimestampError(_) => "Timestamp validation failed".to_string(),
        TdfError::IntegerOverflow(_) => "Integer overflow detected".to_string(),
        TdfError::ParseError(_) => "Parse error".to_string(),
        TdfError::DepthLimitExceeded(_) => "Depth limit exceeded".to_string(),
    }
}

/// Sanitize a message string to remove sensitive information
///
/// Removes:
/// - File paths
/// - Memory addresses
/// - Stack traces
/// - Internal implementation details
fn sanitize_message(msg: &str) -> String {
    let mut sanitized = msg.to_string();
    
    // Remove common path patterns
    sanitized = regex::Regex::new(r"/[^\s]+")
        .unwrap()
        .replace_all(&sanitized, "[path]")
        .to_string();
    
    sanitized = regex::Regex::new(r"[A-Z]:\\[^\s]+")
        .unwrap()
        .replace_all(&sanitized, "[path]")
        .to_string();
    
    // Remove memory addresses (hex patterns)
    sanitized = regex::Regex::new(r"0x[0-9a-fA-F]+")
        .unwrap()
        .replace_all(&sanitized, "[address]")
        .to_string();
    
    // Remove stack trace indicators
    sanitized = regex::Regex::new(r"at\s+[^\n]+")
        .unwrap()
        .replace_all(&sanitized, "[stack]")
        .to_string();
    
    sanitized
}

/// Create a generic error code for logging
///
/// Security Fix (Vuln #11, #12): Provides generic error codes that
/// can be used for logging and monitoring without exposing sensitive
/// information to users.
///
/// # Arguments
/// * `error` - The error to generate code for
///
/// # Returns
/// * Generic error code string
pub fn error_code(error: &TdfError) -> &'static str {
    match error {
        TdfError::Io(_) => "ERR_IO",
        TdfError::CborSer(_) => "ERR_CBOR_SER",
        TdfError::CborDe(_) => "ERR_CBOR_DE",
        TdfError::Json(_) => "ERR_JSON",
        TdfError::Zip(_) => "ERR_ZIP",
        TdfError::InvalidDocument(_) => "ERR_INVALID_DOC",
        TdfError::IntegrityFailure(_) => "ERR_INTEGRITY",
        TdfError::SignatureFailure(_) => "ERR_SIGNATURE",
        TdfError::UnsupportedHashAlgorithm(_) => "ERR_HASH_ALGO",
        TdfError::UnsupportedSignatureAlgorithm(_) => "ERR_SIG_ALGO",
        TdfError::MissingFile(_) => "ERR_MISSING_FILE",
        TdfError::FileSizeExceeded(_) => "ERR_FILE_SIZE",
        TdfError::InvalidContentType(_) => "ERR_CONTENT_TYPE",
        TdfError::VerificationFailed(_) => "ERR_VERIFY",
        TdfError::SignatureRequired(_) => "ERR_SIG_REQUIRED",
        TdfError::UntrustedSigner(_) => "ERR_UNTRUSTED",
        TdfError::RevokedKey(_) => "ERR_REVOKED",
        TdfError::SizeExceeded(_) => "ERR_SIZE",
        TdfError::ReadLimitExceeded(_) => "ERR_READ_LIMIT",
        TdfError::InvalidPath(_) => "ERR_PATH",
        TdfError::PolicyViolation(_) => "ERR_POLICY",
        TdfError::RootHashMismatch(_) => "ERR_ROOT_HASH",
        TdfError::TimestampError(_) => "ERR_TIMESTAMP",
        TdfError::IntegerOverflow(_) => "ERR_OVERFLOW",
        TdfError::ParseError(_) => "ERR_PARSE",
        TdfError::DepthLimitExceeded(_) => "ERR_DEPTH",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_error_removes_paths() {
        let error = TdfError::InvalidDocument(
            "File /home/user/secret.tdf not found".to_string()
        );
        let sanitized = sanitize_error(&error);
        assert!(!sanitized.contains("/home/user/secret.tdf"));
        assert!(!sanitized.contains("secret"));
    }

    #[test]
    fn test_sanitize_error_removes_addresses() {
        let error = TdfError::InvalidDocument(
            "Memory address 0x7fff12345678 invalid".to_string()
        );
        let sanitized = sanitize_error(&error);
        assert!(!sanitized.contains("0x7fff12345678"));
    }

    #[test]
    fn test_error_code_generic() {
        let error = TdfError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File not found"
        ));
        assert_eq!(error_code(&error), "ERR_IO");
    }
}
