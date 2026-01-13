# Security Modules Documentation

Complete documentation for all security modules in the TDF core library.

## Table of Contents

1. [Secure Key Management](#secure-key-management)
2. [Cryptographic Utilities](#cryptographic-utilities)
3. [Secure Random Generation](#secure-random-generation)
4. [Audit Logging](#audit-logging)
5. [Error Sanitization](#error-sanitization)
6. [Integer Safety](#integer-safety)
7. [Resource Limits](#resource-limits)
8. [Secure I/O](#secure-io)
9. [Signer Whitelist](#signer-whitelist)

---

## Secure Key Management

**Module**: `tdf_core::secure_key`  
**Purpose**: Secure key storage with automatic zeroization  
**Vulnerabilities Addressed**: CVE-TDF-026, CVE-TDF-025

### Overview

The `secure_key` module provides secure key handling that automatically zeroizes sensitive key material when it goes out of scope, preventing key material from remaining in memory after use.

### Key Features

- **Automatic Zeroization**: Keys are automatically cleared from memory on drop
- **Memory Safety**: Prevents key material from persisting in memory
- **Zero Unsafe Code**: All operations use safe Rust
- **Drop Protection**: Implements `ZeroizeOnDrop` trait

### API Reference

#### `SecureKey`

Main secure key container.

```rust
use tdf_core::secure_key::SecureKey;

// Create a new secure key
let key = SecureKey::new(vec![1, 2, 3, 4, 5]);

// Access key bytes (read-only)
let bytes = key.as_bytes();

// Get key length
let len = key.len();

// Explicitly zeroize (optional, happens automatically on drop)
key.zeroize();
```

#### `SecureDerivedKey`

Wrapper for derived keys.

```rust
use tdf_core::secure_key::{SecureKey, SecureDerivedKey};

let base_key = SecureKey::new(vec![1, 2, 3, 4, 5]);
let derived = SecureDerivedKey::new(base_key);

// Access derived key bytes
let bytes = derived.as_bytes();
```

### Security Considerations

- Keys are zeroized when they go out of scope
- Manual zeroization is available but not required
- Cloning creates a new key that will also be zeroized
- Keys should be used in the smallest scope possible

### Example Usage

```rust
use tdf_core::secure_key::SecureKey;

fn process_sensitive_data() -> Result<(), TdfError> {
    // Key is created and will be zeroized when function returns
    let key = SecureKey::new(generate_key_material()?);
    
    // Use key for cryptographic operations
    let encrypted = encrypt_data(&data, key.as_bytes())?;
    
    // Key is automatically zeroized here
    Ok(encrypted)
}
```

---

## Cryptographic Utilities

**Module**: `tdf_core::crypto_utils`  
**Purpose**: Constant-time cryptographic operations  
**Vulnerabilities Addressed**: CVE-TDF-024

### Overview

The `crypto_utils` module provides constant-time comparison functions to prevent timing side-channel attacks. All hash and signature comparisons use constant-time operations.

### Key Features

- **Constant-Time Comparisons**: All operations complete in consistent time
- **Timing Attack Resistance**: Prevents side-channel attacks via timing analysis
- **Multiple Comparison Methods**: Byte slices, vectors, and hex strings
- **Hash Verification**: Secure root hash and signature comparison

### API Reference

#### Constant-Time Comparison

```rust
use tdf_core::crypto_utils::ct_eq;

let hash1 = [0u8; 32];
let hash2 = [0u8; 32];

// Constant-time comparison
if ct_eq(&hash1, &hash2) {
    println!("Hashes match");
}
```

#### Hex String Comparison

```rust
use tdf_core::crypto_utils::ct_eq_hex;

let hex1 = "a1b2c3d4e5f67890";
let hex2 = "a1b2c3d4e5f67890";

match ct_eq_hex(hex1, hex2)? {
    true => println!("Hex strings match"),
    false => println!("Hex strings differ"),
}
```

#### Hash Verification

```rust
use tdf_core::crypto_utils::verify_root_hash;

let computed = compute_merkle_root(&components)?;
let expected = get_expected_root()?;

if verify_root_hash(&computed, &expected) {
    println!("Root hash verified");
}
```

### Security Considerations

- Always use constant-time comparisons for cryptographic values
- Never use standard `==` for comparing hashes or signatures
- Timing attacks can reveal information about secret values
- Constant-time operations prevent information leakage

### Example Usage

```rust
use tdf_core::crypto_utils::{ct_eq, verify_root_hash};

fn verify_document_integrity(computed: &[u8], expected: &[u8]) -> bool {
    // Use constant-time comparison
    verify_root_hash(computed, expected)
}
```

---

## Secure Random Generation

**Module**: `tdf_core::secure_random`  
**Purpose**: Cryptographically secure random number generation  
**Vulnerabilities Addressed**: CVE-TDF-025, Vuln #1, #25, #37

### Overview

The `secure_random` module provides cryptographically secure random number generation using OS-provided CSPRNG with defense-in-depth entropy mixing.

### Key Features

- **OS CSPRNG**: Uses operating system cryptographically secure RNG
- **Defense-in-Depth**: Multiple entropy sources with SHA-256 mixing
- **Token Generation**: Secure tokens for sessions and authentication
- **Nonce Generation**: Secure nonces for encryption operations
- **UUID Generation**: Secure UUID v4 generation

### API Reference

#### Generate Random Bytes

```rust
use tdf_core::secure_random::generate_secure_bytes;

// Generate 32 bytes of secure random data
let random_bytes = generate_secure_bytes(32)?;
```

#### Generate Secure Token

```rust
use tdf_core::secure_random::generate_secure_token;

// Generate a 32-byte secure token
let token = generate_secure_token()?;
```

#### Generate Secure Nonce

```rust
use tdf_core::secure_random::generate_secure_nonce;

// Generate a 12-byte nonce for GCM encryption
let nonce = generate_secure_nonce()?;
```

#### Generate Secure UUID

```rust
use tdf_core::secure_random::generate_secure_uuid;

// Generate a UUID v4
let uuid = generate_secure_uuid()?;
```

#### Generate Session ID

```rust
use tdf_core::secure_random::generate_secure_session_id;

// Generate a 64-bit secure session ID
let session_id = generate_secure_session_id()?;
```

### Security Considerations

- Always use secure random generation for cryptographic operations
- Never use predictable random number generators
- Entropy mixing provides defense-in-depth if OS RNG is compromised
- Tokens and nonces must be cryptographically unpredictable

### Example Usage

```rust
use tdf_core::secure_random::{generate_secure_token, generate_secure_nonce};

fn create_secure_session() -> Result<Session, TdfError> {
    let token = generate_secure_token()?;
    let nonce = generate_secure_nonce()?;
    
    // Use token and nonce for secure operations
    Ok(Session { token, nonce })
}
```

---

## Audit Logging

**Module**: `tdf_core::audit`  
**Purpose**: Structured logging for security events  
**Vulnerabilities Addressed**: Compliance requirements, forensics

### Overview

The `audit` module provides structured logging for security-critical events including document verification, signature operations, revocation checks, and security policy enforcement.

### Key Features

- **Structured Logging**: JSON-formatted audit entries
- **Multiple Outputs**: File, memory, stderr, or custom destinations
- **Event Types**: Comprehensive event type system
- **Severity Levels**: Info, Warning, Error, Critical
- **Compliance Ready**: Supports regulatory audit requirements

### API Reference

#### Create Audit Logger

```rust
use tdf_core::audit::{AuditLogger, MemoryOutput};

let mut logger = AuditLogger::new();
logger.add_output(MemoryOutput::new());
logger.set_source("document-verifier");
```

#### Log Verification Event

```rust
use tdf_core::audit::{AuditLogger, VerificationEvent, AuditResult, AuditSignerInfo};

let event = VerificationEvent {
    document_hash: "abc123".to_string(),
    document_id: Some("doc-001".to_string()),
    result: AuditResult::Success,
    signers: vec![/* signer info */],
    warnings: vec![],
    integrity_valid: true,
    all_signatures_valid: true,
    timestamp_valid: true,
};

logger.log_verification(event);
```

#### Log Security Events

```rust
use tdf_core::audit::{AuditLogger, AuditEventType};

// Log info event
logger.log_info(AuditEventType::Verification, "Document verified successfully");

// Log warning
logger.log_warning(AuditEventType::PolicyViolation, "Size limit exceeded");

// Log error
logger.log_error(AuditEventType::IntegrityInvalid, "Integrity check failed");

// Log critical event
logger.log_critical(AuditEventType::PathTraversalDetected, "Attack detected");
```

### Security Considerations

- Audit logs should be stored securely
- Logs should be tamper-evident
- Sensitive information should not be logged
- Logs should be retained per compliance requirements

### Example Usage

```rust
use tdf_core::audit::{AuditLogger, WriterOutput, AuditEventType};
use std::fs::File;

fn setup_audit_logging() -> Result<AuditLogger, TdfError> {
    let mut logger = AuditLogger::new();
    
    // Log to file
    let file = File::create("audit.log")?;
    logger.add_output(WriterOutput::new(Box::new(file)));
    
    // Log to stderr for development
    logger.add_output(WriterOutput::stderr());
    
    logger.set_source("tdf-verifier");
    Ok(logger)
}
```

---

## Error Sanitization

**Module**: `tdf_core::error_sanitization`  
**Purpose**: Prevent information leakage through error messages  
**Vulnerabilities Addressed**: Vuln #11, #12

### Overview

The `error_sanitization` module provides utilities to sanitize error messages, preventing information leakage that could be used for attacks or social engineering.

### Key Features

- **Information Leakage Prevention**: Removes sensitive data from errors
- **Path Sanitization**: Strips file paths and system information
- **Generic Error Codes**: Provides safe error codes for logging
- **Social Engineering Protection**: Prevents information gathering via errors

### API Reference

#### Sanitize Error

```rust
use tdf_core::error_sanitization::sanitize_error;
use tdf_core::error::TdfError;

let error = TdfError::InvalidDocument(
    "File /home/user/secret.tdf not found".to_string()
);

// Sanitize error message
let sanitized = sanitize_error(&error);
// Returns: "Invalid document" (path removed)
```

#### Get Error Code

```rust
use tdf_core::error_sanitization::error_code;
use tdf_core::error::TdfError;

let error = TdfError::Io(std::io::Error::new(
    std::io::ErrorKind::NotFound,
    "File not found"
));

// Get generic error code
let code = error_code(&error);
// Returns: "ERR_IO"
```

### Security Considerations

- Always sanitize errors before displaying to users
- Use error codes for logging instead of full error messages
- Never expose file paths, memory addresses, or stack traces
- Sanitized errors should still be useful for debugging

### Example Usage

```rust
use tdf_core::error_sanitization::{sanitize_error, error_code};

fn handle_error(error: &TdfError) {
    // Log full error internally
    eprintln!("Internal error: {:?}", error);
    
    // Show sanitized error to user
    let user_message = sanitize_error(error);
    show_error_to_user(&user_message);
    
    // Use error code for monitoring
    let code = error_code(error);
    send_to_monitoring_system(code);
}
```

---

## Integer Safety

**Module**: `tdf_core::integer_safety`  
**Purpose**: Prevent integer overflow attacks  
**Vulnerabilities Addressed**: CVE-TDF-021, CVE-TDF-008

### Overview

The `integer_safety` module provides safe integer arithmetic operations that prevent integer overflow attacks, which can lead to buffer overflows or memory corruption.

### Key Features

- **Overflow Protection**: Checked arithmetic prevents integer overflow
- **Safe Type Conversions**: usize/u64 conversions with bounds checking
- **Frame Size Calculation**: Safe calculation of encrypted frame sizes
- **Memory Safety**: Prevents buffer overflows from integer attacks

### API Reference

#### Checked Addition

```rust
use tdf_core::integer_safety::checked_add;

// Safe addition
let sum = checked_add(100, 200)?; // Ok(300)

// Overflow protection
let overflow = checked_add(u64::MAX, 1); // Err(IntegerOverflow)
```

#### Checked Multiplication

```rust
use tdf_core::integer_safety::checked_mul;

let product = checked_mul(100, 200)?; // Ok(20000)
```

#### Checked Sum

```rust
use tdf_core::integer_safety::checked_sum;

let sizes = vec![100u64, 200, 300];
let total = checked_sum(sizes.iter().copied())?; // Ok(600)
```

#### Calculate Frame Size

```rust
use tdf_core::integer_safety::calculate_frame_size;

let frame_size = calculate_frame_size(1000, 64, Some(16))?; // Ok(1080)
```

### Security Considerations

- Always use checked arithmetic for size calculations
- Integer overflow can lead to buffer overflows
- Type conversions must check bounds
- Frame size calculations are security-critical

### Example Usage

```rust
use tdf_core::integer_safety::{checked_add, calculate_frame_size};

fn compute_total_size(components: &[u64]) -> Result<u64, TdfError> {
    let base_size = checked_sum(components.iter().copied())?;
    let frame_size = calculate_frame_size(base_size, 64, Some(16))?;
    Ok(frame_size)
}
```

---

## Resource Limits

**Module**: `tdf_core::resource_limits`  
**Purpose**: Prevent resource exhaustion attacks  
**Vulnerabilities Addressed**: Vuln #7, #9, #10

### Overview

The `resource_limits` module provides utilities to prevent resource exhaustion attacks through rate limiting, circuit breakers, and resource budgets.

### Key Features

- **Circuit Breakers**: Prevents cascade failures from resource exhaustion
- **Rate Limiting**: Token bucket algorithm for request throttling
- **Resource Budgets**: CPU, memory, and operation tracking
- **DoS Protection**: Prevents denial-of-service attacks

### API Reference

#### Circuit Breaker

```rust
use tdf_core::resource_limits::CircuitBreaker;
use std::time::Duration;

let breaker = CircuitBreaker::new(
    3,                              // failure threshold
    Duration::from_secs(1),          // timeout
    Duration::from_millis(500),     // half-open timeout
);

// Check if request is allowed
breaker.check()?;

// Record success
breaker.record_success();

// Record failure
breaker.record_failure();
```

#### Rate Limiter

```rust
use tdf_core::resource_limits::RateLimiter;

let limiter = RateLimiter::new(10, 5); // capacity: 10, refill_rate: 5/sec

// Try to acquire token
limiter.try_acquire()?;
```

#### Resource Budget

```rust
use tdf_core::resource_limits::ResourceBudget;

let budget = ResourceBudget::new(
    1000,           // max CPU time (ms)
    1024 * 1024,    // max memory (bytes)
    100,            // max operations
);

// Check budget
budget.check_budget()?;

// Record usage
budget.record_cpu_time(100);
budget.record_memory(1024);
budget.record_operation();
```

### Security Considerations

- Resource limits prevent DoS attacks
- Circuit breakers prevent cascade failures
- Rate limiting protects against abuse
- Resource budgets protect field devices

### Example Usage

```rust
use tdf_core::resource_limits::{CircuitBreaker, RateLimiter, ResourceBudget};
use std::time::Duration;

fn setup_protection() -> (CircuitBreaker, RateLimiter, ResourceBudget) {
    let breaker = CircuitBreaker::new(5, Duration::from_secs(10), Duration::from_secs(1));
    let limiter = RateLimiter::new(100, 10);
    let budget = ResourceBudget::new(5000, 10 * 1024 * 1024, 1000);
    
    (breaker, limiter, budget)
}
```

---

## Secure I/O

**Module**: `tdf_core::io`  
**Purpose**: Secure I/O with bounded reading  
**Vulnerabilities Addressed**: CVE-TDF-005, CVE-TDF-009, CVE-TDF-032

### Overview

The `io` module provides secure I/O utilities with bounded reading to prevent memory exhaustion attacks and deserialization attacks.

### Key Features

- **Bounded Readers**: Enforces maximum read limits
- **Deserialization Security**: Size and depth limits for CBOR/JSON
- **ZIP Bomb Protection**: Prevents memory exhaustion from malicious archives
- **Depth Limits**: Prevents stack overflow from deeply nested structures

### API Reference

#### Bounded Reader

```rust
use tdf_core::io::{BoundedReader, read_bounded};
use std::io::Cursor;

let data = b"Hello, World!";
let cursor = Cursor::new(data);
let mut reader = BoundedReader::new(cursor, 100);

let mut buf = Vec::new();
reader.read_to_end(&mut buf)?;
```

#### Read with Limit

```rust
use tdf_core::io::read_bounded;
use std::io::Cursor;

let cursor = Cursor::new(data);
let content = read_bounded(cursor, 1024 * 1024)?; // 1MB limit
```

#### Deserialize with Bounds

```rust
use tdf_core::io::{deserialize_cbor_bounded, MAX_CBOR_SIZE};

let parsed: MyStruct = deserialize_cbor_bounded(&cbor_data, MAX_CBOR_SIZE)?;
```

### Security Considerations

- Always use bounded readers for untrusted input
- Set appropriate size limits for deserialization
- Depth limits prevent stack overflow attacks
- ZIP bombs can cause memory exhaustion

### Example Usage

```rust
use tdf_core::io::{read_bounded, deserialize_cbor_bounded, MAX_CBOR_SIZE};

fn read_archive_file(reader: impl Read) -> Result<MyStruct, TdfError> {
    // Read with size limit
    let data = read_bounded(reader, 10 * 1024 * 1024)?; // 10MB limit
    
    // Deserialize with bounds
    let parsed: MyStruct = deserialize_cbor_bounded(&data, MAX_CBOR_SIZE)?;
    Ok(parsed)
}
```

---

## Signer Whitelist

**Module**: `tdf_core::whitelist`  
**Purpose**: Trusted signer management  
**Vulnerabilities Addressed**: CVE-TDF-024

### Overview

The `whitelist` module allows organizations to define trusted signers and validate signatures against them, with optional public key binding for enhanced security.

### Key Features

- **Trusted Signer Management**: Organization-defined signer lists
- **Public Key Binding**: Validates signer keys against whitelist
- **Role-Based Authorization**: Assign roles to trusted signers
- **Strict Mode**: Requires key binding for enhanced security

### API Reference

#### Create Whitelist

```rust
use tdf_core::whitelist::{SignerWhitelist, TrustedSigner};

let mut whitelist = SignerWhitelist::new("ACME Corp".to_string());

// Add trusted signer
whitelist.add_signer(TrustedSigner::new(
    "did:web:cfo.acme.com".to_string(),
    "CFO Jane Smith".to_string(),
));
```

#### Validate Signer

```rust
use tdf_core::whitelist::SignerWhitelist;
use ed25519_dalek::VerifyingKey;

let result = whitelist.validate_signer_key("did:web:cfo.acme.com", &verifying_key);

match result {
    WhitelistValidationResult::Trusted { signer_name, roles } => {
        println!("Signer {} is trusted", signer_name);
    }
    WhitelistValidationResult::KeyMismatch { expected_key, actual_key } => {
        println!("Key mismatch detected");
    }
    WhitelistValidationResult::NotFound => {
        println!("Signer not in whitelist");
    }
    _ => {}
}
```

#### Load from JSON

```rust
use tdf_core::whitelist::SignerWhitelist;

let json_data = std::fs::read("trusted-signers.json")?;
let whitelist = SignerWhitelist::from_json(&json_data)?;
```

### Security Considerations

- Whitelists should be signed and verified
- Public key binding prevents impersonation
- Strict mode requires key binding
- Whitelists should be regularly updated

### Example Usage

```rust
use tdf_core::whitelist::{SignerWhitelist, TrustedSigner};
use ed25519_dalek::VerifyingKey;

fn setup_whitelist() -> Result<SignerWhitelist, TdfError> {
    let mut whitelist = SignerWhitelist::new("My Organization".to_string());
    
    // Add signer with public key binding
    let verifying_key = get_verifying_key()?;
    whitelist.add_signer(TrustedSigner::with_key(
        "did:web:signer.example.com".to_string(),
        "Trusted Signer".to_string(),
        &verifying_key,
    ));
    
    Ok(whitelist)
}
```

---

## Best Practices

### General Security Guidelines

1. **Always use security modules** for cryptographic operations
2. **Sanitize errors** before displaying to users
3. **Use bounded I/O** for all untrusted input
4. **Check integer operations** for overflow
5. **Log security events** for audit and forensics
6. **Use constant-time operations** for comparisons
7. **Zeroize sensitive data** when no longer needed
8. **Set resource limits** to prevent DoS attacks

### Module-Specific Guidelines

- **Secure Key**: Use in smallest scope possible
- **Crypto Utils**: Always use constant-time comparisons
- **Secure Random**: Never use predictable RNG
- **Audit**: Store logs securely and tamper-evidently
- **Error Sanitization**: Sanitize before user display
- **Integer Safety**: Use checked arithmetic for sizes
- **Resource Limits**: Set appropriate limits per use case
- **Secure I/O**: Always set size and depth limits
- **Whitelist**: Verify whitelist integrity

---

**Last Updated**: 2026-01-11  
**Version**: TDF v7.0+
