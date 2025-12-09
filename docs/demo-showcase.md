# TDF Format Demo Showcase

## 📄 Document Structure

A TDF file is a ZIP archive containing:

```
demo-invoice.tdf (ZIP archive)
├── manifest.cbor          # Document metadata and integrity info
├── content.cbor           # Structured content (sections, blocks)
├── styles.css             # Styling rules
├── signatures.cbor        # Cryptographic signatures
└── [assets/]              # Optional: images, fonts, etc.
```

## 🔐 Cryptographic Integrity

Every TDF document has:
- **Merkle Tree Root Hash**: SHA-256 hash of all components
- **Tamper-Evident**: Any modification invalidates the hash
- **Signatures**: Ed25519 or secp256k1 (Web3 compatible)

## 📊 Content Types

TDF supports rich structured content:

### Text Blocks
- Headings (H1-H4)
- Paragraphs
- Lists (ordered/unordered)
- Footnotes

### Tables
- Typed cells (text, number, currency, percentage, date)
- Headers and captions
- Structured data extraction

### Diagrams
- Hierarchical (org charts)
- Flowcharts
- Relationship diagrams
- Rendered as SVG in web viewer

### Figures
- Images with captions
- Supports WebP, AVIF, PNG

## 🛠️ CLI Commands

### Create Document
```bash
tdf create input.json -o output.tdf \
  --key signing-key.signing \
  --signer-id "did:web:example.com" \
  --signer-name "Your Name"
```

### Verify Document
```bash
# Basic integrity check
tdf verify document.tdf

# With signature verification
tdf verify document.tdf --key verifying-key.verifying
```

### Extract Data
```bash
tdf extract document.tdf -o data.json
# Extracts tables, metadata, structured content
```

### Export to PDF
```bash
tdf export document.tdf -o output.pdf
# Creates printable PDF with formatting
```

### Multi-Party Workflow
```bash
# Create workflow
tdf workflow create document.tdf \
  --order ordered \
  --signers "signer1,signer2,signer3" \
  -o workflow.json

# Check status
tdf workflow status workflow.json
```

### Key Generation
```bash
# Ed25519 (default)
tdf keygen --name my-key

# secp256k1 (Web3 compatible)
tdf keygen --name web3-key --secp256k1
```

## 🌐 Browser Support

- **WASM Verification**: Client-side cryptographic verification
- **Web Viewer**: Drag-and-drop file viewing
- **TypeScript SDK**: Full browser integration
- **No Server Required**: All verification happens client-side

## 📋 Example: Invoice

```json
{
  "title": "Invoice #INV-2024-001",
  "sections": [{
    "title": "Invoice Details",
    "content": [
      {
        "type": "table",
        "columns": [
          {"id": "item", "header": "Item", "type": "text"},
          {"id": "qty", "header": "Quantity", "type": "number"},
          {"id": "price", "header": "Price", "type": "currency"}
        ],
        "rows": [
          {
            "cells": {
              "item": {"type": "text", "value": "Consulting Services"},
              "qty": {"type": "number", "value": 40, "display": "40"},
              "price": {"type": "currency", "value": 150.00, "display": "$150.00"}
            }
          }
        ]
      }
    ]
  }]
}
```

## 🔒 Security Features

1. **Tamper-Evident**: Merkle tree detects any modification
2. **Cryptographic Signatures**: Ed25519 or secp256k1
3. **Timestamp Authority**: RFC 3161 support
4. **Multi-Party Signing**: Ordered workflows
5. **Offline Verification**: No network required

## 📈 Use Cases

- Financial reports and statements
- Legal documents and contracts
- Invoices and receipts
- Corporate communications
- Audit trails and compliance
- Blockchain-compatible documents

## 🎯 Key Advantages

- **Open Format**: ZIP + CBOR (no vendor lock-in)
- **Tamper-Evident**: Cryptographic integrity
- **Web3 Ready**: secp256k1 signatures
- **Printable**: PDF export
- **Extractable**: Structured data extraction
- **Browser Compatible**: WASM verification
- **Size Controlled**: Efficient encoding

