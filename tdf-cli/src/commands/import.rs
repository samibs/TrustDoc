use std::fs;
use std::path::PathBuf;
use tdf_convert::convert_file;
use tdf_convert::ConvertError;
use tdf_core::error::TdfResult;
use crate::utils;

pub fn import_from_file(
    input: PathBuf,
    output: Option<PathBuf>,
    signer_id: Option<String>,
    signer_name: Option<String>,
    key: Option<PathBuf>,
) -> TdfResult<()> {
    // Determine output path
    let output_path = output.unwrap_or_else(|| {
        input
            .with_extension("tdf")
            .file_name()
            .map(|n| PathBuf::from(n))
            .unwrap_or_else(|| PathBuf::from("output.tdf"))
    });

    // Load signing key if provided
    let signing_key_bytes = if let Some(key_path) = &key {
        let key = utils::load_signing_key(key_path)?;
        Some(key.to_bytes().to_vec())
    } else {
        None
    };

    // Convert using tdf-convert library
    convert_file(
        &input,
        &output_path,
        signer_id,
        signer_name,
        signing_key_bytes.as_deref(),
    )
    .map_err(|e| tdf_core::error::TdfError::InvalidDocument(format!("Conversion error: {}", e)))?;

    println!("Converted to TDF: {}", output_path.display());
    Ok(())
}

pub fn import_batch_from_folder(
    folder: PathBuf,
    output_folder: Option<PathBuf>,
    signer_id: Option<String>,
    signer_name: Option<String>,
    key: Option<PathBuf>,
) -> TdfResult<()> {
    let output_dir = output_folder.unwrap_or_else(|| folder.clone());
    
    // Create output directory if it doesn't exist
    fs::create_dir_all(&output_dir)?;

    // Get supported formats
    let supported_extensions = tdf_convert::supported_formats();
    
    // Find all supported files
    let files: Vec<PathBuf> = fs::read_dir(&folder)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() {
                let ext = path.extension()?.to_str()?.to_lowercase();
                if supported_extensions.contains(&ext.as_str()) {
                    Some(path)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    if files.is_empty() {
        return Err(tdf_core::error::TdfError::InvalidDocument(
            format!("No supported files found in: {}. Supported formats: {}", 
                folder.display(),
                supported_extensions.join(", "))
        ));
    }

    println!("Found {} file(s) to convert", files.len());
    println!("Supported formats: {}", supported_extensions.join(", "));

    let mut success_count = 0;
    let mut error_count = 0;

    for file_path in files {
        let output_path = output_dir.join(
            file_path
                .file_stem()
                .and_then(|s| s.to_str())
                .map(|s| format!("{}.tdf", s))
                .unwrap_or_else(|| "output.tdf".to_string())
        );

        match import_from_file(
            file_path.clone(),
            Some(output_path.clone()),
            signer_id.clone(),
            signer_name.clone(),
            key.clone(),
        ) {
            Ok(_) => {
                println!("✓ Converted: {} -> {}", 
                    file_path.file_name().unwrap_or_default().to_string_lossy(),
                    output_path.file_name().unwrap_or_default().to_string_lossy()
                );
                success_count += 1;
            }
            Err(e) => {
                eprintln!("✗ Failed to convert {}: {}", 
                    file_path.file_name().unwrap_or_default().to_string_lossy(),
                    e
                );
                error_count += 1;
            }
        }
    }

    println!("\nConversion complete:");
    println!("  ✓ Success: {}", success_count);
    println!("  ✗ Errors: {}", error_count);

    Ok(())
}
