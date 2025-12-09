# 🎯 TDF Format - Visual Demonstration

## 📦 What a TDF File Looks Like

A TDF file is a **ZIP archive** that you can open with any ZIP tool:

```
demo-invoice.tdf (3,015 bytes)
├── manifest.cbor      (308 bytes)  - Document metadata & integrity
├── content.cbor       (1,838 bytes) - Structured content
├── styles.css         (412 bytes)  - CSS styling
├── hashes.bin         (138 bytes)  - Merkle tree hashes
└── signatures.cbor    (319 bytes)  - Cryptographic signatures
```

## 🔍 Document Information

```
TDF Document Information
=======================
Title: Invoice #INV-2025-0427
ID: fe17a4ed-6f83-4ee8-bd60-c398f9bf6c64
Language: en
Created: 2025-12-07 22:58:11 UTC
Modified: 2025-12-07 22:58:11 UTC

Integrity:
  Algorithm: SHA-256
  Root Hash: 4ec3c6c7a17c1a21e649dcad4010b2ac104cc61c491f485843bd23eaaa369566

Content:
  Sections: 4
    - Invoice (2 blocks)
    - Parties (2 blocks)
    - Items (1 blocks)
    - Totals (2 blocks)

Signatures: 1
  ✓ ACME Corp (did:web:acme.com)
    Algorithm: Ed25519
    Timestamp: 2025-12-07 22:58:11 UTC
```

## ✅ Verification Results

```
Verification Report
==================
Document: demo-invoice.tdf
Integrity: ✓ VALID
Root Hash: 4ec3c6c7a17c1a21e649dcad4010b2ac104cc61c491f485843bd23eaaa369566
Signatures: 1

Signature Verification:
  ✓ ACME Corp (signed at 2025-12-07 22:58:11 UTC)
```

## 📊 Extracted Data

The `extract` command pulls structured data from tables:

```json
{
  "metadata": {
    "title": "Invoice #INV-2025-0427",
    "id": "fe17a4ed-6f83-4ee8-bd60-c398f9bf6c64",
    "created": "2025-12-07T22:58:11Z"
  },
  "tables": {
    "tbl-items": {
      "columns": [
        {"id": "description", "header": "Description", "type": "text"},
        {"id": "quantity", "header": "Quantity", "type": "number"},
        {"id": "unit-price", "header": "Unit Price", "type": "currency"},
        {"id": "total", "header": "Total", "type": "currency"}
      ],
      "rows": [
        {
          "description": "Software License - Enterprise",
          "quantity": 10,
          "unit-price": "€2,500.00",
          "total": "€25,000.00"
        }
      ]
    }
  }
}
```

## 🔄 Multi-Party Workflow

### Create Workflow
```bash
$ tdf workflow create demo-invoice.tdf \
    --order unordered \
    --signers "did:web:acme.com,did:web:client.com" \
    -o invoice-workflow.json

Created signing workflow: invoice-workflow.json
  Document ID: fe17a4ed-6f83-4ee8-bd60-c398f9bf6c64
  Order: Unordered
  Required signers: 2
```

### Workflow Status
```bash
$ tdf workflow status invoice-workflow.json

Signing Workflow Status
======================
Workflow ID: 9577797b-3ac4-4079-bbe0-7f889ae567ea
Document ID: fe17a4ed-6f83-4ee8-bd60-c398f9bf6c64
Order: Unordered
Status: InProgress { signed_count: 0, total: 2 }

Required Signers:
  1. did:web:acme.com
  2. did:web:client.com

Progress: 0/2 signatures
Next signer: did:web:acme.com
```

## 🔑 Key Generation

### Ed25519 (Default)
```bash
$ tdf keygen --name demo

Signing key (private) written to: ./demo.signing
  ⚠️  Keep this file secure and never share it!
Verifying key (public) written to: ./demo.verifying
  ✓  This file can be shared publicly

Key Information:
  Signing key size: 32 bytes
  Verifying key size: 32 bytes
```

### secp256k1 (Web3 Compatible)
```bash
$ tdf keygen --name web3-demo --secp256k1

Signing key (private) written to: ./web3-demo.secp256k1.signing
  ⚠️  Keep this file secure and never share it!
Verifying key (public) written to: ./web3-demo.secp256k1.verifying
  ✓  This file can be shared publicly

Key Information:
  Algorithm: secp256k1 (Web3 compatible)
  Signing key size: 32 bytes
  Verifying key size: 33 bytes
```

## 📄 PDF Export

```bash
$ tdf export demo-invoice.tdf -o demo-invoice.pdf

Exported PDF to: demo-invoice.pdf
Note: This is a basic export. Full formatting requires additional implementation.

$ file demo-invoice.pdf
demo-invoice.pdf: PDF document, version 1.3, 1 page(s)

$ ls -lh demo-invoice.pdf
-rw-r--r-- 1 user user 2.4K Dec  7 23:58 demo-invoice.pdf
```

## 🛠️ Complete CLI Command Set

```
TDF (TrustDoc Financial) format tool

Commands:
  create    Create a new TDF document from JSON input
  verify    Verify integrity and signatures of a TDF document
  extract   Extract structured data from a TDF document
  info      Show metadata and signature information
  export    Export TDF document to PDF
  workflow  Multi-party signing workflow
  keygen    Generate a new Ed25519 keypair for signing
```

## 🎨 Content Types Supported

### 1. Text Blocks
- **Headings**: H1, H2, H3, H4
- **Paragraphs**: Multi-line text
- **Lists**: Ordered and unordered
- **Footnotes**: Reference notes

### 2. Tables
- **Typed Cells**: text, number, currency, percentage, date
- **Headers**: Column headers
- **Captions**: Table descriptions
- **Footers**: Summary rows

### 3. Diagrams
- **Hierarchical**: Org charts
- **Flowcharts**: Process flows
- **Relationship**: Entity relationships
- **Rendered as SVG** in web viewer

### 4. Figures
- **Images**: WebP, AVIF, PNG
- **Captions**: Image descriptions

## 🔐 Security Features

1. **Merkle Tree**: SHA-256 hash tree detects any modification
2. **Cryptographic Signatures**: Ed25519 or secp256k1
3. **Timestamp Authority**: RFC 3161 support (optional)
4. **Multi-Party Signing**: Ordered workflows
5. **Offline Verification**: No network required

## 📈 File Sizes

- **Small Document** (< 1MB): Typical invoice, ~3KB
- **Medium Document** (1-5MB): Financial report with tables
- **Large Document** (5-50MB): Extended reports with images

## 🌐 Browser Integration

- **WASM Verification**: Client-side cryptographic verification
- **Web Viewer**: Drag-and-drop file viewing
- **TypeScript SDK**: Full browser integration
- **No Server Required**: All verification happens client-side

## 🎯 Use Cases

✅ Financial reports and statements  
✅ Legal documents and contracts  
✅ Invoices and receipts  
✅ Corporate communications  
✅ Audit trails and compliance  
✅ Blockchain-compatible documents  

