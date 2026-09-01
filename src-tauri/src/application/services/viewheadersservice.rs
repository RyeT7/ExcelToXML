use crate::{
    application::{
        dto::response::tagdto::TagDTO, ports::inbound::viewheadersusecase::ViewHeadersUseCase,
    },
    domain::enums::requiredtags::Tags,
};

pub struct ViewHeadersService;

impl Default for ViewHeadersService {
    fn default() -> Self {
        Self::new()
    }
}

impl ViewHeadersService {
    pub fn new() -> Self {
        Self
    }
}

impl ViewHeadersUseCase for ViewHeadersService {
    fn view_headers(&self) -> Result<Vec<TagDTO>, String> {
        Ok(Tags::MAPPABLE
            .iter()
            .map(|tag| TagDTO {
                literal: tag.as_literal_str(),
                hierarchical: tag.as_hierarchical_str(),
                derived: tag.is_derived(),
                formula: tag.formula(),
            })
            .collect())
    }
}
