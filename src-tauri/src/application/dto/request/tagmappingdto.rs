use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TagMappingDTO {
    pub literal: String,
    pub hierarchical: String,
    pub mapped_column: Option<String>,
    pub default_value: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct TagMappingsDTO {
    pub tag_mappings: Vec<TagMappingDTO>,
}