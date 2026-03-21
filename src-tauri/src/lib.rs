mod classes;
mod services;
use crate::services::container_service::*;
use crate::services::apps_service::*;

#[tokio::main]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
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
