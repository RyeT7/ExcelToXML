use std::sync::Arc;

use crate::application::ports::{
    inbound::getheadersusecase::GetHeadersUseCase,
    outbound::sessionrepository::SessionRepository
};

pub struct GetHeadersService {
    session_repository: Arc<dyn SessionRepository>,
}

impl GetHeadersService {
    pub fn new (
        session_repository: Arc<dyn SessionRepository>,
    ) -> Self {
        Self {
            session_repository: session_repository,
        }
    }
}

impl GetHeadersUseCase for GetHeadersService {
    fn get_headers(&self, session_id: &str) -> Result<Vec<String>, String> {
        let table = self.session_repository.get_table(session_id)?;

        Ok(table.headers)
    }
}