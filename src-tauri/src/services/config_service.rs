use crate::classes::config::{BackupConfig, ConfigState};
use fs_extra::dir::get_size;
use serde::Serialize;
use std::fs;
use tauri::Emitter;

// Response structs
#[derive(Debug, Serialize)]
pub struct DirectorySizeInfo {
    pub path: String,
    pub size_bytes: u64,
    pub size_human: String,
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

// Helper function to format bytes to human readable size
fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

// Add these functions to expose config to frontend
#[tauri::command]
pub async fn get_backup_config(
    config_state: tauri::State<'_, ConfigState>,
) -> Result<BackupConfig, String> {
    let config = config_state.data.read().await.clone();
    Ok(config)
}

#[tauri::command]
pub async fn update_backup_config(
    config_state: tauri::State<'_, ConfigState>,
    new_config: BackupConfig,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    // Update the in-memory config state
    let mut config_lock = config_state.data.write().await;
    *config_lock = new_config.clone();

    // Persist the config to the user config file
    if let Err(e) = config_state.update_backup_config(&new_config) {
        return Err(format!("Failed to save config to file: {}", e));
    }

    // Emit event to notify frontend of config change
    app_handle
        .emit("config-updated", &new_config)
        .map_err(|e| format!("Failed to emit config update event: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn get_package_cache_size(
    config_state: tauri::State<'_, ConfigState>,
) -> Result<DirectorySizeInfo, String> {
    let config = config_state.data.read().await;
    let path = &config.package_cache_path;
    // Get directory size using fs_extra
    let size_bytes = get_size(path).map_err(|e| format!("Failed to get directory size: {}", e))?;

    Ok(DirectorySizeInfo {
        path: path.clone(),
        size_bytes,
        size_human: format_bytes(size_bytes),
    })
}

#[tauri::command]
pub async fn get_backup_directory_list(
    config_state: tauri::State<'_, ConfigState>,
) -> Result<DirectoryListInfo, String> {
    let config = config_state.data.read().await;
    let path = &config.backup_location_path;

    // Read directory entries
    let entries = fs::read_dir(path).map_err(|e| format!("Failed to read directory: {}", e))?;

    let mut files = Vec::new();
    let mut total_size_bytes = 0u64;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let metadata = entry
            .metadata()
            .map_err(|e| format!("Failed to get metadata: {}", e))?;

        let name = entry.file_name().to_str().unwrap_or("").to_string();
        let file_path = entry.path().to_str().unwrap_or("").to_string();
        let size_bytes = metadata.len();
        let is_dir = metadata.is_dir();

        // Only count file sizes for regular files (not directories)
        if !is_dir {
            total_size_bytes += size_bytes;
        }

        files.push(FileInfo {
            name,
            path: file_path,
            size_bytes,
            is_dir,
        });
    }

    // Sort files: directories first, then files, alphabetically
    files.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.cmp(&b.name),
    });

    Ok(DirectoryListInfo {
        path: path.clone(),
        files,
        total_size_bytes,
    })
}
