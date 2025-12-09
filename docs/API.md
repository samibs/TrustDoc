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
