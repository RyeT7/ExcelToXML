use crate::domain::datastructures::table::Table;
use crate::domain::entities::tagmapping::TagMapping;

#[derive(Clone)]
pub struct Session {
    pub table: Option<Table>,
    pub xml: Option<String>,
    pub tag_mappings: Option<Vec<TagMapping>>,
}

impl Session {
    pub fn new() -> Session {
        Session {
            table: None,
            xml: None,
            tag_mappings: None,
        }
    }
}