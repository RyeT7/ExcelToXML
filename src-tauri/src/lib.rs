use calamine::{Reader, Rows, Xlsx, open_workbook, open_workbook_auto};

mod xml;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn convert(path: &str) -> String {
    "test".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            convert
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
