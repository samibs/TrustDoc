use std::path::Path;
use tdf_core::archive::ArchiveBuilder;
use tdf_core::content::{ContentBlock, DocumentContent, Section};
use tdf_core::document::Document;
use crate::error::ConvertError;

pub fn convert_pdf_to_tdf(
    input: &Path,
    output: &Path,
    signer_id: Option<String>,
    signer_name: Option<String>,
    _signing_key: Option<&[u8]>,
) -> Result<(), ConvertError> {
    // Use pdf-extract for text extraction
    let text_result = pdf_extract::extract_text(input)
        .map_err(|e| ConvertError::Pdf(format!("Failed to extract text: {}", e)))?;
    
    // Process extracted text
    let paragraphs: Vec<&str> = text_result
        .split("\n\n")
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .collect();
    
    let mut content_blocks = Vec::new();
    
    // Add title
    let title = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("PDF Import")
        .to_string();
    
    content_blocks.push(ContentBlock::Heading {
        level: 1,
        text: title.clone(),
        id: Some("title".to_string()),
    });
    
    // Process paragraphs
    for (idx, para) in paragraphs.iter().enumerate() {
        let trimmed = para.trim();
        if trimmed.is_empty() {
            continue;
        }
        
        // Simple heading detection
        let is_heading = trimmed.len() < 80
            && (trimmed.chars().all(|c| c.is_uppercase() || c.is_whitespace() || c.is_ascii_punctuation())
                || (trimmed.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
                    && !trimmed.contains('.')
                    && !trimmed.ends_with('.')))
            && trimmed.len() > 3;
        
        if is_heading {
            content_blocks.push(ContentBlock::Heading {
                level: 2,
                text: trimmed.to_string(),
                id: Some(format!("heading-{}", idx)),
            });
        } else {
            content_blocks.push(ContentBlock::Paragraph {
                text: trimmed.to_string(),
                id: Some(format!("para-{}", idx)),
            });
        }
    }
    
    if content_blocks.len() == 1 {
        // Only title, add full text
        content_blocks.push(ContentBlock::Paragraph {
            text: text_result.trim().to_string(),
            id: Some("content".to_string()),
        });
    }
    
    let content = DocumentContent {
        sections: vec![Section {
            id: "content".to_string(),
            title: Some("Content".to_string()),
            content: content_blocks,
        }],
    };
    
    let styles = get_default_styles();
    let mut document = Document::new(title, "en".to_string(), content, styles);
    document.manifest.integrity.algorithm = tdf_core::document::HashAlgorithm::Sha256;
    
    // Build archive
    let mut builder = ArchiveBuilder::new(document);
    builder.build(
        output,
        None,
        signer_id,
        signer_name,
    )?;
    
    Ok(())
}

fn get_default_styles() -> String {
    include_str!("styles/default.css").to_string()
}

