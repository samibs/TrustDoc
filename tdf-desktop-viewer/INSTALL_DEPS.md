# Installing Dependencies for Desktop Viewer

## ✅ Configuration Fixed!

The Tauri configuration is now correct. You just need to install system dependencies.

## Linux (WSL/Ubuntu)

**Run this command:**

```bash
sudo apt-get update
sudo apt-get install -y \
    libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libglib2.0-dev \
    pkg-config
```

**Note**: `libglib2.0-dev` includes gobject-2.0, so you don't need a separate `libgobject-2.0-dev` package.

## Windows

Install **Microsoft Visual C++ Build Tools**:
- Download from: https://visualstudio.microsoft.com/downloads/
- Select "Build Tools for Visual Studio"
- Install "Desktop development with C++" workload

## macOS

Install Xcode Command Line Tools:
```bash
xcode-select --install
```

## After Installing Dependencies

```bash
cd tdf-desktop-viewer
npm run tauri:dev
```

The app should compile and launch! 🎉

## Note for WSL

If you're on WSL and don't have a display server, you'll need:
- **WSLg** (Windows 11 with WSL2) - recommended
- Or X11 forwarding (if using X11)
- Or a remote X server

For headless testing, you can build without running:
```bash
npm run tauri:build
```

## Verification

After installing dependencies, you should see:
```
✅ Compiling tdf-desktop-viewer
✅ Application window opens
```

If you see compilation errors about missing libraries, make sure all packages above are installed.
