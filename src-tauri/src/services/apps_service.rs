use std::collections::HashMap;

use crate::classes::exported_apps::{ ExportedAppsList};
#[tauri::command]
pub async fn get_exported_apps() -> Result<HashMap<String,Vec<String>>, String> {
    let x=ExportedAppsList::get_instance().await;
        Ok(x.apps.clone())
}
