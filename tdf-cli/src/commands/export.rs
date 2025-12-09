use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use tdf_core::archive::ArchiveReader;
use tdf_core::error::TdfResult;

pub fn export_to_pdf(document: PathBuf, output: Option<PathBuf>) -> TdfResult<()> {
    let (doc, _, _) = ArchiveReader::read(&document)?;

    // Determine output path
    let output_path = output.unwrap_or_else(|| {
        document
            .with_extension("pdf")
            .file_name()
            .map(|n| PathBuf::from(n))
            .unwrap_or_else(|| PathBuf::from("output.pdf"))
    });

    // Create PDF document
    let (mut doc_pdf, page1, layer1) = PdfDocument::new("TDF Document", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc_pdf.get_page(page1).get_layer(layer1);

    // Set font
    let font = doc_pdf.add_builtin_font(BuiltinFont::HelveticaBold)
        .map_err(|e| tdf_core::error::TdfError::InvalidDocument(format!("Font error: {}", e)))?;
    let font_regular = doc_pdf.add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|e| tdf_core::error::TdfError::InvalidDocument(format!("Font error: {}", e)))?;

    let mut y_position = 280.0; // Start near top (A4 height is 297mm)

    // Title
    current_layer.use_text(&doc.manifest.document.title, 24.0, Mm(20.0), Mm(y_position), &font);
    y_position -= 15.0;

    // Metadata
    let metadata_text = format!(
        "ID: {}\nCreated: {}\nModified: {}",
        doc.manifest.document.id,
        doc.manifest.document.created.format("%Y-%m-%d %H:%M:%S"),
        doc.manifest.document.modified.format("%Y-%m-%d %H:%M:%S")
    );
    current_layer.use_text(&metadata_text, 10.0, Mm(20.0), Mm(y_position), &font_regular);
    y_position -= 20.0;

    // Render sections
    for section in &doc.content.sections {
        if y_position < 30.0 {
            // Would need to add new page here in full implementation
            break;
        }

        // Section title
        if let Some(ref title) = section.title {
            current_layer.use_text(title, 16.0, Mm(20.0), Mm(y_position), &font);
            y_position -= 10.0;
        }

        // Render content blocks
        for block in &section.content {
            if y_position < 30.0 {
                break;
            }

            match block {
                tdf_core::content::ContentBlock::Heading { level, text, .. } => {
                    let size = match *level {
                        1 => 18.0,
                        2 => 16.0,
                        3 => 14.0,
                        _ => 12.0,
                    };
                    current_layer.use_text(text, size, Mm(20.0), Mm(y_position), &font);
                    y_position -= (size / 2.0) + 2.0;
                }
                tdf_core::content::ContentBlock::Paragraph { text, .. } => {
                    // Simple text rendering (would need word wrapping in full implementation)
                    let lines: Vec<&str> = text.lines().collect();
                    for line in lines.iter().take(3) {
                        // Limit lines per paragraph for simplicity
                        current_layer.use_text(line.to_string(), 10.0, Mm(20.0), Mm(y_position), &font_regular);
                        y_position -= 5.0;
                        if y_position < 30.0 {
                            break;
                        }
                    }
                    y_position -= 3.0;
                }
                tdf_core::content::ContentBlock::Table { id, columns, rows, caption, .. } => {
                    // Enhanced table rendering with borders
                    let mut table_y = y_position;
                    
                    // Caption
                    if let Some(ref cap) = caption {
                        current_layer.use_text(cap, 9.0, Mm(20.0), Mm(table_y + 2.0), &font);
                        table_y -= 5.0;
                    }
                    
                    // Calculate column widths (simplified)
                    let col_count = columns.len().min(4);
                    let col_width: f32 = 170.0 / (col_count as f32);
                    
                    // Header
                    let mut x_pos: f32 = 20.0;
                    for col in columns.iter().take(4) {
                        current_layer.use_text(&col.header, 9.0, Mm(x_pos), Mm(table_y), &font);
                        x_pos += col_width;
                    }
                    table_y -= 8.0;
                    
                    // Rows
                    for row in rows.iter().take(10) {
                        if table_y < 30.0 {
                            break;
                        }
                        
                        x_pos = 20.0;
                        for col in columns.iter().take(4) {
                            if let Some(cell) = row.cells.get(&col.id) {
                                let cell_text = match cell {
                                    tdf_core::content::CellValue::Text(s) => s.clone(),
                                    tdf_core::content::CellValue::Number { display, .. } => display.clone(),
                                    tdf_core::content::CellValue::Currency { display, .. } => display.clone(),
                                    tdf_core::content::CellValue::Percentage { display, .. } => display.clone(),
                                    tdf_core::content::CellValue::Date { display, .. } => display.clone(),
                                };
                                // Truncate long text
                                let display_text = if cell_text.len() > 20 {
                                    format!("{}...", &cell_text[..17])
                                } else {
                                    cell_text
                                };
                                current_layer.use_text(&display_text, 8.0, Mm(x_pos), Mm(table_y), &font_regular);
                            }
                            x_pos += col_width;
                        }
                        table_y -= 6.0;
                    }
                    y_position = table_y - 5.0;
                }
                tdf_core::content::ContentBlock::Diagram { id, diagram_type, nodes, edges, title, .. } => {
                    // Render diagram as text representation
                    if y_position < 50.0 {
                        break;
                    }
                    
                    if let Some(ref t) = title {
                        current_layer.use_text(t, 12.0, Mm(20.0), Mm(y_position), &font);
                        y_position -= 8.0;
                    }
                    
                    // Render nodes
                    let mut node_y = y_position;
                    for node in nodes.iter().take(5) {
                        if node_y < 30.0 {
                            break;
                        }
                        let node_text = format!("• {}", node.label.replace('\n', " - "));
                        current_layer.use_text(&node_text, 9.0, Mm(25.0), Mm(node_y), &font_regular);
                        node_y -= 6.0;
                    }
                    
                    // Render edges
                    if !edges.is_empty() {
                        current_layer.use_text("Connections:", 9.0, Mm(25.0), Mm(node_y), &font_regular);
                        node_y -= 6.0;
                        for edge in edges.iter().take(3) {
                            if node_y < 30.0 {
                                break;
                            }
                            let edge_text = format!("  {} → {}", edge.from, edge.to);
                            current_layer.use_text(&edge_text, 8.0, Mm(30.0), Mm(node_y), &font_regular);
                            node_y -= 5.0;
                        }
                    }
                    
                    y_position = node_y - 5.0;
                }
                tdf_core::content::ContentBlock::List { ordered, items, .. } => {
                    let mut list_y = y_position;
                    for (i, item) in items.iter().take(8).enumerate() {
                        if list_y < 30.0 {
                            break;
                        }
                        let prefix = if *ordered {
                            format!("{}. ", i + 1)
                        } else {
                            "• ".to_string()
                        };
                        current_layer.use_text(&format!("{}{}", prefix, item), 9.0, Mm(25.0), Mm(list_y), &font_regular);
                        list_y -= 5.0;
                    }
                    y_position = list_y - 3.0;
                }
                _ => {
                    // Other block types
                    y_position -= 5.0;
                }
            }
        }

        y_position -= 10.0; // Space between sections
    }

    // Save PDF
    doc_pdf.save(&mut BufWriter::new(File::create(&output_path)?))
        .map_err(|e| tdf_core::error::TdfError::InvalidDocument(format!("PDF save error: {}", e)))?;

    println!("Exported PDF to: {}", output_path.display());
    println!("Note: This is a basic export. Full formatting requires additional implementation.");

    Ok(())
}

