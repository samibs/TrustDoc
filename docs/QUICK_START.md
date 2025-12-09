# TDF Quick Start Guide

Get started with TDF in 5 minutes.

## Installation

### Option 1: From Source (Recommended)

```bash
# Clone repository
git clone <repository-url>
cd TrustDoc

# Build Rust components
cargo build --release

# Install CLI
cargo install --path tdf-cli
```

### Option 2: From Cargo (When Published)

```bash
cargo install tdf-cli
```

## Your First TDF Document

### 1. Create a Simple Document

Create `example.json`:

```json
{
  "title": "My First TDF Document",
  "language": "en",
  "content": {
    "sections": [
      {
        "id": "intro",
        "title": "Introduction",
        "content": [
          {
            "type": "paragraph",
            "text": "This is my first TDF document!"
          }
        ]
      }
    ]
  }
}
```

### 2. Generate Signing Keys

```bash
# Generate Ed25519 key pair
tdf keygen --name my-key

# This creates:
# - my-key.signing (private key)
# - my-key.verifying (public key)
```

### 3. Create TDF Document

```bash
tdf create example.json -o my-document.tdf \
  --key my-key.signing \
  --signer-id "did:web:example.com" \
  --signer-name "Your Name"
```

### 4. Verify Document

```bash
# Basic integrity check
tdf verify my-document.tdf

# With signature verification
tdf verify my-document.tdf --key my-key.verifying
```

### 5. Extract Data

```bash
tdf extract my-document.tdf -o data.json
```

## Using the Web Viewer

1. **Open viewer**: Navigate to viewer URL or open `tdf-viewer/index.html`
2. **Load document**: Drag and drop `my-document.tdf`
3. **View content**: Document renders in browser
4. **Verify**: Check verification status panel

## Using the CLI

### Common Commands

```bash
# Create document
tdf create input.json -o output.tdf --key key.signing

# Verify document
tdf verify document.tdf

# Extract data
tdf extract document.tdf -o data.json

# Show document info
tdf info document.tdf

# Generate keys
tdf keygen --name my-key

# Export to PDF
tdf export document.tdf -o document.pdf
```

### Key Management

```bash
# Generate Ed25519 keys (default)
tdf keygen --name ed25519-key

# Generate secp256k1 keys (Web3 compatible)
tdf keygen --name secp256k1-key --algorithm secp256k1

# List keys (if stored in keyring)
tdf keys list
```

## Programmatic Usage

### Rust

```rust
use tdf_core::{Document, DocumentContent, Section, ContentBlock};
use tdf_core::archive::ArchiveBuilder;
use ed25519_dalek::SigningKey;

// Create document
let content = DocumentContent {
    sections: vec![Section {
        id: "sec-1".to_string(),
        title: Some("Section".to_string()),
        content: vec![ContentBlock::Paragraph {
            text: "Content".to_string(),
            id: None,
        }],
    }],
};

let document = Document::new(
    "Title".to_string(),
    "en".to_string(),
    content,
    "body { }".to_string(),
);

// Create archive
let mut builder = ArchiveBuilder::new(document);
builder.build(
    &std::path::Path::new("output.tdf"),
    Some(&signing_key),
    Some("did:web:example.com".to_string()),
    Some("Name".to_string()),
)?;
```

### TypeScript

```typescript
import { loadDocument } from 'tdf-ts';

// Load document
const file = await fetch('document.tdf').then(r => r.blob());
const document = await loadDocument(file);

// Access content
console.log(document.title);
console.log(document.content.sections);

// Extract data
const data = extractData(document);
```

## Next Steps

- Read [USAGE.md](USAGE.md) for detailed usage
- Check [API.md](API.md) for API reference
- Review [SPEC.md](SPEC.md) for format details
- See [EXAMPLES.md](examples/) for more examples

## Troubleshooting

### Common Issues

**"command not found"**: Install CLI with `cargo install --path tdf-cli`

**"Invalid document"**: Check JSON format matches schema

**"Signature verification failed"**: Ensure correct key used

See [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for more help.

---

*Last updated: 2025-12-09*

