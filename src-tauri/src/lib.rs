use std::sync::Arc;

use tauri::State;

use crate::{application::services::{createsessionservice::CreateSessionService, getheadersservice::GetHeadersService, uploadexcelservice::UploadExcelService, viewexcelservice::ViewExcelService}, excel::reader::{ExcelReader, ExcelReaderTrait}, infrastructure::adapters::outputs::{calamineexcelreader::CalamineExcelReader, taurisessionrepository::TauriSessionRepository, uuidv4generator::Uuidv4Generator}, parser::parser::{Parser, ParserTrait}, state::appstate::AppState};

pub mod xml;
pub mod excel;
pub mod model;
pub mod parser;
pub mod domain;
pub mod state;
pub mod application;
pub mod infrastructure;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // infrastructure
    let session_repository = Arc::new(TauriSessionRepository::new());
    let excel_reader = Arc::new(CalamineExcelReader::new());
    let id_generator = Arc::new(Uuidv4Generator::new());

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            // infrastructure
            session_repository: session_repository.clone(),
            excel_reader: excel_reader.clone(),
            id_generator: id_generator.clone(),
            
            // application
            upload_excel_use_case: Arc::new(UploadExcelService::new(
                excel_reader.clone(),
                session_repository.clone()
            )),
            create_session_use_case: Arc::new(CreateSessionService::new(
                session_repository.clone(),
                id_generator.clone()
            )),
            get_headers_use_case: Arc::new(GetHeadersService::new(
                session_repository.clone()
            )),
            view_excel_use_case: Arc::new(ViewExcelService::new(
                session_repository.clone()
            ))
        })
        .invoke_handler(tauri::generate_handler![
            // Excel Controller
            infrastructure::adapters::inputs::excelcontroller::load_excel,
            infrastructure::adapters::inputs::excelcontroller::get_headers,
            infrastructure::adapters::inputs::excelcontroller::view_excel,

            // Session Controller
            infrastructure::adapters::inputs::sessioncontroller::create_session,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
