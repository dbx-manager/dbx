use podman_api::models::ListContainer;
use podman_api::opts::ContainerListOpts;
use std::collections::HashMap;
use tokio::time::{interval, Duration};
use crate::structs::container::{Container, ContainerList};
use crate::structs::socket::PodmanSocket;



pub async fn self_fetch_data_async() -> HashMap<String, Container> {
    let mut container_list: HashMap<String, Container> = HashMap::new();
    let containers = fetch_containers_data().await;
    for container in containers {
        let name = container.names.unwrap().first().cloned().unwrap_or_default();
        let key = name.clone();

        container_list.insert(
            key,
            Container {
                name: Some(name),
                id: container.id,
                state: container.state,
                exported_apps: None,
                system_apps: None,
                autobackup: None,
                autostart: None,
            },
        );
    }
    container_list
}

// Helper function to fetch container data
pub async fn fetch_containers_data() -> Vec<ListContainer> {
    let podman = PodmanSocket::get_instance().await.socket.clone();
    
    podman
    .containers()
    .list(&ContainerListOpts::builder().all(true).build())
    .await
    .unwrap_or_else(|e| {
        eprintln!("Failed to fetch containers: {}", e);
        Vec::new() // Return empty vec on error instead of panicking
    })
}

// Background task function that updates container data
pub async fn start_container_monitoring(container_state: ContainerList) {
    let mut interval = interval(Duration::from_secs(5));
    loop {
        interval.tick().await;

        let new_containers = fetch_containers_data().await;
        
        let mut data_lock = container_state.containers.write().await;

        for container in new_containers {
            let name = container.names.unwrap().first().cloned().unwrap_or_default();
            let key = name.clone();
            if let Some(existing) = data_lock.get_mut(&name) {
                // Update ONLY the status (or any other specific field)
                existing.state = container.state;
            } else {
                // Insert new container if it doesn't exist
                data_lock.insert(
                    key,
                    Container {
                        name:  Some(name),
                        id: container.id,
                        state: container.state,
                        exported_apps: None,
                        system_apps: None,
                        autobackup: None,
                        autostart: None,
                    },
                );
            }
        }
        
        // TODO: Remove containers that no longer exist in fetch_containers_data
        // This requires collecting IDs from new_containers first and retaining only those keys.
    }
}