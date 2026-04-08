use std::collections::HashMap;

use crate::{datastructures::table::{Table, TableTrait}, model::mapping::HeaderMapping};

pub struct Parser {}

pub trait ParserTrait {
    
}

impl Parser {
    fn map_header(mut table: &Table, mapping: HeaderMapping) -> Result<(), String> {
        Ok(())
    }
}

impl ParserTrait for Parser {
    
}