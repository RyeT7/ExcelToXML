use tauri::State;

use crate::{domain::datastructures::table::Table, state::appstate::AppState};

#[tauri::command]
pub async fn load_excel(
    state: State<'_, AppState>,
    session_id: String,
    path: String,
) -> Result<(), String> {
    state
        .upload_excel_use_case
        .load_excel(&session_id, &path)
}

#[tauri::command]
pub async fn get_headers(
    state: State<'_, AppState>,
    session_id: String,
) -> Result<Vec<String>, String> {
    state
        .get_headers_use_case
        .get_headers(&session_id)
}

#[tauri::command]
pub async fn view_excel(
    state: State<'_, AppState>,
    session_id: String,
) -> Result<Table, String> {
    state
        .view_excel_use_case
        .view_excel(&session_id)
}