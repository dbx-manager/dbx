use crate::structs::container::{Container, ContainerList};
use crate::structs::new_container_request::NewContainerRequest;
use crate::structs::socket::PodmanSocket;
use crate::controllers::distrobox_controller::create_distrobox_container;
use crate::structs::config::ConfigState;

use std::collections::HashMap;
use tauri::State;

/// Return a map of container name → Container struct.
#[tauri::command]
pub async fn get_container_list(
    containers_state: State<'_, ContainerList>,
) -> Result<HashMap<String, Container>, String> {
    let data = containers_state.containers.read().await;
    Ok(data.clone())
}

/// Create a new Distrobox container.
#[tauri::command]
pub async fn create_new_container(request: NewContainerRequest) -> Result<(), String> {
    create_distrobox_container(request).await
}

/// Stop a container. If the container is marked for auto‑backup in the config,
/// a backup is created automatically after the stop operation.
#[tauri::command]
pub async fn stop_container(
    id: String,
    config_state: State<'_, ConfigState>,
) -> Result<(), String> {
    let podman = PodmanSocket::get_instance().await.socket.clone();

    // Stop the container.
    if let Err(e) = podman.containers().get(&id).stop(&Default::default()).await {
        eprintln!("Failed to stop container {}: {}", id, e);
    }

    // Auto‑backup if enabled for this container.
    {
        let cfg = config_state.data.read().await;
        if cfg.backup_containers.contains(&id) {
            // Use the existing backup command.
            let _ = crate::services::config_service::backup_container(
                id.clone(),
                cfg.backup_location_path.clone(),
            )
            .await;
        }
    }

    Ok(())
}

/// Pause a container.
#[tauri::command]
pub async fn pause_container(id: String) -> Result<(), String> {
    let podman = PodmanSocket::get_instance().await.socket.clone();
    if let Err(e) = podman.containers().get(&id).pause().await {
        eprintln!("Failed to pause container {}: {}", id, e);
    }
    Ok(())
}

/// Unpause a container.
#[tauri::command]
pub async fn unpause_container(id: String) -> Result<(), String> {
    let podman = PodmanSocket::get_instance().await.socket.clone();
    if let Err(e) = podman.containers().get(&id).unpause().await {
        eprintln!("Failed to unpause container {}: {}", id, e);
    }
    Ok(())
}