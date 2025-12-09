# TDF Multi-Format Conversion System

## Overview

TDF now supports conversion from **7+ document formats** to provide a comprehensive financial security base. This enables organizations to migrate their existing documents to the secure TDF format.

## Supported Formats

| Format | Extensions | Features |
|--------|-----------|----------|
| **CSV** | `.csv` | Automatic type detection (currency, dates, numbers) |
| **Excel** | `.xlsx`, `.xls` | Multi-sheet support, preserves table structure |
| **Word** | `.docx`, `.doc` | Text extraction, paragraph structure |
| **PowerPoint** | `.pptx`, `.ppt` | Slide-by-slide conversion |
| **Text** | `.txt` | Simple paragraph structure |
| **Markdown** | `.md`, `.markdown` | Full markdown parsing, headings, lists |
| **PDF** | `.pdf` | Advanced text extraction, structure detection |

## Architecture

### Library Structure

```
tdf-convert/
├── src/
│   ├── lib.rs          # Main conversion API
│   ├── csv.rs          # CSV converter
│   ├── excel.rs        # Excel converter
│   ├── docx.rs         # Word converter
│   ├── pptx.rs         # PowerPoint converter
│   ├── text.rs         # Text converter
│   ├── markdown.rs    # Markdown converter
│   ├── pdf.rs          # PDF converter
│   ├── error.rs        # Error types
│   └── styles/
│       └── default.css # Default corporate styling
```

### Integration Points

1. **CLI Tool**: `tdf import` command supports all formats
2. **Rust Library**: `tdf-convert` crate for programmatic use
3. **Third-Party Integration**: Well-documented API in `docs/INTEGRATION.md`

## Usage

### CLI Usage

```bash
# Convert single file
tdf import document.xlsx -o document.tdf

# Convert all supported files in folder
tdf import data/ --batch -o data/tdf/

# With digital signing
tdf import data/ --batch --key signing-key.signing \
  --signer-id "did:example:org" \
  --signer-name "Organization Name"
```

### Programmatic Usage

```rust
use tdf_convert::convert_file;
use std::path::PathBuf;

convert_file(
    &PathBuf::from("document.xlsx"),
    &PathBuf::from("document.tdf"),
    Some("did:example:signer".to_string()),
    Some("Signer Name".to_string()),
    None,  // Optional signing key bytes
)?;
```

## Format-Specific Features

### CSV
- **Type Detection**: Automatically detects currency, dates, percentages, numbers
- **Table Structure**: First row becomes headers, creates structured table
- **Currency Detection**: Recognizes $, €, £ symbols

### Excel (XLSX/XLS)
- **Multi-Sheet**: Each worksheet becomes a separate section
- **Data Types**: Preserves Excel cell types
- **Large Files**: Handles large spreadsheets efficiently

### Word (DOCX)
- **Text Extraction**: Extracts all text content
- **Structure**: Preserves paragraph breaks
- **Formatting**: Basic formatting preserved

### PowerPoint (PPTX)
- **Slide-by-Slide**: Each slide becomes a section
- **Text Extraction**: Extracts text from all slides
- **Order**: Preserves slide order

### PDF
- **Advanced Extraction**: Uses pdf-extract library
- **Structure Detection**: Detects headings vs paragraphs
- **Multi-Page**: Handles documents with many pages

### Markdown
- **Full Parsing**: Complete markdown syntax support
- **Headings**: Converts H1-H6 to TDF headings
- **Lists**: Preserves ordered and unordered lists

### Text
- **Simple**: Paragraph-based structure
- **Clean**: Removes extra whitespace

## Third-Party Integration

### For Developers

The `tdf-convert` library is designed for easy integration:

1. **Simple API**: Single function for all formats
2. **Error Handling**: Comprehensive error types
3. **Format Detection**: Automatic format detection
4. **Documentation**: Complete integration guide

### Integration Examples

See `docs/INTEGRATION.md` for:
- REST API integration
- CLI tool integration
- Batch processing
- Error handling
- Digital signing

## Benefits

1. **Universal Migration**: Convert from any common format
2. **Security**: All converted documents are cryptographically secured
3. **Structure**: Preserves document hierarchy and data types
4. **Automation**: Batch conversion for large migrations
5. **Integration**: Easy to integrate into existing systems

## Future Enhancements

- OCR support for scanned PDFs
- Image extraction from documents
- Advanced table detection
- Style preservation from source documents
- Incremental conversion (only changed files)

## License

MIT OR Apache-2.0

