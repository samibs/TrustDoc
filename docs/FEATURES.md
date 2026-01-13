# TDF Feature Summary

## Core Features

### Cryptographic Integrity
- **Merkle Tree Verification**: SHA-256 based Merkle tree detects any modification
- **Tamper-Evident**: Any byte change invalidates the document
- **Offline Verification**: No server or network required

### Signature Support
- **Ed25519**: Fast, modern signature algorithm (default)
- **secp256k1**: Web3/blockchain compatible signatures
- **Multi-Party Signatures**: Multiple signers can co-sign documents
- **Signature Scopes**: Sign full document, content-only, or specific sections

### Content Primitives
- **Text Blocks**: Paragraphs, headings (1-4), lists, footnotes
- **Tables**: Typed cells (text, number, currency, percentage, date)
- **Diagrams**: Hierarchical, flowchart, relationship diagrams
- **Figures**: Images with captions
- **Structured Data**: Machine-readable extraction

### Format Features
- **ZIP Archive**: Standard ZIP format for universal compatibility
- **CBOR Encoding**: Compact binary encoding for structured data
- **CSS Styling**: Web-native styling support
- **Fixed Layout**: Optional print-deterministic positioning
- **Size Tiers**: Micro (256KB), Standard (5MB), Extended (50MB)

## CLI Commands

### `tdf create`
Create a TDF document from JSON input.

```bash
tdf create input.json -o output.tdf \
  --key signing-key.signing \
  --signer-id "did:web:example.com" \
  --signer-name "John Doe"
```

### `tdf verify`
Verify document integrity and signatures.

```bash
# Basic integrity check
tdf verify document.tdf

# With signature verification
tdf verify document.tdf --key verifying-key.verifying
```

### `tdf extract`
Extract structured data from document.

```bash
tdf extract document.tdf -o data.json
```

### `tdf info`
Show document metadata and signature information.

```bash
tdf info document.tdf
```

### `tdf keygen`
Generate signing keypairs.

```bash
# Ed25519 (default)
tdf keygen --name my-key

# secp256k1 (Web3 compatible)
tdf keygen --name my-key --secp256k1
```

### `tdf export`
Export TDF document to PDF.

```bash
tdf export document.tdf -o output.pdf
```

## TypeScript SDK

- Document loading from ZIP archives
- CBOR parsing
- Data extraction
- Browser-compatible verification (with WASM)

## WASM Bindings

- Browser-based cryptographic verification
- Memory-based ZIP reading (no file system)
- Document info extraction
- Merkle root computation

## Web Viewer

- Drag-and-drop file loading
- HTML/CSS rendering
- SVG diagram rendering
- Integrity verification
- Print support
- Data extraction

## Example Documents

- **Quarterly Report**: Financial tables, org charts
- **Balance Sheet**: Assets, liabilities, equity
- **Invoice**: Line items, totals, VAT

## Security Features

### Core Security
- **Tamper-Evident Design**: Any byte change invalidates the document
- **Cryptographic Signatures**: Ed25519 and secp256k1 support
- **Audit Trail Support**: Comprehensive logging of security events
- **No Vendor Lock-in**: Open format specification
- **Constant-Time Operations**: Timing attack resistance

### Security Modules

#### Secure Key Management (`secure_key`)
- **Automatic Zeroization**: Key material automatically cleared from memory
- **SecureKey Container**: Wraps sensitive keys with automatic cleanup
- **Memory Safety**: Prevents key material from persisting in memory
- **Drop Protection**: Keys zeroized when going out of scope

#### Cryptographic Utilities (`crypto_utils`)
- **Constant-Time Comparisons**: Prevents timing side-channel attacks
- **Hash Verification**: Secure root hash and signature comparison
- **Timing Attack Resistance**: All comparisons use constant-time operations
- **Multiple Comparison Methods**: Byte slices, vectors, and hex strings

#### Secure Random Generation (`secure_random`)
- **OS CSPRNG**: Uses operating system cryptographically secure RNG
- **Defense-in-Depth Entropy**: Multiple entropy sources with SHA-256 mixing
- **Token Generation**: Secure 32-byte tokens for sessions
- **Nonce Generation**: Secure 12-byte nonces for encryption
- **UUID Generation**: Secure UUID v4 generation
- **Session IDs**: Full 256-bit cryptographic randomness

#### Audit Logging (`audit`)
- **Structured Logging**: JSON-formatted audit entries
- **Event Types**: Verification, signatures, revocation, policy enforcement
- **Severity Levels**: Info, Warning, Error, Critical
- **Multiple Outputs**: File, memory, stderr, or custom destinations
- **Compliance Ready**: Supports regulatory audit requirements

#### Error Sanitization (`error_sanitization`)
- **Information Leakage Prevention**: Removes sensitive data from errors
- **Path Sanitization**: Strips file paths and system information
- **Generic Error Codes**: Provides safe error codes for logging
- **Social Engineering Protection**: Prevents information gathering via errors

#### Integer Safety (`integer_safety`)
- **Overflow Protection**: Checked arithmetic prevents integer overflow
- **Safe Type Conversions**: usize/u64 conversions with bounds checking
- **Frame Size Calculation**: Safe calculation of encrypted frame sizes
- **Memory Safety**: Prevents buffer overflows from integer attacks

#### Resource Limits (`resource_limits`)
- **Circuit Breakers**: Prevents cascade failures from resource exhaustion
- **Rate Limiting**: Token bucket algorithm for request throttling
- **Resource Budgets**: CPU, memory, and operation tracking
- **DoS Protection**: Prevents denial-of-service attacks
- **Power Exhaustion Protection**: Protects field devices from battery drain

#### Secure I/O (`io`)
- **Bounded Readers**: Enforces maximum read limits
- **Deserialization Security**: Size and depth limits for CBOR/JSON
- **ZIP Bomb Protection**: Prevents memory exhaustion from malicious archives
- **Depth Limits**: Prevents stack overflow from deeply nested structures

#### Signer Whitelist (`whitelist`)
- **Trusted Signer Management**: Organization-defined signer lists
- **Public Key Binding**: Validates signer keys against whitelist
- **Role-Based Authorization**: Assign roles to trusted signers
- **Strict Mode**: Requires key binding for enhanced security

### Security Standards Compliance
- **ISO 27001/27002**: Aligned with international security standards
- **NIST SP 800-90B**: Entropy requirements for random number generation
- **OWASP Best Practices**: Follows OWASP security guidelines
- **Zero Unsafe Code**: All security-critical paths use safe Rust

## Performance

- Efficient Merkle tree computation
- Compact binary encoding (CBOR)
- Modern compressed formats (WebP, WOFF2)
- Optimized for typical business documents

## Use Cases

- Financial reports and statements
- Legal documents and contracts
- Invoices and receipts
- Corporate communications
- Audit trails and compliance
- Blockchain-compatible documents

