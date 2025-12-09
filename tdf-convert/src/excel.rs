use std::path::Path;
use tdf_core::archive::ArchiveBuilder;
use tdf_core::content::{ContentBlock, CellType, DocumentContent, Section, TableColumn, TableRow};
use tdf_core::document::Document;
use crate::error::ConvertError;
use calamine::{open_workbook, Reader, Xlsx};
use std::collections::HashMap;

pub fn convert_excel_to_tdf(
    input: &Path,
    output: &Path,
    signer_id: Option<String>,
    signer_name: Option<String>,
    _signing_key: Option<&[u8]>,
) -> Result<(), ConvertError> {
    let mut workbook: Xlsx<_> = open_workbook(input)
        .map_err(|e| ConvertError::Excel(format!("Failed to open Excel file: {}", e)))?;
    
    let mut sections = Vec::new();
    
    // Process each sheet
    for sheet_name in workbook.sheet_names().to_vec() {
        if let Ok(range) = workbook.worksheet_range(&sheet_name) {
            let mut rows: Vec<Vec<String>> = Vec::new();
            
            for row in range.rows() {
                let row_data: Vec<String> = row
                    .iter()
                    .map(|cell| format!("{}", cell))
                    .collect();
                rows.push(row_data);
            }
            
            if rows.is_empty() {
                continue;
            }
            
            // First row as headers
            let headers = rows[0].clone();
            let data_rows = rows[1..].to_vec();
            
            // Build columns
            let columns: Vec<TableColumn> = headers
                .iter()
                .enumerate()
                .map(|(idx, header)| {
                    let cell_type = detect_column_type(header);
                    TableColumn {
                        id: format!("col_{}", idx),
                        header: header.clone(),
                        cell_type,
                        currency: None,
                    }
                })
                .collect();
            
            // Build rows
            let table_rows: Vec<TableRow> = data_rows
                .iter()
                .map(|row_data| {
                    let mut cells = HashMap::new();
                    for (idx, cell_value) in row_data.iter().enumerate() {
                        if idx < columns.len() {
                            let col_id = format!("col_{}", idx);
                            let cell_value = parse_cell_value(cell_value, &columns[idx].cell_type);
                            cells.insert(col_id, cell_value);
                        }
                    }
                    TableRow { cells }
                })
                .collect();
            
            // Create table
            let table = ContentBlock::Table {
                id: format!("sheet_{}", sections.len()),
                caption: Some(format!("Sheet: {}", sheet_name)),
                columns,
                rows: table_rows,
                footer: None,
            };
            
            sections.push(Section {
                id: format!("sheet_{}", sections.len()),
                title: Some(sheet_name.clone()),
                content: vec![table],
            });
        }
    }
    
    if sections.is_empty() {
        return Err(ConvertError::Conversion("No data found in Excel file".to_string()));
    }
    
    // Create document
    let title = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Excel Import")
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
        "USD".to_string()
    }
}

fn get_default_styles() -> String {
    include_str!("styles/default.css").to_string()
}

