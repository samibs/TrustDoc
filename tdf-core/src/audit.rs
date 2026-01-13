//! Audit logging infrastructure for TrustDoc Format
//!
//! Provides structured logging for security-critical events including:
//! - Document verification results
//! - Signature operations
//! - Revocation checks
//! - Security policy enforcement
//!
//! # Security Benefits
//! - Compliance: Maintains audit trail for regulatory requirements
//! - Forensics: Enables investigation of security incidents
//! - Monitoring: Supports real-time security alerting

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::sync::{Arc, Mutex};

/// Severity level for audit events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AuditSeverity {
    /// Informational events (successful operations)
    Info,
    /// Warning events (potential issues, policy violations)
    Warning,
    /// Error events (failures, security violations)
    Error,
    /// Critical events (security breaches, critical failures)
    Critical,
}

/// Type of audit event
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuditEventType {
    /// Document verification completed
    Verification,
    /// Document was created/signed
    DocumentCreated,
    /// Signature was verified
    SignatureVerified,
    /// Signature verification failed
    SignatureInvalid,
    /// Revocation check performed
    RevocationCheck,
    /// Key was found to be revoked
    KeyRevoked,
    /// Security policy was enforced
    PolicyEnforced,
    /// Security policy was violated
    PolicyViolation,
    /// Archive was opened/read
    ArchiveAccess,
    /// Timestamp was verified
    TimestampVerified,
    /// Timestamp verification failed
    TimestampInvalid,
    /// Integrity check passed
    IntegrityValid,
    /// Integrity check failed
    IntegrityInvalid,
    /// Size limit was exceeded
    SizeLimitExceeded,
    /// Path traversal attack detected
    PathTraversalDetected,
    /// Unknown or custom event type
    Custom(String),
}

/// Verification result for audit purposes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AuditResult {
    /// Operation succeeded
    Success,
    /// Operation failed
    Failure,
    /// Operation completed with warnings
    Warning,
    /// Operation was blocked by policy
    Blocked,
}

/// Information about a signer for audit purposes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditSignerInfo {
    /// Signer identifier
    pub id: String,
    /// Signer name (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether signature was valid
    pub valid: bool,
    /// Revocation status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked: Option<bool>,
    /// Signature algorithm used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
}

/// A single audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    /// Timestamp of the event
    pub timestamp: DateTime<Utc>,
    /// Severity level
    pub severity: AuditSeverity,
    /// Type of event
    pub event_type: AuditEventType,
    /// Overall result
    pub result: AuditResult,
    /// Document hash (hex-encoded, if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_hash: Option<String>,
    /// Document ID (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_id: Option<String>,
    /// Information about signers involved
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub signers: Vec<AuditSignerInfo>,
    /// Warning messages
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<String>,
    /// Error message (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Additional context/details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// Source of the event (e.g., component name)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Session or request identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

impl AuditEntry {
    /// Create a new audit entry
    pub fn new(event_type: AuditEventType, severity: AuditSeverity, result: AuditResult) -> Self {
        AuditEntry {
            timestamp: Utc::now(),
            severity,
            event_type,
            result,
            document_hash: None,
            document_id: None,
            signers: Vec::new(),
            warnings: Vec::new(),
            error: None,
            details: None,
            source: None,
            session_id: None,
        }
    }

    /// Builder method to set document hash
    pub fn with_document_hash(mut self, hash: impl Into<String>) -> Self {
        self.document_hash = Some(hash.into());
        self
    }

    /// Builder method to set document ID
    pub fn with_document_id(mut self, id: impl Into<String>) -> Self {
        self.document_id = Some(id.into());
        self
    }

    /// Builder method to add a signer
    pub fn with_signer(mut self, signer: AuditSignerInfo) -> Self {
        self.signers.push(signer);
        self
    }

    /// Builder method to add multiple signers
    pub fn with_signers(mut self, signers: Vec<AuditSignerInfo>) -> Self {
        self.signers = signers;
        self
    }

    /// Builder method to add a warning
    pub fn with_warning(mut self, warning: impl Into<String>) -> Self {
        self.warnings.push(warning.into());
        self
    }

