use crate::structs::config::ConfigState;
use std::fs;
use std::path::Path;
use std::process::Command;
use podman_api::opts::ContainerCommitOpts;
use crate::structs::socket::PodmanSocket;
use chrono::Local;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct BackupInfo {
    pub file_path: String,
    pub file_name: String,
    pub size_bytes: u64,
    pub size_human: String,
    pub created_at: String,
    pub container_id: String,
}

#[derive(Debug, serde::Serialize)]
pub struct BackupResult {
    pub success: bool,
    pub backup_file: Option<String>,
    pub error: Option<String>,
}

// Helper to format bytes
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

/// Create a backup for a container using podman commit and save
#[tauri::command]
pub async fn create_backup(
    container_id: String,
    config_state: tauri::State<'_, ConfigState>,
) -> Result<BackupResult, String> {
    let config = config_state.data.read().await.clone();
    let backup_dir = Path::new(&config.backup_location_path);
    // Ensure backup directory exists
    fs::create_dir_all(backup_dir)
        .map_err(|e| format!("Failed to create backup directory: {}", e))?;

    let podman = PodmanSocket::get_instance().await.socket.clone();
    let container = podman.containers().get(&container_id);

    // Check if container exists
    if container.inspect().await.is_err() {
        return Ok(BackupResult {
            success: false,
            backup_file: None,
            error: Some(format!("Container {} not found", container_id)),
        });
    }

    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let short_id = if container_id.len() >= 12 {
        &container_id[..12]
    } else {
        &container_id[..]
    };
    let temp_image_name = format!("backup-temp-{}-{}", short_id, timestamp);
    let export_file = backup_dir.join(format!("container_{}_{}.tar", short_id, timestamp));
    let export_file_str = export_file.to_string_lossy().into_owned();

    // Commit container to temporary image
    let commit_opts = ContainerCommitOpts::builder()
        .tag(&temp_image_name)
        .build();
    if let Err(e) = container.commit(&commit_opts).await {
        return Ok(BackupResult {
            success: false,
            backup_file: None,
            error: Some(format!("Failed to commit container: {}", e)),
        });
    }

    // Export image to tar file using podman save
    let save_output = Command::new("podman")
        .arg("save")
        .arg("-o")
        .arg(&export_file_str)
        .arg(&temp_image_name)
        .output();
    let save_output = match save_output {
        Ok(output) => output,
        Err(e) => {
            // Clean up temporary image
            let _ = podman.images().get(&temp_image_name).remove().await;
            return Ok(BackupResult {
                success: false,
                backup_file: None,
                error: Some(format!("Failed to export image: {}", e)),
            });
        }
    };
    if !save_output.status.success() {
        let _ = podman.images().get(&temp_image_name).remove().await;
        let stderr = String::from_utf8_lossy(&save_output.stderr);
        return Ok(BackupResult {
            success: false,
            backup_file: None,
            error: Some(format!("Failed to export image: {}", stderr)),
        });
    }
    // Clean up temporary image
    let _ = podman.images().get(&temp_image_name).remove().await;

    // Verify file was created
    if export_file.exists() {
        Ok(BackupResult {
            success: true,
            backup_file: Some(export_file_str),
            error: None,
        })
    } else {
        Ok(BackupResult {
            success: false,
            backup_file: None,
            error: Some("Backup file was not created".to_string()),
        })
    }
}

/// Delete a backup file safely
#[tauri::command]
pub async fn delete_backup(
    file_path: String,
    config_state: tauri::State<'_, ConfigState>,
) -> Result<bool, String> {
    let config = config_state.data.read().await.clone();
    let backup_dir = Path::new(&config.backup_location_path);
    let path = Path::new(&file_path);

    // Verify file is inside backup directory
    if !path.starts_with(backup_dir) {
        return Err("File not in backup directory".to_string());
    }

    // Verify file exists and is a .tar file
    if !path.exists() || !path.is_file() || path.extension().and_then(|e| e.to_str()) != Some("tar") {
        return Err("Invalid backup file".to_string());
    }

    fs::remove_file(path)
        .map_err(|e| format!("Failed to delete backup: {}", e))?;

    Ok(true)
}

