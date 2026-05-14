use std::collections::HashMap;
use std::{fs, path::Path};

use tauri::State;

use crate::structs::container::{Container, ContainerList, ExportableApp};


pub async fn parse_distrobox_apps(dir: &Path, state: State<'_, ContainerList>) {
    let mut container_list = state.containers.write().await;

    // Temporary HashMap to group apps by (container_name, executable_name) -> sub_apps (list of Name= values)
    let mut grouped_apps: HashMap<String, Vec<(String, Vec<String>)>> = HashMap::new();

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("desktop") {
                continue;
            }

            if let Ok(content) = fs::read_to_string(&path) {
                let mut app_name: Option<String> = None;
                let mut exec_line: Option<String> = None;

                // Parse the .desktop file for Name= and Exec=
                for line in content.lines() {
                    if let Some(name_val) = line.strip_prefix("Name=") {
                        app_name = Some(name_val.to_string());
                    } else if let Some(exec_val) = line.strip_prefix("Exec=") {
                        exec_line = Some(exec_val.to_string());
                    }
                }

                // Get the .desktop file's basename without extension (e.g., "krita" from "krita.desktop")
                let file_stem = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string();

                // If we have both Name= and Exec=, try to find the container
                if let (Some(name_val), Some(exec_val)) = (app_name, exec_line) {
                    if let Some((container, executable)) = extract_container_and_executable(&exec_val) {
                        // Append the file stem to the Name= value
                        let sub_app_entry = format!("{}|{}", name_val, file_stem);

                        // Group by container name first, then by executable name
                        let container_entry = grouped_apps
                            .entry(container.clone())
                            .or_insert_with(Vec::new);

                        // Check if this executable already exists in the container's list
                        if let Some(existing) = container_entry
                            .iter_mut()
                            .find(|(exec_name, _)| *exec_name == executable)
                        {
                            // Executable already exists, add the sub_app_entry to its sub_apps
                            existing.1.push(sub_app_entry);
                        } else {
                            // New executable for this container
                            container_entry.push((executable, vec![sub_app_entry]));
                        }
                    }
                }
            }
        }
    }

    // Write the grouped data into the container_list state
    for (container_name, apps) in grouped_apps {
        let exported_apps: Vec<ExportableApp> = apps
            .into_iter()
            .map(|(executable, sub_apps)| ExportableApp {
                app_name: executable,
                sub_apps,
            })
            .collect();

        container_list
            .entry(container_name.clone())
            .or_insert_with(|| Container {
                name: Some(container_name.clone()),
                id: None,
                state: None,
                exported_apps: None,
                system_apps: None,
                exportable_apps: None,
                autobackup: None,
                autostart: None,
                performance: None,
            })
            .exported_apps = Some(exported_apps);
    }
}

/// Extracts the container name and the executable basename from a distrobox-enter Exec= line.
///
/// For example, given:
///   Exec=/usr/bin/distrobox-enter  -n Ubuntu  --   krita  %F
/// returns Some(("Ubuntu", "krita"))
fn extract_container_and_executable(exec_line: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = exec_line.split_whitespace().collect();
    let mut iter = parts.iter();

    while let Some(&cmd) = iter.next() {
        if cmd == "/usr/bin/distrobox-enter" {
            // Look for -n <container_name>
            while let Some(&flag) = iter.next() {
                if flag == "-n" {
                    if let Some(&container) = iter.next() {
                        // Skip until we hit '--' (the separator before the actual command)
                        while let Some(&next) = iter.next() {
                            if next == "--" {
                                // The first argument after '--' that is not a %-placeholder is the executable
                                if let Some(&exec_arg) = iter.next() {
                                    // Strip path prefix to get the basename
                                    let basename = exec_arg
                                        .split('/')
                                        .last()
                                        .unwrap_or(exec_arg)
                                        .to_string();
                                    return Some((container.to_string(), basename));
                                }
                            }
                        }
                        // Reached end without finding '--', this is not a valid exported app
                        return None;
                    }
                }
            }
        }
    }
    None
}