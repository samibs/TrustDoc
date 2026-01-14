#!/bin/bash
# Create TDF brand icons from SVG logo
# Generates all required icon formats: PNG, ICO, ICNS

set -e

ICON_DIR="icons"
LOGO_SVG="$ICON_DIR/logo.svg"

echo "🎨 Creating TDF brand icons from logo..."

# Check if logo exists
if [ ! -f "$LOGO_SVG" ]; then
    echo "❌ Error: Logo SVG not found at $LOGO_SVG"
    exit 1
fi

# Check for required tools
HAS_IMAGEMAGICK=false
HAS_INKSCAPE=false
HAS_ICONUTIL=false

if command -v convert &> /dev/null || command -v magick &> /dev/null; then
    HAS_IMAGEMAGICK=true
    echo "✅ ImageMagick found"
fi

if command -v inkscape &> /dev/null; then
    HAS_INKSCAPE=true
    echo "✅ Inkscape found"
fi

if command -v iconutil &> /dev/null; then
    HAS_ICONUTIL=true
    echo "✅ iconutil found (macOS)"
fi

# Function to convert SVG to PNG using ImageMagick
convert_svg_to_png_imagemagick() {
    local size=$1
    local output=$2
    if command -v magick &> /dev/null; then
        magick -background none -density 300 "$LOGO_SVG" -resize "${size}x${size}" "$output"
    else
        convert -background none -density 300 "$LOGO_SVG" -resize "${size}x${size}" "$output"
    fi
}

# Function to convert SVG to PNG using Inkscape
convert_svg_to_png_inkscape() {
    local size=$1
    local output=$2
    inkscape --export-type=png --export-filename="$output" --export-width=$size --export-height=$size "$LOGO_SVG" 2>/dev/null
}

# Convert SVG to PNG at various sizes
echo ""
echo "📐 Generating PNG icons..."

if [ "$HAS_INKSCAPE" = true ]; then
    echo "   Using Inkscape..."
    convert_svg_to_png_inkscape 32 "$ICON_DIR/32x32.png"
    convert_svg_to_png_inkscape 128 "$ICON_DIR/128x128.png"
    convert_svg_to_png_inkscape 256 "$ICON_DIR/128x128@2x.png"
    convert_svg_to_png_inkscape 512 "$ICON_DIR/icon-512.png"
    convert_svg_to_png_inkscape 1024 "$ICON_DIR/icon-1024.png"
elif [ "$HAS_IMAGEMAGICK" = true ]; then
    echo "   Using ImageMagick..."
    convert_svg_to_png_imagemagick 32 "$ICON_DIR/32x32.png"
    convert_svg_to_png_imagemagick 128 "$ICON_DIR/128x128.png"
    convert_svg_to_png_imagemagick 256 "$ICON_DIR/128x128@2x.png"
    convert_svg_to_png_imagemagick 512 "$ICON_DIR/icon-512.png"
    convert_svg_to_png_imagemagick 1024 "$ICON_DIR/icon-1024.png"
else
    echo "⚠️  No SVG converter found. Install ImageMagick or Inkscape:"
    echo "   sudo apt install imagemagick (Linux)"
    echo "   brew install imagemagick (macOS)"
    echo "   sudo apt install inkscape (Linux)"
    exit 1
fi

echo "✅ PNG icons created"

# Create Windows ICO file
echo ""
echo "🪟 Creating Windows ICO file..."

if [ "$HAS_IMAGEMAGICK" = true ]; then
    # ICO file with multiple sizes
    if command -v magick &> /dev/null; then
        magick "$ICON_DIR/16x16.png" "$ICON_DIR/32x32.png" "$ICON_DIR/48x48.png" "$ICON_DIR/128x128.png" \
               -background none \
               "$ICON_DIR/icon.ico" 2>/dev/null || \
        magick "$ICON_DIR/32x32.png" "$ICON_DIR/128x128.png" \
               -background none \
               "$ICON_DIR/icon.ico"
    else
        convert "$ICON_DIR/32x32.png" "$ICON_DIR/128x128.png" \
                -background none \
                "$ICON_DIR/icon.ico" 2>/dev/null || \
        convert "$ICON_DIR/32x32.png" \
                -background none \
                "$ICON_DIR/icon.ico"
    fi
    
    # Create additional sizes if needed
    if [ ! -f "$ICON_DIR/16x16.png" ]; then
        convert_svg_to_png_imagemagick 16 "$ICON_DIR/16x16.png" 2>/dev/null || true
    fi
    if [ ! -f "$ICON_DIR/48x48.png" ]; then
        convert_svg_to_png_imagemagick 48 "$ICON_DIR/48x48.png" 2>/dev/null || true
    fi
    
    echo "✅ ICO file created: $ICON_DIR/icon.ico"
