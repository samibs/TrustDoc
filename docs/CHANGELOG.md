# Changelog

## [0.3.0] - 2025-12-07

### Added

#### RFC 3161 Timestamp Authority Support
- Timestamp provider trait for extensible timestamping
- Manual timestamp provider (default)
- RFC 3161 timestamp provider framework (with `rfc3161` feature)
- Timestamp token verification
- Integration with signature creation

#### Enhanced Multi-Party Workflows
- `MultiPartySigningSession` for managing multi-signer documents
- `SigningWorkflow` for complex signing processes
- Support for unordered, ordered, and simultaneous signing
- Workflow status tracking
- CLI commands: `tdf workflow create` and `tdf workflow status`
- Validation of signing order and required signers

#### Advanced PDF Export
- Enhanced table rendering with better formatting
- Diagram rendering (text representation)
- List rendering (ordered and unordered)
- Improved layout and spacing
- Text truncation for long content
- Caption support for tables

### Improved

- PDF export now handles more content types
- Better error messages for workflow validation
- Enhanced documentation for advanced features

## [0.2.0] - 2025-12-07

### Added

#### secp256k1 Signature Support
- Web3/blockchain compatible signatures
- `tdf keygen --secp256k1` command for generating secp256k1 keypairs
- Full signing and verification support
- Mixed signature verification (Ed25519 + secp256k1)

#### PDF Export
- `tdf export` command for PDF generation
- Basic document rendering (text, tables, headings)
- A4 page format
- Preserves document structure

#### Performance Testing
- Performance test script for various document sizes
- Optimization recommendations
- Benchmarking tools

#### Enhanced Documentation
- Advanced features guide (docs/ADVANCED.md)
- Feature summary (docs/FEATURES.md)
- Build instructions (docs/BUILD.md)

### Improved

- WASM bindings now use memory-based ZIP reading (no file system)
- Better error messages for signature verification
- Enhanced CLI help text

## [0.1.0] - 2025-12-07

### Added

#### Core Library (`tdf-core`)
- Document model with manifest, content, styles, and layout
- Content primitives: text blocks, tables with typed cells, diagrams, figures
- Merkle tree computation and verification (SHA-256)
- Ed25519 signature support with multi-party signing
- ZIP archive handling with CBOR encoding
- Comprehensive error handling
- Unit and integration tests

#### CLI Tool (`tdf-cli`)
- `tdf create` - Generate TDF documents from JSON
- `tdf verify` - Verify integrity and signatures
- `tdf extract` - Extract structured data
- `tdf info` - Show document metadata
- `tdf keygen` - Generate Ed25519 keypairs

#### TypeScript SDK (`tdf-ts`)
- Document loading from ZIP archives
- CBOR parsing support (cbor-web)
- Type definitions for all content primitives
- Data extraction helpers
- Verification API (with WASM support)

#### WASM Bindings (`tdf-wasm`)
- Browser-compatible verification
- Document info extraction
- Merkle root computation
- Memory-based ZIP reading (no file system)

#### Web Viewer (`tdf-viewer`)
- HTML/CSS renderer for text, tables, and figures
- SVG-based diagram renderer
- Drag-and-drop file loading
- Integrity verification (WASM-powered)
- Print support
- Data extraction

#### Examples
- Quarterly financial report with tables and org chart
- Balance sheet with assets and liabilities
- Invoice with line items and totals

#### Documentation
- Format specification (SPEC.md)
- API reference (docs/API.md)
- Usage guide (docs/USAGE.md)
- Build instructions (docs/BUILD.md)

### Features

- **Tamper-evident**: Merkle tree detects any modification
- **Cryptographic signatures**: Ed25519 support
- **Semantic content**: Structured data extraction
- **Print-ready**: Deterministic rendering
- **Open format**: ZIP-based, CBOR-encoded
- **Browser support**: WASM bindings for verification
- **Corporate-ready**: Audit trails, multi-party signatures

### Technical Details

- **Hashing**: SHA-256 (default), BLAKE3 (optional)
- **Signatures**: Ed25519 (primary), secp256k1 (planned)
- **Encoding**: CBOR for structured data
- **Archive**: Standard ZIP format
- **Size tiers**: Micro (256KB), Standard (5MB), Extended (50MB)

### Testing

- All Rust unit tests passing
- Integration tests for document round-trip
- End-to-end workflow tested
- Example documents generated and verified

