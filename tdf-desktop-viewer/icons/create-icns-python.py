#!/usr/bin/env python3
"""
Create macOS ICNS file from iconset directory
Works on Linux, Windows, and macOS (alternative to iconutil)
"""

import os
import struct
from pathlib import Path

try:
    from PIL import Image
except ImportError:
    print("❌ Error: PIL/Pillow not installed")
    print("   Install with: pip install Pillow")
    exit(1)

ICON_DIR = Path(__file__).parent
ICONSET_DIR = ICON_DIR / "icon.iconset"
ICNS_OUTPUT = ICON_DIR / "icon.icns"

# ICNS format structure
# Based on Apple's ICNS format specification

def create_icns_from_iconset():
    """Create ICNS file from iconset directory"""
    
    if not ICONSET_DIR.exists():
        print(f"❌ Error: iconset directory not found: {ICONSET_DIR}")
        print("   Run: python3 icons/create-icons-python.py first")
        return False
    
    print("🍎 Creating macOS ICNS file from iconset...")
    
    # ICNS icon type mappings
    # Format: (filename_pattern, icns_type_code, size)
    icon_types = [
        ("icon_16x16.png", b'ic07', 16),           # 16x16
        ("icon_16x16@2x.png", b'ic08', 32),        # 16x16@2x
        ("icon_32x32.png", b'ic09', 32),           # 32x32
        ("icon_32x32@2x.png", b'ic10', 64),        # 32x32@2x
        ("icon_128x128.png", b'ic11', 128),        # 128x128
        ("icon_128x128@2x.png", b'ic12', 256),     # 128x128@2x
        ("icon_256x256.png", b'ic13', 256),        # 256x256
        ("icon_256x256@2x.png", b'ic14', 512),    # 256x256@2x
        ("icon_512x512.png", b'ic09', 512),       # 512x512 (reuse ic09 for large)
        ("icon_512x512@2x.png", b'ic10', 1024),   # 512x512@2x (reuse ic10 for large)
    ]
    
    # Collect icon data
    icon_data = []
    total_size = 0
    
    for filename, icns_type, size in icon_types:
        icon_path = ICONSET_DIR / filename
        if not icon_path.exists():
            # Try to find alternative size
            alt_files = list(ICONSET_DIR.glob(f"icon_*{size}*.png"))
            if alt_files:
                icon_path = alt_files[0]
            else:
                print(f"   ⚠️  Skipping {filename} (not found)")
                continue
        
        try:
            img = Image.open(icon_path)
            # Convert to RGBA if needed
            if img.mode != 'RGBA':
                img = img.convert('RGBA')
            
            # Get image data
            img_data = img.tobytes('raw', 'RGBA')
            data_size = len(img_data)
            
            icon_data.append({
                'type': icns_type,
                'size': data_size,
                'data': img_data,
                'filename': filename
            })
            
            total_size += 8 + data_size  # Type (4) + Size (4) + Data
            print(f"   ✅ Added {filename} ({size}x{size}, {data_size:,} bytes)")
            
        except Exception as e:
            print(f"   ⚠️  Error processing {filename}: {e}")
            continue
    
    if not icon_data:
        print("❌ Error: No valid icons found in iconset")
        return False
    
    # Write ICNS file
    print(f"\n📝 Writing ICNS file...")
    
    with open(ICNS_OUTPUT, 'wb') as f:
        # ICNS file header: 'icns' magic + total file size
        file_size = 8 + total_size  # Header (8) + all icon data
        f.write(b'icns')  # Magic number
        f.write(struct.pack('>I', file_size))  # File size (big-endian)
        
        # Write each icon
        for icon in icon_data:
            f.write(icon['type'])  # Icon type (4 bytes)
            f.write(struct.pack('>I', icon['size']))  # Data size (big-endian, 4 bytes)
            f.write(icon['data'])  # Icon data
    
    print(f"✅ ICNS file created: {ICNS_OUTPUT}")
    print(f"   Size: {ICNS_OUTPUT.stat().st_size:,} bytes")
    print(f"   Icons: {len(icon_data)}")
    
    return True

def main():
    print("🎨 TDF ICNS Generator (Cross-Platform)")
    print("=" * 40)
    print()
    
    if create_icns_from_iconset():
        print()
        print("✨ ICNS file created successfully!")
        print()
        print("💡 The ICNS file is now ready for macOS builds.")
        print("   It will be automatically included in DMG packages.")
    else:
        print()
        print("❌ Failed to create ICNS file")
        print()
        print("💡 Alternative: Use online converter:")
        print("   https://cloudconvert.com/icns-converter")
        print("   Upload the icon.iconset directory as a ZIP file")

if __name__ == "__main__":
    main()
