#!/bin/bash
# Helper script to create ICNS file on macOS
# This script must be run on macOS

if [[ "$OSTYPE" != "darwin"* ]]; then
    echo "❌ Error: This script must be run on macOS"
    echo ""
    echo "💡 Alternative options:"
    echo "   1. Transfer icons/icon.iconset/ to a macOS machine"
    echo "   2. Run: iconutil -c icns icons/icon.iconset -o icons/icon.icns"
    echo "   3. Or use online converter: https://cloudconvert.com/icns-converter"
    exit 1
fi

ICON_DIR="$(cd "$(dirname "$0")" && pwd)"
ICONSET_DIR="$ICON_DIR/icon.iconset"
ICNS_OUTPUT="$ICON_DIR/icon.icns"

if [ ! -d "$ICONSET_DIR" ]; then
    echo "❌ Error: iconset directory not found: $ICONSET_DIR"
    echo "   Run: python3 icons/create-icons-python.py first"
    exit 1
fi

echo "🍎 Creating macOS ICNS file..."
iconutil -c icns "$ICONSET_DIR" -o "$ICNS_OUTPUT"

if [ -f "$ICNS_OUTPUT" ]; then
    echo "✅ ICNS file created: $ICNS_OUTPUT"
    ls -lh "$ICNS_OUTPUT"
else
    echo "❌ Failed to create ICNS file"
    exit 1
fi
