use std::sync::Arc;

use crate::{application::ports::{inbound::viewexcelusecase::ViewExcelUseCase, outbound::sessionrepository::SessionRepository}, domain::datastructures::table::Table};

pub struct ViewExcelService {
    session_repository: Arc<dyn SessionRepository>,
}

impl ViewExcelService {
    pub fn new (
        session_repository: Arc<dyn SessionRepository>,
    ) -> Self {
        Self {
            session_repository: session_repository,
        }
    }
}

impl ViewExcelUseCase for ViewExcelService {
    fn view_excel(&self, session_id: &str) -> Result<Table, String> {
        let table = self.session_repository.get_table(session_id)?;

        Ok(table)
    }
}