use std::sync::Arc;

use crate::application::ports::{
    inbound::getxmlusecase::GetXmlUseCase, outbound::sessionrepository::SessionRepository,
};

pub struct GetXmlService {
    session_repository: Arc<dyn SessionRepository>,
}

impl GetXmlService {
    pub fn new(session_repository: Arc<dyn SessionRepository>) -> Self {
        GetXmlService { session_repository }
    }
}

impl GetXmlUseCase for GetXmlService {
    fn get_xml(&self, session_id: &str) -> Result<String, String> {
        let session = self.session_repository.get(session_id)?;

        session
            .xml
            .ok_or("No XML generated yet. Please convert first.".to_string())
    }
}
