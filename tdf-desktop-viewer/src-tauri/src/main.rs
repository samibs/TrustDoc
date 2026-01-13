// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod keys;
mod documents;

use tdf_core::archive::ArchiveReader;
use keys::{KeyInfo, list_keys, generate_key, import_key, export_key, delete_key, get_key_details};
use documents::{VerificationDetails, verify_document_enhanced};
use std::path::PathBuf;

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

#[tauri::command]
async fn verify_document_enhanced_command(file_path: String) -> Result<VerificationDetails, String> {
    verify_document_enhanced(&PathBuf::from(file_path))
        .map_err(|e| format!("Verification failed: {}", e))
}

#[derive(serde::Serialize)]
struct VerificationResult {
    integrity_valid: bool,
    root_hash: String,
    signature_count: usize,
    timestamp_warnings: Vec<String>,
}

// Key management commands
#[tauri::command]
async fn list_keys_command() -> Result<Vec<KeyInfo>, String> {
    list_keys().map_err(|e| format!("Failed to list keys: {}", e))
}

#[tauri::command]
async fn generate_key_command(name: String, algorithm: String, signer_id: Option<String>) -> Result<KeyInfo, String> {
    generate_key(name, algorithm, signer_id).map_err(|e| format!("Failed to generate key: {}", e))
}

#[tauri::command]
async fn import_key_command(path: String, name: String, password: Option<String>) -> Result<KeyInfo, String> {
    import_key(&PathBuf::from(path), name, password).map_err(|e| format!("Failed to import key: {}", e))
}

#[tauri::command]
async fn export_key_command(key_id: String, path: String) -> Result<(), String> {
    export_key(&key_id, &PathBuf::from(path)).map_err(|e| format!("Failed to export key: {}", e))
}

#[tauri::command]
async fn delete_key_command(key_id: String) -> Result<(), String> {
    delete_key(&key_id).map_err(|e| format!("Failed to delete key: {}", e))
}

#[tauri::command]
async fn get_key_details_command(key_id: String) -> Result<KeyInfo, String> {
    get_key_details(&key_id).map_err(|e| format!("Failed to get key details: {}", e))
}

#[tauri::command]
async fn read_file_binary(path: String) -> Result<Vec<u8>, String> {
    std::fs::read(&path).map_err(|e| format!("Failed to read file: {}", e))
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            verify_document,
            verify_document_enhanced_command,
            list_keys_command,
            generate_key_command,
            import_key_command,
            export_key_command,
            delete_key_command,
            get_key_details_command,
            read_file_binary
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

