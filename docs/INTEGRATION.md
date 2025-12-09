# TDF Integration Guide for Third-Parties

This guide explains how to integrate TDF (TrustDoc Financial) format conversion into your applications.

## Overview

TDF provides a comprehensive conversion library (`tdf-convert`) that supports converting from multiple formats to TDF:

- **CSV** - Comma-separated values
- **XLSX/XLS** - Microsoft Excel spreadsheets
- **DOCX/DOC** - Microsoft Word documents
- **PPTX/PPT** - Microsoft PowerPoint presentations
- **TXT** - Plain text files
- **MD** - Markdown documents
- **PDF** - Portable Document Format

## Installation

### Rust Projects

Add to your `Cargo.toml`:

```toml
[dependencies]
tdf-convert = { path = "../tdf-convert" }  # Local path
# Or from crates.io when published:
# tdf-convert = "0.1.0"
```

## Basic Usage

### Convert a Single File

```rust
use tdf_convert::convert_file;
use std::path::PathBuf;

let input = PathBuf::from("document.xlsx");
let output = PathBuf::from("document.tdf");

convert_file(
    &input,
    &output,
    Some("did:example:signer".to_string()),  // Optional signer ID
    Some("John Doe".to_string()),            // Optional signer name
    None,                                     // Optional signing key bytes
)?;
```

### Check Supported Formats

```rust
use tdf_convert::{supported_formats, is_supported_format};

// Get all supported formats
let formats = supported_formats();
println!("Supported: {:?}", formats);
// Output: ["csv", "xlsx", "xls", "docx", "doc", "pptx", "ppt", "txt", "md", "markdown", "pdf"]

// Check if a format is supported
if is_supported_format("xlsx") {
    println!("Excel files are supported!");
}
```

### Batch Conversion

```rust
use std::fs;
use std::path::PathBuf;
use tdf_convert::{convert_file, supported_formats};

let input_dir = PathBuf::from("documents");
let output_dir = PathBuf::from("tdf_documents");
fs::create_dir_all(&output_dir)?;

let supported = supported_formats();

for entry in fs::read_dir(&input_dir)? {
    let entry = entry?;
    let path = entry.path();
    
    if let Some(ext) = path.extension() {
        let ext_str = ext.to_str().unwrap_or("").to_lowercase();
        if supported.contains(&ext_str.as_str()) {
            let output = output_dir.join(
                path.file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
            ).with_extension("tdf");
            
            convert_file(&path, &output, None, None, None)?;
            println!("Converted: {:?} -> {:?}", path, output);
        }
    }
}
```

## Format-Specific Details

### CSV Conversion

- Automatically detects column types (text, number, currency, percentage, date)
- First row becomes table headers
- Creates structured table in TDF format

### Excel (XLSX/XLS) Conversion

- Processes all sheets in workbook
- Each sheet becomes a separate section
- Preserves table structure and data types

### Word (DOCX) Conversion

- Extracts text content from document
- Preserves paragraph structure
- Headings are detected and converted

### PowerPoint (PPTX) Conversion

- Each slide becomes a separate section
- Extracts text from slide content
- Preserves slide order

### PDF Conversion

- Uses advanced text extraction
- Detects headings vs paragraphs
- Handles multi-page documents

### Text (TXT) Conversion

- Splits on double newlines (paragraph breaks)
- Simple paragraph structure

### Markdown (MD) Conversion

- Parses markdown syntax
- Converts headings, paragraphs, lists
- Preserves document hierarchy

## Error Handling

```rust
use tdf_convert::{convert_file, ConvertError};

match convert_file(&input, &output, None, None, None) {
    Ok(_) => println!("Conversion successful"),
    Err(ConvertError::UnsupportedFormat(fmt)) => {
        eprintln!("Format '{}' is not supported", fmt);
    }
    Err(ConvertError::Io(e)) => {
        eprintln!("IO error: {}", e);
    }
    Err(ConvertError::Tdf(e)) => {
        eprintln!("TDF error: {}", e);
    }
    Err(e) => {
        eprintln!("Conversion error: {}", e);
    }
}
```

## Digital Signing

To sign converted documents:

```rust
use std::fs;
use ed25519_dalek::SigningKey;

// Load your signing key
let key_bytes = fs::read("signing-key.signing")?;
let signing_key = SigningKey::from_bytes(&key_bytes[..32].try_into()?);

// Convert and sign
convert_file(
    &input,
    &output,
    Some("did:example:your-org".to_string()),
    Some("Your Organization".to_string()),
    Some(&signing_key.to_bytes()),
)?;
```

## Integration Examples

### REST API Integration

```rust
use actix_web::{web, App, HttpServer, Result};
use tdf_convert::convert_file;
use std::path::PathBuf;

async fn convert_document(
    input_path: web::Path<String>,
) -> Result<String> {
    let input = PathBuf::from(&*input_path);
    let output = input.with_extension("tdf");
    
    convert_file(&input, &output, None, None, None)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    
    Ok(format!("Converted to: {}", output.display()))
}
```

### CLI Tool Integration

```rust
use clap::Parser;
use tdf_convert::{convert_file, supported_formats};

#[derive(Parser)]
struct Args {
    input: PathBuf,
    output: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let output = args.output.unwrap_or_else(|| {
        args.input.with_extension("tdf")
    });
    
    convert_file(&args.input, &output, None, None, None)?;
    println!("Converted: {}", output.display());
    Ok(())
}
```

## Best Practices

1. **Check Format Support First**
   ```rust
   if !tdf_convert::is_supported_format("xlsx") {
       return Err("Format not supported");
   }
   ```

2. **Handle Errors Gracefully**
   - Always check for `UnsupportedFormat` errors
   - Provide user-friendly error messages
   - Log conversion failures for debugging

3. **Batch Processing**
   - Process files in parallel when possible
   - Show progress for large batches
   - Handle individual file failures without stopping

4. **Memory Management**
   - For large files, consider streaming
   - Monitor memory usage during batch conversions

## Security Considerations

- **Signing Keys**: Never expose signing keys in logs or error messages
- **Input Validation**: Validate file paths and extensions before conversion
- **Output Permissions**: Set appropriate file permissions on output TDF files
- **Error Messages**: Don't leak sensitive information in error messages

## Performance Tips

- **Parallel Processing**: Use `rayon` or similar for batch conversions
- **Caching**: Cache conversion results when possible
- **Resource Limits**: Set timeouts for large file conversions

## Support

For issues, questions, or contributions:
- GitHub: [Repository URL]
- Documentation: See `SPEC.md` for format details
- Examples: See `examples/` directory

## License

TDF Convert is licensed under MIT OR Apache-2.0.

