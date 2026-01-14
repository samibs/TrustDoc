# TDF Desktop Viewer - Package Building Guide

## Overview

This document explains how to build installable packages for the TDF Desktop Viewer on Windows, macOS, and Linux.

## Quick Start

### Build for Current Platform

```bash
cd tdf-desktop-viewer
npm install
npm run tauri build
```

Packages will be created in `src-tauri/target/release/bundle/`:
- **Windows**: `msi/TDF-Viewer_0.1.0_x64_en-US.msi`
- **macOS**: `dmg/TDF-Viewer_0.1.0_aarch64.dmg` (or `x64.dmg`)
- **Linux**: `appimage/TDF-Viewer_0.1.0_amd64.AppImage` and `deb/TDF-Viewer_0.1.0_amd64.deb`

### Using Build Scripts

```bash
# From project root
./scripts/build-all-packages.sh

# Or from desktop viewer directory
cd tdf-desktop-viewer
./build-packages.sh
```

## Automated Builds

### GitHub Actions

We have a GitHub Actions workflow (`.github/workflows/build-desktop-viewer.yml`) that automatically builds packages for all platforms on:
- Push to `main` branch (when desktop viewer files change)
- Pull requests
- Manual trigger (`workflow_dispatch`)
- Release creation (creates GitHub release with all packages)

### Setting Up GitHub Actions

1. **Secrets** (optional, for code signing):
   - `TAURI_PRIVATE_KEY` - Private key for code signing
   - `TAURI_KEY_PASSWORD` - Password for private key

2. **Trigger Build**:
   - Push changes to trigger automatic build
   - Or go to Actions → "Build Desktop Viewer" → "Run workflow"

3. **Download Artifacts**:
   - Go to the workflow run
   - Download artifacts for each platform
   - Or create a release to automatically attach packages

## Manual Building

See [BUILD.md](BUILD.md) for detailed instructions on:
- Prerequisites for each platform
- Cross-platform building
- Troubleshooting
- Icon creation

## Package Formats

### Windows (.msi)
- **Installer type**: Microsoft Installer
- **File association**: Automatically associates `.tdf` files
- **Size**: ~15-20 MB
- **Installation**: Double-click MSI file
- **Uninstall**: Control Panel → Programs

### macOS (.dmg)
- **Installer type**: Disk Image
- **Architectures**: Intel (x64) and Apple Silicon (aarch64)
- **Size**: ~15-20 MB
- **Installation**: 
  1. Open DMG file
  2. Drag app to Applications folder
  3. First launch: Right-click → Open (to bypass Gatekeeper)
- **Code Signing**: Optional (requires Apple Developer account)

### Linux (.AppImage)
- **Installer type**: Portable application
- **Size**: ~15-20 MB
- **Installation**: 
  ```bash
  chmod +x TDF-Viewer_0.1.0_amd64.AppImage
  ./TDF-Viewer_0.1.0_amd64.AppImage
  ```
- **Desktop Integration**: Optional (double-click to run)

### Linux (.deb)
- **Installer type**: Debian package
- **Size**: ~15-20 MB
- **Installation**:
  ```bash
  sudo dpkg -i TDF-Viewer_0.1.0_amd64.deb
  sudo apt-get install -f  # Fix dependencies if needed
  ```
- **File association**: Automatically associates `.tdf` files
- **Uninstall**: `sudo apt remove tdf-viewer`

## Creating Icons

Icons are required for packages. To create placeholder icons:

```bash
cd tdf-desktop-viewer
./create-icons.sh
```

This creates basic placeholder icons. **Replace with actual TDF logo icons before release.**

Required icon files:
- `icons/32x32.png`
- `icons/128x128.png`
- `icons/128x128@2x.png`
- `icons/icon.icns` (macOS)
- `icons/icon.ico` (Windows)

## Release Process

1. **Update Version**:
   - `package.json`: `"version": "0.1.0"`
   - `src-tauri/Cargo.toml`: `version = "0.1.0"`
   - `src-tauri/tauri.conf.json`: `"version": "0.1.0"`

2. **Create Icons** (if not done):
   ```bash
   cd tdf-desktop-viewer
   ./create-icons.sh
   # Replace placeholders with actual icons
   ```

3. **Build Packages**:
   - Use GitHub Actions (recommended)
   - Or build manually on each platform

4. **Test Packages**:
   - Install on clean systems
   - Test file associations
   - Test document opening
   - Test all features

5. **Create GitHub Release**:
   - Go to GitHub → Releases → Draft new release
   - Tag version (e.g., `v0.1.0`)
   - Upload packages
   - Add release notes

## Troubleshooting

### Icons Not Found
- Run `./create-icons.sh` to create placeholders
- Replace with actual icons before release

### Build Fails
- Check prerequisites (see BUILD.md)
- Ensure all dependencies are installed
- Check error messages for missing tools

### Packages Not Created
- Check `src-tauri/target/release/bundle/` directory
- Verify Tauri configuration in `tauri.conf.json`
- Check build logs for errors

## Next Steps

- [ ] Create actual TDF logo icons
- [ ] Set up code signing (optional)
- [ ] Test packages on all platforms
- [ ] Create first release on GitHub
- [ ] Update README with download links
