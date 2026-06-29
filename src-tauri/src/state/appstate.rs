use std::sync::{Arc, Mutex};

use crate::application::ports::inbound::{
    createsessionusecase::CreateSessionUseCase,
    mapheadersusecase::MapHeadersUseCase,
    uploadexcelusecase::UploadExcelUseCase,
    viewexcelusecase::ViewExcelUseCase,
    viewheadersusecase::ViewHeadersUseCase,
    convertusecase::ConvertUseCase,
    getxmlusecase::GetXmlUseCase
};


pub struct AppState {
    // application
    pub upload_excel_use_case: Arc<dyn UploadExcelUseCase + Send + Sync>,
    pub create_session_use_case: Arc<dyn CreateSessionUseCase + Send + Sync>,
    pub map_headers_use_case: Arc<dyn MapHeadersUseCase + Send + Sync>,
    pub view_excel_use_case: Arc<dyn ViewExcelUseCase + Send + Sync>,
    pub view_headers_use_case: Arc<dyn ViewHeadersUseCase + Send + Sync>,
    pub convert_use_case: Arc<Mutex<dyn ConvertUseCase + Send + Sync>>,
    pub get_xml_use_case: Arc<dyn GetXmlUseCase + Send + Sync>,
}