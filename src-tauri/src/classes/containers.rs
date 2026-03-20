use podman_api::models::ListContainer;
use podman_api::opts::ContainerListOpts;
use tokio::time::{interval, Duration};
use std::sync::Arc;
use tokio::sync::{OnceCell, RwLock}; // Added RwLock
use crate::classes::socket::PodmanSocket;

pub struct Containers {
    // Wrapped in RwLock to allow async updates from the background task
    // and concurrent reads from the rest of the app
    pub data: RwLock<Vec<ListContainer>>,
}

static SINGLETON: OnceCell<Arc<Containers>> = OnceCell::const_new();

impl Containers {
    async fn init() -> Arc<Containers> {
        let initial_data = self_fetch_data_async().await;
        
        let instance = Arc::new(Containers {
            data: RwLock::new(initial_data),
        });

        // Clone the Arc for the background worker
        let worker = Arc::clone(&instance);

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(5));
            loop {
                interval.tick().await;
                
                // Fetch new data independently
                let new_data = self_fetch_data_async().await;
                
                // Lock briefly to swap the data
                let mut data_lock = worker.data.write().await;
                *data_lock = new_data;
                // Lock drops here automatically
            }
        });

        instance
    }

    pub async fn get_instance() -> Arc<Containers> {
        SINGLETON.get_or_init(Self::init).await.clone()
    }

    // Helper to read data safely
    pub async fn list_containers(&self) -> Vec<ListContainer> {
        self.data.read().await.clone()
    }
}

async fn self_fetch_data_async() -> Vec<ListContainer> {
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