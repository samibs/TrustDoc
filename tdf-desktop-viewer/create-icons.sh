#!/bin/bash
# Script to create placeholder icons for TDF Desktop Viewer
# This creates simple colored square icons as placeholders

set -e

ICON_DIR="icons"
mkdir -p "$ICON_DIR"

echo "🎨 Creating placeholder icons..."

# Colors for the icon (TDF brand colors - adjust as needed)
COLOR="#2563eb"  # Blue color

# Create PNG icons using ImageMagick or convert command
if command -v convert &> /dev/null; then
    echo "Using ImageMagick to create icons..."
    
    # 32x32 icon
    convert -size 32x32 xc:"$COLOR" -gravity center -pointsize 20 -fill white -annotate +0+0 "TDF" "$ICON_DIR/32x32.png"
    
    # 128x128 icon
    convert -size 128x128 xc:"$COLOR" -gravity center -pointsize 80 -fill white -annotate +0+0 "TDF" "$ICON_DIR/128x128.png"
    
    # 256x256 icon (for 2x retina)
    convert -size 256x256 xc:"$COLOR" -gravity center -pointsize 160 -fill white -annotate +0+0 "TDF" "$ICON_DIR/128x128@2x.png"
    
    echo "✅ PNG icons created"
else
    echo "⚠️  ImageMagick not found. Creating simple placeholder files..."
    echo "   Install ImageMagick: sudo apt install imagemagick (Linux) or brew install imagemagick (macOS)"
    
    # Create empty placeholder files
    touch "$ICON_DIR/32x32.png"
    touch "$ICON_DIR/128x128.png"
    touch "$ICON_DIR/128x128@2x.png"
    
    echo "⚠️  Placeholder files created. Replace with actual icons before building."
fi

# Note: .icns and .ico files require special tools
# For macOS .icns: Use iconutil or online converters
# For Windows .ico: Use online converters or ImageMagick

echo ""
echo "📝 Next steps:"
echo "   1. Replace placeholder icons with actual TDF logo icons"
echo "   2. Create icon.icns for macOS (use: iconutil -c icns icons.iconset)"
echo "   3. Create icon.ico for Windows (use online converter or ImageMagick)"
echo ""
echo "   Recommended icon sizes:"
echo "   - 32x32, 128x128, 256x256, 512x512, 1024x1024"
echo ""