else
    echo "⚠️  ImageMagick required for ICO. Install: sudo apt install imagemagick"
    echo "   Or use online converter: https://convertio.co/png-ico/"
fi

# Create macOS ICNS file
echo ""
echo "🍎 Creating macOS ICNS file..."

if [ "$HAS_ICONUTIL" = true ]; then
    # Create iconset directory
    ICONSET_DIR="$ICON_DIR/icon.iconset"
    rm -rf "$ICONSET_DIR"
    mkdir -p "$ICONSET_DIR"
    
    # Copy PNG files to iconset with proper naming
    if [ -f "$ICON_DIR/icon-1024.png" ]; then
        cp "$ICON_DIR/icon-1024.png" "$ICONSET_DIR/icon_512x512@2x.png"
    fi
    if [ -f "$ICON_DIR/icon-512.png" ]; then
        cp "$ICON_DIR/icon-512.png" "$ICONSET_DIR/icon_512x512.png"
        cp "$ICON_DIR/icon-512.png" "$ICONSET_DIR/icon_256x256@2x.png"
    fi
    if [ -f "$ICON_DIR/128x128@2x.png" ]; then
        cp "$ICON_DIR/128x128@2x.png" "$ICONSET_DIR/icon_256x256.png"
        cp "$ICON_DIR/128x128@2x.png" "$ICONSET_DIR/icon_128x128@2x.png"
    fi
    if [ -f "$ICON_DIR/128x128.png" ]; then
        cp "$ICON_DIR/128x128.png" "$ICONSET_DIR/icon_128x128.png"
        cp "$ICON_DIR/128x128.png" "$ICONSET_DIR/icon_64x64@2x.png"
        cp "$ICON_DIR/128x128.png" "$ICONSET_DIR/icon_32x32@2x.png"
    fi
    if [ -f "$ICON_DIR/32x32.png" ]; then
        cp "$ICON_DIR/32x32.png" "$ICONSET_DIR/icon_32x32.png"
        cp "$ICON_DIR/32x32.png" "$ICONSET_DIR/icon_16x16@2x.png"
    fi
    
    # Generate additional sizes if needed
    if [ "$HAS_IMAGEMAGICK" = true ]; then
        for size in 16 32 64; do
            if [ ! -f "$ICONSET_DIR/icon_${size}x${size}.png" ]; then
                convert_svg_to_png_imagemagick $size "$ICONSET_DIR/icon_${size}x${size}.png" 2>/dev/null || true
            fi
        done
    fi
    
    # Create ICNS from iconset
    iconutil -c icns "$ICONSET_DIR" -o "$ICON_DIR/icon.icns"
    rm -rf "$ICONSET_DIR"
    
    echo "✅ ICNS file created: $ICON_DIR/icon.icns"
else
    echo "⚠️  iconutil only available on macOS"
    echo "   To create ICNS on Linux/Windows:"
    echo "   1. Transfer iconset to macOS"
    echo "   2. Run: iconutil -c icns icon.iconset -o icon.icns"
    echo "   Or use online converter"
fi

echo ""
echo "✨ Icon generation complete!"
echo ""
echo "📦 Generated files:"
ls -lh "$ICON_DIR"/*.png "$ICON_DIR"/*.ico "$ICON_DIR"/*.icns 2>/dev/null | awk '{print "   " $9 " (" $5 ")"}'
echo ""
echo "💡 Next steps:"
echo "   1. Review the generated icons"
echo "   2. Update tauri.conf.json to include icon.ico and icon.icns"
echo "   3. Test the build on all platforms"
