use crate::application::dto::response::tagdto::TagDTO;

pub trait ViewHeadersUseCase: Send + Sync {
    fn view_headers(&self) -> Result<Vec<TagDTO>, String>;
}