use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct TagMappings {
    pub mappings: HashMap<String, TagMapping>,
    pub invoice_number_column: String,
    pub good_service_identifier_column: String,
}

#[derive(Clone, Debug)]
pub struct TagMapping {
    pub mapped_column: Option<String>,
    pub default_value: Option<String>,
}

impl TagMapping {
    pub fn new(
        mapped_column: Option<String>,
        default_value: Option<String>,
    ) -> Self {
        TagMapping {
            mapped_column,
            default_value,
        }
    }

    pub fn is_mapped(&self) -> bool {
        self.mapped_column.is_some()
    }

    pub fn get_value(&self) -> Option<&str> {
        self.mapped_column.as_deref()
            .or(self.default_value.as_deref())
    }
}