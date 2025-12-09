use std::fs;
use std::path::Path;
use tdf_core::archive::ArchiveBuilder;
use tdf_core::content::{ContentBlock, DocumentContent, Section};
use tdf_core::document::Document;
use crate::error::ConvertError;

pub fn convert_pptx_to_tdf(
    input: &Path,
    output: &Path,
    signer_id: Option<String>,
    signer_name: Option<String>,
    _signing_key: Option<&[u8]>,
) -> Result<(), ConvertError> {
    // Read PPTX file (it's a ZIP archive)
    let file = fs::File::open(input)?;
    let mut archive = zip::ZipArchive::new(std::io::BufReader::new(file))
        .map_err(|e| ConvertError::Pptx(format!("Failed to open PPTX: {}", e)))?;
    
    let mut sections = Vec::new();
    
    // PPTX structure: ppt/slides/slide1.xml, slide2.xml, etc.
    for i in 1..=100 { // Max 100 slides
        let slide_path = format!("ppt/slides/slide{}.xml", i);
        if let Ok(mut slide_file) = archive.by_name(&slide_path) {
            let mut slide_xml = String::new();
            std::io::Read::read_to_string(&mut slide_file, &mut slide_xml)
                .map_err(|e| ConvertError::Pptx(format!("Failed to read slide {}: {}", i, e)))?;
            
            let text = extract_text_from_pptx_xml(&slide_xml);
            
            if !text.trim().is_empty() {
                let mut content_blocks = Vec::new();
                content_blocks.push(ContentBlock::Heading {
                    level: 2,
                    text: format!("Slide {}", i),
                    id: Some(format!("slide-{}", i)),
                });
                
                // Split into paragraphs
                for (idx, para) in text.split("\n\n").enumerate() {
                    let trimmed = para.trim();
                    if !trimmed.is_empty() {
                        content_blocks.push(ContentBlock::Paragraph {
                            text: trimmed.to_string(),
                            id: Some(format!("slide-{}-para-{}", i, idx)),
                        });
                    }
                }
                
                sections.push(Section {
                    id: format!("slide_{}", i),
                    title: Some(format!("Slide {}", i)),
                    content: content_blocks,
                });
            }
        } else {
            break; // No more slides
        }
    }
    
    if sections.is_empty() {
        return Err(ConvertError::Conversion("No content found in PPTX file".to_string()));
    }
    
    // Create document
    let title = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("PPTX Import")
        .to_string();
    
    let content = DocumentContent { sections };
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

fn extract_text_from_pptx_xml(xml: &str) -> String {
    // Simple text extraction from PPTX XML
    let mut text = String::new();
    let mut current_text = String::new();
    
    for line in xml.lines() {
        if line.contains("<a:t>") {
            if let Some(start) = line.find("<a:t>") {
                if let Some(end) = line.find("</a:t>") {
                    let content = &line[start + 5..end];
                    current_text.push_str(content);
                }
            }
        } else if line.contains("</a:t>") {
            if !current_text.is_empty() {
                text.push_str(&current_text);
                text.push_str(" ");
                current_text.clear();
            }
        }
    }
    
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn get_default_styles() -> String {
    include_str!("styles/default.css").to_string()
}

