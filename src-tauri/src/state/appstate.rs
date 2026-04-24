use std::sync::Arc;

use crate::application::ports::{
    inbound::{
        createsessionusecase::CreateSessionUseCase,
        getheadersusecase::GetHeadersUseCase,
        uploadexcelusecase::UploadExcelUseCase, viewexcelusecase::ViewExcelUseCase
    },
    outbound::{
        excelreader::ExcelReader,
        idgenerator::IdGenerator,
        sessionrepository::SessionRepository
    }
};


pub struct AppState {
    // infrastructure
    pub session_repository: Arc<dyn SessionRepository + Send + Sync>,
    pub excel_reader: Arc<dyn ExcelReader + Send + Sync>,
    pub id_generator: Arc<dyn IdGenerator + Send + Sync>,

    // application
    pub upload_excel_use_case: Arc<dyn UploadExcelUseCase + Send + Sync>,
    pub create_session_use_case: Arc<dyn CreateSessionUseCase + Send + Sync>,
    pub get_headers_use_case: Arc<dyn GetHeadersUseCase + Send + Sync>,
    pub view_excel_use_case: Arc<dyn ViewExcelUseCase + Send + Sync>,
}