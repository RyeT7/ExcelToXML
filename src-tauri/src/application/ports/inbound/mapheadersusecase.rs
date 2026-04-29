use crate::application::dto::request::tagmappingdto::TagMappingDTO;

pub trait MapHeadersUseCase: Send + Sync {
    fn map_headers(&self, session_id: &str, mappings: &[TagMappingDTO]) -> Result<(), String>;
}