//! TDF Format Conversion Library
//! 
//! Provides conversion from various document formats to TDF (TrustDoc Financial) format.
//! Supports: CSV, XLSX, DOCX, PPTX, TXT, MD, PDF
//!
//! # Example
//! ```no_run
//! use tdf_convert::convert_file;
//! use std::path::PathBuf;
//!
//! let input = PathBuf::from("document.xlsx");
//! let output = PathBuf::from("document.tdf");
//! convert_file(&input, &output, None, None, None)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub mod csv;
pub mod docx;
pub mod error;
pub mod excel;
pub mod markdown;
pub mod pdf;
pub mod pptx;
pub mod text;

use std::path::Path;

pub use error::ConvertError;

/// Convert a file from any supported format to TDF
pub fn convert_file(
    input: &Path,
    output: &Path,
    signer_id: Option<String>,
    signer_name: Option<String>,
    signing_key: Option<&[u8]>,
) -> Result<(), ConvertError> {
    let extension = input
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match extension.as_str() {
        "csv" => csv::convert_csv_to_tdf(input, output, signer_id, signer_name, signing_key),
        "xlsx" | "xls" => excel::convert_excel_to_tdf(input, output, signer_id, signer_name, signing_key),
        "docx" | "doc" => docx::convert_docx_to_tdf(input, output, signer_id, signer_name, signing_key),
        "pptx" | "ppt" => pptx::convert_pptx_to_tdf(input, output, signer_id, signer_name, signing_key),
        "txt" => text::convert_text_to_tdf(input, output, signer_id, signer_name, signing_key),
        "md" | "markdown" => markdown::convert_markdown_to_tdf(input, output, signer_id, signer_name, signing_key),
        "pdf" => pdf::convert_pdf_to_tdf(input, output, signer_id, signer_name, signing_key),
        _ => Err(ConvertError::UnsupportedFormat(extension)),
    }
}

/// Get list of supported file extensions
pub fn supported_formats() -> Vec<&'static str> {
    vec!["csv", "xlsx", "xls", "docx", "doc", "pptx", "ppt", "txt", "md", "markdown", "pdf"]
}

/// Check if a file format is supported
pub fn is_supported_format(extension: &str) -> bool {
    supported_formats().contains(&extension.to_lowercase().as_str())
}

