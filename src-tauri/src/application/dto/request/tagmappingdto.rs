use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TagMappingDTO {
    pub mapped_column: Option<String>,
    pub default_value: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TagMappingsDTO {
    pub tag_mappings: HashMap<String, TagMappingDTO>,
    pub invoice_number_column: String,
    pub good_service_identifier_column: String,
}