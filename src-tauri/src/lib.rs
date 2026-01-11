use std::path::PathBuf;

use crate::error::{Error, Result};

mod error;


struct Document {
    document: lopdf::Document,
    path: PathBuf,
}


async fn process_pdf_doc(document: Document) {
    let pages: Vec<u32> = document.document.get_pages().keys().cloned().collect();
    println!("{:?}: {:?}", document.path, pages);
}


#[tauri::command]
async fn open_pdf(paths: Vec<String>) -> Result<String> {
    for path in paths {
        let document = lopdf::Document::load(&path).await.map_err(|e| Error::Pdf {
            source: e,
            path: path.clone().into(),
        })?;

        process_pdf_doc(Document {
            document,
            path: path.into(),
        })
        .await;
    }

    Ok("PDFs processed successfully".to_string())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![open_pdf])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
