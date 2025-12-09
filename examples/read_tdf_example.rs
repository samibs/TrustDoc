// Example: How to read a TDF file in Rust

use tdf_core::archive::ArchiveReader;
use tdf_core::error::TdfResult;
use std::path::Path;

fn main() -> TdfResult<()> {
    let path = Path::new("demo-invoice.tdf");
    
    // Read the TDF document
    let (document, merkle_tree, signature_block) = ArchiveReader::read(path)?;
    
    // Print document metadata
    println!("=== Document Information ===");
    println!("Title: {}", document.manifest.document.title);
    println!("ID: {}", document.manifest.document.id);
    println!("Created: {}", document.manifest.document.created);
    println!("Root Hash: {}", document.manifest.integrity.root_hash);
    
    // Print content structure
    println!("\n=== Content Structure ===");
    for section in &document.content.sections {
        println!("\nSection: {}", section.title.as_deref().unwrap_or("Untitled"));
        
        for block in &section.content {
            match block {
                tdf_core::content::ContentBlock::Heading { level, text, .. } => {
                    println!("  H{}: {}", level, text);
                }
                tdf_core::content::ContentBlock::Paragraph { text, .. } => {
                    let preview = if text.len() > 50 {
                        format!("{}...", &text[..47])
                    } else {
                        text.clone()
                    };
                    println!("  Paragraph: {}", preview);
                }
                tdf_core::content::ContentBlock::Table { id, columns, rows, .. } => {
                    println!("  Table: {} ({} columns, {} rows)", id, columns.len(), rows.len());
                    
                    // Print table header
                    print!("    ");
                    for col in columns.iter().take(4) {
                        print!("{:15} ", col.header);
                    }
                    println!();
                    
                    // Print first few rows
                    for row in rows.iter().take(3) {
                        print!("    ");
                        for col in columns.iter().take(4) {
                            if let Some(cell) = row.cells.get(&col.id) {
                                let value = match cell {
                                    tdf_core::content::CellValue::Text(s) => s.clone(),
                                    tdf_core::content::CellValue::Number { display, .. } => display.clone(),
                                    tdf_core::content::CellValue::Currency { display, .. } => display.clone(),
                                    tdf_core::content::CellValue::Percentage { display, .. } => display.clone(),
                                    tdf_core::content::CellValue::Date { display, .. } => display.clone(),
                                };
                                let truncated = if value.len() > 13 {
                                    format!("{}...", &value[..10])
                                } else {
                                    value
                                };
                                print!("{:15} ", truncated);
                            }
                        }
                        println!();
                    }
                    if rows.len() > 3 {
                        println!("    ... ({} more rows)", rows.len() - 3);
                    }
                }
                tdf_core::content::ContentBlock::List { ordered, items, .. } => {
                    let list_type = if *ordered { "Ordered" } else { "Unordered" };
                    println!("  {} List: {} items", list_type, items.len());
                    for item in items.iter().take(3) {
                        println!("    - {}", item);
                    }
                }
                _ => {
                    println!("  [Other content block]");
                }
            }
        }
    }
    
    // Print signature information
    if let Some(sig_block) = &signature_block {
        println!("\n=== Signatures ===");
        for sig in &sig_block.signatures {
            println!("  Signed by: {} ({})", sig.signer.name, sig.signer.id);
            println!("    Algorithm: {:?}", sig.algorithm);
            println!("    Timestamp: {}", sig.timestamp.time);
            println!("    Scope: {:?}", sig.scope);
        }
    } else {
        println!("\n=== No Signatures ===");
    }
    
    // Verify integrity
    println!("\n=== Verification ===");
    let is_valid = ArchiveReader::verify(path)?;
    println!("Integrity: {}", if is_valid { "✓ VALID" } else { "✗ INVALID" });
    
    Ok(())
}

