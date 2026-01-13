# TrustDoc Format (TDF)
## Technical Specification v1.0

**Document Status**: Draft
**Last Updated**: January 2026

---

## 1. Overview

TrustDoc Format (TDF) is a container format for cryptographically-secured documents. It combines structured data storage with Merkle tree integrity verification and multi-party digital signatures.

### 1.1 Design Goals

1. **Integrity**: Any modification to document content must be detectable
2. **Non-repudiation**: Signers cannot deny having approved specific content
3. **Structured Data**: Machine-readable content, not just visual representation
4. **Offline Verification**: No network connectivity required for verification
5. **Open Standard**: No proprietary components, freely implementable
6. **Efficiency**: Compact binary encoding, fast verification

### 1.2 Format Summary

```
document.tdf
├── manifest.cbor       # Document metadata + root hash
├── content.cbor        # Structured document content
├── styles.css          # Optional rendering styles
├── merkle.cbor         # Merkle tree structure
└── signatures.cbor     # Digital signatures
```

**Container**: ZIP archive (PKWARE specification)
**Encoding**: CBOR (RFC 8949)
**Compression**: DEFLATE (per-file within ZIP)

---

## 2. File Structure

### 2.1 Archive Layout

| File | Required | Description |
|------|----------|-------------|
| `manifest.cbor` | Yes | Document metadata and integrity root |
| `content.cbor` | Yes | Structured document content |
| `merkle.cbor` | Yes | Merkle tree for integrity verification |
| `signatures.cbor` | No | Digital signatures block |
| `styles.css` | No | CSS styles for rendering |
| `attachments/*` | No | Binary attachments (images, etc.) |

### 2.2 Manifest Structure

```cbor
{
  "schema_version": "1.0.0",
  "id": "uuid-v4",
  "title": "Document Title",
  "language": "en",
  "created": "2026-01-10T12:00:00Z",
  "modified": "2026-01-10T12:00:00Z",
  "authors": [
    {
      "id": "did:web:author.example.com",
      "name": "Author Name",
      "role": "creator"
    }
  ],
  "integrity": {
    "algorithm": "sha256",
    "root_hash": "hex-encoded-64-chars"
  },
  "content_type": "financial-report",
  "keywords": ["Q4", "2025", "financial"],
  "custom_metadata": {}
}
```

### 2.3 Content Structure

```cbor
{
  "sections": [
    {
      "id": "section-uuid",
      "title": "Section Title",
      "content": [
        {
          "type": "heading",
          "id": "block-uuid",
          "level": 1,
          "text": "Heading Text"
        },
        {
          "type": "paragraph",
          "id": "block-uuid",
          "text": "Paragraph content..."
        },
        {
          "type": "table",
          "id": "block-uuid",
          "caption": "Table Caption",
          "columns": [
            {"id": "col-1", "header": "Column 1", "type": "text"},
            {"id": "col-2", "header": "Amount", "type": "currency", "currency": "EUR"}
          ],
          "rows": [
            {"col-1": {"raw": "Item 1", "display": "Item 1"},
             "col-2": {"raw": 1000.00, "display": "€1,000.00"}}
          ]
        }
      ]
    }
  ]
}
```

#### Content Block Types

| Type | Fields | Description |
|------|--------|-------------|
| `heading` | level, text | Headers (1-6) |
| `paragraph` | text | Text content |
| `table` | columns, rows, caption | Structured data table |
| `list` | items, ordered | Bulleted/numbered list |
| `currency` | amount, currency, display | Monetary value |
| `date` | value, format | Date value |
| `diagram` | diagram_type, nodes, edges | Flowchart/diagram |
| `image` | src, alt, caption | Image reference |
| `code` | language, content | Code block |

### 2.4 Merkle Tree Structure

```cbor
{
  "algorithm": "sha256",
  "leaf_count": 8,
  "tree": [
    {"level": 0, "index": 0, "hash": "..."},
    {"level": 0, "index": 1, "hash": "..."},
    {"level": 1, "index": 0, "hash": "..."},
    {"level": 2, "index": 0, "hash": "..."}  // Root
  ],
  "components": [
    {"name": "manifest", "hash": "..."},
    {"name": "content", "hash": "..."},
    {"name": "styles", "hash": "..."}
  ]
}
```

### 2.5 Signatures Structure

```cbor
{
  "signatures": [
    {
      "signer": {
        "id": "did:web:signer.example.com",
        "name": "Signer Name"
      },
      "algorithm": "ed25519",
      "public_key": "hex-encoded-32-bytes",
      "signature": "hex-encoded-64-bytes",
      "timestamp": {
        "time": "2026-01-10T12:00:00Z",
        "authority": "manual",
        "proof": "",
        "algorithm": "manual"
      },
      "scope": "full",
      "metadata": {}
    }
  ]
}
```

---

## 3. Cryptographic Specifications

### 3.1 Hash Algorithms

| Algorithm | Output Size | Standard | Usage |
|-----------|-------------|----------|-------|
| SHA-256 | 256 bits | FIPS 180-4 | Default, required support |
| BLAKE3 | 256 bits | - | Optional, faster |

