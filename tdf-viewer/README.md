# TDF Web Viewer

A browser-based viewer for TDF (TrustDoc Financial) documents.

## Features

- 📄 **Document Viewing**: View TDF documents in your browser
- 🔐 **Integrity Verification**: Verify document integrity and signatures
- 📊 **Data Extraction**: Extract structured data from documents
- 🖨️ **Print Support**: Print documents directly from the viewer
- 🎨 **Rich Rendering**: Renders text, tables, diagrams, and more
- 📱 **Responsive**: Works on desktop and mobile

## Setup

### Install Dependencies

```bash
cd tdf-viewer
npm install
```

### Development Server

```bash
npm run dev
```

Opens at `http://localhost:5173`

### Build for Production

```bash
npm run build
```

Outputs to `dist/` directory.

### Preview Production Build

```bash
npm run preview
```

## Usage

1. **Open the viewer** in your browser
2. **Drag and drop** a `.tdf` file onto the upload area, or click to browse
3. **View the document** - content is rendered with styling
4. **Verify integrity** - Click "Verify Integrity" to check document
5. **Extract data** - Click "Extract Data" to get structured JSON
6. **Print** - Click "Print" to print the document

## Features in Detail

### Document Rendering

- **Headings**: H1-H4 with proper hierarchy
- **Paragraphs**: Formatted text with line breaks
- **Tables**: Rendered with headers and styled cells
- **Lists**: Ordered and unordered lists
- **Diagrams**: SVG rendering for hierarchical and flowchart diagrams
- **Figures**: Images with captions

### Verification

Uses WASM bindings for client-side cryptographic verification:
- Merkle tree integrity check
- Signature verification (Ed25519 and secp256k1)
- No server required - all verification happens in browser

### Data Extraction

Extracts:
- Document metadata (title, ID, dates)
- All tables as structured JSON
- Content blocks

## Architecture

- **TypeScript**: Main viewer logic
- **Vite**: Build tool and dev server
- **tdf-ts SDK**: Document loading and parsing
- **WASM**: Cryptographic verification (tdf-wasm)
- **CBOR**: Binary format parsing (cbor-web)
- **JSZip**: ZIP archive handling

## Browser Support

- Chrome/Edge (latest)
- Firefox (latest)
- Safari (latest)

Requires:
- ES6 modules
- WebAssembly support
- File API

## Development

### Project Structure

```
tdf-viewer/
├── index.html          # Main HTML file
├── src/
│   ├── viewer.ts      # Main viewer logic
│   ├── renderer.ts    # Document rendering
│   ├── diagram.ts     # Diagram rendering
│   ├── verification.ts # Verification logic
│   └── styles.css     # Styling
├── package.json
└── vite.config.ts
```

### Adding Features

1. **New content types**: Add rendering in `renderer.ts`
2. **New diagram types**: Add in `diagram.ts`
3. **Verification features**: Extend `verification.ts`
4. **Styling**: Update `styles.css`

## Troubleshooting

### "Module not found" errors

Make sure dependencies are installed:
```bash
npm install
```

### WASM verification not working

Build the WASM module:
```bash
cd ../tdf-wasm
wasm-pack build --target web --out-dir pkg
cd ../tdf-viewer
```

Then update the import in `verification.ts` to point to the correct path.

### Document not loading

- Check browser console for errors
- Verify the TDF file is valid (use `tdf verify` CLI)
- Ensure file is a valid ZIP archive

## Integration

### Embed in Your Site

```html
<iframe src="path/to/tdf-viewer/index.html"></iframe>
```

### Use as Component

```typescript
import { loadDocument, renderDocument } from 'tdf-viewer';

const file = // File object
const doc = await loadDocument(file);
renderDocument(doc, document.getElementById('container'));
```

