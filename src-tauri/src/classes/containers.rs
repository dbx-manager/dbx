use podman_api::models::ListContainer;
use podman_api::opts::ContainerListOpts;
use tokio::time::{interval, Duration};
use std::sync::Arc;
use tokio::sync::RwLock; 
use crate::classes::socket::PodmanSocket;

// AppState structure for Tauri managed state
pub struct ContainersState {
    pub data: Arc<RwLock<Vec<ListContainer>>>,
}

impl ContainersState {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

// Background task function that updates container data
pub async fn start_container_monitoring(containers_state: Arc<RwLock<Vec<ListContainer>>>) {
    let mut interval = interval(Duration::from_secs(5));
    loop {
        interval.tick().await;
        
        let new_data = fetch_containers_data().await;
        
        let mut data_lock = containers_state.write().await;
        *data_lock = new_data;
    }
}

// Helper function to fetch container data
pub async fn fetch_containers_data() -> Vec<ListContainer> {
    let podman = PodmanSocket::get_instance().await.socket.clone();

    podman
        .containers()
        .list(
            &ContainerListOpts::builder()
                .all(true)
                .build(),
        )
        .await
        .unwrap_or_else(|e| {
            eprintln!("Failed to fetch containers: {}", e);
            Vec::new() // Return empty vec on error instead of panicking
        })
}
