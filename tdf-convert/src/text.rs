use std::fs;
use std::path::Path;
use tdf_core::archive::ArchiveBuilder;
use tdf_core::content::{ContentBlock, DocumentContent, Section};
use tdf_core::document::Document;
use crate::error::ConvertError;

pub fn convert_text_to_tdf(
    input: &Path,
    output: &Path,
    signer_id: Option<String>,
    signer_name: Option<String>,
    _signing_key: Option<&[u8]>,
) -> Result<(), ConvertError> {
    let content_text = fs::read_to_string(input)?;
    
    // Split into paragraphs (double newlines)
    let paragraphs: Vec<&str> = content_text
        .split("\n\n")
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .collect();
    
    let mut content_blocks = Vec::new();
    
    // Add title
    let title = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Text Import")
        .to_string();
    
    content_blocks.push(ContentBlock::Heading {
        level: 1,
        text: title.clone(),
        id: Some("title".to_string()),
    });
    
    // Add paragraphs
    for (idx, para) in paragraphs.iter().enumerate() {
        content_blocks.push(ContentBlock::Paragraph {
            text: para.to_string(),
            id: Some(format!("para-{}", idx)),
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