    /// Builder method to set error message
    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }

    /// Builder method to set details
    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }

    /// Builder method to set source component
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Builder method to set session ID
    pub fn with_session_id(mut self, session_id: impl Into<String>) -> Self {
        self.session_id = Some(session_id.into());
        self
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Serialize to pretty JSON string
    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

/// Audit event for verification operations
#[derive(Debug, Clone)]
pub struct VerificationEvent {
    pub document_hash: String,
    pub document_id: Option<String>,
    pub result: AuditResult,
    pub signers: Vec<AuditSignerInfo>,
    pub warnings: Vec<String>,
    pub integrity_valid: bool,
    pub all_signatures_valid: bool,
    pub timestamp_valid: bool,
}

impl VerificationEvent {
    /// Convert to an AuditEntry
    pub fn to_audit_entry(&self) -> AuditEntry {
        let severity = match (&self.result, self.warnings.is_empty()) {
            (AuditResult::Success, true) => AuditSeverity::Info,
            (AuditResult::Success, false) => AuditSeverity::Warning,
            (AuditResult::Warning, _) => AuditSeverity::Warning,
            (AuditResult::Failure, _) => AuditSeverity::Error,
            (AuditResult::Blocked, _) => AuditSeverity::Critical,
        };

        let mut details_parts = Vec::new();
        if self.integrity_valid {
            details_parts.push("integrity=valid");
        } else {
            details_parts.push("integrity=invalid");
        }
        if self.all_signatures_valid {
            details_parts.push("signatures=valid");
        } else {
            details_parts.push("signatures=invalid");
        }
        if self.timestamp_valid {
            details_parts.push("timestamp=valid");
        } else {
            details_parts.push("timestamp=invalid");
        }

        AuditEntry::new(AuditEventType::Verification, severity, self.result.clone())
            .with_document_hash(&self.document_hash)
            .with_signers(self.signers.clone())
            .with_details(details_parts.join(", "))
    }
}

/// Trait for audit log output destinations
pub trait AuditOutput: Send + Sync {
    /// Write an audit entry to the output
    fn write(&self, entry: &AuditEntry) -> std::io::Result<()>;
}

/// Audit output that writes to a generic writer (file, stderr, etc.)
pub struct WriterOutput {
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
}

impl WriterOutput {
    /// Create a new writer output
    pub fn new(writer: Box<dyn Write + Send>) -> Self {
        WriterOutput {
            writer: Arc::new(Mutex::new(writer)),
        }
    }

    /// Create a writer that outputs to stderr
    pub fn stderr() -> Self {
        Self::new(Box::new(std::io::stderr()))
    }
}

impl AuditOutput for WriterOutput {
    fn write(&self, entry: &AuditEntry) -> std::io::Result<()> {
        let json = entry.to_json().map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, e)
        })?;

        let mut writer = self.writer.lock()
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "lock poisoned"))?;

        writeln!(writer, "{}", json)
    }
}

/// Audit output that collects entries in memory (useful for testing)
pub struct MemoryOutput {
    entries: Arc<Mutex<Vec<AuditEntry>>>,
}

impl MemoryOutput {
    /// Create a new memory output
    pub fn new() -> Self {
        MemoryOutput {
            entries: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Get all collected entries
    pub fn entries(&self) -> Vec<AuditEntry> {
        self.entries.lock().unwrap().clone()
    }

    /// Clear all collected entries
    pub fn clear(&self) {
        self.entries.lock().unwrap().clear();
    }
}

impl Default for MemoryOutput {
    fn default() -> Self {
        Self::new()
    }
}

impl AuditOutput for MemoryOutput {
    fn write(&self, entry: &AuditEntry) -> std::io::Result<()> {
        let mut entries = self.entries.lock()
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "lock poisoned"))?;
        entries.push(entry.clone());
        Ok(())
    }
}

/// No-op audit output (discards all entries)
pub struct NullOutput;

impl AuditOutput for NullOutput {
    fn write(&self, _entry: &AuditEntry) -> std::io::Result<()> {
        Ok(())
    }
}

/// Main audit logger
pub struct AuditLogger {
    outputs: Vec<Box<dyn AuditOutput>>,
    source: Option<String>,
    session_id: Option<String>,
}

impl AuditLogger {
    /// Create a new audit logger with no outputs
    pub fn new() -> Self {
        AuditLogger {
            outputs: Vec::new(),
            source: None,
            session_id: None,
        }
    }

    /// Create an audit logger that discards all events
    pub fn null() -> Self {
        AuditLogger {
            outputs: vec![Box::new(NullOutput)],
            source: None,
            session_id: None,
        }
    }

    /// Add an output destination
    pub fn add_output(&mut self, output: impl AuditOutput + 'static) {
        self.outputs.push(Box::new(output));
    }

    /// Set the source component name
    pub fn set_source(&mut self, source: impl Into<String>) {
        self.source = Some(source.into());
    }

    /// Set the session ID
    pub fn set_session_id(&mut self, session_id: impl Into<String>) {
        self.session_id = Some(session_id.into());
    }

