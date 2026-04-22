use crate::structs::container::{Container, ContainerList};
use crate::structs::socket::PodmanSocket;

use std::{collections::HashMap};
use tauri::State;

#[tauri::command]
pub async fn get_container_list(
    containers_state: State<'_, ContainerList>,
) -> Result<HashMap<String,Container>, String> {
    let data = containers_state.containers.read().await;
    Ok(data.clone())
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
    // execute_script_in_container(id).await;

    Ok(())
}
