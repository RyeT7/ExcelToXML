use std::sync::Arc;

use crate::application::{
    ports::{
        inbound::createsessionusecase::CreateSessionUseCase,
        outbound::{
            idgenerator::IdGenerator,
            sessionrepository::SessionRepository
        }
    },
    session::session::Session
};

pub struct CreateSessionService {
    session_repository: Arc<dyn SessionRepository>,
    id_generator: Arc<dyn IdGenerator>,
}

impl CreateSessionService {
    pub fn new (
        session_repository: Arc<dyn SessionRepository>,
        id_generator: Arc<dyn IdGenerator>,
    ) -> Self {
        Self {
            session_repository: session_repository,
            id_generator: id_generator
        }
    }
}

impl CreateSessionUseCase for CreateSessionService {
    fn create_session(&self) -> Result<String, String> {
        let session_id = self.id_generator.create()?;

        self.session_repository.insert(&session_id, Session::new())?;

        Ok(session_id)
    }
}