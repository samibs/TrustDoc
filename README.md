# 🔐 TrustDoc Financial (TDF)

<div align="center">

**Cryptographically secure document format with built-in integrity verification**

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.0+-blue.svg)](https://www.typescriptlang.org/)

[Features](#-features) • [Viewers](#-viewers) • [Quick Start](#-quick-start) • [Documentation](#-documentation) • [Security](#-security)

</div>

---

## 🎯 What is TDF?

**TrustDoc Financial (TDF)** is an open document format designed for financial and corporate documents requiring cryptographic integrity verification. Unlike PDF or Word documents, TDF provides:

- ✅ **100% Tamper Detection** - Any modification is cryptographically detectable
- ✅ **Offline Verification** - No server or network required
- ✅ **Structured Data** - Extract tables, diagrams, and metadata as JSON
- ✅ **Digital Signatures** - Ed25519 and secp256k1 (Web3 compatible)
- ✅ **Open Format** - No vendor lock-in, fully open specification (CC0)

Perfect for **financial services**, **legal documents**, **healthcare records**, and **government transparency**.

---

## ✨ Features

### 🔒 Security & Integrity

- **Merkle Tree Verification** - SHA-256 based integrity checking
- **Digital Signatures** - Ed25519 and secp256k1 support
- **Multi-Party Signatures** - Multiple signers can co-sign documents
- **Key Revocation** - Revoke compromised keys
- **Timestamp Authority** - RFC 3161 timestamp support
- **9 Security Modules** - Comprehensive security hardening

### 📄 Document Features

- **Rich Content** - Text, tables, diagrams, figures, lists
- **CSS Styling** - Web-native styling support
- **Print Fidelity** - Deterministic rendering for legal use
- **Structured Data** - Machine-readable extraction
- **Size Tiers** - Micro (256KB), Standard (5MB), Extended (50MB)

### 🛠️ Developer Tools

- **Rust Core Library** - High-performance cryptographic operations
- **TypeScript SDK** - Browser and Node.js support
- **WASM Bindings** - Client-side verification in browsers
- **CLI Tool** - Create, verify, extract, and manage documents
- **Multiple Viewers** - Desktop, web, and mobile apps

---

## 🖥️ Viewers

### 🖥️ Desktop Viewer (Tauri)

**Cross-platform native desktop application** for Windows, macOS, and Linux.

**Features:**
- 📄 Rich document rendering with HTML/CSS
- 🔐 Enhanced integrity verification with detailed signature analysis
- 🔑 **Key Management** - Generate, import, export, and manage signing keys
- 📊 Data extraction to JSON
- 🖨️ Print support
- 🎨 Native OS integration (file associations, drag & drop)
- ⚡ Fast performance with Rust backend

**Download:**
- Windows: `.msi` installer
- macOS: `.dmg` (Intel & Apple Silicon)
- Linux: `.AppImage`

[📖 Desktop Viewer Documentation →](tdf-desktop-viewer/README.md)

### 🌐 Web Viewer

**Browser-based viewer** - no installation required!

**Features:**
- 📄 View TDF documents directly in your browser
- 🔐 Client-side integrity verification (WASM)
- 📊 Extract structured data
- 🖨️ Print documents
- 📱 Responsive design (works on mobile)
- 🎨 Rich rendering (tables, diagrams, SVG)

**Try it:** Open any `.tdf` file in the web viewer at `tdf-viewer/`

[📖 Web Viewer Documentation →](tdf-viewer/README.md)

### 📱 Mobile Viewer (React Native)

**Native mobile apps** for iOS and Android.

**Features:**
- 📄 Native mobile UI
- 🔐 Integrity verification
- 📊 Data extraction
- 📱 Native file picker integration

[📖 Mobile Viewer Documentation →](docs/VIEWER_IMPLEMENTATION.md)

---

## 🚀 Quick Start

### Installation

#### Desktop Viewer (Recommended)

Download the desktop viewer for your platform:
- **Windows**: Download `.msi` installer
- **macOS**: Download `.dmg` file
- **Linux**: Download `.AppImage`

#### CLI Tool

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone repository
git clone https://github.com/samibs/TrustDoc.git
cd TrustDoc

# Build CLI tool
cargo build --release --bin tdf

# Install globally (optional)
cargo install --path tdf-cli
```

### Create Your First Document

```bash
# Generate a signing key
tdf keygen --name my-key

# Create a TDF document from JSON
tdf create document.json -o document.tdf \
  --key my-key.signing \
  --signer-id "did:web:example.com" \
  --signer-name "Your Name"

# Verify the document
tdf verify document.tdf

# Extract structured data
tdf extract document.tdf -o data.json
```

### Using the Desktop Viewer

1. **Open TDF File**
   - Double-click a `.tdf` file, or
   - File → Open, or
   - Drag & drop file onto window

2. **View Document**
   - Document renders automatically with rich formatting
   - Scroll to view content

3. **Verify Integrity**
   - Click "Verify" button
   - View detailed verification results:
     - Integrity status
     - Signature validation
     - Timestamp verification
     - Signer information

4. **Manage Keys**
   - Generate new signing keypairs
   - Import/export keys
   - View key details

5. **Extract Data**
   - Click "Extract" to export structured JSON

### Using the Web Viewer

1. Open `tdf-viewer/index.html` in your browser
2. Drag and drop a `.tdf` file
3. View and verify the document

---

## 📚 Documentation

### Getting Started
- [Quick Start Guide](docs/QUICK_START.md)
- [Installation Instructions](docs/INSTALL.md)
- [Usage Examples](docs/USAGE.md)

### Format & Specification
- [Complete Format Specification](docs/SPEC.md)
- [How to Read TDF Files](docs/HOW_TO_READ_TDF.md)
- [Security Verification Guide](docs/HOW_TO_VERIFY_SECURITY.md)

### Development
- [Developer Guide](docs/DEVELOPER_GUIDE.md)
- [API Reference](docs/API.md)
- [Security Modules](docs/SECURITY_MODULES.md)
- [Architecture Overview](docs/ARCHITECTURE.md)

### Viewers
- [Desktop Viewer Guide](tdf-desktop-viewer/README.md)
- [Web Viewer Guide](tdf-viewer/README.md)
- [Viewer Implementation](docs/VIEWER_IMPLEMENTATION.md)

### Security
- [Security Implementation Summary](docs/SECURITY_IMPLEMENTATION_SUMMARY.md)
- [Threat Model](docs/THREAT_MODEL.md)
- [Security Testing](docs/SECURITY_TESTING.md)

---

## 🛡️ Security

TDF implements **comprehensive security hardening** with 9 dedicated security modules:

| Module | Purpose | Vulnerabilities Addressed |
|--------|---------|---------------------------|
| `secure_key` | Key zeroization | CVE-TDF-026, CVE-TDF-025 |
| `crypto_utils` | Constant-time operations | CVE-TDF-024 |
| `secure_random` | Secure RNG | CVE-TDF-025, Vuln #1, #25 |
| `audit` | Audit logging | Compliance, forensics |
| `error_sanitization` | Information leakage prevention | Vuln #11, #12 |
| `integer_safety` | Integer overflow protection | CVE-TDF-021, CVE-TDF-008 |
| `resource_limits` | DoS protection | Vuln #7, #9, #10 |
| `io` | Secure I/O | CVE-TDF-005, CVE-TDF-009 |
| `whitelist` | Signer management | CVE-TDF-024 |

**Security Testing:**
- ✅ 100% attack detection rate in brute force testing
- ✅ Comprehensive security test suite
- ✅ Zero unsafe code in security-critical paths
- ✅ ISO 27001/27002 aligned

[📖 Full Security Documentation →](docs/SECURITY_MODULES.md)

---

## 🏗️ Project Structure

```
TrustDoc/
├── tdf-core/              # Rust core library (cryptography, verification)
├── tdf-cli/               # Command-line tool
├── tdf-ts/                # TypeScript SDK
├── tdf-wasm/              # WASM bindings for browsers
├── tdf-viewer/            # Web viewer (HTML/JS)
├── tdf-desktop-viewer/    # Desktop viewer (Tauri)
├── tdf-mobile/            # Mobile viewer (React Native)
├── tdf-convert/           # Format conversion utilities
├── docs/                  # Complete documentation
├── examples/              # Sample documents
└── marketing/             # Marketing materials
```

---

## 💻 CLI Commands

```bash
# Create a document
tdf create input.json -o output.tdf --key signing-key.signing

# Verify integrity and signatures
tdf verify document.tdf --key verifying-key.verifying

# Extract structured data
tdf extract document.tdf -o data.json

# Show document information
tdf info document.tdf

# Generate signing keys
tdf keygen --name my-key                    # Ed25519 (default)
tdf keygen --name my-key --secp256k1       # secp256k1 (Web3)

# Export to PDF
tdf export document.tdf -o output.pdf

# Multi-party workflow
tdf workflow create workflow.json
tdf workflow status workflow-id
```

---

## 🎯 Use Cases

### Financial Services
- Regulatory compliance documents
- Audit trails
- Financial statements
- Transaction records

### Legal Documents
- Contracts with tamper-evidence
- Legal filings
- Court documents
- Non-repudiation

### Healthcare
- HIPAA-compliant patient records
- Medical reports
- Prescription records

### Government
- Public records
- Transparency documents
- Official communications

---

## 🔧 Building from Source

### Prerequisites

- **Rust** (latest stable) - [Install Rust](https://rustup.rs/)
- **Node.js** 18+ - [Install Node.js](https://nodejs.org/)
- **System dependencies** (see [INSTALL.md](docs/INSTALL.md))

### Build Everything

```bash
# Clone repository
git clone https://github.com/samibs/TrustDoc.git
cd TrustDoc

# Build Rust components
cargo build --workspace --release

# Build WASM bindings
cd tdf-wasm
wasm-pack build --target web --out-dir pkg
cd ..

# Build TypeScript SDK
cd tdf-ts
npm install && npm run build
cd ..

# Build web viewer
cd tdf-viewer
npm install && npm run build
cd ..

# Build desktop viewer
cd tdf-desktop-viewer
npm install
npm run tauri build
cd ..
```

---

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](docs/CONTRIBUTING.md) for guidelines.

### Areas for Contribution

- 🐛 Bug fixes
- ✨ New features
- 📚 Documentation improvements
- 🧪 Test coverage
- 🌐 Translations
- 🎨 UI/UX improvements

---

## 📄 License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

The TDF format specification is released into the public domain under [CC0](https://creativecommons.org/publicdomain/zero/1.0/).

---

## 🌟 Star History

If you find TDF useful, please consider giving us a ⭐ on GitHub!

---

## 📞 Contact & Support

- **GitHub Issues**: [Report bugs or request features](https://github.com/samibs/TrustDoc/issues)
- **Documentation**: [Full documentation](docs/)
- **Security**: Report security issues privately (see [SECURITY.md](docs/security/) if available)

---

<div align="center">

**Built with ❤️ for secure, tamper-evident documents**

[⬆ Back to Top](#-trustdoc-financial-tdf)</div>
