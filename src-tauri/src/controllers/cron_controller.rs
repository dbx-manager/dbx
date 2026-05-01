use std::process::Command;
use regex::Regex;
use crate::{controllers::config_controller::get_user_config_path, structs::config::ConfigState};
use log::{info, warn, error};
use std::fs;
//TODO: add check if cron is installed and if not disable all these functionlaitys
const CRON_IDENTIFIER: &str = "Dbx-Backup";
const BACKUP_SCRIPT_FILENAME: &str = "dbx-backup.sh";

/// Get the path where backup script should reside in user config directory
pub fn get_backup_script_path() -> std::path::PathBuf {
    let mut config_dir = get_user_config_path();
    config_dir.pop(); // Remove backup.conf filename to get the dbx config directory
    config_dir.push(BACKUP_SCRIPT_FILENAME);
    config_dir
}

/// Ensure backup script exists in user config directory, copy from application resources if missing
pub fn ensure_backup_script_exists() -> Result<std::path::PathBuf, String> {
    let target_script_path = get_backup_script_path();

    if !target_script_path.exists() {
        info!("Backup script not found in config directory, copying...");
        
        // Create parent directories if needed
        if let Some(parent) = target_script_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        // Locate source script (first try relative to executable, then fallback to source path)
        let source_script_path = std::env::current_exe()
            .map_err(|e| format!("Failed to get current exe path: {}", e))?
            .parent()
            .ok_or_else(|| "Failed to get parent directory".to_string())?
            .join("scripts")
            .join(BACKUP_SCRIPT_FILENAME);

        // If not found relative to exe, try source location (for development)
        let source_script = if source_script_path.exists() {
            source_script_path
        } else {
            std::path::PathBuf::from("src-tauri/src/scripts/").join(BACKUP_SCRIPT_FILENAME)
        };

        if !source_script.exists() {
            return Err(format!("Backup script source not found at: {}", source_script.display()));
        }

        // Copy the script
        fs::copy(&source_script, &target_script_path)
            .map_err(|e| format!("Failed to copy backup script: {}", e))?;

        // Set executable permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&target_script_path)
                .map_err(|e| format!("Failed to get script metadata: {}", e))?
                .permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&target_script_path, perms)
                .map_err(|e| format!("Failed to set script permissions: {}", e))?;
        }

        info!("Backup script copied successfully to: {}", target_script_path.display());
    }

    Ok(target_script_path)
}

/// Check if Dbx-Backup crontab entry exists
pub fn check_backup_cron() -> Result<bool, String> {
    let crontab_output = Command::new("crontab")
        .arg("-l")
        .output()
        .map_err(|e| format!("Failed to read crontab: {}", e))?;

    if !crontab_output.status.success() {
        // No crontab exists for user yet
        return Ok(false);
    }

    let crontab_content = String::from_utf8_lossy(&crontab_output.stdout);
    // Detect Dbx-Backup cron entry (identifier at end of line as comment)
    let re = Regex::new(r"(?m)#\s*Dbx-Backup\s*$").map_err(|e| format!("Regex error: {}", e))?;
    Ok(re.is_match(&crontab_content))
}

