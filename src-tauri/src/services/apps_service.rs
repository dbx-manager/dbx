
use std::path::Path;

use crate::controllers::export_an_app_controller::{delete_exported_apps, export_app_to_host, parse_container_exportable_apps};
use crate::controllers::exported_apps_controller::parse_distrobox_apps;
use crate::controllers::system_apps_controller::parse_container_system_packages;
use crate::structs::container::ContainerList;
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

#[tauri::command]
pub async fn get_export_an_app_list(
    state: State<'_, ContainerList>,
    container_id: String,
) -> Result<bool, String> {
    parse_container_exportable_apps(container_id, state).await
}

#[tauri::command]
pub async fn export_an_app(
    state: State<'_, ContainerList>,
    container_id: String,
    app_name: String,
) -> Result<bool, String> {
    export_app_to_host(container_id, app_name, state).await
}

#[tauri::command]
pub async fn delete_exported_an_app(
    state: State<'_, ContainerList>,
    container_id: String,
    app_names: Vec<String>,
) -> Result<bool, String> {
    delete_exported_apps(container_id, app_names, state).await
}
