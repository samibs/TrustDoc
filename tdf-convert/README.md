# tdf-convert

Format conversion library for TDF (TrustDoc Financial) format.

## Supported Formats

- **CSV** - Comma-separated values
- **XLSX/XLS** - Microsoft Excel spreadsheets  
- **DOCX/DOC** - Microsoft Word documents
- **PPTX/PPT** - Microsoft PowerPoint presentations
- **TXT** - Plain text files
- **MD** - Markdown documents
- **PDF** - Portable Document Format

## Quick Start

```rust
use tdf_convert::convert_file;
use std::path::PathBuf;

// Convert any supported file to TDF
convert_file(
    &PathBuf::from("document.xlsx"),
    &PathBuf::from("document.tdf"),
    None,  // Optional signer ID
    None,  // Optional signer name
    None,  // Optional signing key
)?;
```

## Features

- **Multi-format Support**: Convert from 7+ common formats
- **Automatic Type Detection**: Detects currency, dates, numbers in tables
- **Structure Preservation**: Maintains document hierarchy and formatting
- **Digital Signing**: Optional cryptographic signing during conversion
- **Batch Processing**: Convert multiple files efficiently

## Documentation

See `docs/INTEGRATION.md` for detailed integration guide.

## License

MIT OR Apache-2.0

