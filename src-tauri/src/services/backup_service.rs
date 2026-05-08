use crate::controllers::backup_controller::*;
use crate::structs::config::ConfigState;

#[tauri::command]
pub async fn create_backup(
    config_state: tauri::State<'_, ConfigState>,
    container_id: String,
) -> Result<(), String> {
    match create_container_backup(container_id, config_state.data.read().await.clone()).await {
        Ok(_) => {}
        Err(e) => {
            println!("{}",e)
        }
    };
    Ok(())
}
#[tauri::command]
pub async fn restore_backup(tar_file_path: String) -> Result<(), String> {
    match restore_distrobox_container(tar_file_path).await {
        Ok(_) => {}
        Err(e) => {
            println!("{}",e)
        }
    };
    Ok(())
}
#[tauri::command]
pub async fn delete_backup(tar_file_path: String) -> Result<(), String> {
    match delete_backup_tarfile(tar_file_path) {
        Ok(_) => {}
        Err(e) => {
            println!("{}",e)
        }
    };
    Ok(())
}
