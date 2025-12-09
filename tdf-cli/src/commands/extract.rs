use std::fs;
use std::path::PathBuf;
use tdf_core::archive::ArchiveReader;
use tdf_core::error::TdfResult;

pub fn extract_data(document: PathBuf, output: Option<PathBuf>) -> TdfResult<()> {
    let (doc, _, _) = ArchiveReader::read(&document)?;

    // Extract structured data
    let mut extracted = serde_json::Map::new();

    // Extract tables
    let mut tables = serde_json::Map::new();
    for section in &doc.content.sections {
        for block in &section.content {
            if let tdf_core::content::ContentBlock::Table { id, columns, rows, .. } = block {
                let mut table_data = serde_json::Map::new();
                table_data.insert("columns".to_string(), serde_json::to_value(columns)?);
                table_data.insert("rows".to_string(), serde_json::to_value(rows)?);
                tables.insert(id.clone(), serde_json::Value::Object(table_data));
            }
        }
    }
    if !tables.is_empty() {
        extracted.insert("tables".to_string(), serde_json::Value::Object(tables));
    }

    // Extract document metadata
    let mut metadata = serde_json::Map::new();
    metadata.insert("title".to_string(), serde_json::Value::String(doc.manifest.document.title));
    metadata.insert("id".to_string(), serde_json::Value::String(doc.manifest.document.id));
    metadata.insert("created".to_string(), serde_json::Value::String(doc.manifest.document.created.to_rfc3339()));
    metadata.insert("modified".to_string(), serde_json::Value::String(doc.manifest.document.modified.to_rfc3339()));
    extracted.insert("metadata".to_string(), serde_json::Value::Object(metadata));

    // Use data.json from document if available, otherwise use extracted
    let output_data = if let Some(data) = doc.data {
        data
    } else {
        serde_json::Value::Object(extracted)
    };

    // Write output
    let output_path = output.unwrap_or_else(|| {
        document
            .with_extension("json")
            .file_name()
            .map(|n| PathBuf::from(n))
            .unwrap_or_else(|| PathBuf::from("extracted.json"))
    });

    let json_str = serde_json::to_string_pretty(&output_data)?;
    fs::write(&output_path, json_str)?;

    println!("Extracted data to: {}", output_path.display());
    Ok(())
}

