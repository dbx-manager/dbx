use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::controllers::containers_controller::self_fetch_data_async; // Use RwLock for interior mutability if you need to update data later

#[derive(Clone)]
pub struct ContainerList {
    // Wrap in Arc<RwLock> if you need thread-safe mutable access from commands
    pub containers: Arc<RwLock<HashMap<String, Container>>>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Container {
    pub name: Option<String>,
    pub id: Option<String>,
    pub state: Option<String>,
    pub exported_apps: Option<Vec<String>>,
    pub system_apps: Option<Vec<String>>,
    pub autobackup: Option<bool>,
    pub autostart: Option<bool>,
}

impl ContainerList {
    /// Factory method to create the initial state
    pub async fn new() -> Self {
        let data = self_fetch_data_async().await;
        ContainerList {
            containers: Arc::new(RwLock::new(data)),
        }
    }
}
