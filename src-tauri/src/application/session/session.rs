use crate::domain::datastructures::table::Table;

#[derive(Clone)]
pub struct Session {
    pub table: Option<Table>,
    pub xml: Option<String>,
}

impl Session {
    pub fn new() -> Session {
        Session {
            table: None,
            xml: None,
        }
    }
}