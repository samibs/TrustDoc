# TDF Brand Icons

This directory contains all brand icons and assets for the TDF Desktop Viewer.

## Logo Design

The TDF logo features:
- **Shield**: Security and protection
- **Document**: TDF format representation
- **Lock**: Cryptographic security
- **Checkmark**: Verification and integrity

## Files

### Master Logo
- `logo.svg` - Master SVG logo (editable source)

### PNG Icons
- `32x32.png` - Small icon (32x32px)
- `128x128.png` - Standard icon (128x128px)
- `128x128@2x.png` - Retina icon (256x256px)
- `icon-16.png` - Extra small (16x16px)
- `icon-48.png` - Medium (48x48px)
- `icon-256.png` - Large (256x256px)
- `icon-512.png` - Extra large (512x512px)
- `icon-1024.png` - Maximum size (1024x1024px)

### Platform-Specific Icons
- `icon.ico` - Windows icon (multi-size ICO file)
- `icon.icns` - macOS icon (create on macOS)
- `icon.iconset/` - macOS iconset directory

## Generating Icons

### Using Python (Recommended)

```bash
cd tdf-desktop-viewer
python3 icons/create-icons-python.py
```

**Requirements**: `pip install Pillow`

### Using ImageMagick/Inkscape

```bash
cd tdf-desktop-viewer
./icons/create-brand-icons.sh
```

**Requirements**: ImageMagick or Inkscape

### Creating ICNS on macOS

After generating the iconset:

```bash
iconutil -c icns icons/icon.iconset -o icons/icon.icns
```

## Icon Specifications

### Windows ICO
- Format: Multi-size ICO file
- Sizes: 16, 32, 48, 128, 256px
- Usage: Application icon, file associations

### macOS ICNS
- Format: ICNS bundle
- Sizes: 16, 32, 64, 128, 256, 512, 1024px (including @2x variants)
- Usage: Application icon, dock icon

### Linux
- Format: PNG
- Sizes: 32, 128, 256px
- Usage: Application icon, desktop entry

## Color Palette

- **Primary Blue**: #1e40af (Deep Blue)
- **Accent Purple**: #7c3aed (Purple)
- **Success Green**: #10b981 (Green - for checkmark)

## Updating Icons

1. Edit `logo.svg` (master source)
2. Run icon generation script
3. Test icons in the application
4. Commit updated icons

## Notes

- Icons are generated programmatically from the SVG logo
- All icons maintain the same design language
- Icons are optimized for their respective platforms
- High-resolution versions (@2x) are included for retina displays

---

**Last Updated**: 2026-01-11
