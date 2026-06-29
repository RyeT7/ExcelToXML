use crate::application::dto::request::tagmappingdto::TagMappingsDTO;

pub trait MapHeadersUseCase: Send + Sync {
    fn map_headers(&self, session_id: &str, mappings: &TagMappingsDTO) -> Result<(), String>;
}