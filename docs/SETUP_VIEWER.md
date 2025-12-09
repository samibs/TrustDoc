# Setting Up the TDF Web Viewer

## Quick Setup

```bash
# 1. Build the TypeScript SDK
cd tdf-ts
npm install
npm run build
cd ..

# 2. Install viewer dependencies
cd tdf-viewer
npm install
cd ..

# 3. Start the viewer
cd tdf-viewer
npm run dev
```

Then open `http://localhost:5173` in your browser.

## Troubleshooting

### Error: "Failed to resolve import 'tdf-ts'"

**Solution**: Build the `tdf-ts` package first:

```bash
cd tdf-ts
npm install
npm run build
cd ../tdf-viewer
npm install
```

### TypeScript Errors in tdf-ts

If you see errors about `cbor-web` types, the type declaration file should be in `tdf-ts/src/cbor-web.d.ts`. If missing, create it:

```typescript
// tdf-ts/src/cbor-web.d.ts
declare module 'cbor-web' {
  export function decode(input: Uint8Array | ArrayBuffer): any;
  export function encode(input: any): Uint8Array;
}
```

### Vite Can't Find tdf-ts

Check `vite.config.ts` - it should have:

```typescript
resolve: {
  alias: {
    'tdf-ts': '../tdf-ts/dist',
  },
}
```

Make sure `tdf-ts/dist` exists (run `npm run build` in `tdf-ts`).

## Verify Setup

1. **Check tdf-ts is built**:
   ```bash
   ls tdf-ts/dist/
   # Should show: index.js, index.d.ts, document.js, etc.
   ```

2. **Check viewer dependencies**:
   ```bash
   ls tdf-viewer/node_modules/tdf-ts
   # Should be a symlink to ../../tdf-ts
   ```

3. **Start dev server**:
   ```bash
   cd tdf-viewer
   npm run dev
   # Should start without errors
   ```

## Using the Viewer

1. Open `http://localhost:5173`
2. Drag and drop a `.tdf` file (e.g., `demo-invoice.tdf`)
3. View the document
4. Use buttons to verify, extract, or print

## Production Build

```bash
cd tdf-viewer
npm run build
# Output in dist/
```

Serve the `dist/` directory with any static file server.

