# Building TDF

## Prerequisites

- Rust (latest stable)
- Node.js 18+ and npm
- wasm-pack (for WASM bindings)

## Build Steps

### 1. Rust Core and CLI

```bash
cargo build --release --workspace
```

This builds:
- `tdf-core` - Core library
- `tdf-cli` - Command-line tool
- `tdf-wasm` - WASM bindings (needs separate step)

### 2. WASM Bindings

```bash
# Install wasm-pack if needed
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build WASM
cd tdf-wasm
wasm-pack build --target web --out-dir pkg
cd ..
```

Output: `tdf-wasm/pkg/` directory with WASM files

### 3. TypeScript SDK

```bash
cd tdf-ts
npm install
npm run build
cd ..
```

Output: `tdf-ts/dist/` directory

### 4. Web Viewer

```bash
cd tdf-viewer
npm install
npm run build
cd ..
```

Output: `tdf-viewer/dist/` directory

## Development Mode

### Web Viewer (with hot reload)

```bash
cd tdf-viewer
npm run dev
```

Opens at `http://localhost:5173`

### TypeScript SDK (watch mode)

```bash
cd tdf-ts
npm run watch
```

## Testing

```bash
# Rust tests
cargo test --workspace

# Create test document
cargo run --release --bin tdf -- create examples/quarterly-report.json -o test.tdf

# Verify
cargo run --release --bin tdf -- verify test.tdf
```

## Example Documents

Generate TDF files from examples:

```bash
# Quarterly report
tdf create examples/quarterly-report.json -o quarterly-report.tdf

# Balance sheet
tdf create examples/balance-sheet.json -o balance-sheet.tdf

# Invoice
tdf create examples/invoice.json -o invoice.tdf
```

## Troubleshooting

### WASM build fails

- Ensure wasm-pack is installed: `wasm-pack --version`
- Check Rust target: `rustup target add wasm32-unknown-unknown`

### TypeScript compilation errors

- Run `npm install` in `tdf-ts` and `tdf-viewer`
- Check Node.js version: `node --version` (should be 18+)

### CBOR parsing errors

- Ensure `cbor-web` is installed in `tdf-ts`: `cd tdf-ts && npm install`

