mod classes;
mod scripts;
mod services;
use crate::classes::config::{start_config_monitoring, ConfigState};
use crate::services::config_service::*;
use crate::classes::containers::{start_container_monitoring, ContainersState};
use crate::classes::exported_apps::{start_exported_apps_monitoring, ExportedAppsState};
use crate::classes::system_apps::SystemAppsState;
use crate::services::apps_service::*;
use crate::services::container_service::*;

#[tokio::main]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    // Create the containers state
    let containers_state = ContainersState::new();
    let containers_state_clone = containers_state.data.clone();

    // Create the exported apps state
    let exported_apps_state = ExportedAppsState::new();
    let exported_apps_state_clone = exported_apps_state.data.clone();

    // Create the system apps state
    let system_apps_state = SystemAppsState::new();
    // let system_apps_state_clone = system_apps_state.data.clone();

    // Create the config state
    let config_state = ConfigState::new();
    let config_state_clone = config_state.data.clone();

    // Start the background monitoring tasks
    tokio::spawn(async move {
        start_container_monitoring(containers_state_clone).await;
    });

    tokio::spawn(async move {
        start_exported_apps_monitoring(exported_apps_state_clone).await;
    });

    tokio::spawn(async move {
        start_config_monitoring(config_state_clone).await;
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(containers_state)
        .manage(exported_apps_state)
        .manage(system_apps_state)
        .manage(config_state)
        .invoke_handler(tauri::generate_handler![
            get_container_list,
            start_container,
            stop_container,
            pause_container,
            unpause_container,
            get_exported_apps,
            get_system_apps,
            get_backup_config,
            update_backup_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
