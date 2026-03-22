mod classes;
mod services;
use crate::services::container_service::*;
use crate::services::apps_service::*;
use crate::classes::containers::{ContainersState, start_container_monitoring};
use crate::classes::exported_apps::{ExportedAppsState, start_exported_apps_monitoring};

#[tokio::main]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    // Create the containers state
    let containers_state = ContainersState::new();
    let containers_state_clone = containers_state.data.clone();
    
    // Create the exported apps state
    let exported_apps_state = ExportedAppsState::new();
    let exported_apps_state_clone = exported_apps_state.data.clone();
    
    // Start the background monitoring tasks
    tokio::spawn(async move {
        start_container_monitoring(containers_state_clone).await;
    });
    
    tokio::spawn(async move {
        start_exported_apps_monitoring(exported_apps_state_clone).await;
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(containers_state)
        .manage(exported_apps_state)
        .invoke_handler(tauri::generate_handler![
            get_container_list,
            start_container,
            stop_container,
            pause_container,
            unpause_container,
            get_exported_apps
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
