# TDF WASM Bindings

WebAssembly bindings for TDF core library, enabling cryptographic verification in the browser.

## Building

```bash
# Install wasm-pack if needed
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build
./build.sh
# or
wasm-pack build --target web --out-dir pkg
```

## Usage

```javascript
import init, { verify_document, get_document_info } from './pkg/tdf_wasm.js';

await init();

// Verify document from File/Blob
const file = // ... File object
const arrayBuffer = await file.arrayBuffer();
const uint8Array = new Uint8Array(arrayBuffer);

const result = verify_document(uint8Array);
console.log('Integrity valid:', result.integrity_valid());
console.log('Root hash:', result.root_hash());
```

## API

### `verify_document(data: Uint8Array): VerificationResult`

Verifies the integrity of a TDF document.

### `get_document_info(data: Uint8Array): DocumentInfo`

Extracts metadata from a TDF document without full verification.

### `compute_merkle_root(components: Map<string, Uint8Array>): string`

Computes Merkle root hash from components map.

