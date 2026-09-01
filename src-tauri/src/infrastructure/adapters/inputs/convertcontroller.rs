use tauri::State;

use crate::state::appstate::AppState;

#[tauri::command]
pub async fn convert(
    state: State<'_, AppState>,
    session_id: String,
    tin: String,
) -> Result<(), String> {
    state
        .convert_use_case
        .lock()
        .map_err(|e| format!("Failed to acquire Convert Use Case lock: {e}"))?
        .convert(&session_id, &tin)
}
