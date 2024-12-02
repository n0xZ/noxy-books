use std::env;

use tauri::Manager;
use tauri_plugin_store::StoreExt;

mod book;

#[tauri::command]
fn get_default_folder(app_handle: tauri::AppHandle) -> Result<String, String> {
    let store = app_handle
        .store("settings.json")
        .map_err(|e| e.to_string())?;

    if let Some(res) = store.get("default_folder") {
        return Ok(res.as_str().unwrap_or_default().to_string());
    }

    Err("Current settings store does not exist".to_string())
}
#[tauri::command]
fn define_base_book_folder(
    dir_path: &str,
    app_handle: tauri::AppHandle,
) -> Result<(), tauri::Error> {
    let app_result = app_handle.path().local_data_dir().unwrap();
    let app_dir = app_result.join("settings.json");
    let default_settings = serde_json::json!({ "default_folder": dir_path });
    std::fs::write(app_dir, default_settings.to_string()).map_err(|e| {
        println!("An error ocurred while writing file: {:?}", e);
        e
    })?;

    Ok(())
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            book::get_books_from_existing_dir,
            define_base_book_folder,
            book::retrieve_book_by_title,
            get_default_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
