use dirs;
use futures_util::future::ok;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    pub containers: Vec<String>,
    pub cron_schedule: Option<String>,
    pub package_cache_path: String,
    pub backup_location_path: String,
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
            print!("Warning: no configuration file were found, writing new one configuration from sample");
            // Try to copy to user config directory
            if Self::copy_sample_to_user_config().is_ok() {
                return sample_config;
            }
        }

        // Fallback to sample config even if copy failed
        Self::load_from_sample().unwrap()
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

        return Some(BackupConfig {
            containers: Vec::new(),
            backup_location_path: dirs::home_dir()
                .unwrap()
                .join("dbx-backup")
                .to_string_lossy()
                .into_owned(),
            cron_schedule: Some(String::from("0 2 * * *")),
            package_cache_path: dirs::cache_dir()
                .unwrap()
                .join("dbx")
                .to_string_lossy()
                .into_owned(),
        });
    }

    fn copy_sample_to_user_config() -> Result<(), Box<dyn std::error::Error>> {
        let user_config_path = Self::get_user_config_path();
        let user_confegeration_sample=Self::load_from_sample().unwrap();
        // Create parent directory if it doesn't exist
        if let Some(parent) = user_config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let e=fs::write(user_config_path.as_path(),serde_json::to_string_pretty(&user_confegeration_sample).unwrap());
        if e.is_err() {
            eprintln!("Error adding sample configeration to default confegeration path: {}",e.err().unwrap());
        }
        
        Ok(())
    }

    fn get_user_config_path() -> std::path::PathBuf {
        let mut config_dir = dirs::config_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        config_dir.push("dbx");
        config_dir.push("backup.conf");
        config_dir
    }

    pub fn update_backup_config(
        &self,
        new_config: &BackupConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
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
