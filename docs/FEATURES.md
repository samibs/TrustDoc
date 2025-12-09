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

- Tamper-evident design
- Cryptographic signatures
- Audit trail support
- No vendor lock-in
- Open format specification

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

