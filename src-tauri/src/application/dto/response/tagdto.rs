use serde::Serialize;

#[derive(Serialize)]
pub struct TagDTO {
    pub literal: &'static str,
    pub hierarchical: &'static str,
    /// True when the value is computed during conversion instead of being
    /// mapped to a column. The frontend renders these read-only and leaves
    /// them out of the mapping it submits.
    pub derived: bool,
    /// Formula shown next to a derived tag; `None` for mappable tags.
    pub formula: Option<&'static str>,
}
