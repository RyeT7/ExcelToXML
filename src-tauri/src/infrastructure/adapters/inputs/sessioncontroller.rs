use tauri::State;

use crate::state::appstate::AppState;

#[tauri::command]
pub async fn create_session(
    state: State<'_, AppState>,
) -> Result<String, String> {
    state
        .create_session_use_case
        .create_session()
}