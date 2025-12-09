use crate::utils;
use std::fs;
use std::path::{Path, PathBuf};
use tdf_core::archive::ArchiveBuilder;
use tdf_core::content::{ContentBlock, DocumentContent, Section};
use tdf_core::document::Document;
use tdf_core::error::{TdfError, TdfResult};
use tdf_core::merkle::HashAlgorithm;

pub fn create_document(
    input: PathBuf,
    output: Option<PathBuf>,
    signer_id: Option<String>,
    signer_name: Option<String>,
    key: Option<PathBuf>,
) -> TdfResult<()> {
    // Read input JSON
    let json_str = fs::read_to_string(&input)?;
    let json_value: serde_json::Value = serde_json::from_str(&json_str)?;

    // Parse document structure
    let title = json_value
        .get("title")
        .and_then(|v| v.as_str())
        .ok_or_else(|| TdfError::InvalidDocument("Missing 'title' field".to_string()))?
        .to_string();

    let language = json_value
        .get("language")
        .and_then(|v| v.as_str())
        .unwrap_or("en")
        .to_string();

    let styles = json_value
        .get("styles")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    // Parse sections
    let sections: Vec<Section> = json_value
        .get("sections")
        .and_then(|v| v.as_array())
        .ok_or_else(|| TdfError::InvalidDocument("Missing 'sections' array".to_string()))?
        .iter()
        .map(|section_json| {
            let id = section_json
                .get("id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let title = section_json.get("title").and_then(|v| v.as_str()).map(|s| s.to_string());
            let content = section_json
                .get("content")
                .and_then(|v| v.as_array())
                .unwrap_or(&vec![])
                .iter()
                .filter_map(|block| {
                    serde_json::from_value(block.clone()).ok()
                })
                .collect();

            Section { id, title, content }
        })
        .collect();

    let content = DocumentContent { sections };

    // Create document
    let mut document = Document::new(title, language, content, styles);
    document.manifest.integrity.algorithm = tdf_core::document::HashAlgorithm::Sha256;

    // Load signing key if provided
    let signing_key = if let Some(key_path) = key {
        Some(utils::load_signing_key(&key_path)?)
    } else {
        None
    };

    // Determine output path
    let output_path = output.unwrap_or_else(|| {
        input
            .with_extension("tdf")
            .file_name()
            .map(|n| PathBuf::from(n))
            .unwrap_or_else(|| PathBuf::from("output.tdf"))
    });

    // Build archive
    let mut builder = ArchiveBuilder::new(document);
    builder.build(
        &output_path,
        signing_key.as_ref(),
        signer_id,
        signer_name,
    )?;

    println!("Created TDF document: {}", output_path.display());
    Ok(())
}

