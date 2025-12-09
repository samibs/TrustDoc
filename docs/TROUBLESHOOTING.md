# TDF Troubleshooting Guide

Common issues and solutions for working with TDF.

## Table of Contents

1. [Installation Issues](#installation-issues)
2. [Build Issues](#build-issues)
3. [Runtime Issues](#runtime-issues)
4. [Verification Issues](#verification-issues)
5. [Performance Issues](#performance-issues)
6. [Platform-Specific Issues](#platform-specific-issues)
7. [Getting Help](#getting-help)

## Installation Issues

### Rust Installation

**Problem**: `cargo: command not found`

**Solution**:
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Problem**: Rust version too old

**Solution**:
```bash
rustup update
rustc --version  # Should be 1.70+
```

### Node.js Installation

**Problem**: `npm: command not found`

**Solution**:
```bash
# Install Node.js 20+
# Using nvm (recommended)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 20
nvm use 20
```

### System Dependencies (Linux)

**Problem**: Missing GTK/WebKit libraries

**Solution**:
```bash
sudo apt-get update
sudo apt-get install -y \
    libwebkit2gtk-4.1-dev \
    build-essential \
    pkg-config \
    libssl-dev \
    libgtk-3-dev
```

## Build Issues

### Cargo Build Fails

**Problem**: `error: failed to compile`

**Solution**:
```bash
# Clean build
cargo clean
cargo build --workspace

# Check for specific errors
cargo build -p tdf-core 2>&1 | grep error
```

**Problem**: `error: linker 'cc' not found`

**Solution**:
```bash
# Install build tools
sudo apt-get install build-essential  # Linux
xcode-select --install  # macOS
```

### WASM Build Issues

**Problem**: `wasm-pack: command not found`

**Solution**:
```bash
cargo install wasm-pack
```

**Problem**: WASM build fails

**Solution**:
```bash
# Clean and rebuild
cd tdf-wasm
rm -rf pkg target
wasm-pack build --target web --out-dir pkg
```

### TypeScript Build Issues

**Problem**: Type errors in TypeScript

**Solution**:
```bash
cd tdf-ts
npm install
npm run build

# Check specific errors
npx tsc --noEmit
```

## Runtime Issues

### CLI Command Not Found

**Problem**: `tdf: command not found`

**Solution**:
```bash
# Build and install
cargo build --release -p tdf-cli
sudo cp target/release/tdf /usr/local/bin/

# Or use cargo install
cargo install --path tdf-cli
```

### File Not Found Errors

**Problem**: `Error: No such file or directory`

**Solution**:
```bash
# Check file exists
ls -la document.tdf

# Use absolute path
tdf verify /absolute/path/to/document.tdf

# Check permissions
chmod +r document.tdf
```

### ZIP Archive Errors

**Problem**: `Error: Invalid ZIP archive`

**Solution**:
```bash
# Verify it's a valid ZIP
unzip -t document.tdf

# Check file isn't corrupted
file document.tdf

# Try extracting manually
unzip -l document.tdf
```

## Verification Issues

### Integrity Verification Fails

**Problem**: `integrity_valid: false`

**Possible Causes**:
1. File was modified
2. Archive is corrupted
3. Merkle tree mismatch

**Solution**:
```bash
# Check for tampering
tdf verify document.tdf --verbose

# Inspect archive
unzip -l document.tdf

# Recreate if needed
tdf create input.json -o new-document.tdf
```

### Signature Verification Fails

**Problem**: `signature verification failed`

**Possible Causes**:
1. Wrong public key
2. Key was revoked
3. Signature algorithm mismatch

**Solution**:
```bash
# Verify with correct key
tdf verify document.tdf --key verifying-key.verifying

# Check revocation list
tdf check-revocation document.tdf revocation.cbor

# Check signature algorithm
tdf info document.tdf
```

### Timestamp Validation Fails

**Problem**: `timestamp validation failed`

**Possible Causes**:
1. Clock skew
2. Expired timestamp
3. Invalid timestamp authority

**Solution**:
```bash
# Check system time
date

# Verify timestamp manually
tdf verify document.tdf --verbose

# Check timestamp config
# See docs/TIMESTAMP.md
```

## Performance Issues

### Slow Document Creation

**Problem**: Creating large documents is slow

**Solution**:
```bash
# Use release build
cargo build --release

# Check for bottlenecks
cargo build --release --features profiling

# Consider parallel processing
# Already implemented in Merkle tree computation
```

### High Memory Usage

**Problem**: Out of memory errors

**Solution**:
- Use streaming for very large files
- Process in chunks
- Increase system memory
- Use 64-bit build

### Slow Verification

**Problem**: Verification takes too long

**Solution**:
```bash
# Use release build
cargo build --release

# Verify specific components only
# (if API supports it)

# Check system resources
top
htop
```

## Platform-Specific Issues

### Windows

**Problem**: Path issues with backslashes

**Solution**:
```powershell
# Use forward slashes or double backslashes
tdf verify "C:/path/to/document.tdf"
tdf verify "C:\\path\\to\\document.tdf"
```

**Problem**: Antivirus blocking

**Solution**:
- Add exception for TDF executables
- Check Windows Defender settings

### macOS

**Problem**: Gatekeeper blocking

**Solution**:
```bash
# Remove quarantine attribute
xattr -d com.apple.quarantine tdf

# Or allow in System Preferences
```

**Problem**: Code signing issues

**Solution**:
```bash
# Sign manually (if you have certificate)
codesign --sign "Developer ID" tdf
```

### Linux

**Problem**: Missing libraries

**Solution**:
```bash
# Install missing dependencies
sudo apt-get install -y <package-name>

# Check library paths
ldd $(which tdf)
```

**Problem**: Permission denied

**Solution**:
```bash
# Check permissions
ls -la document.tdf

# Fix permissions
chmod +r document.tdf
```

### WSL (Windows Subsystem for Linux)

**Problem**: GTK/display errors

**Solution**:
```bash
# Use WSLg (Windows 11)
# Or set up X11 forwarding
export DISPLAY=:0

# For headless operation
# Build without running GUI
cargo build --release
```

## Common Error Messages

### "InvalidDocument"

**Cause**: Document structure is invalid

**Solution**:
- Check document format
- Verify all required files present
- Validate CBOR encoding

### "InvalidSignature"

**Cause**: Signature verification failed

**Solution**:
- Verify correct key used
- Check signature algorithm
- Verify key not revoked

### "IntegrityMismatch"

**Cause**: Merkle tree doesn't match

**Solution**:
- Document was modified
- Archive is corrupted
- Recreate document

### "FileNotFound"

**Cause**: File doesn't exist

**Solution**:
- Check file path
- Verify file exists
- Check permissions

## Debugging Tips

### Enable Verbose Output

```bash
# CLI verbose mode
tdf verify document.tdf --verbose

# Rust debug logging
RUST_LOG=debug cargo test

# TypeScript debug
npm run dev -- --debug
```

### Inspect TDF Files

```bash
# List archive contents
unzip -l document.tdf

# Extract and inspect
unzip document.tdf -d extracted/
cd extracted/
cat manifest.cbor | cbor2json
cat content.cbor | cbor2json
```

### Check System State

```bash
# Check Rust version
rustc --version
cargo --version

# Check Node version
node --version
npm --version

# Check system libraries
ldd $(which tdf)  # Linux
otool -L $(which tdf)  # macOS
```

## Getting Help

### Documentation

1. Check [INDEX.md](INDEX.md) for documentation
2. Read [API.md](API.md) for API reference
3. Review [SPEC.md](SPEC.md) for format details

### Community

- **GitHub Issues**: Report bugs
- **GitHub Discussions**: Ask questions
- **Email**: dev@trustdoc.org

### Reporting Issues

When reporting issues, include:

1. **Error message**: Full error text
2. **Steps to reproduce**: Clear steps
3. **Environment**: OS, Rust version, etc.
4. **Expected behavior**: What should happen
5. **Actual behavior**: What actually happens
6. **Logs**: Relevant log output

Example issue report:

```
**Error**: `TdfError::InvalidDocument("manifest.cbor not found")`

**Steps to reproduce**:
1. Create document: `tdf create test.json -o test.tdf`
2. Verify: `tdf verify test.tdf`

**Environment**:
- OS: Ubuntu 22.04
- Rust: 1.75.0
- TDF: 0.1.0

**Expected**: Verification succeeds
**Actual**: Error about missing manifest

**Logs**:
```
[DEBUG] Opening archive: test.tdf
[ERROR] manifest.cbor not found in archive
```
```

---

*Last updated: 2025-12-09*

