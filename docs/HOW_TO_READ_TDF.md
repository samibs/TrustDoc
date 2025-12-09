# How to Read a TDF File

There are several ways to read and access TDF files, depending on your use case.

## 1. Using the CLI Tool (Easiest)

### View Document Information
```bash
tdf info document.tdf
```

Shows:
- Document title, ID, dates
- Integrity hash
- Content structure (sections, blocks)
- Signature information

### Extract Structured Data
```bash
tdf extract document.tdf -o data.json
```

Extracts:
- Metadata (title, ID, dates)
- All tables as structured JSON
- Content blocks

### Verify Integrity
```bash
# Basic integrity check
tdf verify document.tdf

# With signature verification
tdf verify document.tdf --key verifying-key.verifying
```

## 2. Programmatically in Rust

### Basic Reading
```rust
use tdf_core::archive::ArchiveReader;
use tdf_core::error::TdfResult;

fn read_tdf(path: &Path) -> TdfResult<()> {
    // Read document
    let (document, merkle_tree, signature_block) = ArchiveReader::read(path)?;
    
    // Access document metadata
    println!("Title: {}", document.manifest.document.title);
    println!("ID: {}", document.manifest.document.id);
    println!("Root Hash: {}", document.manifest.integrity.root_hash);
    
    // Access content
    for section in &document.content.sections {
        println!("Section: {}", section.title.as_deref().unwrap_or("Untitled"));
        for block in &section.content {
            match block {
                tdf_core::content::ContentBlock::Paragraph { text, .. } => {
                    println!("  Paragraph: {}", text);
                }
                tdf_core::content::ContentBlock::Table { columns, rows, .. } => {
                    println!("  Table with {} columns, {} rows", columns.len(), rows.len());
                }
                _ => {}
            }
        }
    }
    
    // Access signatures
    if let Some(sig_block) = &signature_block {
        for sig in &sig_block.signatures {
            println!("Signed by: {} ({})", sig.signer.name, sig.signer.id);
        }
    }
    
    Ok(())
}
```

### Verify Document
```rust
use tdf_core::archive::ArchiveReader;
use ed25519_dalek::VerifyingKey;

fn verify_tdf(path: &Path, verifying_key: &VerifyingKey) -> TdfResult<()> {
    let (document, merkle_tree, signature_block) = ArchiveReader::read(path)?;
    
    // Verify integrity
    let is_valid = ArchiveReader::verify(path)?;
    println!("Integrity: {}", if is_valid { "VALID" } else { "INVALID" });
    
    // Verify signatures
    if let Some(sig_block) = &signature_block {
        let root_hash = hex::decode(&document.manifest.integrity.root_hash)?;
        let keys = vec![("did:web:example.com".to_string(), verifying_key.clone())];
        
        let results = tdf_core::signature::SignatureManager::verify_signature_block(
            sig_block,
            &root_hash,
            &keys,
        )?;
        
        for result in results {
            match result {
                tdf_core::signature::VerificationResult::Valid { signer, timestamp } => {
                    println!("✓ {} signed at {}", signer, timestamp);
                }
                tdf_core::signature::VerificationResult::Invalid { signer, reason } => {
                    println!("✗ {}: {}", signer, reason);
                }
                _ => {}
            }
        }
    }
    
    Ok(())
}
```

## 3. Programmatically in TypeScript/JavaScript

### Using the TypeScript SDK
```typescript
import { loadDocument, extractData } from 'tdf-ts';

// Load document from File object (browser)
async function readTdfFile(file: File) {
    const doc = await loadDocument(file);
    
    console.log('Title:', doc.manifest.document.title);
    console.log('ID:', doc.manifest.document.id);
    console.log('Root Hash:', doc.manifest.integrity.root_hash);
    
    // Access content
    doc.content.sections.forEach(section => {
        console.log('Section:', section.title);
        section.content.forEach(block => {
            if (block.type === 'paragraph') {
                console.log('  Paragraph:', block.text);
            } else if (block.type === 'table') {
                console.log('  Table:', block.columns.length, 'columns');
            }
        });
    });
    
    // Extract structured data
    const extracted = extractData(doc);
    console.log('Tables:', extracted.tables);
    console.log('Metadata:', extracted.metadata);
}

// Load from URL
async function readTdfFromUrl(url: string) {
    const response = await fetch(url);
    const blob = await response.blob();
    const file = new File([blob], 'document.tdf');
    await readTdfFile(file);
}
```

### Node.js Example
```typescript
import * as fs from 'fs';
import { loadDocument } from 'tdf-ts';

async function readTdfNode(path: string) {
    const buffer = fs.readFileSync(path);
    const blob = new Blob([buffer]);
    const file = new File([blob], 'document.tdf');
    
    const doc = await loadDocument(file);
    console.log('Document:', doc.manifest.document.title);
}
```

