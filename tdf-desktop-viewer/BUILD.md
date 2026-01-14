# Building TDF Desktop Viewer Packages

This guide explains how to build installable packages for Windows, macOS, and Linux.

## Prerequisites

### All Platforms
- **Rust** (latest stable) - [Install Rust](https://rustup.rs/)
- **Node.js** 18+ - [Install Node.js](https://nodejs.org/)
- **Tauri CLI** - Installed automatically with npm dependencies

### Windows
- **Microsoft Visual C++ Build Tools** - [Download](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- **WebView2** - Usually installed automatically, or [download](https://developer.microsoft.com/microsoft-edge/webview2/)

### macOS
- **Xcode Command Line Tools**: `xcode-select --install`
- **macOS 10.13+** for building

### Linux
- **System dependencies**:
  ```bash
  # Ubuntu/Debian
  sudo apt update
  sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

  # Fedora
  sudo dnf install webkit2gtk3-devel.x86_64 \
    openssl-devel \
    curl \
    wget \
    libappindicator-gtk3 \
    librsvg2-devel

  # Arch Linux
  sudo pacman -S webkit2gtk \
    base-devel \
    curl \
    wget \
    openssl \
    appmenu-gtk-module \
    gtk3 \
    libappindicator-gtk3 \
    librsvg \
    libvips
  ```

## Building Packages

### Quick Build (Current Platform)

```bash
cd tdf-desktop-viewer

# Install dependencies
npm install

# Build for current platform
npm run tauri build
```

Packages will be in `src-tauri/target/release/bundle/`:
- **Windows**: `msi/TDF-Viewer_0.1.0_x64_en-US.msi`
- **macOS**: `dmg/TDF-Viewer_0.1.0_aarch64.dmg` (or `x64.dmg`)
- **Linux**: `appimage/TDF-Viewer_0.1.0_amd64.AppImage` and `deb/TDF-Viewer_0.1.0_amd64.deb`

### Using Build Script

```bash
cd tdf-desktop-viewer
chmod +x build-packages.sh
./build-packages.sh
```

Packages will be copied to `dist-packages/` directory.

## Cross-Platform Building

### Building Windows MSI on Linux/macOS

Requires cross-compilation setup:

```bash
# Install cross-compilation toolchain
rustup target add x86_64-pc-windows-msvc

# Install Windows linker (on Linux)
sudo apt install mingw-w64

# Build
npm run tauri build -- --target x86_64-pc-windows-msvc
```

### Building macOS DMG on Linux

**Not possible** - macOS packages must be built on macOS due to code signing requirements.

### Building Linux AppImage on Windows/macOS

```bash
# Install cross-compilation toolchain
rustup target add x86_64-unknown-linux-gnu

# Install Linux linker (on macOS)
brew install SergioBenitez/osxct/x86_64-unknown-linux-gnu

# Build
npm run tauri build -- --target x86_64-unknown-linux-gnu
```

## GitHub Actions CI/CD

For automated builds, see `.github/workflows/build-desktop-viewer.yml` (if it exists).

## Package Details

### Windows MSI
- **Format**: Microsoft Installer
- **Size**: ~15-20 MB
- **Auto-associates**: `.tdf` files
- **Installation**: Double-click MSI file
- **Uninstall**: Control Panel → Programs

### macOS DMG
- **Format**: Disk Image
- **Size**: ~15-20 MB
- **Architectures**: Intel (x64) and Apple Silicon (aarch64)
- **Installation**: 
  1. Open DMG file
  2. Drag app to Applications folder
  3. First launch: Right-click → Open (to bypass Gatekeeper)
- **Code Signing**: Optional (requires Apple Developer account)

### Linux AppImage
- **Format**: Portable application
- **Size**: ~15-20 MB
- **Installation**: 
  ```bash
  chmod +x TDF-Viewer_0.1.0_amd64.AppImage
  ./TDF-Viewer_0.1.0_amd64.AppImage
  ```
- **Desktop Integration**: Optional (double-click to run)

### Linux DEB
- **Format**: Debian package
- **Size**: ~15-20 MB
- **Installation**:
  ```bash
  sudo dpkg -i TDF-Viewer_0.1.0_amd64.deb
  sudo apt-get install -f  # Fix dependencies if needed
  ```
- **Auto-associates**: `.tdf` files
- **Uninstall**: `sudo apt remove tdf-viewer`

## Troubleshooting

### Windows: "WebView2 not found"
- Install WebView2 Runtime: https://developer.microsoft.com/microsoft-edge/webview2/

### macOS: "Gatekeeper blocked"
- Right-click app → Open (first time only)
- Or: System Preferences → Security & Privacy → Allow

### Linux: "Missing dependencies"
- Install system dependencies (see Prerequisites)
- For AppImage: May need `fuse` package

### Build fails with "icon not found"
- Create placeholder icons in `icons/` directory
- Or update `tauri.conf.json` to remove icon references temporarily

## Creating Icons

Icons are required for packages. Create:
- `icons/32x32.png`
- `icons/128x128.png`
- `icons/128x128@2x.png`
- `icons/icon.icns` (macOS)
- `icons/icon.ico` (Windows)

Use a tool like [IconGenerator](https://icon.kitchen/) or create manually.

## Version Numbering

Update version in:
- `package.json` - `"version": "0.1.0"`
- `src-tauri/Cargo.toml` - `version = "0.1.0"`
- `src-tauri/tauri.conf.json` - `"version": "0.1.0"`

## Release Checklist

- [ ] Update version numbers
- [ ] Update changelog
- [ ] Create/update icons
- [ ] Test build on each platform
- [ ] Test installation on clean systems
- [ ] Verify file associations work
- [ ] Test document opening
- [ ] Create release notes
- [ ] Upload packages to GitHub Releases
