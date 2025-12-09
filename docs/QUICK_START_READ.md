# Quick Start: Reading TDF Files

## 🚀 Fastest Way (CLI)

```bash
# View document info
tdf info document.tdf

# Extract all data to JSON
tdf extract document.tdf -o data.json

# Verify integrity
tdf verify document.tdf
```

## 📋 Real Example

```bash
# Read the demo invoice
$ tdf info demo-invoice.tdf

TDF Document Information
=======================
Title: Invoice #INV-2025-0427
ID: fe17a4ed-6f83-4ee8-bd60-c398f9bf6c64
Created: 2025-12-07 22:58:11 UTC
Root Hash: 4ec3c6c7a17c1a21e649dcad4010b2ac104cc61c491f485843bd23eaaa369566

Content:
  Sections: 4
    - Invoice (2 blocks)
    - Parties (2 blocks)
    - Items (1 blocks)
    - Totals (2 blocks)

Signatures: 1
  ✓ ACME Corp (did:web:acme.com)
```

## 💻 In Your Code

### Rust
```rust
use tdf_core::archive::ArchiveReader;

let (doc, _, sigs) = ArchiveReader::read("document.tdf")?;
println!("Title: {}", doc.manifest.document.title);
```

### TypeScript/JavaScript
```typescript
import { loadDocument } from 'tdf-ts';

const doc = await loadDocument(file);
console.log('Title:', doc.manifest.document.title);
```

### Python (Manual)
```python
import zipfile
import cbor2

with zipfile.ZipFile('document.tdf', 'r') as z:
    manifest = cbor2.loads(z.read('manifest.cbor'))
    print(manifest['document']['title'])
```

## 🔍 What's Inside?

A TDF file is a ZIP archive:
```
document.tdf
├── manifest.cbor    # Metadata & integrity
├── content.cbor      # Your content
├── styles.css        # Styling
├── signatures.cbor   # Signatures
└── hashes.bin        # Merkle tree
```

## 📖 Full Guide

See `docs/HOW_TO_READ_TDF.md` for complete examples.

