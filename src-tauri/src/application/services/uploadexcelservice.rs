use std::sync::Arc;

use crate::application::ports::{
    inbound::uploadexcelusecase::UploadExcelUseCase,
    outbound::{
        excelreader::ExcelReader,
        sessionrepository::SessionRepository
    }
};

pub struct UploadExcelService {
    reader: Arc<dyn ExcelReader>,
    session_repository: Arc<dyn SessionRepository>,
}

impl UploadExcelService {
    pub fn new(
        reader: Arc<dyn ExcelReader>,
        session_repository: Arc<dyn SessionRepository>,
    ) -> Self {
        Self {
            reader: reader,
            session_repository: session_repository,
        }
    }
}

impl UploadExcelUseCase for UploadExcelService {
    fn load_excel(
        &self,
        session_id: &str,
        path: &str
    ) -> Result<(), String> {
        let table = self.reader.load_excel(path)?;

        let mut session = self.session_repository.get(session_id)?;

        session.table = Some(table);

        self.session_repository.update(session_id, session)?;

        Ok(())
    }
}