/// List all backup files for a specific container
#[tauri::command]
pub async fn list_backups_for_container(
    container_id: String,
    config_state: tauri::State<'_, ConfigState>,
) -> Result<Vec<BackupInfo>, String> {
    let config = config_state.data.read().await.clone();
    let backup_dir = Path::new(&config.backup_location_path);
    let mut backups = Vec::new();

    if !backup_dir.exists() {
        return Ok(backups);
    }

    let entries = fs::read_dir(backup_dir)
        .map_err(|e| format!("Failed to read backup directory: {}", e))?;

    let short_id = if container_id.len() >= 12 {
        &container_id[..12]
    } else {
        &container_id[..]
    };

    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("tar") {
            continue;
        }
        let file_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name.to_string(),
            None => continue,
        };

        // Check if filename contains the container short ID
        if !file_name.contains(&format!("container_{}", short_id)) {
            continue;
        }

        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        let size = metadata.len();
        let modified = metadata.modified().unwrap_or(std::time::UNIX_EPOCH);
        let datetime: chrono::DateTime<chrono::Local> = modified.into();
        let created_at = datetime.format("%Y-%m-%d %H:%M:%S").to_string();

        backups.push(BackupInfo {
            file_path: path.to_string_lossy().into_owned(),
            file_name,
            size_bytes: size,
            size_human: format_bytes(size),
            created_at,
            container_id: container_id.clone(),
        });
    }

    // Sort by created_at descending (newest first)
    backups.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Ok(backups)
}

/// Get latest backup for a container
#[tauri::command]
pub async fn get_latest_backup(
    container_id: String,
    config_state: tauri::State<'_, ConfigState>,
) -> Result<Option<BackupInfo>, String> {
    let backups = list_backups_for_container(container_id, config_state).await?;
    Ok(backups.into_iter().next())
}

/// Restore a container from a backup tar file
#[tauri::command]
pub async fn restore_container_from_backup(
    container_id: String,
    backup_file: String,
    config_state: tauri::State<'_, ConfigState>,
) -> Result<bool, String> {
    let config = config_state.data.read().await.clone();
    let backup_path = Path::new(&backup_file);

    // Verify backup file exists and is inside backup directory
    let backup_dir = Path::new(&config.backup_location_path);
    if !backup_path.starts_with(backup_dir) || !backup_path.exists() {
        return Err("Invalid backup file".to_string());
    }

    let podman = PodmanSocket::get_instance().await.socket.clone();

    // Stop and remove existing container if it exists
    let container = podman.containers().get(&container_id);
    if container.inspect().await.is_ok() {
        let _ = container.stop(&Default::default()).await;
        let _ = container.delete(&Default::default()).await;
    }

    // Load the backup tar as an image
    let load_output = Command::new("podman")
        .arg("load")
        .arg("-i")
        .arg(backup_file)
        .output();
    let image_name = match load_output {
        Ok(output) if output.status.success() => {
            // Parse output to get image name (usually last line)
            // Example output: "Loaded image: docker.io/library/backup-temp-..."
            // We'll just use a fixed name for now
            format!("restored-image-{}", container_id)
        }
        Ok(_) => format!("restored-image-{}", container_id),
        Err(e) => return Err(format!("Failed to load backup: {}", e)),
    };

    // Create new container with distrobox using the loaded image
    let distrobox_result = Command::new("distrobox")
        .arg("create")
        .arg("--name")
        .arg(&container_id)
        .arg("--image")
        .arg(&image_name)
        .output();

    match distrobox_result {
        Ok(output) if output.status.success() => {
            // Clean up temporary image
            let _ = podman.images().get(&image_name).remove().await;
            Ok(true)
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("Failed to create container: {}", stderr))
        }
        Err(e) => Err(format!("Failed to execute distrobox: {}", e)),
    }
}