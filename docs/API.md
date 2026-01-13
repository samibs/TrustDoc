# TDF API Reference

Complete API reference for all TDF components.

## Table of Contents

1. [Rust Core Library (`tdf-core`)](#rust-core-library-tdf-core)
2. [CLI Tool (`tdf`)](#cli-tool-tdf)
3. [TypeScript SDK (`tdf-ts`)](#typescript-sdk-tdf-ts)
4. [WASM Bindings (`tdf-wasm`)](#wasm-bindings-tdf-wasm)
5. [Error Types](#error-types)
6. [Content Primitives](#content-primitives)

## Rust Core Library (`tdf-core`)

### Document Module

#### `Document`

Main document structure.

```rust
pub struct Document {
    pub manifest: Manifest,
    pub content: DocumentContent,
    pub styles: String,
    pub layout: Option<Layout>,
    pub data: Option<serde_json::Value>,
}
```

**Methods**:

```rust
impl Document {
    /// Create a new document
    pub fn new(
        title: String,
        language: String,
        content: DocumentContent,
        styles: String,
    ) -> Self;

    /// Validate document structure
    pub fn validate(&self) -> TdfResult<()>;
}
```

#### `Manifest`

Document metadata and integrity information.

```rust
pub struct Manifest {
    pub schema_version: String,
    pub document: DocumentMeta,
    pub authors: Vec<Author>,
    pub classification: Option<Classification>,
    pub integrity: IntegrityBlock,
}
```

#### `DocumentMeta`

Document-level metadata.

```rust
pub struct DocumentMeta {
    pub id: String,
    pub title: String,
    pub language: String,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
}
```

### Archive Module

#### `ArchiveBuilder`

Constructs TDF archive files.

```rust
pub struct ArchiveBuilder {
    // ...
}

impl ArchiveBuilder {
    /// Create a new builder
    pub fn new(document: Document) -> Self;

    /// Set security configuration
    pub fn with_security_config(self, config: SecurityConfig) -> Self;

    /// Set revocation list
    pub fn with_revocation_list(self, list: RevocationList) -> Self;

    /// Add an asset (image, font, etc.)
    pub fn add_asset(&mut self, path: String, data: Vec<u8>);

    /// Build archive without timestamp
    pub fn build(
        &mut self,
        output_path: &Path,
        signing_key: Option<&SigningKey>,
        signer_id: Option<String>,
        signer_name: Option<String>,
    ) -> TdfResult<()>;

    /// Build archive with timestamp
    pub fn build_with_timestamp(
        &mut self,
        output_path: &Path,
        ed25519_key: Option<&SigningKey>,
        secp256k1_key: Option<&Secp256k1SigningKey>,
        signer_id: Option<String>,
        signer_name: Option<String>,
        signature_algorithm: Option<SignatureAlgorithm>,
        timestamp_provider: Option<&dyn TimestampProvider>,
    ) -> TdfResult<()>;
}
```

**Example**:

```rust
use tdf_core::archive::ArchiveBuilder;
use ed25519_dalek::SigningKey;

let mut builder = ArchiveBuilder::new(document);
builder.add_asset("assets/logo.png".to_string(), logo_data);
builder.build(
    &Path::new("output.tdf"),
    Some(&signing_key),
    Some("did:web:example.com".to_string()),
    Some("Signer Name".to_string()),
)?;
```

#### `ArchiveReader`

Reads and verifies TDF archives.

```rust
pub struct ArchiveReader;

impl ArchiveReader {
    /// Read document from archive
    pub fn read(path: &Path) -> TdfResult<Document>;

    /// Verify document integrity and signatures
    pub fn verify(path: &Path) -> TdfResult<VerificationReport>;

    /// Verify with security configuration
    pub fn verify_with_config(
        path: &Path,
        config: &SecurityConfig,
    ) -> TdfResult<VerificationReport>;

    /// Verify with revocation list
    pub fn verify_with_revocation(
        path: &Path,
        revocation_manager: &RevocationManager,
    ) -> TdfResult<VerificationReport>;
}
```

**Example**:

```rust
use tdf_core::archive::ArchiveReader;

let report = ArchiveReader::verify(&Path::new("document.tdf"))?;
if report.integrity_valid {
    println!("Document is valid");
    println!("Signatures: {}", report.signature_count);
}
```

#### `VerificationReport`

Result of document verification.

```rust
pub struct VerificationReport {
    pub integrity_valid: bool,
    pub root_hash: String,
    pub signature_count: usize,
    pub valid_signatures: usize,
    pub invalid_signatures: usize,
    pub revoked_signatures: usize,
    pub timestamp_warnings: Vec<String>,
}
```

### Merkle Tree Module

#### `MerkleTree`

Merkle tree for integrity verification.

```rust
pub struct MerkleTree {
    // ...
}

impl MerkleTree {
    /// Create a new Merkle tree
    pub fn new(algorithm: HashAlgorithm) -> Self;

    /// Compute root hash from components
    pub fn compute_root(&mut self, components: &HashMap<String, Vec<u8>>) -> TdfResult<Vec<u8>>;

    /// Verify components against root hash
    pub fn verify(
        &self,
        components: &HashMap<String, Vec<u8>>,
        root_hash: &[u8],
    ) -> TdfResult<bool>;
}
```

**Example**:

```rust
use tdf_core::merkle::{MerkleTree, HashAlgorithm};
use std::collections::HashMap;

let mut tree = MerkleTree::new(HashAlgorithm::Sha256);
let mut components = HashMap::new();
components.insert("file1".to_string(), data1);
components.insert("file2".to_string(), data2);

let root_hash = tree.compute_root(&components)?;
let is_valid = tree.verify(&components, &root_hash)?;
```

#### `HashAlgorithm`

Supported hash algorithms.

```rust
pub enum HashAlgorithm {
    Sha256,
    Blake3,
}
```

### Signature Module

#### `SignatureManager`

Manages cryptographic signatures.

```rust
pub struct SignatureManager;

impl SignatureManager {
    /// Create signature block
    pub fn create_signature_block(
        signing_key: &SigningKey,
        root_hash: &[u8],
        signer_id: String,
        signer_name: String,
        algorithm: SignatureAlgorithm,
        scope: SignatureScope,
    ) -> TdfResult<SignatureBlock>;

    /// Verify signature block
    pub fn verify_signature_block_mixed(
        signature: &DocumentSignature,
        root_hash: &[u8],
        revocation_manager: Option<&RevocationManager>,
    ) -> VerificationResult;
}
```

#### `SignatureBlock`

Individual signature data.

```rust
pub struct SignatureBlock {
    pub signer: SignerInfo,
    pub algorithm: SignatureAlgorithm,
    pub signature: Vec<u8>,
    pub scope: SignatureScope,
    pub timestamp: Option<TimestampInfo>,
}
```

#### `SignatureAlgorithm`

Supported signature algorithms.

```rust
pub enum SignatureAlgorithm {
    Ed25519,
    Secp256k1,
}
```

#### `VerificationResult`

Result of signature verification.

```rust
pub enum VerificationResult {
    Valid,
    Invalid(String),
    Revoked { reason: String, revoked_at: DateTime<Utc> },
}
```

### Content Module

#### `ContentBlock`

All supported content block types.

```rust
pub enum ContentBlock {
    Heading {
        level: u8,
        text: String,
        id: Option<String>,
    },
    Paragraph {
        text: String,
        id: Option<String>,
    },
    List {
        ordered: bool,
        items: Vec<String>,
        id: Option<String>,
    },
    Table {
        id: String,
        caption: Option<String>,
        columns: Vec<TableColumn>,
        rows: Vec<TableRow>,
        footer: Option<Vec<String>>,
    },
    Diagram {
        id: String,
        diagram_type: DiagramType,
        title: Option<String>,
        nodes: Vec<DiagramNode>,
        edges: Vec<DiagramEdge>,
        layout: Option<DiagramLayout>,
    },
    Figure {
        id: String,
        asset: String,
        alt: String,
        caption: Option<String>,
        width: Option<u32>,
    },
    Footnote {
        id: String,
        text: String,
    },
}
```

#### `DocumentContent`

Document content structure.

```rust
pub struct DocumentContent {
    pub sections: Vec<Section>,
}

pub struct Section {
    pub id: String,
    pub title: Option<String>,
    pub content: Vec<ContentBlock>,
}
```

### Security Modules

#### Secure Key Management (`secure_key`)

##### `SecureKey`

Secure key container with automatic zeroization.

```rust
pub struct SecureKey {
    // key: Vec<u8> - automatically zeroized on drop
}

impl SecureKey {
    /// Create a new secure key from bytes
    pub fn new(key_bytes: Vec<u8>) -> Self;

    /// Get a reference to the key bytes
    pub fn as_bytes(&self) -> &[u8];

    /// Get the key length
    pub fn len(&self) -> usize;

    /// Check if the key is empty
    pub fn is_empty(&self) -> bool;

    /// Consume the key and return the bytes
    pub fn into_bytes(self) -> Vec<u8>;

    /// Explicitly zeroize the key
    pub fn zeroize(&mut self);
}
```

**Example**:
```rust
use tdf_core::secure_key::SecureKey;

let key = SecureKey::new(vec![1, 2, 3, 4, 5]);
// Key is automatically zeroized when it goes out of scope
```

##### `SecureDerivedKey`

Wrapper for derived keys with automatic zeroization.

```rust
pub struct SecureDerivedKey {
    key: SecureKey,
}

impl SecureDerivedKey {
    pub fn new(key: SecureKey) -> Self;
    pub fn as_bytes(&self) -> &[u8];
    pub fn into_secure_key(self) -> SecureKey;
}
```

#### Cryptographic Utilities (`crypto_utils`)

##### Constant-Time Comparison Functions

```rust
/// Constant-time equality comparison for byte slices
pub fn ct_eq(a: &[u8], b: &[u8]) -> bool;

/// Constant-time equality comparison for Vec<u8>
pub fn ct_eq_vecs(a: &Vec<u8>, b: &Vec<u8>) -> bool;

/// Constant-time equality comparison for hex-encoded strings
pub fn ct_eq_hex(hex_a: &str, hex_b: &str) -> Result<bool, hex::FromHexError>;

/// Constant-time selection between two values
pub fn ct_select<T>(condition: bool, a: T, b: T) -> T
where
    T: subtle::ConditionallySelectable + Copy;

/// Verify root hash in constant time
pub fn verify_root_hash(computed: &[u8], expected: &[u8]) -> bool;

/// Verify signature bytes in constant time
pub fn verify_signature_bytes(computed: &[u8], expected: &[u8]) -> bool;
```

**Example**:
```rust
use tdf_core::crypto_utils::ct_eq;

let hash1 = [0u8; 32];
let hash2 = [0u8; 32];
assert!(ct_eq(&hash1, &hash2));
```

#### Secure Random Generation (`secure_random`)

##### Random Generation Functions

```rust
/// Generate cryptographically secure random bytes
pub fn generate_secure_bytes(len: usize) -> TdfResult<Vec<u8>>;

/// Generate a secure random token (32 bytes)
pub fn generate_secure_token() -> TdfResult<[u8; 32]>;

/// Generate a secure random nonce (12 bytes for GCM)
pub fn generate_secure_nonce() -> TdfResult<[u8; 12]>;

/// Generate a secure random UUID v4
pub fn generate_secure_uuid() -> TdfResult<String>;

/// Generate a secure random session ID (64-bit)
pub fn generate_secure_session_id() -> TdfResult<u64>;
```

**Example**:
```rust
use tdf_core::secure_random::generate_secure_token;

let token = generate_secure_token()?;
// Use token for secure operations
```

#### Audit Logging (`audit`)

##### `AuditLogger`

Main audit logger for security events.

```rust
pub struct AuditLogger {
    // ...
}

impl AuditLogger {
    /// Create a new audit logger
    pub fn new() -> Self;

    /// Create a null logger (discards all events)
    pub fn null() -> Self;

    /// Add an output destination
    pub fn add_output(&mut self, output: impl AuditOutput + 'static);

    /// Set the source component name
    pub fn set_source(&mut self, source: impl Into<String>);

    /// Set the session ID
    pub fn set_session_id(&mut self, session_id: impl Into<String>);

    /// Log a verification event
    pub fn log_verification(&self, event: VerificationEvent);

    /// Log a simple info event
    pub fn log_info(&self, event_type: AuditEventType, message: impl Into<String>);

    /// Log a warning event
    pub fn log_warning(&self, event_type: AuditEventType, message: impl Into<String>);

    /// Log an error event
    pub fn log_error(&self, event_type: AuditEventType, error: impl Into<String>);

    /// Log a critical security event
    pub fn log_critical(&self, event_type: AuditEventType, error: impl Into<String>);

    /// Log a signature verification result
    pub fn log_signature_verification(
        &self,
        document_hash: &str,
        signer: AuditSignerInfo,
        valid: bool,
    );

    /// Log a revocation check result
    pub fn log_revocation_check(
        &self,
        signer_id: &str,
        revoked: bool,
        reason: Option<&str>,
    );
}
```

##### `AuditEntry`

Individual audit log entry.

```rust
pub struct AuditEntry {
    pub timestamp: DateTime<Utc>,
    pub severity: AuditSeverity,
    pub event_type: AuditEventType,
    pub result: AuditResult,
    pub document_hash: Option<String>,
    pub document_id: Option<String>,
    pub signers: Vec<AuditSignerInfo>,
    pub warnings: Vec<String>,
    pub error: Option<String>,
    pub details: Option<String>,
    pub source: Option<String>,
    pub session_id: Option<String>,
}
```

##### Audit Output Types

```rust
/// Trait for audit log output destinations
pub trait AuditOutput: Send + Sync {
    fn write(&self, entry: &AuditEntry) -> std::io::Result<()>;
}

/// Writer output (file, stderr, etc.)
pub struct WriterOutput { /* ... */ }

/// Memory output (for testing)
pub struct MemoryOutput { /* ... */ }

/// Null output (discards all entries)
pub struct NullOutput;
```

**Example**:
```rust
use tdf_core::audit::{AuditLogger, MemoryOutput, AuditEventType};

let mut logger = AuditLogger::new();
logger.add_output(MemoryOutput::new());
logger.log_info(AuditEventType::Verification, "Document verified");
```

#### Error Sanitization (`error_sanitization`)

##### Sanitization Functions

```rust
/// Sanitize error messages to prevent information leakage
pub fn sanitize_error(error: &TdfError) -> String;

/// Create a generic error code for logging
pub fn error_code(error: &TdfError) -> &'static str;
```

**Example**:
```rust
use tdf_core::error_sanitization::sanitize_error;

let error = TdfError::InvalidDocument("File /secret/path.tdf not found".to_string());
let sanitized = sanitize_error(&error);
// Returns: "Invalid document" (path removed)
```

#### Integer Safety (`integer_safety`)

##### Safe Arithmetic Functions

```rust
/// Safely add two u64 values with overflow checking
pub fn checked_add(a: u64, b: u64) -> TdfResult<u64>;

/// Safely multiply two u64 values with overflow checking
pub fn checked_mul(a: u64, b: u64) -> TdfResult<u64>;

/// Safely calculate total size from multiple components
pub fn checked_sum<I>(sizes: I) -> TdfResult<u64>
where
    I: Iterator<Item = u64>;

/// Safely calculate frame size including MAC and padding
pub fn calculate_frame_size(
    frame_size: u64,
    mac_size: u64,
    padding_size: Option<u64>,
) -> TdfResult<u64>;

/// Safely convert usize to u64
pub fn usize_to_u64(value: usize) -> TdfResult<u64>;

/// Safely convert u64 to usize
pub fn u64_to_usize(value: u64) -> TdfResult<usize>;
```

**Example**:
```rust
use tdf_core::integer_safety::checked_add;

let sum = checked_add(100, 200)?; // Ok(300)
let overflow = checked_add(u64::MAX, 1); // Err(IntegerOverflow)
```

#### Resource Limits (`resource_limits`)

##### `CircuitBreaker`

Circuit breaker for preventing cascade failures.

```rust
pub struct CircuitBreaker {
    // ...
}

impl CircuitBreaker {
    /// Create a new circuit breaker
    pub fn new(
        failure_threshold: u64,
        timeout: Duration,
        half_open_timeout: Duration,
    ) -> Self;

    /// Check if request should be allowed
    pub fn check(&self) -> TdfResult<()>;

    /// Record a successful operation
    pub fn record_success(&self);

    /// Record a failed operation
    pub fn record_failure(&self);
}
```

##### `RateLimiter`

Token bucket rate limiter.

```rust
pub struct RateLimiter {
    // ...
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new(capacity: u64, refill_rate: u64) -> Self;

    /// Try to consume a token
    pub fn try_acquire(&self) -> TdfResult<()>;
}
```

##### `ResourceBudget`

Resource budget tracker.

```rust
pub struct ResourceBudget {
    // ...
}

impl ResourceBudget {
    /// Create a new resource budget
    pub fn new(max_cpu_time: u64, max_memory: u64, max_operations: u64) -> Self;

    /// Check if operation is allowed within budget
    pub fn check_budget(&self) -> TdfResult<()>;

    /// Record CPU time usage
    pub fn record_cpu_time(&self, ms: u64);

    /// Record memory usage
    pub fn record_memory(&self, bytes: u64);

    /// Record an operation
    pub fn record_operation(&self);

    /// Reset the budget
    pub fn reset(&self);
}
```

**Example**:
```rust
use tdf_core::resource_limits::{CircuitBreaker, RateLimiter, ResourceBudget};
use std::time::Duration;

let breaker = CircuitBreaker::new(3, Duration::from_secs(1), Duration::from_millis(500));
breaker.check()?;

let limiter = RateLimiter::new(10, 5);
limiter.try_acquire()?;

let budget = ResourceBudget::new(1000, 1024 * 1024, 100);
budget.check_budget()?;
```

#### Secure I/O (`io`)

##### Bounded Readers

```rust
/// A reader wrapper that enforces a maximum read limit
pub struct BoundedReader<R: Read> {
    // ...
}

impl<R: Read> BoundedReader<R> {
    pub fn new(reader: R, limit: u64) -> Self;
    pub fn bytes_read(&self) -> u64;
    pub fn remaining(&self) -> u64;
}

/// Read a file with size bounds
pub fn read_bounded<R: Read>(reader: R, limit: u64) -> TdfResult<Vec<u8>>;

/// Read with limit and pre-allocation
pub fn read_with_limit<R: Read>(
    reader: R,
    expected_size: usize,
    limit: u64,
) -> TdfResult<Vec<u8>>;
```

##### Deserialization Security

```rust
/// Maximum CBOR recursion depth
pub const MAX_CBOR_DEPTH: usize;

/// Maximum size for CBOR deserialization
pub const MAX_CBOR_SIZE: usize;

/// Deserialize CBOR with size bounds and depth limits
pub fn deserialize_cbor_bounded<T: serde::de::DeserializeOwned>(
    data: &[u8],
    max_size: usize,
) -> TdfResult<T>;

/// Deserialize JSON with size bounds
pub fn deserialize_json_bounded<T: serde::de::DeserializeOwned>(
    data: &[u8],
    max_size: usize,
) -> TdfResult<T>;
```

**Example**:
```rust
use tdf_core::io::{read_bounded, deserialize_cbor_bounded, MAX_CBOR_SIZE};
use std::io::Cursor;

let cursor = Cursor::new(data);
let content = read_bounded(cursor, 1024 * 1024)?; // 1MB limit

let parsed: MyStruct = deserialize_cbor_bounded(&cbor_data, MAX_CBOR_SIZE)?;
```

#### Signer Whitelist (`whitelist`)

##### `SignerWhitelist`

Whitelist of trusted signers.

```rust
pub struct SignerWhitelist {
    pub name: String,
    pub description: Option<String>,
    pub trusted_signers: Vec<TrustedSigner>,
}

impl SignerWhitelist {
    /// Create a new empty whitelist
    pub fn new(name: String) -> Self;

    /// Check if a signer ID is in the whitelist
    pub fn is_trusted(&self, signer_id: &str) -> bool;

    /// Get a trusted signer by ID
    pub fn get_signer(&self, signer_id: &str) -> Option<&TrustedSigner>;

    /// Add a trusted signer to the whitelist
    pub fn add_signer(&mut self, signer: TrustedSigner);

    /// Remove a signer from the whitelist
    pub fn remove_signer(&mut self, signer_id: &str) -> bool;

    /// Validate a signer with public key binding
    pub fn validate_signer_key(
        &self,
        signer_id: &str,
        public_key: &VerifyingKey,
    ) -> WhitelistValidationResult;

    /// Validate a signer with public key binding (strict mode)
    pub fn validate_signer_key_strict(
        &self,
        signer_id: &str,
        public_key: &VerifyingKey,
    ) -> TdfResult<&TrustedSigner>;

    /// Load whitelist from JSON
    pub fn from_json(data: &[u8]) -> TdfResult<Self>;

    /// Serialize whitelist to JSON
    pub fn to_json(&self) -> TdfResult<Vec<u8>>;
}
```

##### `TrustedSigner`

Information about a trusted signer.

```rust
pub struct TrustedSigner {
    pub id: String,
    pub name: String,
    pub public_key: Option<String>,
    pub roles: Vec<String>,
    pub email: Option<String>,
}

impl TrustedSigner {
    pub fn new(id: String, name: String) -> Self;
    pub fn with_roles(id: String, name: String, roles: Vec<String>) -> Self;
    pub fn with_key(id: String, name: String, public_key: &VerifyingKey) -> Self;
}
```

**Example**:
```rust
use tdf_core::whitelist::{SignerWhitelist, TrustedSigner};

let mut whitelist = SignerWhitelist::new("ACME Corp".to_string());
whitelist.add_signer(TrustedSigner::new(
    "did:web:cfo.acme.com".to_string(),
    "CFO Jane Smith".to_string(),
));

let result = whitelist.validate_signer_key("did:web:cfo.acme.com", &verifying_key);
```

#### `RevocationManager`

Manages key revocation.

```rust
pub struct RevocationManager {
    // ...
}

impl RevocationManager {
    /// Create from revocation list
    pub fn from_list(list: RevocationList) -> Self;

    /// Check if key is revoked
    pub fn is_revoked(&self, signer_id: &str) -> Option<RevocationEntry>;
}
```

#### `SecurityConfig`

Security configuration.

```rust
pub struct SecurityConfig {
    pub max_file_size_bytes: u64,
    pub max_decompression_ratio: u64,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        SecurityConfig {
            max_file_size_bytes: 100 * 1024 * 1024, // 100 MB
            max_decompression_ratio: 100, // 100:1
        }
    }
}
```

### Timestamp Module

#### `TimestampProvider`

Trait for timestamp providers.

```rust
pub trait TimestampProvider: Send + Sync {
    fn request_timestamp(&self, data: &[u8]) -> TdfResult<TimestampToken>;
}
```

#### `ManualTimestampProvider`

Manual timestamp provider.

```rust
pub struct ManualTimestampProvider;

impl TimestampProvider for ManualTimestampProvider {
    fn request_timestamp(&self, _data: &[u8]) -> TdfResult<TimestampToken> {
        Ok(TimestampToken {
            timestamp: Utc::now(),
            authority: "manual".to_string(),
            signature: vec![],
        })
    }
}
```

## CLI Tool (`tdf`)

### Commands

#### `tdf create`

Create a TDF document from JSON input.

```bash
tdf create <input.json> -o <output.tdf> \
  --key <signing-key.signing> \
  --signer-id <did:web:example.com> \
  --signer-name <Name>
```

**Options**:
- `-o, --output <FILE>`: Output TDF file
- `--key <FILE>`: Signing key file
- `--signer-id <ID>`: Signer identifier (DID)
- `--signer-name <NAME>`: Signer name
- `--algorithm <ALG>`: Signature algorithm (ed25519|secp256k1)
- `--timestamp`: Include timestamp

#### `tdf verify`

Verify document integrity and signatures.

```bash
tdf verify <document.tdf> [--key <verifying-key.verifying>] [--verbose]
```

**Options**:
- `--key <FILE>`: Public key for signature verification
- `--verbose`: Show detailed verification information

#### `tdf extract`

Extract structured data from document.

```bash
tdf extract <document.tdf> -o <data.json>
```

**Options**:
- `-o, --output <FILE>`: Output JSON file

#### `tdf info`

Show document metadata.

```bash
tdf info <document.tdf>
```

#### `tdf keygen`

Generate cryptographic key pair.

```bash
tdf keygen --name <key-name> [--algorithm <ed25519|secp256k1>]
```

**Options**:
- `--name <NAME>`: Base name for key files
- `--algorithm <ALG>`: Algorithm (default: ed25519)

#### `tdf export`

Export document to PDF.

```bash
tdf export <document.tdf> -o <output.pdf>
```

#### `tdf revoke`

Add entry to revocation list.

```bash
tdf revoke <signer-id> --reason <reason> -o <revocation.cbor>
```

#### `tdf check-revocation`

Check signatures against revocation list.

```bash
tdf check-revocation <document.tdf> <revocation.cbor>
```

## TypeScript SDK (`tdf-ts`)

### Main Functions

#### `loadDocument`

Load a TDF document from a file or blob.

```typescript
import { loadDocument } from 'tdf-ts';

const file: File | Blob = // ... file from input
const document = await loadDocument(file);
```

**Returns**: `Promise<Document>`

#### `extractData`

Extract structured data from document.

```typescript
import { extractData } from 'tdf-ts';

const data = extractData(document);
// Returns: { tables: [...], metadata: {...}, ... }
```

**Returns**: `ExtractedData`

#### `verifyIntegrity`

Verify document integrity (client-side).

```typescript
import { verifyIntegrity } from 'tdf-ts';

const isValid = verifyIntegrity(document);
```

**Returns**: `boolean`

### Types

#### `Document`

```typescript
interface Document {
    manifest: Manifest;
    content: DocumentContent;
    styles: string;
    layout?: Layout;
    data?: any;
}
```

#### `Manifest`

```typescript
interface Manifest {
    schema_version: string;
    document: DocumentMeta;
    authors: Author[];
    classification?: Classification;
    integrity: IntegrityBlock;
}
```

## WASM Bindings (`tdf-wasm`)

### Functions

#### `verify_document`

Verify document integrity (WASM).

```javascript
import init, { verify_document } from 'tdf-wasm';

await init();
const result = verify_document(fileData);
// Returns: { integrity_valid: boolean, ... }
```

#### `get_document_info`

Get document metadata.

```javascript
import { get_document_info } from 'tdf-wasm';

const info = get_document_info(fileData);
// Returns: { title: string, ... }
```

## Error Types

### `TdfError`

All errors returned by TDF operations.

```rust
pub enum TdfError {
    InvalidDocument(String),
    IntegrityFailure(String),
    SignatureFailure(String),
    MissingFile(String),
    IoError(std::io::Error),
    CborError(serde_cbor::Error),
    ZipError(zip::result::ZipError),
    // ...
}
```

### `TdfResult<T>`

Result type alias.

```rust
pub type TdfResult<T> = Result<T, TdfError>;
```

## Content Primitives

### Tables

```json
{
  "type": "table",
  "id": "tbl-1",
  "caption": "Financial Summary",
  "columns": [
    {
      "id": "item",
      "header": "Item",
      "type": "text"
    },
    {
      "id": "amount",
      "header": "Amount",
      "type": "currency",
      "currency": "EUR"
    }
  ],
  "rows": [
    {
      "item": "Revenue",
      "amount": {
        "raw": 100000.50,
        "display": "€100,000.50",
        "currency": "EUR"
      }
    }
  ]
}
```

### Diagrams

```json
{
  "type": "diagram",
  "id": "diag-1",
  "diagram_type": "hierarchical",
  "title": "Organization Chart",
  "nodes": [
    {
      "id": "ceo",
      "label": "CEO",
      "shape": "box"
    }
  ],
  "edges": [
    {
      "from": "ceo",
      "to": "cto",
      "type": "solid"
    }
  ]
}
```

### Figures

```json
{
  "type": "figure",
  "id": "fig-1",
  "asset": "assets/images/chart.png",
  "alt": "Sales chart",
  "caption": "Q2 2025 Sales",
  "width": 800
}
```

---

*Last updated: 2025-12-09*
