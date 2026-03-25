use podman_api::opts::ContainerListOpts;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path, sync::Arc};
use tokio::sync::RwLock;

use crate::classes::socket::PodmanSocket;

// New struct to represent exported apps for a container
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExportedAppContainer {
    pub id: String,
    pub name: String,
    pub exported_apps: Vec<String>,
}

// AppState structure for Tauri managed state
pub struct ExportedAppsState {
    pub data: Arc<RwLock<Vec<ExportedAppContainer>>>,
}

impl ExportedAppsState {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

// Background task function that updates exported apps data
pub async fn start_exported_apps_monitoring(
    exported_apps_state: Arc<RwLock<Vec<ExportedAppContainer>>>,
) {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(10));
    loop {
        interval.tick().await;

        let new_data = fetch_exported_apps_data().await;

        let mut data_lock = exported_apps_state.write().await;
        *data_lock = new_data;
    }
}

// Helper function to fetch exported apps data
pub async fn fetch_exported_apps_data() -> Vec<ExportedAppContainer> {
    parse_distrobox_apps(Path::new("/home/ag/.local/share/applications")).await
}

pub async fn parse_distrobox_apps(dir: &Path) -> Vec<ExportedAppContainer> {
    //the result that will be returned , its the processed version of apps
    let mut exported_apps: Vec<ExportedAppContainer> = Vec::new();
    //hold the temporary std:in output as string before processing 
    let mut apps: HashMap<String, Vec<String>> = HashMap::new();
    //holding all the info of the containers that are regestered
    let mut container_map: HashMap<String, String> = HashMap::new();
    async {
        //getting the containers fromt he podman-api
        let containers = PodmanSocket::get_instance()
            .await
            .socket
            .containers()
            .list(&ContainerListOpts::builder().all(true).build())
            .await;
        for container in containers.unwrap() {
            container_map.insert(container.names.unwrap().first().unwrap().clone(), container.id.unwrap());
        }
    }.await;

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
                        apps.entry(container).or_insert_with(Vec::new).push( format!("{x}:{y}",x=app,y=path.file_name().unwrap().to_str().unwrap().replace(".desktop", "")));
                        }
                    }
                }
            }
        }
    }

    //conver from hashmap to the needed structure for return 
    for( container_name,container_id) in container_map{
        exported_apps.push(ExportedAppContainer {
            id: container_id, 
            name: container_name.clone(),
            exported_apps: apps.get(&container_name).unwrap_or(&Vec::new()).clone(),
        });
    }
    exported_apps
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
                        // Skip until we find the actual app command after '--' or direct command
                        //  Find the first argument that looks like an absolute path or command
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
