use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tdf_core::archive::ArchiveBuilder;
use tdf_core::content::{ContentBlock, CellType, DocumentContent, Section, TableColumn, TableRow};
use tdf_core::document::Document;
use crate::error::ConvertError;
use std::collections::HashMap;

pub fn convert_csv_to_tdf(
    input: &Path,
    output: &Path,
    signer_id: Option<String>,
    signer_name: Option<String>,
    _signing_key: Option<&[u8]>,
) -> Result<(), ConvertError> {
    let file = File::open(input)?;
    let mut rdr = csv::Reader::from_reader(BufReader::new(file));
    
    // Read headers
    let headers = rdr.headers()?.iter().map(|h| h.to_string()).collect::<Vec<_>>();
    
    // Build table columns
    let columns: Vec<TableColumn> = headers
        .iter()
        .enumerate()
        .map(|(idx, header)| {
            // Try to detect column type
            let cell_type = detect_column_type(header);
            TableColumn {
                id: format!("col_{}", idx),
                header: header.clone(),
                cell_type,
                currency: None,
            }
        })
        .collect();
    
    // Read rows
    let mut rows = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let mut cells = HashMap::new();
        
        for (idx, field) in record.iter().enumerate() {
            if idx < columns.len() {
                let col_id = format!("col_{}", idx);
                let cell_value = parse_cell_value(field, &columns[idx].cell_type);
                cells.insert(col_id, cell_value);
            }
        }
        
        rows.push(TableRow { cells });
    }
    
    // Create table block
    let table = ContentBlock::Table {
        id: "data".to_string(),
        caption: Some(format!("Data from {}", input.file_name().unwrap_or_default().to_string_lossy())),
        columns,
        rows,
        footer: None,
    };
    
    // Create document
    let title = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("CSV Import")
        .to_string();
    
    let content = DocumentContent {
        sections: vec![Section {
            id: "data".to_string(),
            title: Some("Data".to_string()),
            content: vec![table],
        }],
    };
    
    let styles = get_default_styles();
    let mut document = Document::new(title, "en".to_string(), content, styles);
    document.manifest.integrity.algorithm = tdf_core::document::HashAlgorithm::Sha256;
    
    // Build archive
    let mut builder = ArchiveBuilder::new(document);
    builder.build(
        output,
        None, // Signing key would need to be converted from bytes
        signer_id,
        signer_name,
    )?;
    
    Ok(())
}

fn detect_column_type(header: &str) -> CellType {
    let header_lower = header.to_lowercase();
    if header_lower.contains("amount") || header_lower.contains("price") || header_lower.contains("cost") {
        CellType::Currency
    } else if header_lower.contains("date") {
        CellType::Date
    } else if header_lower.contains("percent") || header_lower.contains("%") {
        CellType::Percentage
    } else if header_lower.contains("number") || header_lower.contains("count") || header_lower.contains("qty") {
        CellType::Number
    } else {
        CellType::Text
    }
}

fn parse_cell_value(value: &str, cell_type: &CellType) -> tdf_core::content::CellValue {
    match cell_type {
        CellType::Number => {
            if let Ok(num) = value.parse::<f64>() {
                tdf_core::content::CellValue::Number {
                    raw: num,
                    display: value.to_string(),
                }
            } else {
                tdf_core::content::CellValue::Text(value.to_string())
            }
        }
        CellType::Currency => {
            // Try to parse currency (remove $, €, etc.)
            let cleaned = value.replace("$", "").replace("€", "").replace(",", "").trim().to_string();
            if let Ok(num) = cleaned.parse::<f64>() {
                tdf_core::content::CellValue::Currency {
                    raw: num,
                    display: value.to_string(),
                    currency: detect_currency(value),
                }
            } else {
                tdf_core::content::CellValue::Text(value.to_string())
            }
        }
        CellType::Percentage => {
            let cleaned = value.replace("%", "").trim().to_string();
            if let Ok(num) = cleaned.parse::<f64>() {
                tdf_core::content::CellValue::Percentage {
                    raw: num / 100.0,
                    display: value.to_string(),
                }
            } else {
                tdf_core::content::CellValue::Text(value.to_string())
            }
        }
        CellType::Date => {
            tdf_core::content::CellValue::Date {
                raw: value.to_string(),
                display: value.to_string(),
            }
        }
        _ => tdf_core::content::CellValue::Text(value.to_string()),
    }
}

fn detect_currency(value: &str) -> String {
    if value.contains("€") {
        "EUR".to_string()
    } else if value.contains("$") {
        "USD".to_string()
    } else if value.contains("£") {
        "GBP".to_string()
    } else {
        "USD".to_string() // Default
    }
}

fn get_default_styles() -> String {
    include_str!("styles/default.css").to_string()
}

