use dirs;
use std::fs;
use tokio::time::{interval, Duration};
use crate::structs::config::Config;
use crate::structs::config::ConfigState;


pub fn load_config() -> Config {
    // Try to load from user config directory first
    if let Some(mut user_config) = load_from_user_config() {
        // Also load autostart containers from the autostart script
        user_config.autostart_containers = load_autostart_containers();
        
        return user_config;
    }

    // If user config doesn't exist, copy from sample and use that
    if let Some(sample_config) = load_from_sample() {
        print!(
            "Warning: no configuration file were found, writing new one configuration from sample"
        );
        // Try to copy to user config directory
        if copy_sample_to_user_config().is_ok() {
            // Also create autostart sample if it doesn't exist
            let _ = create_autostart_sample();
            return sample_config;
        }
    }

    // Fallback to sample config even if copy failed
    load_from_sample().unwrap()
}

pub fn load_from_user_config() -> Option<Config> {
    let user_config_path = get_user_config_path();

    if user_config_path.exists() {
        if let Ok(content) = fs::read_to_string(&user_config_path) {
            if let Ok(config) = serde_json::from_str(&content) {
                return Some(config);
            }
        }
    }
    None
}

pub fn load_from_sample() -> Option<Config> {
    return Some(Config {
        backup_containers: Vec::new(),
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
        autostart_containers: Vec::new(),
    });
}

pub fn copy_sample_to_user_config() -> Result<(), Box<dyn std::error::Error>> {
    let user_config_path = get_user_config_path();
    let user_confegeration_sample = load_from_sample().unwrap();
    // Create parent directory if it doesn't exist
    if let Some(parent) = user_config_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let e = fs::write(
        user_config_path.as_path(),
        serde_json::to_string_pretty(&user_confegeration_sample).unwrap(),
    );
    if e.is_err() {
        eprintln!(
            "Error adding sample configeration to default confegeration path: {}",
            e.err().unwrap()
        );
    }

    Ok(())
}

pub fn get_user_config_path() -> std::path::PathBuf {
    let mut config_dir = dirs::config_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    config_dir.push("dbx");
    config_dir.push("backup.conf");
    config_dir
}

pub fn get_autostart_script_path() -> std::path::PathBuf {
    let mut config_dir = dirs::config_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    config_dir.push("dbx");
    config_dir.push("autostart.sh");
    config_dir
}

pub fn get_autostart_desktop_path() -> std::path::PathBuf {
    let mut autostart_dir = dirs::config_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
    autostart_dir.push("autostart");
    autostart_dir.push("dbx-autostart.desktop");
    autostart_dir
}

pub fn load_autostart_containers() -> Vec<String> {
    let script_path = get_autostart_script_path();
    if !script_path.exists() {
        return Vec::new();
    }

    if let Ok(content) = fs::read_to_string(&script_path) {
        // Parse containers=("item1" "item2" ...) from the bash script
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("containers=(") {
                // Extract the content inside the parentheses
                if let Some(start) = trimmed.find('(') {
                    if let Some(end) = trimmed.rfind(')') {
                        let inner = &trimmed[start + 1..end];
                        // Parse quoted strings
                        let mut containers = Vec::new();
                        let mut current = String::new();
                        let mut in_quotes = false;
                        for ch in inner.chars() {
                            match ch {
                                '"' => {
                                    if in_quotes {
                                        if !current.is_empty() {
                                            containers.push(current.clone());
                                            current.clear();
                                        }
                                        in_quotes = false;
                                    } else {
                                        in_quotes = true;
                                    }
                                }
                                _ if in_quotes => current.push(ch),
                                _ => {}
                            }
                        }
                        return containers;
                    }
                }
            }
        }
    }
    Vec::new()
}

