use tauri::State;

use crate::controllers::backup_controller::create_distrobox_container;
use crate::controllers::containers_controller::delete_container as controller_delete_container;
use crate::structs::container::{Container, ContainerList};
use crate::structs::new_container_request::NewContainerRequest;
use crate::structs::socket::PodmanSocket;

use std::{collections::HashMap};

#[tauri::command]
pub async fn get_container_list(
    containers_state: State<'_, ContainerList>,
) -> Result<HashMap<String,Container>, String> {
    let data = containers_state.containers.read().await;
    Ok(data.clone())
}
#[tauri::command]
pub async fn create_new_container(request: NewContainerRequest) -> Result<(), String> {
    create_distrobox_container(request).await
}

#[tauri::command]
pub async fn start_container(id: String) -> Result<(), String> {
    let podman = PodmanSocket::get_instance().await.socket.clone();
    if let Err(e) = podman.containers().get(id).start(None).await {
        eprintln!("{}", e);
    }
    Ok(())
}
#[tauri::command]
pub async fn stop_container(id: String) -> Result<(), String> {
    let podman = PodmanSocket::get_instance().await.socket.clone();
    if let Err(e) = podman.containers().get(id).stop(&Default::default()).await {
        eprintln!("{}", e);
    }
   
    Ok(())
}
#[tauri::command]
pub async fn pause_container(id: String) -> Result<(), String> {
    let podman = PodmanSocket::get_instance().await.socket.clone();
    if let Err(e) = podman.containers().get(id).pause().await {
        eprintln!("{}", e);
    }
    Ok(())
}
#[tauri::command]
pub async fn unpause_container(id: String) -> Result<(), String> {
    let podman = PodmanSocket::get_instance().await.socket.clone();
    if let Err(e) = podman.containers().get(id).unpause().await {
        eprintln!("{}", e);
    }

    Ok(())
}

#[tauri::command]
pub async fn delete_container(id: String) -> Result<(), String> {
    controller_delete_container(id).await
}

