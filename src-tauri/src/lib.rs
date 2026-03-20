mod classes;
use podman_api::models::ListContainer;

use crate::classes::containers::Containers;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
async fn get_container_list ()-> Result<Vec<ListContainer>, String> {
    let instance = Containers::get_instance().await;
    Ok(instance.data.clone())     
}

#[tokio::main]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {


        
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_container_list,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
