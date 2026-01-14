#!/usr/bin/env python3
"""
TDF Brand Icon Generator
Creates PNG, ICO, and ICNS icons from SVG logo
"""

import os
import sys
from pathlib import Path

try:
    from PIL import Image, ImageDraw, ImageFont
    import struct
except ImportError:
    print("❌ Error: PIL/Pillow not installed")
    print("   Install with: pip install Pillow")
    sys.exit(1)

ICON_DIR = Path(__file__).parent
LOGO_SVG = ICON_DIR / "logo.svg"

# TDF Brand Colors
PRIMARY_BLUE = "#1e40af"
ACCENT_PURPLE = "#7c3aed"
SUCCESS_GREEN = "#10b981"

def hex_to_rgb(hex_color):
    """Convert hex color to RGB tuple"""
    hex_color = hex_color.lstrip('#')
    return tuple(int(hex_color[i:i+2], 16) for i in (0, 2, 4))

def create_tdf_logo_image(size):
    """Create TDF logo as PIL Image"""
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    
    # Calculate scaling
    scale = size / 512
    center = size / 2
    
    # Background circle (subtle)
    bg_radius = int(240 * scale)
    bg_color = (*hex_to_rgb(PRIMARY_BLUE), 25)  # 10% opacity
    draw.ellipse(
        [center - bg_radius, center - bg_radius, 
         center + bg_radius, center + bg_radius],
        fill=bg_color
    )
    
    # Shield shape (simplified - using rounded rectangle as base)
    shield_width = int(152 * scale)
    shield_height = int(250 * scale)
    shield_x = center - shield_width // 2
    shield_y = center - shield_height // 2 + int(20 * scale)
    
    # Create shield path (simplified)
    shield_points = [
        (shield_x + int(76 * scale), shield_y),  # Top center
        (shield_x, shield_y + int(30 * scale)),  # Top left
        (shield_x, shield_y + shield_height - int(140 * scale)),  # Left curve start
        (shield_x, shield_y + shield_height - int(40 * scale)),  # Left bottom
        (shield_x + shield_width // 2, shield_y + shield_height),  # Bottom center
        (shield_x + shield_width, shield_y + shield_height - int(40 * scale)),  # Right bottom
        (shield_x + shield_width, shield_y + shield_height - int(140 * scale)),  # Right curve start
        (shield_x + shield_width, shield_y + int(30 * scale)),  # Top right
    ]
    
    # Draw shield with gradient effect (simplified - solid color)
    shield_color = hex_to_rgb(PRIMARY_BLUE)
    draw.polygon(shield_points, fill=shield_color)
    
    # Document symbol inside shield
    doc_x = shield_x + int(40 * scale)
    doc_y = shield_y + int(120 * scale)
    doc_width = int(72 * scale)
    doc_height = int(96 * scale)
    
    # Document background
    doc_bg = (255, 255, 255, 240)  # White with slight transparency
    draw.rounded_rectangle(
        [doc_x, doc_y, doc_x + doc_width, doc_y + doc_height],
        radius=int(4 * scale),
        fill=doc_bg
    )
    
    # Document lines
    line_color = hex_to_rgb(PRIMARY_BLUE)
    line_y1 = doc_y + int(20 * scale)
    line_y2 = doc_y + int(40 * scale)
    line_y3 = doc_y + int(60 * scale)
    line_x_start = doc_x + int(20 * scale)
    line_x_end = doc_x + doc_width - int(20 * scale)
    line_x_mid = doc_x + int(60 * scale)
    
    # Three lines of text
    draw.line([line_x_start, line_y1, line_x_end, line_y1], fill=line_color, width=int(3 * scale))
    draw.line([line_x_start, line_y2, line_x_end, line_y2], fill=line_color, width=int(2 * scale))
    draw.line([line_x_start, line_y3, line_x_mid, line_y3], fill=line_color, width=int(2 * scale))
    
    # Lock symbol
    lock_x = doc_x + int(36 * scale)
    lock_y = doc_y + int(80 * scale)
    lock_width = int(40 * scale)
    lock_height = int(32 * scale)
    
    # Lock body
    draw.rounded_rectangle(
        [lock_x, lock_y, lock_x + lock_width, lock_y + lock_height],
        radius=int(2 * scale),
        fill=line_color
    )
    
    # Lock shackle
    shackle_radius = int(8 * scale)
    shackle_y = lock_y - int(12 * scale)
    draw.arc(
        [lock_x + int(12 * scale), shackle_y - shackle_radius,
         lock_x + lock_width - int(12 * scale), shackle_y + shackle_radius],
        start=180,
        end=0,
        fill=line_color,
        width=int(4 * scale)
    )
    
    # Checkmark (verification)
    check_color = hex_to_rgb(SUCCESS_GREEN)
    check_x1 = shield_x + int(20 * scale)
    check_y1 = shield_y + shield_height - int(40 * scale)
    check_x2 = shield_x + int(40 * scale)
    check_y2 = shield_y + shield_height - int(20 * scale)
    check_x3 = shield_x + shield_width - int(20 * scale)
    check_y3 = shield_y + shield_height - int(80 * scale)
    
    # Draw checkmark
    draw.line([check_x1, check_y1, check_x2, check_y2], fill=check_color, width=int(12 * scale))
    draw.line([check_x2, check_y2, check_x3, check_y3], fill=check_color, width=int(12 * scale))
    
    return img

def create_png_icons():
    """Create PNG icons at various sizes"""
    print("📐 Generating PNG icons...")
    
    sizes = [16, 32, 48, 128, 256, 512, 1024]
    
    for size in sizes:
        img = create_tdf_logo_image(size)
        if size == 256:
            output = ICON_DIR / "128x128@2x.png"
        elif size == 128:
            output = ICON_DIR / "128x128.png"
        elif size == 32:
            output = ICON_DIR / "32x32.png"
        else:
            output = ICON_DIR / f"icon-{size}.png"
        
        img.save(output, "PNG")
        print(f"   ✅ Created {output.name} ({size}x{size})")

def create_ico_file():
    """Create Windows ICO file with multiple sizes"""
    print("\n🪟 Creating Windows ICO file...")
    
    # ICO format requires multiple sizes
    sizes = [16, 32, 48, 128, 256]
    images = []
    
    for size in sizes:
        img = create_tdf_logo_image(size)
        # ICO format needs RGBA converted properly
        if img.mode != 'RGBA':
            img = img.convert('RGBA')
        images.append((size, img))
    
    # Create ICO file
    ico_path = ICON_DIR / "icon.ico"
    
    # Write ICO header
    with open(ico_path, 'wb') as f:
        # ICO file header
        f.write(struct.pack('<HHH', 0, 1, len(images)))  # Reserved, Type, Count
        
        # Image directory entries
        offset = 6 + (len(images) * 16)  # Header + directory entries
        for size, img in images:
            # ICO directory entry
            width = size if size < 256 else 0
            height = size if size < 256 else 0
            colors = 0  # No palette
            reserved = 0
            planes = 1
            bpp = 32  # 32-bit RGBA
            size_bytes = len(img.tobytes())
            
            f.write(struct.pack('<BBBBHHII', width, height, colors, reserved, planes, bpp, size_bytes, offset))
            offset += size_bytes
        
        # Write image data
        for size, img in images:
            # Convert to ICO format (BMP-like)
            # ICO uses BMP format for image data
            bmp_data = img.tobytes('raw', 'BGRA')
            f.write(bmp_data)
    
    print(f"   ✅ Created {ico_path}")

def create_icns_iconset():
    """Create macOS iconset directory structure"""
    print("\n🍎 Creating macOS iconset...")
    
    iconset_dir = ICON_DIR / "icon.iconset"
    iconset_dir.mkdir(exist_ok=True)
    
    # macOS iconset naming convention
    icon_sizes = {
        "icon_16x16.png": 16,
        "icon_16x16@2x.png": 32,
        "icon_32x32.png": 32,
        "icon_32x32@2x.png": 64,
        "icon_128x128.png": 128,
        "icon_128x128@2x.png": 256,
        "icon_256x256.png": 256,
        "icon_256x256@2x.png": 512,
        "icon_512x512.png": 512,
        "icon_512x512@2x.png": 1024,
    }
    
    for filename, size in icon_sizes.items():
        img = create_tdf_logo_image(size)
        output = iconset_dir / filename
        img.save(output, "PNG")
        print(f"   ✅ Created {filename}")
    
    print(f"\n   📁 Iconset created at: {iconset_dir}")
    print(f"   💡 To create ICNS on macOS, run:")
    print(f"      iconutil -c icns {iconset_dir} -o {ICON_DIR}/icon.icns")

def main():
    print("🎨 TDF Brand Icon Generator")
    print("=" * 40)
    print()
    
    # Create PNG icons
    create_png_icons()
    
    # Create ICO file
    try:
        create_ico_file()
    except Exception as e:
        print(f"   ⚠️  ICO creation failed: {e}")
        print("   💡 You can create ICO manually or use online converter")
    
    # Create iconset for macOS
    create_icns_iconset()
    
    print()
    print("✨ Icon generation complete!")
    print()
    print("📦 Generated files:")
    for ext in ['png', 'ico']:
        for file in ICON_DIR.glob(f"*.{ext}"):
            size = file.stat().st_size
            print(f"   {file.name} ({size:,} bytes)")
    
    iconset_dir = ICON_DIR / "icon.iconset"
    if iconset_dir.exists():
        print(f"   icon.iconset/ (directory)")
    
    print()
    print("💡 Next steps:")
    print("   1. Review the generated icons")
    print("   2. On macOS: iconutil -c icns icons/icon.iconset -o icons/icon.icns")
    print("   3. Update tauri.conf.json to include icon.ico and icon.icns")

if __name__ == "__main__":
    main()
