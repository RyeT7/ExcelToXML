use crate::{
    application::{
        ports::outbound::sessionrepository::SessionRepository, session::session::Session,
    },
    domain::{datastructures::table::Table, entities::tagmapping::TagMappings},
};
use std::{collections::HashMap, sync::RwLock};

pub struct TauriSessionRepository {
    sessions: RwLock<HashMap<String, Session>>,
}

impl Default for TauriSessionRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl TauriSessionRepository {
    pub fn new() -> Self {
        TauriSessionRepository {
            sessions: RwLock::new(HashMap::new()),
        }
    }
}

impl SessionRepository for TauriSessionRepository {
    fn get(&self, session_id: &str) -> Result<Session, String> {
        let sessions = self.sessions.read().unwrap();

        sessions
            .get(session_id)
            .cloned()
            .ok_or("Session does not exist".to_string())
    }

    fn insert(&self, session_id: &str, session: Session) -> Result<(), String> {
        let mut sessions = self.sessions.write().unwrap();

        sessions.insert(session_id.to_string(), session);

        Ok(())
    }

    fn update(&self, session_id: &str, session: Session) -> Result<(), String> {
        let mut sessions = self.sessions.write().map_err(|e| e.to_string())?;

        if !sessions.contains_key(session_id) {
            return Err("Session does not exist".to_string());
        }

        sessions.insert(session_id.to_string(), session);

        Ok(())
    }

    fn set_table(&self, session_id: &str, table: Table) -> Result<(), String> {
        let mut sessions = self.sessions.write().unwrap();

        let session = sessions
            .get_mut(session_id)
            .ok_or("Session does not exist".to_string())?;

        session.table = Some(table);

        Ok(())
    }

    fn get_table(&self, session_id: &str) -> Result<Table, String> {
        let sessions = self.sessions.read().unwrap();

        let session = sessions.get(session_id).ok_or("Session does not exist")?;

        session
            .table
            .clone()
            .ok_or("Table does not exist in session yet".to_string())
    }

    fn get_tag_mappings(&self, session_id: &str) -> Result<TagMappings, String> {
        let sessions = self.sessions.read().unwrap();

        let session = sessions.get(session_id).ok_or("Session does not exist")?;

        session
            .tag_mappings
            .clone()
            .ok_or("Tag Mappings do not exist in session yet".to_string())
    }
}
