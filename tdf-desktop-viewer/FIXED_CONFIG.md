# ✅ Configuration Fixed!

The `tauri.conf.json` has been moved to the correct location: `src-tauri/tauri.conf.json`

## What Was Fixed

1. ✅ Moved `tauri.conf.json` from root to `src-tauri/` (Tauri v2 requirement)
2. ✅ Updated configuration format for Tauri v2
3. ✅ Fixed package.json scripts (separated `dev`/`build` from `tauri:dev`/`tauri:build`)

## Current Status

The configuration is now correct! However, you need to install Linux system dependencies.

## Install Dependencies (Run this command)

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
    pkg-config \
    libgobject-2.0-dev
```

## After Installing Dependencies

```bash
cd tdf-desktop-viewer
npm run tauri:dev
```

The app should now compile and launch! 🎉

## Note for WSL

If you're on WSL and don't have a display server, you'll need:
- X11 forwarding (if using X11)
- Or WSLg (Windows 11 with WSL2)
- Or a remote X server

For headless testing, you can build without running:
```bash
npm run tauri:build
```