**Merkle Tree Construction**:
```
H(parent) = SHA256(H(left) || H(right))
```

If odd number of leaves, last leaf is duplicated.

### 3.2 Signature Algorithms

| Algorithm | Key Size | Signature Size | Standard |
|-----------|----------|----------------|----------|
| Ed25519 | 32 bytes | 64 bytes | RFC 8032 |
| secp256k1 | 32 bytes | 64-72 bytes | SEC 2 |

**Signature Input**:
```
message = root_hash (32 bytes, raw)
signature = Sign(private_key, message)
```

### 3.3 Key Encoding

**Ed25519 Keys**:
- Private key: 32 bytes raw
- Public key: 32 bytes raw
- File format: Raw bytes, no PEM wrapper

**secp256k1 Keys**:
- Private key: 32 bytes raw
- Public key: 33 bytes compressed or 65 bytes uncompressed

### 3.4 Identifier Format

Signer identifiers use DID (Decentralized Identifier) format:
```
did:web:domain.example.com
did:key:z6Mk...
did:ethr:0x...
```

---

## 4. Integrity Verification

### 4.1 Verification Algorithm

```rust
fn verify_integrity(archive: &TdfArchive) -> Result<bool, Error> {
    // 1. Extract all components
    let manifest = parse_cbor(archive.read("manifest.cbor")?)?;
    let content = archive.read("content.cbor")?;
    let styles = archive.read("styles.css").unwrap_or_default();
    let merkle = parse_cbor(archive.read("merkle.cbor")?)?;

    // 2. Compute component hashes
    let component_hashes = [
        sha256(&manifest_bytes),
        sha256(&content),
        sha256(&styles),
    ];

    // 3. Verify Merkle tree leaves match
    for (i, expected) in merkle.components.iter().enumerate() {
        if component_hashes[i] != expected.hash {
            return Ok(false);  // Component tampered
        }
    }

    // 4. Recompute Merkle root
    let computed_root = compute_merkle_root(&component_hashes);

    // 5. Compare with manifest root hash
    Ok(computed_root == manifest.integrity.root_hash)
}
```

### 4.2 Signature Verification

```rust
fn verify_signatures(
    archive: &TdfArchive,
    trusted_keys: &[PublicKey],
    revocation_list: Option<&RevocationList>
) -> Vec<SignatureResult> {
    let root_hash = get_root_hash(archive);
    let sig_block = parse_cbor(archive.read("signatures.cbor")?)?;

    let mut results = Vec::new();

    for sig in sig_block.signatures {
        // 1. Check revocation status
        if let Some(revoked) = revocation_list.check(&sig.signer.id) {
            results.push(SignatureResult::Revoked(revoked));
            continue;
        }

        // 2. Find matching public key
        let public_key = match find_key(&sig.signer.id, trusted_keys) {
            Some(k) => k,
            None => {
                results.push(SignatureResult::UnknownSigner);
                continue;
            }
        };

        // 3. Verify signature
        let valid = match sig.algorithm {
            Algorithm::Ed25519 => verify_ed25519(
                &public_key,
                &root_hash,
                &sig.signature
            ),
            Algorithm::Secp256k1 => verify_secp256k1(
                &public_key,
                &root_hash,
                &sig.signature
            ),
        };

        results.push(if valid {
            SignatureResult::Valid(sig.timestamp)
        } else {
            SignatureResult::Invalid
        });
    }

    results
}
```

---

## 5. Security Considerations

### 5.1 Threat Model

| Threat | Protection |
|--------|------------|
| Content modification after signing | Merkle tree integrity check |
| Signature forgery | Cryptographic signatures |
| Signature stripping | Signatures referenced in manifest |
| Replay attack | Document ID + timestamp uniqueness |
| Hash collision | SHA-256 collision resistance |
| Key compromise | Revocation list support |
| ZIP bomb (DoS) | Size limits, decompression ratio limits |
| Path traversal | Strict filename validation |

### 5.2 Security Configuration

```rust
pub struct SecurityConfig {
    pub max_archive_size: u64,        // Bytes
    pub max_decompression_ratio: f64, // Compressed:Uncompressed
    pub max_file_count: usize,
    pub allowed_algorithms: Vec<Algorithm>,
}

// Predefined tiers
pub enum SizeTier {
    Micro,    // 256 KB, 100:1 ratio
    Standard, // 5 MB, 1000:1 ratio
    Extended, // 50 MB, 10000:1 ratio
}
```

### 5.3 Key Management Recommendations

1. **Key Generation**: Use cryptographically secure RNG (OS-provided)
2. **Key Storage**: HSM for production, encrypted file for development
3. **Key Rotation**: Annual rotation recommended
4. **Revocation**: Immediate revocation on suspected compromise
5. **Backup**: Secure offline backup of signing keys

---

## 6. Timestamp Authority

### 6.1 Manual Timestamps

```cbor
{
  "time": "2026-01-10T12:00:00Z",
  "authority": "manual",
  "proof": "",
  "algorithm": "manual"
}
```

- Uses local system time
- No cryptographic proof of time
- Suitable for internal documents

### 6.2 RFC 3161 Timestamps (Optional)

