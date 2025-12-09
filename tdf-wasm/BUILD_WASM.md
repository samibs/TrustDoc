# Building WASM Module

## Prerequisites

To build the WASM module, you need:

1. **Rust toolchain** (already installed)
2. **wasm-pack** (already installed: `cargo install wasm-pack`)
3. **clang** (C compiler) - **Required for native dependencies**

### Install clang

**Ubuntu/Debian/WSL:**
```bash
sudo apt-get update
sudo apt-get install -y clang
```

**macOS:**
```bash
xcode-select --install
```

**Windows (native):**
Install Visual Studio Build Tools or LLVM

## Build WASM

```bash
cd tdf-wasm
RUSTFLAGS='--cfg getrandom_backend="wasm_js"' wasm-pack build --target web --out-dir pkg
```

**Note**: The `RUSTFLAGS` environment variable is required for `getrandom 0.3` to work correctly with WASM.

## Troubleshooting

### Error: "failed to find tool clang"

**Solution**: Install clang (see above)

### Error: "getrandom js feature"

**Solution**: Already fixed in `Cargo.toml` - `getrandom 0.2` has `features = ["js"]`

### Error: "zstd-sys requires clang" or "bzip2-sys requires clang"

**Solution**: Fixed by disabling these features:
- `zip` uses `deflate` only (no bzip2)
- `flate2` uses `rust_backend` (no C code)
- `blake3` has `zstd` disabled

### Error: "uuid RngImp not found" or "getrandom imp not found"

**Solution**: Fixed! Both `getrandom 0.2` (for ed25519-dalek) and `getrandom 0.3` (for uuid) are now configured:
- `getrandom 0.2` with `features = ["js"]`
- `getrandom 0.3` with `features = ["wasm_js"]` (aliased as `getrandom03`)

Build with: `RUSTFLAGS='--cfg getrandom_backend="wasm_js"' wasm-pack build --target web --out-dir pkg`

## Note

The viewer works **without WASM**! It can:
- ✅ View documents
- ✅ Extract data  
- ✅ Basic structure validation
- ⚠️  Full cryptographic verification requires WASM

WASM is **optional** - the viewer is fully functional without it.