/// Install or update Dbx-Backup crontab entry
pub fn install_backup_cron(schedule: &str, config_path: &str) -> Result<(), String> {
    // Ensure script exists in config directory and get its path
    let script_path = ensure_backup_script_exists()?;

    let script_path_str = script_path.to_str()
        .ok_or_else(|| "Invalid script path".to_string())?;

    // Build cron command line
    let cron_entry = format!(
        "{} /bin/bash {} {} # {}\n",
        schedule,
        script_path_str,
        config_path,
        CRON_IDENTIFIER
    );

    // Read existing crontab
    let existing_crontab = match Command::new("crontab").arg("-l").output() {
        Ok(output) if output.status.success() => String::from_utf8_lossy(&output.stdout).to_string(),
        _ => String::new(),
    };

    // Remove existing Dbx-Backup entries if present
    // Matches entire line ending with # Dbx-Backup identifier
    let re = Regex::new(r"(?m)^\s*.*#\s*Dbx-Backup\s*$\s*").map_err(|e| format!("Regex error: {}", e))?;
    let mut updated_crontab = re.replace_all(&existing_crontab, "").to_string();

    // Add new entry
    updated_crontab.push_str(&cron_entry);

    // Write back to crontab
    let mut child = Command::new("crontab")
        .arg("-")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn crontab process: {}", e))?;

    if let Some(stdin) = child.stdin.as_mut() {
        use std::io::Write;
        stdin.write_all(updated_crontab.as_bytes())
            .map_err(|e| format!("Failed to write to crontab: {}", e))?;
    }

    let status = child.wait()
        .map_err(|e| format!("Failed to wait for crontab process: {}", e))?;

    if !status.success() {
        return Err("Failed to update crontab".to_string());
    }

    info!("Backup cron installed with schedule: {}", schedule);
    Ok(())
}

/// Remove Dbx-Backup crontab entry
pub fn remove_backup_cron() -> Result<(), String> {
    let existing_crontab = match Command::new("crontab").arg("-l").output() {
        Ok(output) if output.status.success() => String::from_utf8_lossy(&output.stdout).to_string(),
        _ => {
            info!("No existing crontab found, nothing to remove");
            return Ok(());
        }
    };

    // Remove existing Dbx-Backup entries
    // Matches entire line ending with # Dbx-Backup identifier
    let re = Regex::new(r"(?m)^\s*.*#\s*Dbx-Backup\s*$\s*").map_err(|e| format!("Regex error: {}", e))?;
    let updated_crontab = re.replace_all(&existing_crontab, "").to_string();

    // If same as original, nothing changed
    if updated_crontab == existing_crontab {
        info!("No Dbx-Backup cron entry found to remove");
        return Ok(());
    }

    // Write back to crontab
    let mut child = Command::new("crontab")
        .arg("-")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn crontab process: {}", e))?;

    if let Some(stdin) = child.stdin.as_mut() {
        use std::io::Write;
        stdin.write_all(updated_crontab.as_bytes())
            .map_err(|e| format!("Failed to write to crontab: {}", e))?;
    }

    let status = child.wait()
        .map_err(|e| format!("Failed to wait for crontab process: {}", e))?;

    if !status.success() {
        return Err("Failed to update crontab".to_string());
    }

    info!("Backup cron entry removed");
    Ok(())
}

/// Initialize cron on application startup - checks and installs if missing
pub async fn init_backup_cron(config_state: ConfigState) {
    let config = config_state.data.read().await;
    let config_path = get_user_config_path();
    if let Some(schedule) = &config.cron_schedule {
        if schedule.is_empty() {
            info!("No cron schedule configured, skipping cron initialization");
            return;
        }

        match check_backup_cron() {
            Ok(exists) => {
                if !exists {
                    info!("Dbx-Backup cron not found, installing...");
                    if let Err(e) = install_backup_cron(schedule, config_path.to_str().unwrap()) {
                        error!("Failed to install backup cron: {}", e);
                    }
                } else {
                    info!("Dbx-Backup cron already exists");
                }
            }
            Err(e) => {
                warn!("Could not check cron status: {}", e);
            }
        }
    } else {
        info!("Cron schedule not set in config, skipping cron initialization");
    }
}

/// Update cron schedule when config changes
pub async fn update_cron_schedule(config_state: ConfigState) {
    let config = config_state.data.read().await;
    let config_path = get_user_config_path();

    if let Some(schedule) = &config.cron_schedule {
        if schedule.is_empty() {
            let _ = remove_backup_cron();
        } else {
            if let Err(e) = install_backup_cron(schedule, config_path.to_str().unwrap()) {
                error!("Failed to update backup cron: {}", e);
            }
        }
    } else {
        let _ = remove_backup_cron();
    }
}