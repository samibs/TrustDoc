use std::path::PathBuf;
use tdf_core::archive::ArchiveReader;
use tdf_core::error::TdfResult;

pub fn show_info(document: PathBuf) -> TdfResult<()> {
    let (doc, merkle_tree, sig_block) = ArchiveReader::read(&document)?;

    println!("TDF Document Information");
    println!("=======================");
    println!("Title: {}", doc.manifest.document.title);
    println!("ID: {}", doc.manifest.document.id);
    println!("Language: {}", doc.manifest.document.language);
    println!("Created: {}", doc.manifest.document.created);
    println!("Modified: {}", doc.manifest.document.modified);
    println!("Schema Version: {}", doc.manifest.schema_version);

    if let Some(ref classification) = doc.manifest.classification {
        println!("Classification: {:?}", classification);
    }

    println!("\nAuthors:");
    for author in &doc.manifest.authors {
        println!("  - {} ({})", author.name, author.id);
        if let Some(ref role) = author.role {
            println!("    Role: {}", role);
        }
    }

    println!("\nIntegrity:");
    println!("  Algorithm: {:?}", doc.manifest.integrity.algorithm);
    println!("  Root Hash: {}", doc.manifest.integrity.root_hash);

    println!("\nContent:");
    println!("  Sections: {}", doc.content.sections.len());
    for section in &doc.content.sections {
        println!("    - {} ({} blocks)", 
            section.title.as_ref().unwrap_or(&section.id),
            section.content.len()
        );
    }

    println!("\nSignatures: {}", sig_block.signatures.len());
    for (i, sig) in sig_block.signatures.iter().enumerate() {
        println!("  [{}] {} ({})", i + 1, sig.signer.name, sig.signer.id);
        println!("      Algorithm: {:?}", sig.algorithm);
        println!("      Scope: {:?}", sig.scope);
        println!("      Timestamp: {}", sig.timestamp.time);
    }

    Ok(())
}