    /// Log a raw audit entry
    pub fn log(&self, mut entry: AuditEntry) {
        // Add source and session_id if configured
        if entry.source.is_none() {
            entry.source = self.source.clone();
        }
        if entry.session_id.is_none() {
            entry.session_id = self.session_id.clone();
        }

        for output in &self.outputs {
            if let Err(e) = output.write(&entry) {
                eprintln!("Audit log error: {}", e);
            }
        }
    }

    /// Log a verification event
    pub fn log_verification(&self, event: VerificationEvent) {
        self.log(event.to_audit_entry());
    }

    /// Log a simple info event
    pub fn log_info(&self, event_type: AuditEventType, message: impl Into<String>) {
        self.log(
            AuditEntry::new(event_type, AuditSeverity::Info, AuditResult::Success)
                .with_details(message)
        );
    }

    /// Log a warning event
    pub fn log_warning(&self, event_type: AuditEventType, message: impl Into<String>) {
        self.log(
            AuditEntry::new(event_type, AuditSeverity::Warning, AuditResult::Warning)
                .with_warning(message)
        );
    }

    /// Log an error event
    pub fn log_error(&self, event_type: AuditEventType, error: impl Into<String>) {
        self.log(
            AuditEntry::new(event_type, AuditSeverity::Error, AuditResult::Failure)
                .with_error(error)
        );
    }

    /// Log a critical security event
    pub fn log_critical(&self, event_type: AuditEventType, error: impl Into<String>) {
        self.log(
            AuditEntry::new(event_type, AuditSeverity::Critical, AuditResult::Blocked)
                .with_error(error)
        );
    }

    /// Log a signature verification result
    pub fn log_signature_verification(
        &self,
        document_hash: &str,
        signer: AuditSignerInfo,
        valid: bool,
    ) {
        let (event_type, severity, result) = if valid {
            (AuditEventType::SignatureVerified, AuditSeverity::Info, AuditResult::Success)
        } else {
            (AuditEventType::SignatureInvalid, AuditSeverity::Error, AuditResult::Failure)
        };

        self.log(
            AuditEntry::new(event_type, severity, result)
                .with_document_hash(document_hash)
                .with_signer(signer)
        );
    }

    /// Log a revocation check result
    pub fn log_revocation_check(
        &self,
        signer_id: &str,
        revoked: bool,
        reason: Option<&str>,
    ) {
        let (event_type, severity, result) = if revoked {
            (AuditEventType::KeyRevoked, AuditSeverity::Warning, AuditResult::Warning)
        } else {
            (AuditEventType::RevocationCheck, AuditSeverity::Info, AuditResult::Success)
        };

        let mut entry = AuditEntry::new(event_type, severity, result)
            .with_details(format!("signer_id={}", signer_id));

        if let Some(r) = reason {
            entry = entry.with_warning(format!("revocation_reason={}", r));
        }

        self.log(entry);
    }

    /// Log a policy violation
    pub fn log_policy_violation(&self, policy: &str, details: impl Into<String>) {
        self.log(
            AuditEntry::new(
                AuditEventType::PolicyViolation,
                AuditSeverity::Warning,
                AuditResult::Blocked
            )
            .with_details(format!("policy={}, {}", policy, details.into()))
        );
    }

    /// Log a size limit exceeded event
    pub fn log_size_exceeded(&self, actual: u64, limit: u64) {
        self.log(
            AuditEntry::new(
                AuditEventType::SizeLimitExceeded,
                AuditSeverity::Warning,
                AuditResult::Blocked
            )
            .with_details(format!("size={}, limit={}", actual, limit))
        );
    }

    /// Log a path traversal attack detection
    pub fn log_path_traversal(&self, path: &str) {
        self.log(
            AuditEntry::new(
                AuditEventType::PathTraversalDetected,
                AuditSeverity::Critical,
                AuditResult::Blocked
            )
            .with_details(format!("malicious_path={}", path))
        );
    }
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_entry_creation() {
        let entry = AuditEntry::new(
            AuditEventType::Verification,
            AuditSeverity::Info,
            AuditResult::Success
        );

        assert_eq!(entry.event_type, AuditEventType::Verification);
        assert_eq!(entry.severity, AuditSeverity::Info);
        assert_eq!(entry.result, AuditResult::Success);
    }

    #[test]
    fn test_audit_entry_builder() {
        let entry = AuditEntry::new(
            AuditEventType::SignatureVerified,
            AuditSeverity::Info,
            AuditResult::Success
        )
        .with_document_hash("abc123")
        .with_document_id("doc-001")
        .with_signer(AuditSignerInfo {
            id: "signer-1".to_string(),
            name: Some("Test Signer".to_string()),
            valid: true,
            revoked: Some(false),
            algorithm: Some("Ed25519".to_string()),
        })
        .with_details("Test details")
        .with_source("test-component");

        assert_eq!(entry.document_hash, Some("abc123".to_string()));
        assert_eq!(entry.document_id, Some("doc-001".to_string()));
        assert_eq!(entry.signers.len(), 1);
        assert_eq!(entry.details, Some("Test details".to_string()));
        assert_eq!(entry.source, Some("test-component".to_string()));
    }

