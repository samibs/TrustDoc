// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tdf_core::archive::ArchiveReader;

#[tauri::command]
async fn verify_document(file_path: String) -> Result<VerificationResult, String> {
    let report = ArchiveReader::verify(&std::path::Path::new(&file_path))
        .map_err(|e| format!("Verification failed: {}", e))?;
    
    Ok(VerificationResult {
        integrity_valid: report.integrity_valid,
        root_hash: report.root_hash,
        signature_count: report.signature_count,
        timestamp_warnings: report.timestamp_warnings,
    })
}

#[derive(serde::Serialize)]
struct VerificationResult {
    integrity_valid: bool,
    root_hash: String,
    signature_count: usize,
    timestamp_warnings: Vec<String>,
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![verify_document])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