pub fn create_autostart_sample() -> Result<(), Box<dyn std::error::Error>> {
    let script_path = get_autostart_script_path();
    let desktop_path = get_autostart_desktop_path();

    // Create parent directories if they don't exist
    if let Some(parent) = script_path.parent() {
        fs::create_dir_all(parent)?;
    }
    if let Some(parent) = desktop_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Create the autostart bash script if it doesn't exist
    if !script_path.exists() {
        let script_content = r#"#!/bin/bash
containers=("" "")
for c in "${containers[@]}"; do
    if [ -n "$c" ]; then
        distrobox-enter "$c" -- /bin/true
    fi
done
"#;
        fs::write(&script_path, script_content)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&script_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&script_path, perms)?;
        }
    }

    // Create the .desktop entry if it doesn't exist
    if !desktop_path.exists() {
        let desktop_content = format!(
            r#"[Desktop Entry]
Type=Application
Name=DBX Autostart
Exec={}
Hidden=false
NoDisplay=false
X-GNOME-Autostart-enabled=true
"#,
            script_path.to_string_lossy()
        );
        fs::write(&desktop_path, desktop_content)?;
    }

    Ok(())
}

pub fn save_autostart_script(containers: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let script_path = get_autostart_script_path();

    // Create parent directory if it doesn't exist
    if let Some(parent) = script_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Build the containers array string
    let containers_array = containers
        .iter()
        .map(|c| format!("\"{}\"", c))
        .collect::<Vec<_>>()
        .join(" ");

    let script_content = format!(
        r#"#!/bin/bash
containers=({})
for c in "${{containers[@]}}"; do
    if [ -n "$c" ]; then
        distrobox-enter "$c" -- /bin/true
    fi
done
"#,
        containers_array
    );

    fs::write(&script_path, script_content)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&script_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms)?;
    }

    // Ensure the .desktop entry exists
    let desktop_path = get_autostart_desktop_path();
    if !desktop_path.exists() {
        if let Some(parent) = desktop_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let desktop_content = format!(
            r#"[Desktop Entry]
Type=Application
Name=DBX Autostart
Exec={}
Hidden=false
NoDisplay=false
X-GNOME-Autostart-enabled=true
"#,
            script_path.to_string_lossy()
        );
        fs::write(&desktop_path, desktop_content)?;
    }

    Ok(())
}

pub fn set_backup_config(new_config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let user_config_path = get_user_config_path();

    // Create parent directory if it doesn't exist
    if let Some(parent) = user_config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Serialize the config to JSON
    let config_json = serde_json::to_string_pretty(new_config)?;

    // Write the config to the user config file
    fs::write(&user_config_path, config_json)?;

    // Also save the autostart script with the current autostart containers
    save_autostart_script(&new_config.autostart_containers)?;

    Ok(())
}

// Background task function that monitors config file changes
pub async fn start_config_monitoring(config_state: ConfigState) {
    let mut interval = interval(Duration::from_secs(2)); // Check every 2 seconds
    let mut last_modified_time = std::time::UNIX_EPOCH;
    let mut last_autostart_modified_time = std::time::UNIX_EPOCH;

    loop {
        interval.tick().await;

        // Check if user config file has been modified
        let user_config_path = get_user_config_path();
        if user_config_path.exists() {
            if let Ok(metadata) = fs::metadata(&user_config_path) {
                let modified_time = metadata.modified().unwrap_or(std::time::UNIX_EPOCH);

                if modified_time > last_modified_time {
                    last_modified_time = modified_time;

                    // Reload config from user file
                    if let Some(mut new_config) = load_from_user_config() {
                        // Also reload autostart containers from script
                        new_config.autostart_containers = load_autostart_containers();
                        let mut config_lock = config_state.data.write().await;
                        *config_lock = new_config;
                    }
                }
            }
        }

        // Check if autostart script has been modified
        let autostart_script_path = get_autostart_script_path();
        if autostart_script_path.exists() {
            if let Ok(metadata) = fs::metadata(&autostart_script_path) {
                let modified_time = metadata.modified().unwrap_or(std::time::UNIX_EPOCH);

                if modified_time > last_autostart_modified_time {
                    last_autostart_modified_time = modified_time;

                    // Reload autostart containers from script
                    let containers = load_autostart_containers();
                    let mut config_lock = config_state.data.write().await;
                    config_lock.autostart_containers = containers;
                }
            }
        }
    }
}
