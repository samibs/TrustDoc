use std::fs;
use std::path::Path;
use tdf_core::archive::ArchiveBuilder;
use tdf_core::content::{ContentBlock, DocumentContent, Section};
use tdf_core::document::Document;
use crate::error::ConvertError;
use pulldown_cmark::{Parser, Event, Tag, HeadingLevel};

pub fn convert_markdown_to_tdf(
    input: &Path,
    output: &Path,
    signer_id: Option<String>,
    signer_name: Option<String>,
    _signing_key: Option<&[u8]>,
) -> Result<(), ConvertError> {
    let content_text = fs::read_to_string(input)?;
    let parser = Parser::new(&content_text);
    
    let mut content_blocks = Vec::new();
    let mut current_heading_level: Option<HeadingLevel> = None;
    let mut current_text = String::new();
    let mut in_code_block = false;
    
    for event in parser {
        match event {
            Event::Start(Tag::Heading(level, _, _)) => {
                // Flush current paragraph if any
                if !current_text.trim().is_empty() {
                    content_blocks.push(ContentBlock::Paragraph {
                        text: current_text.trim().to_string(),
                        id: Some(format!("para-{}", content_blocks.len())),
                    });
                    current_text.clear();
                }
                current_heading_level = Some(level);
            }
            Event::End(Tag::Heading(level, _, _)) => {
                if !current_text.trim().is_empty() {
                    let level_num = match level {
                        HeadingLevel::H1 => 1,
                        HeadingLevel::H2 => 2,
                        HeadingLevel::H3 => 3,
                        HeadingLevel::H4 => 4,
                        HeadingLevel::H5 => 5,
                        HeadingLevel::H6 => 6,
                    };
                    content_blocks.push(ContentBlock::Heading {
                        level: level_num,
                        text: current_text.trim().to_string(),
                        id: Some(format!("heading-{}", content_blocks.len())),
                    });
                    current_text.clear();
                }
                current_heading_level = None;
            }
            Event::Start(Tag::CodeBlock(_)) => {
                in_code_block = true;
            }
            Event::End(Tag::CodeBlock(_)) => {
                in_code_block = false;
            }
            Event::Text(text) => {
                if !in_code_block {
                    current_text.push_str(&text);
                }
            }
            Event::SoftBreak => {
                current_text.push(' ');
            }
            Event::HardBreak => {
                current_text.push('\n');
            }
            Event::End(Tag::Paragraph) => {
                if !current_text.trim().is_empty() && current_heading_level.is_none() {
                    content_blocks.push(ContentBlock::Paragraph {
                        text: current_text.trim().to_string(),
                        id: Some(format!("para-{}", content_blocks.len())),
                    });
                    current_text.clear();
                }
            }
            _ => {}
        }
    }
    
    // Flush remaining text
    if !current_text.trim().is_empty() {
        content_blocks.push(ContentBlock::Paragraph {
            text: current_text.trim().to_string(),
            id: Some(format!("para-{}", content_blocks.len())),
        });
    }
    
    // Extract title from first heading or filename
    let title = if let Some(ContentBlock::Heading { text, .. }) = content_blocks.first() {
        text.clone()
    } else {
        input
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Markdown Import")
            .to_string()
    };
    
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

