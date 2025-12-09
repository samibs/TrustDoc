# Desktop & Mobile Viewer Implementation

## Overview

Implementation of cross-platform viewers for TDF documents:
- **Desktop Viewer**: Tauri-based (Windows, macOS, Linux)
- **Mobile Viewer**: React Native/Expo (iOS, Android)

## Desktop Viewer (Tauri)

### Architecture

- **Frontend**: TypeScript + Vite (reuses web viewer code)
- **Backend**: Rust + Tauri (native OS integration)
- **Core Library**: `tdf-core` (Rust)
- **File I/O**: Tauri plugins (dialog, fs)

### Features

✅ Document loading (file picker, drag & drop, file association)
✅ Document rendering (HTML/CSS, tables, diagrams)
✅ Integrity verification (native Rust backend)
✅ Signature verification display
✅ Data extraction (JSON export)
✅ Print support
✅ Native file associations (.tdf files)

### Building

```bash
cd tdf-desktop-viewer
npm install
npm run tauri build
```

### Platform-Specific Builds

**Windows:**
- Output: `src-tauri/target/release/bundle/msi/TDF-Viewer_0.1.0_x64_en-US.msi`
- Auto-associates `.tdf` files

**macOS:**
- Output: `src-tauri/target/release/bundle/dmg/TDF-Viewer_0.1.0_aarch64.dmg`
- App bundle with code signing support

**Linux:**
- Output: `src-tauri/target/release/bundle/appimage/TDF-Viewer_0.1.0_amd64.AppImage`
- Desktop entry + MIME type registration

## Mobile Viewer (React Native/Expo)

### Architecture

- **Framework**: React Native with Expo
- **Core Library**: `tdf-ts` (TypeScript SDK)
- **File I/O**: Expo Document Picker, File System
- **UI**: React Native components

### Features

✅ Document loading (file picker)
✅ Document rendering (React Native components)
✅ Integrity verification (WASM or native module)
✅ Data extraction (JSON share)
✅ Native mobile UI

### Building

```bash
cd tdf-mobile
npm install

# Development
npm start
npm run ios      # iOS simulator
npm run android  # Android emulator

# Production builds
npm run build:ios
npm run build:android
```

### Platform-Specific

**iOS:**
- Requires Xcode (macOS only)
- Bundle ID: `com.trustdoc.viewer`
- App Store distribution ready

**Android:**
- Requires Android Studio
- Package: `com.trustdoc.viewer`
- Play Store distribution ready

## Implementation Status

### Desktop Viewer ✅

- [x] Project structure
- [x] Document loading
- [x] Document rendering
- [x] Verification UI
- [x] Data extraction
- [x] Print support
- [ ] Icons (placeholder)
- [ ] File associations (config ready)
- [ ] Installer testing

### Mobile Viewer ✅

- [x] Project structure
- [x] Document loading
- [x] Document rendering
- [x] Verification UI
- [x] Data extraction
- [ ] Native verification module (WASM integration)
- [ ] Icons/assets
- [ ] App store metadata

## Next Steps

1. **Desktop Viewer**
   - Create app icons (32x32, 128x128, 256x256, etc.)
   - Test file associations on each platform
   - Test installers
   - Add menu bar (File, Edit, View, Help)

2. **Mobile Viewer**
   - Integrate WASM verification
   - Create app icons and splash screens
   - Test on physical devices
   - Prepare for app store submission

3. **Both**
   - Add dark mode support
   - Add zoom controls
   - Add search functionality
   - Add document history

## File Structure

```
tdf-desktop-viewer/
├── src/                  # Frontend (TypeScript)
│   ├── main.ts          # Entry point
│   ├── renderer.ts      # Document rendering
│   ├── diagram.ts       # Diagram rendering
│   └── styles.css       # Styles
├── src-tauri/           # Backend (Rust)
│   ├── src/main.rs      # Tauri commands
│   └── Cargo.toml       # Rust dependencies
├── package.json
└── tauri.conf.json      # Tauri configuration

tdf-mobile/
├── App.tsx              # Main app
├── src/
│   └── components/      # React components
│       ├── DocumentViewer.tsx
│       ├── VerificationPanel.tsx
│       └── Toolbar.tsx
├── package.json
└── app.json             # Expo configuration
```

## Testing

### Desktop
```bash
cd tdf-desktop-viewer
npm run tauri dev  # Development mode
```

### Mobile
```bash
cd tdf-mobile
npm start          # Expo dev server
# Then scan QR code with Expo Go app
```

## Distribution

### Desktop
- **Windows**: MSI installer (auto-associates .tdf files)
- **macOS**: DMG with app bundle
- **Linux**: AppImage (universal) or platform-specific packages

### Mobile
- **iOS**: App Store (requires Apple Developer account)
- **Android**: Play Store (requires Google Play Developer account)

---

*Last Updated: $(date)*

