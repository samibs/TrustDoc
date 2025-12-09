use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConvertError {
    #[error("Unsupported file format: {0}")]
    UnsupportedFormat(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("TDF error: {0}")]
    Tdf(#[from] tdf_core::error::TdfError),
    
    #[error("CSV parsing error: {0}")]
    Csv(#[from] csv::Error),
    
    #[error("Excel parsing error: {0}")]
    Excel(String),
    
    #[error("DOCX parsing error: {0}")]
    Docx(String),
    
    #[error("PPTX parsing error: {0}")]
    Pptx(String),
    
    #[error("PDF extraction error: {0}")]
    Pdf(String),
    
    #[error("Markdown parsing error: {0}")]
    Markdown(String),
    
    #[error("Conversion error: {0}")]
    Conversion(String),
}