## 4. In the Browser (Web Viewer)

### Using WASM for Verification
```typescript
import init, { verify_document } from './pkg/tdf_wasm.js';

async function verifyInBrowser(file: File) {
    await init();
    
    const arrayBuffer = await file.arrayBuffer();
    const uint8Array = new Uint8Array(arrayBuffer);
    
    const result = verify_document(uint8Array);
    
    console.log('Integrity Valid:', result.integrity_valid());
    console.log('Root Hash:', result.root_hash());
    console.log('Signatures:', result.signatures());
}
```

### Web Viewer Integration
```html
<!DOCTYPE html>
<html>
<head>
    <script type="module" src="tdf-viewer.js"></script>
</head>
<body>
    <input type="file" id="tdf-file" accept=".tdf" />
    <div id="viewer"></div>
    
    <script type="module">
        import { loadDocument, renderDocument } from './tdf-viewer.js';
        
        document.getElementById('tdf-file').addEventListener('change', async (e) => {
            const file = e.target.files[0];
            if (file) {
                const doc = await loadDocument(file);
                renderDocument(doc, document.getElementById('viewer'));
            }
        });
    </script>
</body>
</html>
```

## 5. Manual Inspection (ZIP Archive)

Since TDF files are ZIP archives, you can inspect them manually:

### Extract and View
```bash
# Extract TDF file
unzip document.tdf -d extracted/

# View structure
ls -lh extracted/

# View manifest (requires CBOR decoder)
# Install: cargo install cbor-cli
cbor decode extracted/manifest.cbor | jq

# View content (requires CBOR decoder)
cbor decode extracted/content.cbor | jq

# View CSS
cat extracted/styles.css

# View signatures
cbor decode extracted/signatures.cbor | jq
```

### Using Python
```python
import zipfile
import cbor2
import json

def read_tdf_manual(tdf_path):
    with zipfile.ZipFile(tdf_path, 'r') as zip_file:
        # Read manifest
        manifest_data = zip_file.read('manifest.cbor')
        manifest = cbor2.loads(manifest_data)
        print("Title:", manifest['document']['title'])
        print("Root Hash:", manifest['integrity']['root_hash'])
        
        # Read content
        content_data = zip_file.read('content.cbor')
        content = cbor2.loads(content_data)
        print("Sections:", len(content['sections']))
        
        # Read styles
        styles = zip_file.read('styles.css').decode('utf-8')
        print("CSS:", len(styles), "bytes")
        
        # Read signatures
        if 'signatures.cbor' in zip_file.namelist():
            sig_data = zip_file.read('signatures.cbor')
            signatures = cbor2.loads(sig_data)
            print("Signatures:", len(signatures['signatures']))
```

## 6. Quick Examples

### Read and Print Title
```bash
tdf info document.tdf | grep "Title:"
```

### Extract Tables to CSV
```bash
tdf extract document.tdf -o data.json
# Then use jq to convert to CSV
jq -r '.tables."table-id".rows[] | @csv' data.json
```

### Verify and Show Signers
```bash
tdf verify document.tdf | grep "✓"
```

## 7. Complete Example: Read Invoice

```rust
use tdf_core::archive::ArchiveReader;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("invoice.tdf");
    let (doc, _, sig_block) = ArchiveReader::read(path)?;
    
    // Print invoice info
    println!("Invoice: {}", doc.manifest.document.title);
    println!("Created: {}", doc.manifest.document.created);
    
    // Find and print invoice table
    for section in &doc.content.sections {
        for block in &section.content {
            if let tdf_core::content::ContentBlock::Table { columns, rows, .. } = block {
                println!("\nInvoice Items:");
                // Print header
                for col in columns {
                    print!("{:20} ", col.header);
                }
                println!();
                
                // Print rows
                for row in rows {
                    for col in columns {
                        if let Some(cell) = row.cells.get(&col.id) {
                            let value = match cell {
                                tdf_core::content::CellValue::Text(s) => s.clone(),
                                tdf_core::content::CellValue::Currency { display, .. } => display.clone(),
                                _ => format!("{:?}", cell),
                            };
                            print!("{:20} ", value);
                        }
                    }
                    println!();
                }
            }
        }
    }
    
    Ok(())
}
```

## Summary

- **CLI**: Use `tdf info`, `tdf extract`, `tdf verify` for quick access
- **Rust**: Use `ArchiveReader::read()` for full programmatic access
- **TypeScript**: Use `loadDocument()` from `tdf-ts` SDK
- **Browser**: Use WASM bindings or web viewer
- **Manual**: Extract ZIP and decode CBOR files

Choose the method that best fits your use case!

