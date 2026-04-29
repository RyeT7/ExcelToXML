use tauri::State;

use crate::{
    application::dto::response::tagdto::TagDTO,
    state::appstate::AppState
};

#[tauri::command]
pub async fn view_headers(
    state: State<'_, AppState>,
) -> Result<Vec<TagDTO>, String> {
    state
        .view_headers_use_case
        .view_headers()
}