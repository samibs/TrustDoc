# TDF Developer Guide

Complete guide for developers working with the TDF format and ecosystem.

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Project Structure](#project-structure)
3. [Core Concepts](#core-concepts)
4. [Building from Source](#building-from-source)
5. [Development Workflow](#development-workflow)
6. [Testing](#testing)
7. [Code Organization](#code-organization)
8. [Adding Features](#adding-features)
9. [Debugging](#debugging)
10. [Performance Considerations](#performance-considerations)

## Architecture Overview

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    TDF Ecosystem                         │
├─────────────────────────────────────────────────────────┤
│                                                           │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │   tdf-core   │  │   tdf-cli   │  │   tdf-ts     │  │
│  │   (Rust)     │  │   (Rust)    │  │ (TypeScript) │  │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘  │
│         │                 │                 │          │
│         └─────────────────┼─────────────────┘          │
│                           │                            │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ │
│  │  tdf-wasm    │  │ tdf-viewer   │  │ tdf-convert  │ │
│  │  (WASM)      │  │   (Web)      │  │   (Rust)     │ │
│  └──────────────┘  └──────────────┘  └──────────────┘ │
│                                                           │
│  ┌──────────────┐  ┌──────────────┐                     │
│  │ tdf-desktop  │  │  tdf-mobile  │                     │
│  │  (Tauri)     │  │ (React Native)│                    │
│  └──────────────┘  └──────────────┘                     │
└─────────────────────────────────────────────────────────┘
```

### Core Library (`tdf-core`)

The core library provides:

- **Document Model**: Structured representation of TDF documents
- **Archive Operations**: ZIP-based file format handling
- **Cryptography**: Signatures, hashing, Merkle trees
- **Verification**: Integrity and signature verification
- **Multi-party**: Workflow management for multiple signers
- **Security**: Revocation, timestamp validation, DoS protection

### Module Structure

```
tdf-core/
├── src/
│   ├── lib.rs           # Public API exports
│   ├── document.rs      # Document and manifest structures
│   ├── content.rs       # Content blocks (text, tables, diagrams)
│   ├── archive.rs        # ZIP archive operations
│   ├── merkle.rs        # Merkle tree implementation
│   ├── signature.rs     # Cryptographic signatures
│   ├── timestamp.rs     # Timestamp authority
│   ├── multiparty.rs    # Multi-party workflows
│   ├── revocation.rs   # Key revocation
│   ├── config.rs        # Security configuration
│   └── error.rs         # Error types
```

## Project Structure

### Workspace Layout

```
TrustDoc/
├── Cargo.toml              # Workspace configuration
├── README.md               # Project overview
├── docs/                   # Documentation
├── examples/               # Example documents
├── scripts/                # Build and utility scripts
├── tdf-core/               # Core Rust library
├── tdf-cli/                # Command-line tool
├── tdf-ts/                 # TypeScript SDK
├── tdf-wasm/               # WASM bindings
├── tdf-viewer/             # Web viewer
├── tdf-desktop-viewer/     # Tauri desktop app
├── tdf-mobile/             # React Native mobile app
└── tdf-convert/            # Format conversion library
```

### Key Directories

- **`tdf-core/`**: Core library implementation
- **`tdf-cli/src/commands/`**: CLI command implementations
- **`tdf-ts/src/`**: TypeScript SDK source
- **`tdf-wasm/src/`**: WASM bindings
- **`docs/`**: All documentation
- **`examples/`**: Sample TDF documents

## Core Concepts

### Document Lifecycle

1. **Creation**: Build a `Document` with content
2. **Archiving**: Create ZIP archive with `ArchiveBuilder`
3. **Signing**: Add cryptographic signatures
4. **Verification**: Verify integrity and signatures
5. **Extraction**: Extract structured data

### Merkle Tree Integrity

Every TDF document uses a Merkle tree to ensure integrity:

```
Root Hash
├── manifest.cbor hash
├── content.cbor hash
├── styles.css hash
└── assets/ directory hash
    ├── image1.webp hash
    └── image2.png hash
```

Any modification to any component invalidates the root hash.

### Signature Verification

Signatures are verified against:
1. The Merkle root hash
2. The signer's public key
3. The revocation list (if present)
4. The timestamp (if present)

### Multi-Party Workflows

- **Unordered**: Signers can sign in any order
- **Ordered**: Signers must sign in a specific sequence
- **Workflow State**: Tracks completion status

## Building from Source

### Prerequisites

```bash
# Rust (1.70+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Node.js (20+)
# Install from nodejs.org or use nvm

# WASM tools
cargo install wasm-pack

# System dependencies (Linux)
sudo apt-get install -y \
    libwebkit2gtk-4.1-dev \
    build-essential \
    pkg-config \
    libssl-dev
```

### Build Commands

```bash
# Build all Rust components
cargo build --workspace

# Build with optimizations
cargo build --workspace --release

# Build specific component
cargo build -p tdf-core
cargo build -p tdf-cli

# Build WASM bindings
cd tdf-wasm
wasm-pack build --target web --out-dir pkg

# Build TypeScript SDK
cd tdf-ts
npm install
npm run build

# Build web viewer
cd tdf-viewer
npm install
npm run build
```

## Development Workflow

### Setting Up Development Environment

```bash
# Clone repository
git clone <repository-url>
cd TrustDoc

# Install dependencies
cargo build --workspace
cd tdf-ts && npm install && cd ..
cd tdf-viewer && npm install && cd ..
cd tdf-wasm && wasm-pack build --target web && cd ..

# Run tests
cargo test --workspace
```

### Code Style

- **Rust**: Follow `rustfmt` defaults
- **TypeScript**: Use ESLint and Prettier
- **Documentation**: All public APIs must be documented
- **Tests**: Aim for 80%+ coverage

### Git Workflow

1. Create feature branch: `git checkout -b feature/my-feature`
2. Make changes and commit
3. Run tests: `cargo test --workspace`
4. Update documentation
5. Create pull request

## Testing

### Running Tests

```bash
# All tests
cargo test --workspace

# Specific crate
cargo test -p tdf-core

# With output
cargo test --workspace -- --nocapture

# Security tests
cargo test -p tdf-core --test security_tests
cargo test -p tdf-core --test e2e_security_tests
cargo test -p tdf-core --test security_hardening_tests
cargo test -p tdf-core --test brute_force_attack_tests
```

### Test Structure

```
tdf-core/tests/
├── security_tests.rs           # Unit security tests
├── e2e_security_tests.rs      # End-to-end security tests
├── security_hardening_tests.rs # Hardening feature tests
└── brute_force_attack_tests.rs  # Attack simulation tests
```

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Arrange
        let input = create_test_input();
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert!(result.is_ok());
    }
}
```

## Code Organization

### Module Responsibilities

- **`document.rs`**: Document structure, manifest, metadata
- **`content.rs`**: Content blocks, tables, diagrams
- **`archive.rs`**: ZIP operations, file I/O
- **`merkle.rs`**: Hash computation, tree building
- **`signature.rs`**: Signature creation and verification
- **`timestamp.rs`**: Timestamp authority integration
- **`multiparty.rs`**: Workflow management
- **`revocation.rs`**: Key revocation lists
- **`config.rs`**: Security configuration
- **`error.rs`**: Error types and handling

### Error Handling

All functions return `TdfResult<T>`:

```rust
use tdf_core::error::{TdfError, TdfResult};

fn my_function() -> TdfResult<()> {
    if condition {
        Err(TdfError::InvalidDocument("reason".to_string()))
    } else {
        Ok(())
    }
}
```

### Public API Design

- Keep public API minimal and stable
- Use builder patterns for complex construction
- Provide both high-level and low-level APIs
- Document all public items

## Adding Features

### Feature Checklist

1. **Design**: Document the feature design
2. **Implementation**: Write code following patterns
3. **Tests**: Add unit and integration tests
4. **Documentation**: Update API docs and guides
5. **Security**: Consider security implications
6. **Performance**: Benchmark if performance-critical

### Example: Adding a New Content Block Type

1. **Update `content.rs`**:
```rust
pub enum ContentBlock {
    // ... existing variants
    NewBlockType {
        id: Option<String>,
        data: NewBlockData,
    },
}
```

2. **Update serialization** (if needed):
```rust
impl Serialize for ContentBlock {
    // Handle new variant
}
```

3. **Add tests**:
```rust
#[test]
fn test_new_block_type() {
    // Test creation, serialization, extraction
}
```

4. **Update documentation**:
   - API.md
   - SPEC.md (if format change)
   - Examples

## Debugging

### Common Issues

1. **ZIP Archive Errors**: Check file paths and permissions
2. **Signature Verification Failures**: Verify key format and algorithm
3. **Merkle Tree Mismatches**: Ensure all components are included
4. **CBOR Serialization Errors**: Check data types match schema

### Debug Tools

```bash
# Enable debug logging
RUST_LOG=debug cargo test

# Inspect TDF file
unzip -l document.tdf
cbor2json < document.tdf/manifest.cbor

# CLI debug mode
tdf verify document.tdf --verbose
```

### Debugging Tips

- Use `println!` or `dbg!` for quick debugging
- Enable `RUST_BACKTRACE=1` for stack traces
- Use `cargo test -- --nocapture` to see test output
- Check `target/debug/` for build artifacts

## Performance Considerations

### Optimization Targets

1. **Archive Creation**: Minimize file I/O
2. **Merkle Tree**: Parallel hash computation
3. **Signature Verification**: Batch operations
4. **CBOR Parsing**: Use streaming for large files

### Benchmarking

```bash
# Run benchmarks
cargo bench

# Profile with perf (Linux)
perf record cargo test
perf report
```

### Memory Management

- Use `Vec<u8>` for binary data
- Avoid unnecessary cloning
- Use references where possible
- Consider streaming for large files

## Next Steps

- Read [ARCHITECTURE.md](ARCHITECTURE.md) for detailed architecture
- Review [API.md](API.md) for API reference
- Check [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines
- See [TROUBLESHOOTING.md](TROUBLESHOOTING.md) for common issues

---

*Last updated: 2025-12-09*

