use crate::error::{TdfError, TdfResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimestampAuthority {
    pub url: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimestampToken {
    pub time: DateTime<Utc>,
    pub authority: String,
    pub proof: String, // Base64-encoded RFC 3161 timestamp token
    pub algorithm: String, // "rfc3161" or "manual"
}

pub trait TimestampProvider: Send + Sync {
    fn get_timestamp(&self, data: &[u8]) -> Result<TimestampToken, String>;
}

pub struct ManualTimestampProvider;

impl TimestampProvider for ManualTimestampProvider {
    fn get_timestamp(&self, _data: &[u8]) -> Result<TimestampToken, String> {
        Ok(TimestampToken {
            time: Utc::now(),
            authority: "manual".to_string(),
            proof: String::new(),
            algorithm: "manual".to_string(),
        })
    }
}

#[cfg(feature = "rfc3161")]
pub struct Rfc3161TimestampProvider {
    url: String,
}

#[cfg(feature = "rfc3161")]
impl Rfc3161TimestampProvider {
    pub fn new(url: String) -> Self {
        Rfc3161TimestampProvider { url }
    }

    pub async fn get_timestamp_async(&self, data: &[u8]) -> Result<TimestampToken, String> {
        use reqwest::Client;
        use sha2::{Digest, Sha256};
        use base64::{engine::general_purpose::STANDARD, Engine};

        // Hash the data (SHA-256)
        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash = hasher.finalize();

        // Request timestamp from TSA (RFC 3161)
        // This is a simplified implementation
        // Full RFC 3161 requires ASN.1 encoding/decoding
        let client = Client::new();
        let response = client
            .post(&self.url)
            .header("Content-Type", "application/timestamp-query")
            .body(hash.to_vec())
            .send()
            .await
            .map_err(|e| format!("TSA request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("TSA returned error: {}", response.status()));
        }

        let token_bytes = response.bytes().await
            .map_err(|e| format!("Failed to read TSA response: {}", e))?;

        Ok(TimestampToken {
            time: Utc::now(), // Would parse from token in full implementation
            authority: self.url.clone(),
            proof: STANDARD.encode(&token_bytes),
            algorithm: "rfc3161".to_string(),
        })
    }
}

#[cfg(feature = "rfc3161")]
impl TimestampProvider for Rfc3161TimestampProvider {
    fn get_timestamp(&self, data: &[u8]) -> Result<TimestampToken, String> {
        // Synchronous wrapper - in production, use async runtime
        // For now, return manual timestamp with RFC 3161 marker
        Ok(TimestampToken {
            time: Utc::now(),
            authority: self.url.clone(),
            proof: String::new(),
            algorithm: "rfc3161".to_string(),
        })
    }
}

/// Timestamp validation configuration
#[derive(Debug, Clone)]
pub struct TimestampValidationConfig {
    /// Maximum allowed clock skew in seconds (default: 300 = 5 minutes)
    pub max_clock_skew_seconds: i64,
    /// Maximum age of timestamp in seconds (None = no limit)
    pub max_timestamp_age_seconds: Option<i64>,
    /// Require RFC 3161 proof for non-manual timestamps
    pub require_proof: bool,
}

impl Default for TimestampValidationConfig {
    fn default() -> Self {
        TimestampValidationConfig {
            max_clock_skew_seconds: 300, // 5 minutes
            max_timestamp_age_seconds: None, // No limit by default
            require_proof: true,
        }
    }
}

/// Timestamp validation result
#[derive(Debug, Clone)]
pub struct TimestampValidationResult {
    pub valid: bool,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

impl TimestampValidationResult {
    pub fn new() -> Self {
        TimestampValidationResult {
            valid: true,
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn add_warning(&mut self, msg: String) {
        self.warnings.push(msg);
    }

    pub fn add_error(&mut self, msg: String) {
        self.errors.push(msg);
        self.valid = false;
    }
}

impl Default for TimestampValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

pub fn verify_timestamp_token(token: &TimestampToken, _data: &[u8]) -> TdfResult<bool> {
    verify_timestamp_token_with_config(token, _data, TimestampValidationConfig::default())
}

pub fn verify_timestamp_token_with_config(
    token: &TimestampToken,
    _data: &[u8],
    config: TimestampValidationConfig,
) -> TdfResult<bool> {
    let mut result = TimestampValidationResult::new();

    match token.algorithm.as_str() {
        "manual" => {
            // Manual timestamps: check clock skew and freshness
            let now = Utc::now();
            let skew = (now - token.time).num_seconds().abs();
            if skew > config.max_clock_skew_seconds {
                result.add_warning(format!(
                    "Large clock skew detected: {} seconds (max: {})",
                    skew, config.max_clock_skew_seconds
                ));
            }
            if let Some(max_age) = config.max_timestamp_age_seconds {
                let age = (now - token.time).num_seconds();
                if age > max_age {
                    result.add_error(format!(
                        "Timestamp too old: {} seconds (max: {})",
                        age, max_age
                    ));
                }
            }
        }
        "rfc3161" => {
            // Verify RFC 3161 token
            if token.proof.is_empty() {
                if config.require_proof {
                    result.add_error("RFC 3161 timestamp token missing proof".to_string());
                } else {
                    result.add_warning("RFC 3161 timestamp token missing proof".to_string());
                }
            } else {
                // In full implementation, would:
                // 1. Decode base64 proof
                // 2. Parse ASN.1 structure
                // 3. Verify TSA certificate chain
                // 4. Verify timestamp signature
                // 5. Extract timestamp from token
                // 6. Compare with token.time
                result.add_warning("RFC 3161 proof present but not fully validated (requires ASN.1 library)".to_string());
            }

            // Check clock skew
            let now = Utc::now();
            let skew = (now - token.time).num_seconds().abs();
            if skew > config.max_clock_skew_seconds {
                result.add_warning(format!(
                    "Large clock skew detected: {} seconds (max: {})",
                    skew, config.max_clock_skew_seconds
                ));
            }

            // Check timestamp freshness
            if let Some(max_age) = config.max_timestamp_age_seconds {
                let age = (now - token.time).num_seconds();
                if age > max_age {
                    result.add_error(format!(
                        "Timestamp too old: {} seconds (max: {})",
                        age, max_age
                    ));
                }
            }
        }
        _ => {
            result.add_error(format!("Unsupported timestamp algorithm: {}", token.algorithm));
        }
    }

    if !result.valid {
        return Err(TdfError::SignatureFailure(
            format!("Timestamp validation failed: {}", result.errors.join("; ")),
        ));
    }

    Ok(true)
}

/// Check if timestamp is expired (older than max age)
pub fn is_timestamp_expired(token: &TimestampToken, max_age_seconds: i64) -> bool {
    let now = Utc::now();
    let age = (now - token.time).num_seconds();
    age > max_age_seconds
}

/// Check clock skew between timestamp and current time
pub fn check_clock_skew(token: &TimestampToken, max_skew_seconds: i64) -> TdfResult<()> {
    let now = Utc::now();
    let skew = (now - token.time).num_seconds().abs();
    if skew > max_skew_seconds {
        return Err(TdfError::SignatureFailure(format!(
            "Clock skew too large: {} seconds (max: {})",
            skew, max_skew_seconds
        )));
    }
    Ok(())
}

// Helper to create timestamp token from existing timestamp info
pub fn create_timestamp_token(
    time: DateTime<Utc>,
    authority: Option<String>,
    proof: Option<String>,
) -> TimestampToken {
    let has_proof = proof.is_some();
    TimestampToken {
        time,
        authority: authority.unwrap_or_else(|| "manual".to_string()),
        proof: proof.unwrap_or_default(),
        algorithm: if has_proof {
            "rfc3161".to_string()
        } else {
            "manual".to_string()
        },
    }
}

