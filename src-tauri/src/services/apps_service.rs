use tauri::State;
use crate::classes::exported_apps::{ExportedAppsState, ExportedAppContainer};
use crate::classes::system_apps::SystemAppsState;

#[tauri::command]
pub async fn get_exported_apps(state: State<'_, ExportedAppsState>) -> Result<Vec<ExportedAppContainer>, String> {
    let data = state.data.read().await;
    Ok(data.clone())
}

#[tauri::command]
pub async fn get_system_apps(state: State<'_, SystemAppsState>,container_id:String) -> Result<Vec<String>, String> {
    let data= state.get_or_insert(container_id).await;
    Ok(data.clone()) 
}
