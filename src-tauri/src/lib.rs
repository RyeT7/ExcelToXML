use std::sync::{Arc, Mutex};

use tauri::State;

use crate::{application::services::{convertservice::ConvertService, createsessionservice::CreateSessionService, getxmlservice::GetXmlService, mapheadersservice::MapHeadersService, uploadexcelservice::UploadExcelService, viewexcelservice::ViewExcelService, viewheadersservice::ViewHeadersService}, excel::reader::{ExcelReader, ExcelReaderTrait}, infrastructure::adapters::outputs::{calamineexcelreader::CalamineExcelReader, customxmlwriter::CustomXMLWriter, taurisessionrepository::TauriSessionRepository, uuidv4generator::Uuidv4Generator}, parser::parser::{Parser, ParserTrait}, state::appstate::AppState, xml::writer::XMLWriter};

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
    let xml_writer = Arc::new(Mutex::new(CustomXMLWriter::new()));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            // application
            upload_excel_use_case: Arc::new(UploadExcelService::new(
                excel_reader.clone(),
                session_repository.clone()
            )),
            create_session_use_case: Arc::new(CreateSessionService::new(
                session_repository.clone(),
                id_generator.clone()
            )),
            map_headers_use_case: Arc::new(MapHeadersService::new(
                session_repository.clone()
            )),
            view_excel_use_case: Arc::new(ViewExcelService::new(
                session_repository.clone()
            )),
            view_headers_use_case: Arc::new(ViewHeadersService::new()),
            convert_use_case: Arc::new(
                Mutex::new(ConvertService::new(
                    session_repository.clone(),
                    xml_writer.clone()
                ))
            ),
            get_xml_use_case: Arc::new(GetXmlService::new(
                session_repository.clone()
            )),
        })
        .invoke_handler(tauri::generate_handler![
            // Excel Controller
            infrastructure::adapters::inputs::excelcontroller::load_excel,
            infrastructure::adapters::inputs::excelcontroller::view_excel,

            // Session Controller
            infrastructure::adapters::inputs::sessioncontroller::create_session,

            // Parser Controller
            infrastructure::adapters::inputs::parsercontroller::view_headers,

            // Map Headers Controller
            infrastructure::adapters::inputs::mapheaderscontroller::map_headers,

            // Convert Controller
            infrastructure::adapters::inputs::convertcontroller::convert,

            // Get XML Controller
            infrastructure::adapters::inputs::getxmlcontroller::get_xml,
            infrastructure::adapters::inputs::getxmlcontroller::save_xml,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