```cbor
{
  "time": "2026-01-10T12:00:00Z",
  "authority": "https://timestamp.example.com",
  "proof": "base64-encoded-tst",
  "algorithm": "rfc3161"
}
```

- Cryptographic proof from Time Stamp Authority
- Required for legal/regulatory compliance
- TSA certificate chain validation required

---

## 7. API Reference

### 7.1 Core Library (Rust)

```rust
// Create document
let mut doc = Document::new(title, language, content, styles);
let mut builder = ArchiveBuilder::new(doc);
builder.build_with_timestamp(
    &path,
    Some(&signing_key),
    None, // secp256k1 key
    Some("did:web:signer.example.com".to_string()),
    Some("Signer Name".to_string()),
    None, // default algorithm
    Some(&ManualTimestampProvider),
)?;

// Verify document
let report = ArchiveReader::verify_with_config(
    &path,
    SecurityConfig::for_tier(SizeTier::Standard),
    Some(&revocation_manager),
)?;

// Read document
let (document, merkle_tree, signatures) = ArchiveReader::read(&path)?;

// Extract content
let json = document.to_json()?;
```

### 7.2 CLI Commands

```bash
# Create document
tdf create input.json --output doc.tdf \
  --key signing.key \
  --signer-id "did:web:example.com" \
  --signer-name "Name" \
  --timestamp-manual

# Verify document
tdf verify doc.tdf \
  --key verifying.key \
  --security-tier standard \
  --trusted-signers whitelist.json \
  --revocation-list revoked.cbor \
  --strict

# Extract content
tdf extract doc.tdf --output content.json

# Show info
tdf info doc.tdf

# Export to PDF
tdf export doc.tdf --output doc.pdf

# Generate keys
tdf keygen --output ./keys --name mykey
tdf keygen --secp256k1 --name web3key

# Revoke key
tdf revoke --key-id "did:web:example.com" \
  --reason key-compromise \
  --output revoked.cbor
```

### 7.3 WASM Module

```javascript
import { TdfDocument, TdfVerifier } from 'tdf-wasm';

// Verify document
const verifier = new TdfVerifier();
const result = await verifier.verify(fileBuffer, {
  securityTier: 'standard',
  trustedSigners: ['did:web:trusted.example.com']
});

console.log(result.integrity); // true/false
console.log(result.signatures); // array of signature results

// Extract content
const doc = TdfDocument.fromBuffer(fileBuffer);
const content = doc.extractContent();
const tables = doc.extractTables();
```

---

## 8. Conformance

### 8.1 Implementation Requirements

**MUST** support:
- SHA-256 hash algorithm
- Ed25519 signature algorithm
- CBOR encoding/decoding
- ZIP archive reading/writing
- Merkle tree verification

**SHOULD** support:
- BLAKE3 hash algorithm
- secp256k1 signature algorithm
- RFC 3161 timestamps
- Revocation list checking

**MAY** support:
- Additional signature algorithms
- Custom metadata schemas
- Extended content block types

### 8.2 Validation Test Suite

Reference test vectors available at:
- `tests/vectors/valid/` - Valid documents for positive testing
- `tests/vectors/invalid/` - Invalid documents for negative testing
- `tests/vectors/attacks/` - Attack scenarios for security testing

---

## 9. References

- RFC 8949: Concise Binary Object Representation (CBOR)
- RFC 8032: Edwards-Curve Digital Signature Algorithm (EdDSA)
- RFC 3161: Internet X.509 Time-Stamp Protocol (TSP)
- FIPS 180-4: Secure Hash Standard (SHA-256)
- SEC 2: Recommended Elliptic Curve Domain Parameters (secp256k1)
- W3C DID Core: Decentralized Identifiers

---

## Appendix A: CBOR Schema Definitions

```cddl
manifest = {
  schema_version: tstr,
  id: tstr,
  title: tstr,
  language: tstr,
  created: tstr,
  modified: tstr,
  ? authors: [* author],
  integrity: integrity-info,
  ? content_type: tstr,
  ? keywords: [* tstr],
  ? custom_metadata: any
}

author = {
  id: tstr,
  name: tstr,
  ? role: tstr
}

integrity-info = {
  algorithm: "sha256" / "blake3",
  root_hash: tstr
}

signature-block = {
  signatures: [* signature]
}

signature = {
  signer: signer-info,
  algorithm: "ed25519" / "secp256k1",
  public_key: tstr,
  signature: tstr,
  timestamp: timestamp-info,
  scope: "full" / "partial",
  ? metadata: any
}
```

---

## Appendix B: Example Documents

### Minimal Valid TDF

```
minimal.tdf
├── manifest.cbor (145 bytes)
├── content.cbor (89 bytes)
└── merkle.cbor (234 bytes)
```

Total size: ~800 bytes (compressed)

### Signed Financial Report

```
financial-report.tdf
├── manifest.cbor (312 bytes)
├── content.cbor (4,521 bytes)
├── styles.css (1,200 bytes)
├── merkle.cbor (456 bytes)
└── signatures.cbor (892 bytes)
```

Total size: ~3.2 KB (compressed)

---

*End of Technical Specification*
