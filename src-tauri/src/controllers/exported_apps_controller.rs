use std::{fs, path::Path};

use tauri::State;

use crate::structs::container::{Container, ContainerList};


pub async fn parse_distrobox_apps(dir: &Path, state: State<'_, ContainerList>) {
    //hold the temporary std:in output as string before processing
    let mut container_list = state.containers.write().await;

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) != Some("desktop") {
                continue;
            }

            if let Ok(content) = fs::read_to_string(&path) {
                for line in content.lines() {
                    if let Some(exec_part) = line.strip_prefix("Exec=") {
                        if let Some((container, app)) = extract_container_and_app(exec_part) {
                            // if the container dosent exsist for the parsed app ,add a dummy one and add the exported app to it
                            container_list
                                .entry(container.clone())
                                .or_insert_with(|| Container {
                                    name: Some(container.clone()),
                                    id: None,
                                    state: None,
                                    exported_apps: None,
                                    system_apps: None,
                                    autobackup: None,
                                    autostart: None,
                                })
                                .exported_apps
                                .get_or_insert_with(Vec::new)
                                .push(format!(
                                    "{x}:{y}",
                                    x = app,
                                    y = path
                                        .file_name()
                                        .unwrap()
                                        .to_str()
                                        .unwrap()
                                        .replace(".desktop", "")
                                ));
                        }
                    }
                }
            }
        }
    }

    // //write the changes to the container status
    // let mut write_state: tokio::sync::RwLockWriteGuard<'_, HashMap<String, Container>> =
    //     state.containers.write().await;
    // *write_state = container_list;
}

fn extract_container_and_app(exec_line: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = exec_line.split_whitespace().collect();
    let mut iter = parts.iter();

    while let Some(&cmd) = iter.next() {
        if cmd == "/usr/bin/distrobox-enter" {
            // Look for -n <container_name>
            while let Some(&flag) = iter.next() {
                if flag == "-n" {
                    if let Some(&container) = iter.next() {
                        // Skip until we hit '--' or run out, then take the next as app
                        // Find the first argument that looks like an absolute path or command
                        // after the container name and optional '--'

                        // Consume args until we hit '--' or run out, then take the next as app
                        while let Some(&next) = iter.next() {
                            if next == "--" {
                                let app = iter.fold(String::new(), |mut acc: String, s: &&str| {
                                    acc = acc.split("/").last().unwrap().to_string();
                                    let tmp = s.split("/").last().unwrap();

                                    if acc.contains('%') {
                                        return s.to_string();
                                    }
                                    if s.contains('%') {
                                        return acc.to_string();
                                    }

                                    acc.push(' ');
                                    acc.push_str(tmp);
                                    acc
                                });
                                return Some((container.to_string(), app.to_string()));
                            } else {
                                //TODO: if there is no "--" then this should not be added to the list
                                continue;
                            }
                        }
                    }
                }
            }
        }
    }
    None
}