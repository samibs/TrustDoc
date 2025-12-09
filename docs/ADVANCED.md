# Advanced TDF Features

## secp256k1 Signatures (Web3 Compatibility)

TDF supports secp256k1 signatures for blockchain and Web3 integration.

### Generate secp256k1 Keys

```bash
tdf keygen --name web3-key --secp256k1
```

This creates:
- `web3-key.secp256k1.signing` - Private key (32 bytes)
- `web3-key.secp256k1.verifying` - Public key (33 bytes, compressed)

### Sign with secp256k1

```rust
use tdf_core::archive::ArchiveBuilder;
use k256::ecdsa::SigningKey;

let signing_key = // Load secp256k1 key
let mut builder = ArchiveBuilder::new(document);
builder.build_with_secp256k1(
    &output_path,
    None,  // No Ed25519 key
    Some(&signing_key),
    Some("did:web:example.com".to_string()),
    Some("Signer Name".to_string()),
    Some(crate::signature::SignatureAlgorithm::Secp256k1),
)?;
```

### Verify secp256k1 Signatures

```rust
use tdf_core::signature::SignatureManager;
use k256::ecdsa::VerifyingKey;

let verifying_key = // Load secp256k1 public key
let keys_secp256k1 = vec![("did:web:example.com".to_string(), verifying_key)];
let results = SignatureManager::verify_signature_block_mixed(
    &signature_block,
    &root_hash,
    &[],  // No Ed25519 keys
    &keys_secp256k1,
)?;
```

## PDF Export

Export TDF documents to PDF for printing and archival.

```bash
tdf export document.tdf -o output.pdf
```

### PDF Export Features

- Preserves document structure
- Renders tables and text
- Basic formatting (fonts, spacing)
- A4 page size by default

**Note**: Current implementation is basic. Full formatting (diagrams, complex layouts) requires additional work.

## Performance Considerations

### Large Documents

TDF handles documents up to 50MB (Extended tier). For optimal performance:

1. **Use Modern Formats**: WebP/AVIF for images, WOFF2 for fonts
2. **Subset Fonts**: Only include used characters
3. **Optimize Images**: Compress before embedding
4. **Limit History**: Use `snapshot` mode for version chains

### Verification Performance

- **Small documents (<1MB)**: <10ms verification
- **Medium documents (1-5MB)**: 10-50ms verification
- **Large documents (5-50MB)**: 50-200ms verification

Merkle tree computation is O(n log n) where n is the number of components.

## Multi-Party Signatures

Multiple parties can sign the same document:

```rust
// First signature
let sig1 = SignatureManager::sign_ed25519(key1, &root_hash, ...);
signatures.push(sig1);

// Second signature (same root hash)
let sig2 = SignatureManager::sign_ed25519(key2, &root_hash, ...);
signatures.push(sig2);
```

All signatures are stored in `signatures.cbor` and can be verified independently.

## Signature Scopes

Control what part of the document is signed:

- **Full**: Entire document (default)
- **ContentOnly**: Content and styles, not metadata
- **Sections**: Specific sections only

```rust
let signature = SignatureManager::sign_ed25519(
    key,
    &root_hash,
    signer_id,
    signer_name,
    SignatureScope::Sections(vec!["sec-1".to_string(), "sec-2".to_string()]),
);
```

## Browser Integration

### WASM Verification

```javascript
import init, { verify_document } from './pkg/tdf_wasm.js';

await init();

const file = // File object
const arrayBuffer = await file.arrayBuffer();
const uint8Array = new Uint8Array(arrayBuffer);

const result = verify_document(uint8Array);
console.log('Valid:', result.integrity_valid());
console.log('Hash:', result.root_hash());
```

### TypeScript SDK

```typescript
import { loadDocument, extractData } from 'tdf-ts';

const doc = await loadDocument(file);
const extracted = extractData(doc);
console.log(extracted.tables);
```

## Best Practices

### Key Management

1. **Store signing keys securely**: Encrypted, access-controlled
2. **Backup keys**: Multiple secure backups
3. **Key rotation**: Generate new keys periodically
4. **Revocation**: Maintain revocation lists

### Document Creation

1. **Validate input**: Check JSON structure before creating
2. **Optimize assets**: Compress images, subset fonts
3. **Use semantic IDs**: Meaningful IDs for extraction
4. **Include metadata**: Author, classification, etc.

### Verification

1. **Always verify**: Never trust unverified documents
2. **Check signatures**: Verify signer identity
3. **Validate timestamps**: Check timestamp authority proofs
4. **Audit trail**: Log all verifications

## Future Enhancements

- RFC 3161 timestamp authority integration
- Enhanced PDF export with full formatting
- Streaming for very large documents
- BLS signature aggregation
- Zero-knowledge proofs for selective disclosure

