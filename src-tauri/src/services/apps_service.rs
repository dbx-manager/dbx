
use std::path::Path;

use crate::controllers::exported_apps_controller::parse_distrobox_apps;
use crate::controllers::system_apps_controller::parse_container_system_packages;
use crate::structs::container::{ ContainerList};
use tauri::State;

#[tauri::command]
pub async fn get_exported_apps(state: State<'_, ContainerList>) -> Result<bool, String> {
    //TODO find better way to get the path of the host applecation dir
    parse_distrobox_apps(Path::new("/home/ag/.local/share/applications"),state).await;
    Ok(true)
}


#[tauri::command]
pub async fn get_system_apps(
    state: State<'_, ContainerList>,
    container_id: String,
) -> Result<bool, String> {
    //TODO error handliling
    let _=parse_container_system_packages(container_id,state).await;
    Ok(true)
}
