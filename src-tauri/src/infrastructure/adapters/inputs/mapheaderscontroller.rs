use tauri::State;

use crate::{
    application::dto::request::tagmappingdto::TagMappingsDTO,
    state::appstate::AppState
};

#[tauri::command]
pub async fn map_headers(
    state: State<'_, AppState>,
    session_id: String,
    tag_mappings: TagMappingsDTO,
) -> Result<(), String> {
    state
        .map_headers_use_case
        .map_headers(&session_id, &tag_mappings)
}
