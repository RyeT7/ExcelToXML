use tauri::State;

use crate::state::appstate::AppState;

#[tauri::command]
pub async fn get_xml(state: State<'_, AppState>, session_id: String) -> Result<String, String> {
    state.get_xml_use_case.get_xml(&session_id)
}

#[tauri::command]
pub async fn save_xml(
    state: State<'_, AppState>,
    session_id: String,
    path: String,
) -> Result<(), String> {
    let xml = state.get_xml_use_case.get_xml(&session_id)?;

    std::fs::write(&path, xml).map_err(|e| format!("Failed to write XML to '{path}': {e}"))
}
