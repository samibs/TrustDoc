# Viewer Quick Start Guide

## Desktop Viewer

### First Time Setup

```bash
cd tdf-desktop-viewer
npm install
```

### Run in Development

```bash
npm run tauri dev
```

This will:
1. Start Vite dev server (frontend)
2. Compile Rust backend
3. Launch desktop window
4. Enable hot reload

### Build for Production

```bash
npm run tauri build
```

**Output:**
- Windows: `src-tauri/target/release/bundle/msi/TDF-Viewer_0.1.0_x64_en-US.msi`
- macOS: `src-tauri/target/release/bundle/dmg/TDF-Viewer_0.1.0_aarch64.dmg`
- Linux: `src-tauri/target/release/bundle/appimage/TDF-Viewer_0.1.0_amd64.AppImage`

### Features

- 📂 **Open Files**: File → Open or drag & drop
- 🔍 **Verify**: Click "Verify" button to check integrity
- 📊 **Extract**: Click "Extract" to export JSON data
- 🖨️ **Print**: Click "Print" to print document

## Mobile Viewer

### First Time Setup

```bash
cd tdf-mobile
npm install
```

### Run in Development

```bash
# Start Expo dev server
npm start

# Then:
# - Scan QR code with Expo Go app (iOS/Android)
# - Or press 'i' for iOS simulator
# - Or press 'a' for Android emulator
```

### Build for Production

**Using EAS (Recommended):**
```bash
npm install -g eas-cli
eas login
eas build:configure
eas build --platform ios
eas build --platform android
```

**Local Builds:**
```bash
# iOS (macOS only)
npm run ios -- --configuration Release

# Android
cd android && ./gradlew assembleRelease
```

### Features

- 📂 **Open Files**: Tap "Open" button, select TDF file
- 🔍 **Verify**: Tap "Verify" to check integrity
- 📊 **Extract**: Tap "Extract" to share JSON data
- 📱 **Native UI**: Platform-specific design

## Troubleshooting

### Desktop: "tauri command not found"
```bash
npm install -g @tauri-apps/cli
```

### Desktop: Build fails on Linux
```bash
sudo apt install libwebkit2gtk-4.0-dev libgtk-3-dev
```

### Mobile: "expo command not found"
```bash
npm install -g expo-cli
```

### Mobile: Cannot find 'tdf-ts'
```bash
cd ../tdf-ts && npm install && npm run build
cd ../tdf-mobile && npm install
```

## File Structure

```
tdf-desktop-viewer/
├── src/              # Frontend (TypeScript)
├── src-tauri/        # Backend (Rust)
├── package.json
└── tauri.conf.json

tdf-mobile/
├── App.tsx           # Main app
├── src/components/   # React components
├── package.json
└── app.json          # Expo config
```

## Next Steps

1. **Test Viewers**
   - Desktop: Open a TDF file
   - Mobile: Test on device/emulator

2. **Create Icons**
   - Desktop: Generate icons for all sizes
   - Mobile: Create app icons and splash screens

3. **Distribution**
   - Desktop: Test installers on each platform
   - Mobile: Prepare for app stores

---

*For detailed setup, see VIEWER_SETUP_GUIDE.md*

