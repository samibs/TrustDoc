# TDF (TrustDoc Financial) Format

An open document format designed for financial and corporate documents requiring cryptographic integrity verification.

## Overview

TDF is a tamper-evident document format that provides:
- **Cryptographic integrity**: Any modification is detectable via Merkle tree verification
- **Semantic content**: Structured data that's easy to extract
- **Print fidelity**: Deterministic rendering for legal/compliance use
- **Open format**: No vendor lock-in, fully open specification

## Project Structure

```
TrustDoc/
├── Cargo.toml           # Rust workspace configuration
├── README.md            # This file
├── docs/                # Documentation (SPEC.md, guides, etc.)
├── examples/            # Sample documents and test files
├── scripts/             # Automation scripts and keys
├── tdf-core/            # Rust core library
├── tdf-cli/             # CLI tool
├── tdf-ts/              # TypeScript SDK
├── tdf-viewer/          # Web viewer
└── tdf-wasm/            # WASM bindings
```

## Quick Start

### Building

```bash
# Build Rust components
cargo build --workspace

# Build WASM bindings (for browser verification)
cd tdf-wasm
wasm-pack build --target web --out-dir pkg
cd ..

# Build TypeScript SDK
cd tdf-ts
npm install
npm run build
cd ..

# Build web viewer
cd tdf-viewer
npm install
npm run build
cd ..
```

### Using the CLI

```bash
# Create a TDF document from JSON
tdf create examples/quarterly-report.json -o report.tdf \
  --signer-id "did:web:cfo.acme.com" \
  --signer-name "Jane Smith" \
  --key signing-key.bin

# Verify document integrity
tdf verify report.tdf

# Extract structured data
tdf extract report.tdf -o data.json

# Show document info
tdf info report.tdf
```

## Format Specification

See [docs/SPEC.md](docs/SPEC.md) for the complete format specification.

## License

MIT OR Apache-2.0

