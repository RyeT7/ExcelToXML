use serde::Serialize;

#[derive(Serialize)]
pub struct TagDTO {
    pub literal: &'static str,
    pub hierarchical: &'static str
}