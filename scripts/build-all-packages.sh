#!/bin/bash
# Master build script to build packages for all platforms
# This script coordinates building on different platforms

set -e

echo "🔨 TDF Desktop Viewer - Package Builder"
echo "========================================"
echo ""

# Check if we're in the project root
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Must run from project root directory"
    exit 1
fi

cd tdf-desktop-viewer

echo "📋 Platform: $(uname -s)"
echo ""

# Check prerequisites
echo "🔍 Checking prerequisites..."

if ! command -v rustc &> /dev/null; then
    echo "❌ Rust not found. Install from https://rustup.rs/"
    exit 1
fi

if ! command -v node &> /dev/null; then
    echo "❌ Node.js not found. Install from https://nodejs.org/"
    exit 1
fi

if ! command -v npm &> /dev/null; then
    echo "❌ npm not found. Install Node.js from https://nodejs.org/"
    exit 1
fi

echo "✅ Prerequisites OK"
echo ""

# Check/create icons
if [ ! -f "icons/32x32.png" ]; then
    echo "⚠️  Icons not found. Creating placeholders..."
    ./create-icons.sh
    echo "⚠️  Please replace placeholder icons with actual icons before release"
    echo ""
fi

# Install dependencies
if [ ! -d "node_modules" ]; then
    echo "📦 Installing dependencies..."
    npm install
    echo ""
fi

# Build frontend
echo "🏗️  Building frontend..."
npm run build
echo ""

# Build Tauri app
echo "📦 Building Tauri application..."
echo ""

PLATFORM=$(uname -s)
case "$PLATFORM" in
    Linux*)
        echo "🐧 Building for Linux..."
        npm run tauri build
        echo ""
        echo "✅ Linux packages created in:"
        echo "   - AppImage: src-tauri/target/release/bundle/appimage/"
        echo "   - DEB: src-tauri/target/release/bundle/deb/"
        ;;
    Darwin*)
        echo "🍎 Building for macOS..."
        # Build for both architectures if on Apple Silicon
        if [[ $(uname -m) == "arm64" ]]; then
            echo "   Building for Apple Silicon (aarch64)..."
            npm run tauri build -- --target aarch64-apple-darwin
            echo "   Building for Intel (x86_64)..."
            npm run tauri build -- --target x86_64-apple-darwin
        else
            npm run tauri build
        fi
        echo ""
        echo "✅ macOS packages created in:"
        echo "   - DMG: src-tauri/target/*/release/bundle/dmg/"
        ;;
    MINGW*|MSYS*|CYGWIN*)
        echo "🪟 Building for Windows..."
        npm run tauri build
        echo ""
        echo "✅ Windows packages created in:"
        echo "   - MSI: src-tauri/target/release/bundle/msi/"
        ;;
    *)
        echo "⚠️  Unknown platform: $PLATFORM"
        echo "   Attempting generic build..."
        npm run tauri build
        ;;
esac

echo ""
echo "✨ Build complete!"
echo ""
echo "📦 Packages are ready in: src-tauri/target/release/bundle/"
echo ""
echo "💡 To build for other platforms:"
echo "   - Use GitHub Actions workflow for automated builds"
echo "   - Or use cross-compilation (see BUILD.md)"
echo ""
