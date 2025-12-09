# TDF Deployment and Distribution Guide

Complete guide for deploying and distributing TDF applications and libraries.

## Table of Contents

1. [Overview](#overview)
2. [Library Distribution](#library-distribution)
3. [CLI Tool Distribution](#cli-tool-distribution)
4. [Desktop Viewer Distribution](#desktop-viewer-distribution)
5. [Mobile Viewer Distribution](#mobile-viewer-distribution)
6. [Web Viewer Deployment](#web-viewer-deployment)
7. [Package Repositories](#package-repositories)
8. [Code Signing](#code-signing)
9. [Release Process](#release-process)

## Overview

TDF components can be distributed through various channels:

- **Rust Libraries**: crates.io
- **NPM Packages**: npm registry
- **Desktop Apps**: Platform-specific installers
- **Mobile Apps**: App stores
- **Web Apps**: Static hosting

## Library Distribution

### Rust Crates (crates.io)

#### Publishing `tdf-core`

```bash
cd tdf-core

# Update version in Cargo.toml
# Run tests
cargo test

# Check package
cargo package

# Publish
cargo publish
```

#### Publishing Other Crates

```bash
# tdf-cli
cd tdf-cli
cargo publish

# tdf-convert
cd tdf-convert
cargo publish

# tdf-wasm (requires wasm-pack)
cd tdf-wasm
wasm-pack publish
```

### NPM Packages

#### Publishing `tdf-ts`

```bash
cd tdf-ts

# Update version in package.json
npm version patch  # or minor, major

# Build
npm run build

# Publish
npm publish
```

#### Publishing `tdf-wasm`

```bash
cd tdf-wasm

# Build WASM
wasm-pack build --target web --out-dir pkg

# Publish to npm
cd pkg
npm publish
```

## CLI Tool Distribution

### Building Binaries

```bash
# Build for current platform
cargo build --release -p tdf-cli

# Cross-compile (requires cross tool)
cargo install cross --git https://github.com/cross-rs/cross

# Linux
cross build --release --target x86_64-unknown-linux-gnu -p tdf-cli

# Windows
cross build --release --target x86_64-pc-windows-msvc -p tdf-cli

# macOS
cross build --release --target x86_64-apple-darwin -p tdf-cli
cross build --release --target aarch64-apple-darwin -p tdf-cli
```

### Distribution Methods

1. **GitHub Releases**: Upload binaries to releases
2. **Package Managers**: 
   - Homebrew (macOS)
   - Chocolatey (Windows)
   - apt/yum (Linux)
3. **Direct Download**: Host on website

### Homebrew Formula

```ruby
class Tdf < Formula
  desc "TrustDoc Financial format CLI"
  homepage "https://trustdoc.org"
  url "https://github.com/trustdoc/tdf/releases/download/v0.1.0/tdf-cli-0.1.0-x86_64-apple-darwin.tar.gz"
  sha256 "..."
  
  def install
    bin.install "tdf"
  end
  
  test do
    system "#{bin}/tdf", "--version"
  end
end
```

## Desktop Viewer Distribution

### Tauri Build

```bash
cd tdf-desktop-viewer

# Install dependencies
npm install

# Build for current platform
npm run tauri:build

# Build for specific platform
npm run tauri:build -- --target x86_64-pc-windows-msvc
npm run tauri:build -- --target x86_64-apple-darwin
npm run tauri:build -- --target x86_64-unknown-linux-gnu
```

### Output Locations

- **Windows**: `src-tauri/target/release/bundle/msi/`
- **macOS**: `src-tauri/target/release/bundle/dmg/`
- **Linux**: `src-tauri/target/release/bundle/appimage/`

### Distribution Channels

1. **GitHub Releases**: Upload installers
2. **Website**: Direct download links
3. **App Stores**: 
   - Microsoft Store (Windows)
   - Mac App Store (macOS)
   - Snap Store (Linux)

## Mobile Viewer Distribution

### React Native/Expo Build

```bash
cd tdf-mobile

# Install dependencies
npm install

# Build for iOS
eas build --platform ios

# Build for Android
eas build --platform android
```

### App Store Distribution

#### iOS (App Store)

1. Build with EAS or Xcode
2. Upload to App Store Connect
3. Submit for review

#### Android (Play Store)

1. Build APK/AAB with EAS or Gradle
2. Upload to Google Play Console
3. Submit for review

### Configuration

Update `app.json`:

```json
{
  "expo": {
    "name": "TDF Viewer",
    "slug": "tdf-viewer",
    "version": "1.0.0",
    "ios": {
      "bundleIdentifier": "com.trustdoc.viewer"
    },
    "android": {
      "package": "com.trustdoc.viewer"
    }
  }
}
```

## Web Viewer Deployment

### Static Hosting

#### Build

```bash
cd tdf-viewer
npm install
npm run build
```

#### Deploy to Various Platforms

**Netlify**:
```bash
netlify deploy --prod --dir=dist
```

**Vercel**:
```bash
vercel --prod
```

**GitHub Pages**:
```bash
npm run build
# Push dist/ to gh-pages branch
```

**AWS S3**:
```bash
aws s3 sync dist/ s3://your-bucket-name --delete
```

### Docker Deployment

```dockerfile
FROM nginx:alpine
COPY dist/ /usr/share/nginx/html/
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
```

Build and run:

```bash
docker build -t tdf-viewer .
docker run -p 80:80 tdf-viewer
```

## Package Repositories

### Cargo (crates.io)

1. Create account on crates.io
2. Get API token
3. Configure: `cargo login <token>`
4. Publish: `cargo publish`

### NPM

1. Create account on npmjs.com
2. Login: `npm login`
3. Publish: `npm publish`

### GitHub Packages

```toml
# Cargo.toml
[package]
publish = ["registry-name"]

[registries]
registry-name = { index = "https://github.com/org/registry-index" }
```

## Code Signing

### Windows

1. Obtain code signing certificate
2. Configure in `tauri.conf.json`:

```json
{
  "bundle": {
    "windows": {
      "certificateThumbprint": "YOUR_THUMBPRINT"
    }
  }
}
```

### macOS

1. Join Apple Developer Program
2. Create signing certificate
3. Configure in Xcode or `tauri.conf.json`

### Linux

- GPG signing for packages
- Notarization not required

## Release Process

### Version Numbering

Follow [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes

### Release Checklist

- [ ] Update version numbers
- [ ] Update CHANGELOG.md
- [ ] Run all tests
- [ ] Build all components
- [ ] Create release tag
- [ ] Build distribution packages
- [ ] Sign binaries (if applicable)
- [ ] Create GitHub release
- [ ] Upload binaries
- [ ] Publish to package repositories
- [ ] Update documentation
- [ ] Announce release

### Release Script

```bash
#!/bin/bash
set -e

VERSION=$1
if [ -z "$VERSION" ]; then
    echo "Usage: ./release.sh <version>"
    exit 1
fi

# Update versions
echo "Updating versions to $VERSION..."
# Update Cargo.toml files
# Update package.json files

# Run tests
echo "Running tests..."
cargo test --workspace
npm test

# Build
echo "Building..."
cargo build --release --workspace
npm run build

# Create tag
git tag -a "v$VERSION" -m "Release v$VERSION"
git push origin "v$VERSION"

# Create GitHub release
gh release create "v$VERSION" \
    --title "Release v$VERSION" \
    --notes "Release notes here"

echo "Release $VERSION complete!"
```

### Automated Releases

Use GitHub Actions for automated releases:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Build
        run: cargo build --release
      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: binaries
          path: target/release/
```

## Distribution Checklist

### Before Release

- [ ] All tests pass
- [ ] Documentation updated
- [ ] Version numbers updated
- [ ] CHANGELOG updated
- [ ] Security audit completed
- [ ] Performance benchmarks acceptable

### Release Day

- [ ] Create release tag
- [ ] Build all platforms
- [ ] Sign binaries
- [ ] Upload to repositories
- [ ] Create GitHub release
- [ ] Announce on website/social media

### Post-Release

- [ ] Monitor for issues
- [ ] Respond to bug reports
- [ ] Plan next release

---

*Last updated: 2025-12-09*

