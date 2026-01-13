# TDF Desktop Viewer

Cross-platform desktop application for viewing and verifying TDF documents.

## Features

- 📄 **Document Viewing**: View TDF documents with rich rendering
- 🔐 **Integrity Verification**: Enhanced verification with detailed signature analysis
- 🔑 **Key Management**: Generate, import, export, and manage signing keys
- 📊 **Data Extraction**: Extract structured data from documents
- 🖨️ **Print Support**: Print documents directly
- 🎨 **Native Feel**: Native OS integration (file associations, drag & drop)
- ⚡ **Fast**: Built with Rust + Tauri for performance
- 🛡️ **Security**: Uses all TDF security modules for maximum protection

## Supported Platforms

- ✅ Windows (x64)
- ✅ macOS (Intel & Apple Silicon)
- ✅ Linux (x64, AppImage)

## Building

### Prerequisites

- Rust (latest stable)
- Node.js 18+
- System dependencies:
  - **Windows**: Microsoft Visual C++ Build Tools
  - **macOS**: Xcode Command Line Tools
  - **Linux**: `libwebkit2gtk-4.0-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev`

### Development

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev
```

### Build for Production

```bash
# Build for current platform
npm run tauri build

# Build for specific platform (requires cross-compilation setup)
npm run tauri build -- --target x86_64-pc-windows-msvc  # Windows
npm run tauri build -- --target x86_64-apple-darwin     # macOS Intel
npm run tauri build -- --target aarch64-apple-darwin     # macOS Apple Silicon
npm run tauri build -- --target x86_64-unknown-linux-gnu # Linux
```

## Installation

### Windows
- Download `TDF-Viewer_0.1.0_x64_en-US.msi` installer
- Run installer and follow prompts
- TDF files will be associated with the viewer

### macOS
- Download `TDF-Viewer_0.1.0_aarch64.app.dmg` (Apple Silicon) or `TDF-Viewer_0.1.0_x64.dmg` (Intel)
- Open DMG and drag app to Applications
- First launch: Right-click → Open (to bypass Gatekeeper)

### Linux
- Download `TDF-Viewer_0.1.0_amd64.AppImage`
- Make executable: `chmod +x TDF-Viewer_0.1.0_amd64.AppImage`
- Run: `./TDF-Viewer_0.1.0_amd64.AppImage`

## Usage

1. **Open TDF File**
   - Double-click a `.tdf` file, or
   - File → Open, or
   - Drag & drop file onto window

2. **View Document**
   - Document renders automatically
   - Scroll to view content
   - Use zoom controls if needed

3. **Verify Integrity**
   - Click "Verify" button in toolbar
   - View enhanced verification results with:
     - Integrity status
     - Signature validation details
     - Timestamp verification
     - Signer information
   - Check signature status for each signer

4. **Manage Keys**
   - Generate new signing keypairs (Ed25519 or secp256k1)
   - Import existing keys from files
   - Export keys for backup or sharing
   - View key details and metadata
   - Delete keys when no longer needed

5. **Extract Data**
   - Click "Extract" button
   - Data exported as JSON
   - Save to file

6. **Print**
   - Click "Print" button
   - Use system print dialog

## Architecture

- **Frontend**: TypeScript + Vite (reuses web viewer code)
- **Backend**: Rust + Tauri (native OS integration)
- **Core Library**: `tdf-core` (Rust)
- **WASM**: `tdf-wasm` (browser verification)

## File Associations

The installer automatically associates `.tdf` files with the viewer:
- **Windows**: Registry entries
- **macOS**: Launch Services
- **Linux**: Desktop entry + MIME type

## Development

### Project Structure

```
tdf-desktop-viewer/
├── src/                      # Frontend (TypeScript)
│   ├── main.ts               # Entry point
│   ├── app.ts                # Main application logic
│   ├── documents.ts          # Document management
│   ├── keys.ts               # Key management UI
│   ├── key-list.ts           # Key list component
│   ├── key-details.ts        # Key details component
│   ├── verification-panel.ts # Verification panel
│   ├── settings.ts           # Settings management
│   ├── components/           # Reusable UI components
│   ├── services/             # Service layer
│   ├── layout/               # Layout components
│   └── styles/               # CSS styles
├── src-tauri/                # Backend (Rust)
│   ├── Cargo.toml            # Rust dependencies
│   └── src/
│       ├── main.rs           # Tauri commands
│       ├── keys.rs           # Key management backend
│       └── documents.rs      # Document verification backend
├── package.json              # Node dependencies
└── tauri.conf.json           # Tauri configuration
```

### Adding Features

1. **Frontend**: Edit files in `src/`
2. **Backend**: Add Rust commands in `src-tauri/src/main.rs`
3. **Build**: Run `npm run tauri build`

## License

MIT OR Apache-2.0