    #[test]
    fn test_audit_entry_json_serialization() {
        let entry = AuditEntry::new(
            AuditEventType::Verification,
            AuditSeverity::Info,
            AuditResult::Success
        )
        .with_document_hash("abc123");

        let json = entry.to_json().unwrap();
        assert!(json.contains("\"event_type\":\"VERIFICATION\""));
        assert!(json.contains("\"severity\":\"INFO\""));
        assert!(json.contains("\"result\":\"SUCCESS\""));
        assert!(json.contains("\"document_hash\":\"abc123\""));
    }

    #[test]
    fn test_memory_output() {
        let output = MemoryOutput::new();

        let entry = AuditEntry::new(
            AuditEventType::Verification,
            AuditSeverity::Info,
            AuditResult::Success
        );

        output.write(&entry).unwrap();
        output.write(&entry).unwrap();

        let entries = output.entries();
        assert_eq!(entries.len(), 2);

        output.clear();
        assert_eq!(output.entries().len(), 0);
    }

    #[test]
    fn test_audit_logger() {
        let output = Arc::new(MemoryOutput::new());
        let output_clone = output.clone();

        let mut logger = AuditLogger::new();
        logger.add_output(MemoryOutput::new()); // Can't use Arc directly, need to test differently
        logger.set_source("test-component");
        logger.set_session_id("session-123");

        logger.log_info(AuditEventType::ArchiveAccess, "Test message");

        // The entry was logged to the first output only
        // We'd need to restructure to test this properly with shared state
    }

    #[test]
    fn test_verification_event() {
        let event = VerificationEvent {
            document_hash: "abc123".to_string(),
            document_id: Some("doc-001".to_string()),
            result: AuditResult::Success,
            signers: vec![
                AuditSignerInfo {
                    id: "signer-1".to_string(),
                    name: Some("Test".to_string()),
                    valid: true,
                    revoked: None,
                    algorithm: None,
                },
            ],
            warnings: vec![],
            integrity_valid: true,
            all_signatures_valid: true,
            timestamp_valid: true,
        };

        let entry = event.to_audit_entry();
        assert_eq!(entry.event_type, AuditEventType::Verification);
        assert_eq!(entry.severity, AuditSeverity::Info);
        assert!(entry.details.unwrap().contains("integrity=valid"));
    }

    #[test]
    fn test_verification_event_with_failures() {
        let event = VerificationEvent {
            document_hash: "abc123".to_string(),
            document_id: None,
            result: AuditResult::Failure,
            signers: vec![],
            warnings: vec!["Test warning".to_string()],
            integrity_valid: false,
            all_signatures_valid: false,
            timestamp_valid: true,
        };

        let entry = event.to_audit_entry();
        assert_eq!(entry.severity, AuditSeverity::Error);
        assert!(entry.details.unwrap().contains("integrity=invalid"));
    }

    #[test]
    fn test_custom_event_type() {
        let entry = AuditEntry::new(
            AuditEventType::Custom("MY_CUSTOM_EVENT".to_string()),
            AuditSeverity::Info,
            AuditResult::Success
        );

        let json = entry.to_json().unwrap();
        assert!(json.contains("MY_CUSTOM_EVENT"));
    }

    #[test]
    fn test_logger_helpers() {
        let output = MemoryOutput::new();
        let mut logger = AuditLogger::new();

        // Since we can't share the output reference easily,
        // we'll just verify the logger methods don't panic
        logger.log_warning(AuditEventType::PolicyViolation, "Test warning");
        logger.log_error(AuditEventType::IntegrityInvalid, "Test error");
        logger.log_critical(AuditEventType::PathTraversalDetected, "Attack detected");

        logger.log_signature_verification(
            "hash123",
            AuditSignerInfo {
                id: "signer".to_string(),
                name: None,
                valid: true,
                revoked: None,
                algorithm: None,
            },
            true
        );

        logger.log_revocation_check("signer-id", true, Some("key_compromise"));
        logger.log_policy_violation("size_limit", "exceeded by 100KB");
        logger.log_size_exceeded(1000, 500);
        logger.log_path_traversal("../../../etc/passwd");
    }

    #[test]
    fn test_null_logger() {
        let logger = AuditLogger::null();

        // Should not panic
        logger.log_info(AuditEventType::Verification, "Discarded");
        logger.log_error(AuditEventType::IntegrityInvalid, "Discarded");
    }
}
