use std::collections::HashMap;
use tauri::State;
use crate::classes::exported_apps::ExportedAppsState;

#[tauri::command]
pub async fn get_exported_apps(state: State<'_, ExportedAppsState>) -> Result<HashMap<String,Vec<String>>, String> {
    let data = state.data.read().await;
    Ok(data.clone())
}
// #[tauri::command]
// pub async fn get_system_apps() -> Result<HashMap<String,Vec<String>>, String> {
//     let x=ExportedAppsList::get_instance().await;
//         Ok(x.apps.clone())
// }