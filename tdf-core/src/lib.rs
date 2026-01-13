pub mod audit;
pub mod content;
pub mod crypto_utils;
pub mod document;
pub mod error;
pub mod merkle;
pub mod signature;
pub mod archive;
pub mod timestamp;
pub mod multiparty;
pub mod revocation;
pub mod config;
pub mod whitelist;
pub mod io;
pub mod secure_key;
pub mod integer_safety;
pub mod secure_random;
pub mod error_sanitization;
pub mod resource_limits;

// Re-export commonly used security utilities
pub use secure_random::{generate_secure_bytes, generate_secure_token, generate_secure_nonce};
pub use error_sanitization::{sanitize_error, error_code};
pub use resource_limits::{CircuitBreaker, RateLimiter, ResourceBudget};

pub use document::Document;
pub use error::{TdfError, TdfResult};
pub use content::*;
pub use merkle::*;
pub use signature::*;
pub use archive::*;

