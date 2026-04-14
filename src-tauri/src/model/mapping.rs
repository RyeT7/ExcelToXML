use std::collections::HashMap;

pub struct HeaderMapping {
    pub mapping: HashMap<String, String>,
    pub invoice_number_column: String,
}

pub trait HeaderMappingTrait {
    fn get_mapped_header(&mut self, old_header_name: &str) -> Option<&String>;
}

impl HeaderMappingTrait for HeaderMapping {
    fn get_mapped_header(&mut self, old_header_name: &str) -> Option<&String> {
        self.mapping.get(old_header_name)
    }
}