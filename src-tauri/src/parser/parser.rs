use std::collections::HashMap;

use crate::{datastructures::table::{Table, TableTrait}, model::mapping::HeaderMapping};

pub struct Parser {
    pub table: Table,
}

pub trait ParserTrait {
    
}

impl Parser {
    fn map_header(&mut self, mapping: HeaderMapping) -> Result<(), String> {
        self.table.rename_key(mapping.mapping)
    }
}

impl ParserTrait for Parser {
    
}