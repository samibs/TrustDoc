# TDF Mobile Viewer

Cross-platform mobile application for viewing and verifying TDF documents on iOS and Android.

## Features

- 📄 **Document Viewing**: View TDF documents with rich rendering
- 🔐 **Integrity Verification**: Verify document integrity and signatures
- 📊 **Data Extraction**: Extract structured data from documents
- 📱 **Native Feel**: Native mobile UI with platform-specific design
- ⚡ **Fast**: Optimized for mobile performance

## Supported Platforms

- ✅ iOS 13+
- ✅ Android 8.0+ (API 26+)

## Architecture

- **Framework**: React Native (Expo)
- **Core Library**: `tdf-ts` (TypeScript SDK)
- **Verification**: Native modules for cryptographic operations
- **UI**: React Native components with platform-specific styling

## Development

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

### Run on iOS

```bash
npm run ios
```

### Run on Android

```bash
npm run android
```

### Build for Production

```bash
# iOS
npm run build:ios

# Android
npm run build:android
```

## Project Structure

```
tdf-mobile/
├── App.tsx              # Main app component
├── src/
│   ├── components/      # React components
│   │   ├── DocumentViewer.tsx
│   │   ├── VerificationPanel.tsx
│   │   └── Toolbar.tsx
│   ├── screens/         # Screen components
│   │   ├── HomeScreen.tsx
│   │   └── DocumentScreen.tsx
│   ├── services/        # Business logic
│   │   ├── documentLoader.ts
│   │   └── verification.ts
│   └── utils/           # Utilities
├── ios/                 # iOS native code
├── android/             # Android native code
└── package.json
```

## Usage

1. **Open TDF File**
   - Tap "Open Document" button
   - Select from file picker
   - Or share from another app

2. **View Document**
   - Scroll to view content
   - Pinch to zoom
   - Tap sections to expand

3. **Verify Integrity**
   - Tap "Verify" button
   - View verification results
   - Check signature status

4. **Extract Data**
   - Tap "Extract" button
   - Data exported as JSON
   - Share or save

## License

MIT OR Apache-2.0

