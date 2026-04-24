use crate::{application::session::session::Session, domain::datastructures::table::Table};

pub trait SessionRepository: Send + Sync {
    fn get(&self, session_id: &str) -> Result<Session, String>;
    
    fn insert(&self, session_id: &str, session: Session) -> Result<(), String>;
    
    fn update(&self, session_id: &str, session: Session) -> Result<(), String>;

    fn set_table(&self, session_id: &str, table: Table) -> Result<(), String>;

    fn get_table(&self, session_id: &str) -> Result<Table, String>;
}