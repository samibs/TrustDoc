# Viewer Setup & Build Guide

## Desktop Viewer (Tauri)

### Prerequisites

**All Platforms:**
- Rust (latest stable) - `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Node.js 18+ - `https://nodejs.org/`

**Windows:**
- Microsoft Visual C++ Build Tools
- Or install Visual Studio with C++ workload

**macOS:**
- Xcode Command Line Tools: `xcode-select --install`

**Linux:**
```bash
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

### Setup

```bash
cd tdf-desktop-viewer
npm install
```

### Development

```bash
npm run tauri dev
```

Opens a development window with hot reload.

### Building

```bash
# Build for current platform
npm run tauri build

# Output locations:
# Windows: src-tauri/target/release/bundle/msi/
# macOS: src-tauri/target/release/bundle/dmg/
# Linux: src-tauri/target/release/bundle/appimage/
```

### File Associations

The Tauri configuration (`tauri.conf.json`) includes file association settings. After installation:
- **Windows**: `.tdf` files open with TDF Viewer
- **macOS**: `.tdf` files open with TDF Viewer
- **Linux**: `.tdf` files open with TDF Viewer (via desktop entry)

## Mobile Viewer (React Native/Expo)

### Prerequisites

- Node.js 18+
- npm or yarn
- **iOS**: Xcode 14+ (macOS only)
- **Android**: Android Studio with Android SDK

### Setup

```bash
cd tdf-mobile
npm install
```

### Development

```bash
# Start Expo dev server
npm start

# Run on iOS simulator (macOS only)
npm run ios

# Run on Android emulator
npm run android

# Run on web (for testing)
npm run web
```

### Building

**Using Expo Application Services (EAS):**

```bash
# Install EAS CLI
npm install -g eas-cli

# Login
eas login

# Configure
eas build:configure

# Build for iOS
eas build --platform ios

# Build for Android
eas build --platform android
```

**Local Builds:**

```bash
# iOS (requires macOS + Xcode)
cd ios
pod install
cd ..
npm run ios -- --configuration Release

# Android
cd android
./gradlew assembleRelease
```

### Testing on Physical Devices

**iOS:**
1. Connect iPhone/iPad via USB
2. Run `npm run ios`
3. Select device from list

**Android:**
1. Enable USB debugging on device
2. Connect via USB
3. Run `npm run android`
4. Select device from list

## Troubleshooting

### Desktop Viewer Issues

**"tauri command not found"**
```bash
npm install -g @tauri-apps/cli
```

**"Failed to compile Rust code"**
- Ensure Rust is installed: `rustc --version`
- Update Rust: `rustup update`

**"WebView not found" (Linux)**
```bash
sudo apt install libwebkit2gtk-4.0-dev
```

### Mobile Viewer Issues

**"expo command not found"**
```bash
npm install -g expo-cli
```

**"Cannot find module 'tdf-ts'"**
```bash
cd ../tdf-ts
npm install
npm run build
cd ../tdf-mobile
npm install
```

**iOS Build Fails**
- Ensure Xcode is installed and updated
- Run `cd ios && pod install && cd ..`
- Check bundle identifier in `app.json`

**Android Build Fails**
- Ensure Android SDK is installed
- Set `ANDROID_HOME` environment variable
- Run `cd android && ./gradlew clean && cd ..`

## Next Steps

1. **Create Icons**
   - Desktop: Generate icons for all sizes (32x32 to 512x512)
   - Mobile: Generate app icons and splash screens

2. **Test File Associations**
   - Install desktop viewer
   - Double-click a `.tdf` file
   - Verify it opens in the viewer

3. **App Store Preparation**
   - Desktop: Code signing certificates
   - Mobile: App Store Connect / Play Console accounts

---

*Last Updated: $(date)*

