mod structs;
mod scripts;
mod controllers;
mod services;
use crate::controllers::config_controller::start_config_monitoring;
use crate::controllers::containers_controller::start_container_monitoring;
use crate::structs::config::{ ConfigState};
use crate::services::config_service::*;
use crate::structs::container::{ContainerList};
use crate::services::apps_service::*;
use crate::services::container_service::*;

#[tokio::main]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    // Create the containers state
    let containers_state = ContainerList::new().await;
    let containers_state_clone = containers_state.clone();

    let config_state = ConfigState::new();
    let config_state_clone = config_state.clone();

    // Start the background monitoring tasks
    tokio::spawn(async move {
        start_container_monitoring(containers_state_clone).await;
    });


    tokio::spawn(async move {
        start_config_monitoring(config_state_clone).await;
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(containers_state)
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
            update_backup_config,
            get_package_cache_size,
            get_backup_directory_list
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
