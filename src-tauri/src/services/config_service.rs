use crate::{
    controllers::config_controller::set_backup_config,
    structs::{
        config::{Config, ConfigState, DirectoryListInfo, DirectorySizeInfo, FileInfo},
        container::ContainerList,
        socket::PodmanSocket,
    },
};
use fs_extra::dir::get_size;
use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use chrono::Utc;
use futures_util::StreamExt;

#[tauri::command]
pub async fn get_backup_config(
    config_state: tauri::State<'_, ConfigState>,
) -> Result<Config, String> {
    let config = config_state.data.read().await.clone();
    Ok(config)
}

#[tauri::command]
pub async fn update_backup_config(
    config_state: tauri::State<'_, ConfigState>,
    new_config: Config,
) -> Result<(), String> {
    // Update the in-memory config state
    let mut config_lock = config_state.data.write().await;
    *config_lock = new_config.clone();

    // Persist the config to the user config file
    if let Err(e) = set_backup_config(&new_config) {
        return Err(format!("Failed to save config to file: {}", e));
    }

    Ok(())
}
#[tauri::command]
pub async fn match_config_container(
    config_state: tauri::State<'_, ConfigState>,
    container_state: tauri::State<'_, ContainerList>,
) -> Result<bool, String> {
    let configstate = config_state.data.read().await.clone();
    let mut containerstate = container_state.containers.write().await;

    //TODO: find a better way to mach them 

    for backup_id in configstate.backup_containers {
        for container in containerstate.values_mut() {
            if let Some(current_id) = &container.id {
                if current_id == &backup_id {
                    container.autobackup = Some(true);
                    break; 
                }
            }
        }
    }
    for autostart_id in configstate.autostart_containers {
        for container in containerstate.values_mut() {
            if let Some(current_id) = &container.id {
                if current_id == &autostart_id {
                    container.autostart = Some(true);
                    break; 
                }
            }
        }
    }
    Ok(true)
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
        let size_human = format_bytes(size_bytes);
        let is_dir = metadata.is_dir();

        // Only count file sizes for regular files (not directories)
        if !is_dir {
            total_size_bytes += size_bytes;
        }

        files.push(FileInfo {
            name,
            path: file_path,
            size_bytes,
            size_human,
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
 
// Create a backup of a container by exporting it to a tar file.
#[tauri::command]
pub async fn backup_container(container_id: String, backup_dir: String) -> Result<String, String> {
    // Ensure backup directory exists
    let backup_path = Path::new(&backup_dir);
    if !backup_path.exists() {
        std::fs::create_dir_all(&backup_path).map_err(|e| format!("Failed to create backup dir: {}", e))?;
    }

    // Build backup file name with timestamp
    let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
    let file_name = format!("{}_{}.tar", container_id, timestamp);
    let full_path = backup_path.join(&file_name);

    // Get podman instance and container
    let podman = PodmanSocket::get_instance().await.socket.clone();
    let container = podman.containers().get(&container_id);

    // Export container to tar stream
    let mut stream = container.export();

    // Open file for writing
    let mut file = File::create(&full_path).map_err(|e| format!("Failed to create backup file: {}", e))?;

    // Write stream chunks to file
    while let Some(chunk) = stream.next().await {
        let data = chunk.map_err(|e| format!("Error streaming export: {}", e))?;
        file.write_all(&data).map_err(|e| format!("Failed to write to backup file: {}", e))?;
    }

    Ok(full_path.to_string_lossy().to_string())
}

// Delete a backup file.
#[tauri::command]
pub async fn delete_backup(backup_path: String) -> Result<(), String> {
    let path = Path::new(&backup_path);
    if !path.exists() {
        return Err("Backup file does not exist".to_string());
    }
    std::fs::remove_file(path).map_err(|e| format!("Failed to delete backup: {}", e))?;
    Ok(())
}

// Recreate a container from a backup tar file using distrobox.
#[tauri::command]
pub async fn recreate_container_from_backup(
    backup_path: String,
    new_container_name: String,
) -> Result<String, String> {
    // First, import the container from the backup tar
    let _podman = PodmanSocket::get_instance().await.socket.clone();
    
    // Load the image from tar (podman load)
    let import_result = std::process::Command::new("podman")
        .args(&["load", "-i", &backup_path])
        .output()
        .map_err(|e| format!("Failed to execute podman load: {}", e))?;
    
    if !import_result.status.success() {
        let err_msg = String::from_utf8_lossy(&import_result.stderr);
        return Err(format!("Failed to load backup: {}", err_msg));
    }
    
    // Parse the output to get the image ID/name
    let output = String::from_utf8_lossy(&import_result.stdout);
    let image_name = output
        .lines()
        .find(|line| line.contains("Loaded image:"))
        .and_then(|line| line.split(": ").nth(1))
        .unwrap_or("imported-image")
        .trim()
        .to_string();
    
    // Create new container using distrobox with the imported image
    let create_result = std::process::Command::new("distrobox")
        .args(&["create", "--name", &new_container_name, &image_name])
        .output()
        .map_err(|e| format!("Failed to create distrobox container: {}", e))?;
    
    if !create_result.status.success() {
        let err_msg = String::from_utf8_lossy(&create_result.stderr);
        return Err(format!("Failed to create container: {}", err_msg));
    }
    
    Ok(new_container_name)
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
