# TDF Architecture Documentation

Comprehensive architecture documentation for the TDF format and ecosystem.

## Table of Contents

1. [System Architecture](#system-architecture)
2. [Format Architecture](#format-architecture)
3. [Cryptographic Architecture](#cryptographic-architecture)
4. [Component Architecture](#component-architecture)
5. [Data Flow](#data-flow)
6. [Security Architecture](#security-architecture)
7. [Extension Points](#extension-points)

## System Architecture

### Overview

TDF is designed as a modular ecosystem with clear separation of concerns:

```
┌─────────────────────────────────────────────────────────────┐
│                      Application Layer                      │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐ │
│  │   CLI    │  │   Web     │  │ Desktop  │  │  Mobile  │ │
│  └────┬─────┘  └────┬──────┘  └────┬─────┘  └────┬─────┘ │
└───────┼──────────────┼──────────────┼──────────────┼───────┘
        │              │              │              │
┌───────┼──────────────┼──────────────┼──────────────┼───────┐
│       │              │              │              │        │
│  ┌────▼──────┐  ┌────▼──────┐  ┌────▼──────┐  ┌────▼──────┐│
│  │  tdf-ts   │  │  tdf-wasm  │  │  tdf-core │  │  tdf-core ││
│  │  (JS)     │  │  (WASM)    │  │  (Rust)   │  │  (Rust)   ││
│  └───────────┘  └───────────┘  └───────────┘  └───────────┘│
│                                                               │
│                    Core Library Layer                         │
│  ┌────────────────────────────────────────────────────────┐ │
│  │  Document │ Archive │ Crypto │ Verification │ Security │ │
│  └────────────────────────────────────────────────────────┘ │
└───────────────────────────────────────────────────────────────┘
```

### Layer Responsibilities

1. **Application Layer**: User interfaces and workflows
2. **SDK Layer**: Language-specific bindings (TypeScript, WASM)
3. **Core Library**: Format implementation and cryptography
4. **System Layer**: File system, network, platform APIs

## Format Architecture

### File Structure

A TDF file is a ZIP archive with a specific internal structure:

```
document.tdf (ZIP Archive)
│
├── manifest.cbor          # Document metadata and integrity info
├── content.cbor           # Structured document content
├── styles.css             # Presentation styles
├── layout.cbor            # Optional: Print layout
├── data.json              # Optional: Machine-readable extract
├── hashes.bin             # Merkle tree binary representation
├── signatures.cbor        # Cryptographic signatures
├── revocation.cbor        # Optional: Key revocation list
└── assets/                # Optional: Embedded resources
    ├── images/
    │   ├── logo.webp
    │   └── chart.png
    └── fonts/
        └── custom.woff2
```

### Component Relationships

```
┌─────────────┐
│  Document   │
│  (Model)    │
└──────┬──────┘
       │
       ├──────────────┬──────────────┐
       │              │              │
┌──────▼──────┐ ┌────▼──────┐ ┌────▼──────┐
│   Content   │ │  Manifest  │ │  Styles   │
│   (CBOR)    │ │   (CBOR)   │ │   (CSS)   │
└──────┬──────┘ └────┬───────┘ └───────────┘
       │             │
       └──────┬──────┘
              │
       ┌──────▼──────┐
       │ Merkle Tree │
       │   (hashes)  │
       └──────┬──────┘
              │
       ┌──────▼──────┐
       │ Signatures  │
       │   (CBOR)   │
       └────────────┘
```

### Data Encoding

- **CBOR**: Binary format for structured data (manifest, content, signatures)
- **ZIP**: Archive format for file bundling
- **CSS**: Text format for styling
- **JSON**: Optional text format for data extraction

## Cryptographic Architecture

### Integrity Verification

```
┌─────────────────────────────────────────┐
│         Merkle Tree Construction        │
├─────────────────────────────────────────┤
│                                         │
│  Root Hash (SHA-256)                    │
│  ├── Hash(manifest.cbor)                │
│  ├── Hash(content.cbor)                 │
│  ├── Hash(styles.css)                   │
│  └── Hash(assets/)                      │
│      ├── Hash(image1.webp)              │
│      └── Hash(image2.png)               │
│                                         │
└─────────────────────────────────────────┘
```

### Signature Architecture

```
┌─────────────────────────────────────────┐
│         Signature Creation               │
├─────────────────────────────────────────┤
│                                         │
│  1. Compute Merkle Root Hash            │
│  2. Create Signature Block:              │
│     - Signer ID (DID)                    │
│     - Signer Name                        │
│     - Algorithm (Ed25519/secp256k1)      │
│     - Signature (over root hash)         │
│     - Timestamp (optional)                │
│     - Scope (document/component)          │
│  3. Add to signatures.cbor               │
│                                         │
└─────────────────────────────────────────┘
```

### Verification Flow

```
┌─────────────────────────────────────────┐
│         Verification Process             │
├─────────────────────────────────────────┤
│                                         │
│  1. Read ZIP archive                    │
│  2. Extract all components               │
│  3. Recompute Merkle tree                │
│  4. Compare root hash with manifest      │
│  5. For each signature:                   │
│     a. Verify signature against root     │
│     b. Check revocation list              │
│     c. Validate timestamp                │
│  6. Return verification report            │
│                                         │
└─────────────────────────────────────────┘
```

## Component Architecture

### Core Library Components

#### Document Module (`document.rs`)

- **Document**: Main document structure
- **Manifest**: Metadata and integrity anchors
- **DocumentMeta**: Document-level metadata
- **Author**: Author information
- **Layout**: Print layout configuration

#### Content Module (`content.rs`)

- **ContentBlock**: Enum of all content types
- **Table**: Structured table data
- **Diagram**: Graph/diagram structures
- **Figure**: Image with caption
- **Text Blocks**: Headings, paragraphs, lists

#### Archive Module (`archive.rs`)

- **ArchiveBuilder**: Constructs TDF archives
- **ArchiveReader**: Reads and verifies archives
- **VerificationReport**: Verification results

#### Merkle Module (`merkle.rs`)

- **MerkleTree**: Tree construction and verification
- **HashAlgorithm**: Supported algorithms (SHA-256, BLAKE3)

#### Signature Module (`signature.rs`)

- **SignatureManager**: Signature operations
- **SignatureBlock**: Individual signature data
- **VerificationResult**: Signature verification results

#### Security Modules

- **Revocation**: Key revocation management
- **Timestamp**: Timestamp authority integration
- **Config**: Security configuration (DoS protection, etc.)

### CLI Architecture

```
tdf-cli/
├── src/
│   ├── main.rs              # CLI entry point
│   └── commands/
│       ├── create.rs        # Document creation
│       ├── verify.rs        # Verification
│       ├── extract.rs       # Data extraction
│       ├── info.rs          # Metadata display
│       ├── keygen.rs        # Key generation
│       ├── export.rs        # PDF export
│       ├── workflow.rs      # Multi-party workflows
│       └── revoke.rs        # Key revocation
```

### SDK Architecture

#### TypeScript SDK (`tdf-ts`)

```
tdf-ts/
├── src/
│   ├── index.ts             # Main entry point
│   ├── document.ts          # Document loading
│   ├── parser.ts            # CBOR parsing
│   ├── extractor.ts         # Data extraction
│   └── types.ts             # TypeScript types
```

#### WASM Bindings (`tdf-wasm`)

```
tdf-wasm/
├── src/
│   ├── lib.rs               # WASM exports
│   ├── archive.rs           # Archive operations
│   ├── verification.rs      # Verification functions
│   └── utils.rs             # Utilities
```

## Data Flow

### Document Creation Flow

```
User Input (JSON/CLI)
    │
    ▼
Document Model (Rust)
    │
    ├──► Content Blocks
    ├──► Manifest
    └──► Styles
    │
    ▼
ArchiveBuilder
    │
    ├──► Serialize to CBOR
    ├──► Build Merkle Tree
    └──► Create ZIP Archive
    │
    ▼
Add Signatures
    │
    ▼
Write to File (.tdf)
```

### Document Verification Flow

```
TDF File (.tdf)
    │
    ▼
ArchiveReader
    │
    ├──► Extract ZIP
    ├──► Parse CBOR
    └──► Read Components
    │
    ▼
Recompute Merkle Tree
    │
    ├──► Compare Root Hash
    └──► Verify Integrity
    │
    ▼
Verify Signatures
    │
    ├──► Check Revocation
    ├──► Validate Timestamps
    └──► Verify Signatures
    │
    ▼
VerificationReport
```

### Multi-Party Workflow Flow

```
Workflow Creation
    │
    ├──► Define Signers
    ├──► Set Order (if ordered)
    └──► Create Session
    │
    ▼
Signer 1 Signs
    │
    ▼
Update Workflow State
    │
    ▼
Signer 2 Signs (if allowed)
    │
    ▼
... (continue for all signers)
    │
    ▼
Workflow Complete
```

## Security Architecture

### Defense in Depth

```
┌─────────────────────────────────────────┐
│         Security Layers                  │
├─────────────────────────────────────────┤
│                                         │
│  Layer 1: Format Validation              │
│  - ZIP structure                        │
│  - CBOR validity                        │
│  - Required files                        │
│                                         │
│  Layer 2: Integrity Verification         │
│  - Merkle tree                          │
│  - Hash comparison                       │
│                                         │
│  Layer 3: Cryptographic Verification    │
│  - Signature verification                │
│  - Key validation                        │
│                                         │
│  Layer 4: Security Controls              │
│  - Revocation checks                     │
│  - Timestamp validation                  │
│  - DoS protection                        │
│                                         │
└─────────────────────────────────────────┘
```

### Threat Mitigation

| Threat | Mitigation |
|--------|------------|
| Content Tampering | Merkle tree integrity |
| Signature Forgery | Cryptographic signatures |
| Replay Attacks | Timestamps |
| Key Compromise | Revocation lists |
| DoS Attacks | Size limits, decompression limits |
| Path Traversal | ZIP path validation |

## Extension Points

### Custom Content Blocks

Add new content types by extending `ContentBlock` enum:

```rust
pub enum ContentBlock {
    // ... existing types
    CustomBlock {
        id: Option<String>,
        custom_data: CustomData,
    },
}
```

### Custom Hash Algorithms

Implement `HashAlgorithm` trait:

```rust
impl HashAlgorithm for CustomHash {
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        // Implementation
    }
}
```

### Custom Signature Algorithms

Extend `SignatureAlgorithm` enum and implement verification:

```rust
pub enum SignatureAlgorithm {
    // ... existing
    CustomAlgo,
}
```

### Custom Timestamp Providers

Implement `TimestampProvider` trait:

```rust
impl TimestampProvider for CustomProvider {
    fn request_timestamp(&self, data: &[u8]) -> TdfResult<TimestampToken> {
        // Implementation
    }
}
```

## Performance Architecture

### Optimization Strategies

1. **Parallel Processing**: Use `rayon` for parallel hash computation
2. **Streaming**: Stream large files instead of loading into memory
3. **Caching**: Cache computed hashes and parsed structures
4. **Lazy Loading**: Load components only when needed

### Memory Management

- Use `Vec<u8>` for binary data
- Avoid unnecessary cloning
- Use references for read-only access
- Consider streaming for very large files

## Future Architecture Considerations

### Planned Enhancements

1. **Streaming Verification**: Verify without loading entire file
2. **Incremental Signing**: Add signatures without rebuilding archive
3. **Compression**: Optional compression for large documents
4. **Encryption**: Optional encryption for sensitive documents
5. **Distributed Verification**: Verify across multiple nodes

---

*Last updated: 2025-12-09*

