use crate::classes::containers::Containers;
use crate::classes::socket::PodmanSocket;
use podman_api::models::ListContainer;
#[tauri::command]
pub async fn get_container_list() -> Result<Vec<ListContainer>, String> {
    Ok(Containers::get_instance().await.data.read().await.clone())
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
pub async fn pause_container(id: String) -> Result<(), String>{
    let podman = PodmanSocket::get_instance().await.socket.clone();
    if let Err(e) = podman.containers().get(id).pause().await {
        eprintln!("{}", e);
    }
    Ok(())
}
#[tauri::command]
pub async fn unpause_container(id: String) -> Result<(), String>{
    let podman = PodmanSocket::get_instance().await.socket.clone();
    if let Err(e) = podman.containers().get(id).unpause().await {
        eprintln!("{}", e);
    }
    Ok(())
}
