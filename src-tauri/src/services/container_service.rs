use crate::classes::containers::ContainersState;
use crate::classes::socket::PodmanSocket;
use podman_api::models::ListContainer;
use tauri::State;

#[tauri::command]
pub async fn get_container_list(containers_state: State<'_, ContainersState>) -> Result<Vec<ListContainer>, String> {
    let data = containers_state.data.read().await;
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
