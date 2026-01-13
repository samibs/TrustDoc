use thiserror::Error;

#[derive(Error, Debug)]
pub enum TdfError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("CBOR serialization error: {0}")]
    CborSer(#[from] ciborium::ser::Error<std::io::Error>),

    #[error("CBOR deserialization error: {0}")]
    CborDe(#[from] ciborium::de::Error<std::io::Error>),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("ZIP archive error: {0}")]
    Zip(#[from] zip::result::ZipError),

    #[error("Invalid document: {0}")]
    InvalidDocument(String),

    #[error("Integrity check failed: {0}")]
    IntegrityFailure(String),

    #[error("Signature verification failed: {0}")]
    SignatureFailure(String),

    #[error("Hash algorithm not supported: {0}")]
    UnsupportedHashAlgorithm(String),

    #[error("Signature algorithm not supported: {0}")]
    UnsupportedSignatureAlgorithm(String),

    #[error("Missing required file in archive: {0}")]
    MissingFile(String),

    #[error("File size exceeds tier limit: {0}")]
    FileSizeExceeded(String),

    #[error("Invalid content type: {0}")]
    InvalidContentType(String),

    // === SECURITY HARDENING ERRORS (Phase 1) ===

    #[error("Verification failed: {0}")]
    VerificationFailed(String),

    #[error("Signature required: {0}")]
    SignatureRequired(String),

    #[error("Untrusted signer: {0}")]
    UntrustedSigner(String),

    #[error("Revoked key: {0}")]
    RevokedKey(String),

    #[error("Size limit exceeded: {0}")]
    SizeExceeded(String),

    #[error("Read limit exceeded: {0}")]
    ReadLimitExceeded(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Policy violation: {0}")]
    PolicyViolation(String),

    #[error("Root hash mismatch: {0}")]
    RootHashMismatch(String),

    #[error("Timestamp error: {0}")]
    TimestampError(String),

    #[error("Integer overflow: {0}")]
    IntegerOverflow(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Depth limit exceeded: {0}")]
    DepthLimitExceeded(String),
}

pub type TdfResult<T> = Result<T, TdfError>;

