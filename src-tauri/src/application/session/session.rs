use crate::domain::{datastructures::table::Table, entities::tagmapping::TagMappings};

#[derive(Clone)]
pub struct Session {
    pub table: Option<Table>,
    pub xml: Option<String>,
    pub tag_mappings: Option<TagMappings>,
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
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
