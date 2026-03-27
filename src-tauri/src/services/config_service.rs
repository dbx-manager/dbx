use crate::classes::config::{BackupConfig, ConfigState};
use tauri::Emitter;


// Add these functions to expose config to frontend
#[tauri::command]
pub async fn get_backup_config(config_state: tauri::State<'_, ConfigState>) -> Result<BackupConfig, String> {
    let config = config_state.data.read().await.clone();
    Ok(config)
}

#[tauri::command]
pub async fn update_backup_config(
    config_state: tauri::State<'_, ConfigState>,
    new_config: BackupConfig,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let mut config_lock = config_state.data.write().await;
    *config_lock = new_config.clone();
    
    // Emit event to notify frontend of config change
    app_handle.emit("config-updated", &new_config)
        .map_err(|e| format!("Failed to emit config update event: {}", e))?;
    
    Ok(())
}
