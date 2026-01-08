#[tauri::command]
fn process_pdf(paths: Vec<String>) -> Result<String, String> {
    for path in paths {
        // Read file from disk
        match std::fs::read(&path) {
            Ok(file_data) => {
                println!("Processing PDF: {} ({} bytes)", path, file_data.len());
            }
            Err(e) => {
                return Err(format!("Failed to read file {}: {}", path, e));
            }
        }
    }
    Ok("PDFs processed successfully".to_string())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .invoke_handler(tauri::generate_handler![process_pdf])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
