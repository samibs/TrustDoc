use std::fs;
use std::path::Path;
use tdf_core::archive::ArchiveBuilder;
use tdf_core::content::{ContentBlock, DocumentContent, Section};
use tdf_core::document::Document;
use crate::error::ConvertError;

pub fn convert_docx_to_tdf(
    input: &Path,
    output: &Path,
    signer_id: Option<String>,
    signer_name: Option<String>,
    _signing_key: Option<&[u8]>,
) -> Result<(), ConvertError> {
    // Read DOCX file
    let file = fs::File::open(input)?;
    let mut archive = zip::ZipArchive::new(std::io::BufReader::new(file))
        .map_err(|e| ConvertError::Docx(format!("Failed to open DOCX: {}", e)))?;
    
    // Read document.xml
    let mut doc_xml = String::new();
    let mut doc_file = archive.by_name("word/document.xml")
        .map_err(|e| ConvertError::Docx(format!("Failed to read document.xml: {}", e)))?;
    std::io::Read::read_to_string(&mut doc_file, &mut doc_xml)
        .map_err(|e| ConvertError::Docx(format!("Failed to read document content: {}", e)))?;
    
    // Parse XML and extract text
    // This is a simplified parser - for production, use a proper XML parser
    let text = extract_text_from_docx_xml(&doc_xml);
    
    // Process text into blocks
    let paragraphs: Vec<&str> = text
        .split("\n\n")
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .collect();
    
    let mut content_blocks = Vec::new();
    
    // Add title
    let title = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("DOCX Import")
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

fn extract_text_from_docx_xml(xml: &str) -> String {
    // Simple text extraction from DOCX XML
    // Remove XML tags and extract text content
    let mut text = String::new();
    let mut in_text = false;
    let mut current_text = String::new();
    
    for line in xml.lines() {
        if line.contains("<w:t>") {
            in_text = true;
            if let Some(start) = line.find("<w:t>") {
                if let Some(end) = line.find("</w:t>") {
                    let content = &line[start + 5..end];
                    current_text.push_str(content);
                }
            }
        } else if line.contains("</w:t>") {
            in_text = false;
            if !current_text.is_empty() {
                text.push_str(&current_text);
                text.push_str(" ");
                current_text.clear();
            }
        } else if in_text && line.contains("</w:t>") {
            if let Some(end) = line.find("</w:t>") {
                let content = &line[..end];
                current_text.push_str(content);
                text.push_str(&current_text);
                text.push_str(" ");
                current_text.clear();
            }
            in_text = false;
        }
    }
    
    // Clean up multiple spaces
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn get_default_styles() -> String {
    include_str!("styles/default.css").to_string()
}

