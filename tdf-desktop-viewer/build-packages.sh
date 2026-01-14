#!/bin/bash
# Build script for creating TDF Desktop Viewer packages for all platforms

set -e

echo "🔨 Building TDF Desktop Viewer packages..."
echo ""

# Check if we're in the right directory
if [ ! -f "package.json" ]; then
    echo "❌ Error: Must run from tdf-desktop-viewer directory"
    exit 1
fi

# Install dependencies if needed
if [ ! -d "node_modules" ]; then
    echo "📦 Installing dependencies..."
    npm install
fi

# Build frontend
echo "🏗️  Building frontend..."
npm run build

# Create output directory
mkdir -p dist-packages

# Build for current platform
echo ""
echo "📦 Building package for current platform..."
npm run tauri build

# Copy packages to dist-packages
echo ""
echo "📋 Copying packages to dist-packages/..."

# Find and copy Windows MSI
if [ -f "src-tauri/target/release/bundle/msi/"*.msi ]; then
    cp src-tauri/target/release/bundle/msi/*.msi dist-packages/ 2>/dev/null || true
    echo "✅ Windows MSI package created"
fi

# Find and copy macOS DMG
if [ -f "src-tauri/target/release/bundle/dmg/"*.dmg ]; then
    cp src-tauri/target/release/bundle/dmg/*.dmg dist-packages/ 2>/dev/null || true
    echo "✅ macOS DMG package created"
fi

# Find and copy Linux AppImage
if [ -f "src-tauri/target/release/bundle/appimage/"*.AppImage ]; then
    cp src-tauri/target/release/bundle/appimage/*.AppImage dist-packages/ 2>/dev/null || true
    echo "✅ Linux AppImage package created"
fi

# Find and copy Linux DEB
if [ -f "src-tauri/target/release/bundle/deb/"*.deb ]; then
    cp src-tauri/target/release/bundle/deb/*.deb dist-packages/ 2>/dev/null || true
    echo "✅ Linux DEB package created"
fi

echo ""
echo "✨ Build complete! Packages are in dist-packages/"
ls -lh dist-packages/ 2>/dev/null || echo "No packages found (may need to build on specific platform)"
