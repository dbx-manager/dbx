use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::controllers::config_controller::load_config;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    pub containers: Vec<String>,
    pub cron_schedule: Option<String>,
    pub package_cache_path: String,
    pub backup_location_path: String,
}
#[derive(Clone)]
pub struct ConfigState {
    pub data: Arc<RwLock<BackupConfig>>,
}

impl ConfigState {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(load_config())),
        }
    }
}


#[derive(Debug, Serialize)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub is_dir: bool,
}

#[derive(Debug, Serialize)]
pub struct DirectoryListInfo {
    pub path: String,
    pub files: Vec<FileInfo>,
    pub total_size_bytes: u64,
}

// Response structs
#[derive(Debug, Serialize)]
pub struct DirectorySizeInfo {
    pub path: String,
    pub size_bytes: u64,
    pub size_human: String,
}
