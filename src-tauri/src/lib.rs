use crate::{excel::reader::{ExcelReader, ExcelReaderTrait}, parser::parser::{Parser, ParserTrait}};

pub mod xml;
pub mod excel;
pub mod model;
pub mod parser;
pub mod datastructures;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn convert(path: &str) -> Result<Vec<String>, String> {
    let mut reader = ExcelReader::new(path)?;

    reader.read_excel()?;

    Ok(reader.header)
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
