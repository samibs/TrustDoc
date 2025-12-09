use thiserror::Error;

#[derive(Error, Debug)]
pub enum TdfError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("CBOR encoding error: {0}")]
    Cbor(#[from] serde_cbor::Error),

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
}

pub type TdfResult<T> = Result<T, TdfError>;

