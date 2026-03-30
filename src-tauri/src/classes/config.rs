use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    pub containers: Vec<String>,
    pub export_path: String,
    pub cron_schedule: Option<String>,
}

pub struct ConfigState {
    pub data: Arc<RwLock<BackupConfig>>,
}

impl ConfigState {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(Self::load_config())),
        }
    }

    fn load_config() -> BackupConfig {
        // Try to load from user config directory first
        if let Some(user_config) = Self::load_from_user_config() {
            return user_config;
        }

        // If user config doesn't exist, copy from sample and use that
        if let Some(sample_config) = Self::load_from_sample() {
            // Try to copy to user config directory
            if Self::copy_sample_to_user_config().is_ok() {
                return sample_config;
            }
        }

        // Fallback to sample config even if copy failed
        Self::load_from_sample().unwrap_or_else(|| {
            // Default config if everything fails
            BackupConfig {
                containers: Vec::new(),
                export_path: "/home/user/dbx-backup/containers".to_string(),
                cron_schedule: None,
            }
        })
    }

    fn load_from_user_config() -> Option<BackupConfig> {
        let user_config_path = Self::get_user_config_path();
        
        if user_config_path.exists() {
            if let Ok(content) = fs::read_to_string(&user_config_path) {
                if let Ok(config) = serde_json::from_str(&content) {
                    return Some(config);
                }
            }
        }
        None
    }

    fn load_from_sample() -> Option<BackupConfig> {
        let sample_path = Path::new("src-tauri/src/scripts/backup.conf");
        
        if sample_path.exists() {
            if let Ok(content) = fs::read_to_string(sample_path) {
                if let Ok(config) = serde_json::from_str(&content) {
                    return Some(config);
                }
            }
        }
        None
    }

    fn copy_sample_to_user_config() -> Result<(), Box<dyn std::error::Error>> {
        let sample_path = Path::new("src-tauri/src/scripts/backup.conf");
        let user_config_path = Self::get_user_config_path();
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = user_config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Copy the sample config to user directory
        fs::copy(sample_path, &user_config_path)?;
        Ok(())
    }

    fn get_user_config_path() -> std::path::PathBuf {
        let mut config_dir = dirs::config_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        config_dir.push("dbx");
        config_dir.push("backup.conf");
        config_dir
    }

    pub fn update_backup_config(&self, new_config: &BackupConfig) -> Result<(), Box<dyn std::error::Error>> {
        let user_config_path = Self::get_user_config_path();
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = user_config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Serialize the config to JSON
        let config_json = serde_json::to_string_pretty(new_config)?;
        
        // Write the config to the user config file
        fs::write(&user_config_path, config_json)?;
        
        Ok(())
    }
}

// Background task function that monitors config file changes
pub async fn start_config_monitoring(config_state: Arc<RwLock<BackupConfig>>) {
    let mut interval = interval(Duration::from_secs(2)); // Check every 2 seconds
    let mut last_modified_time = std::time::UNIX_EPOCH;
    
    loop {
        interval.tick().await;
        
        // Check if user config file has been modified
        let user_config_path = ConfigState::get_user_config_path();
        if user_config_path.exists() {
            if let Ok(metadata) = fs::metadata(&user_config_path) {
                let modified_time = metadata.modified().unwrap_or(std::time::UNIX_EPOCH);
                
                if modified_time > last_modified_time {
                    last_modified_time = modified_time;
                    
                    // Reload config from user file
                    if let Some(new_config) = ConfigState::load_from_user_config() {
                        let mut config_lock = config_state.write().await;
                        *config_lock = new_config;
                    }
                }
            }
        }
    }
}
