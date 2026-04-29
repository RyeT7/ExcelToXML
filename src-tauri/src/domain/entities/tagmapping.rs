#[derive(Clone, Debug)]
pub struct TagMapping {
    pub literal: String,
    pub hierarchical: String,
    pub mapped_column: Option<String>,
    pub default_value: Option<String>,
}

impl TagMapping {
    pub fn new(
        literal: String,
        hierarchical: String,
        mapped_column: Option<String>,
        default_value: Option<String>,
    ) -> Self {
        TagMapping {
            literal,
            hierarchical,
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
