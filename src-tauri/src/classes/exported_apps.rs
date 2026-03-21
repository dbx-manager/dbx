use std::{collections::HashMap, fs, path::Path, sync::Arc};
use tokio::sync::OnceCell;

pub struct ExportedAppsList {
    pub apps: HashMap<String, Vec<String>>,
}

static SINGLE: OnceCell<Arc<ExportedAppsList>> = OnceCell::const_new();

impl ExportedAppsList {
    async fn init() -> Arc<ExportedAppsList> {
        let data = self_fetch_data_async().await;
        Arc::new(ExportedAppsList { apps: data })
    }

    pub async fn get_instance() -> Arc<ExportedAppsList> {
        SINGLE.get_or_init(Self::init).await.clone()
    }
}

//this is the "constructor" for the singleton
async fn self_fetch_data_async() -> HashMap<String, Vec<String>> {
    parse_distrobox_apps(Path::new("/home/ag/.local/share/applications"))
}

pub fn parse_distrobox_apps(dir: &Path) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
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
                            map.entry(container).or_insert_with(Vec::new).push( format!("{x}:{y}",x=app,y=path.file_name().unwrap().to_str().unwrap().replace(".desktop", "")));
                        }
                    }
                }
            }
        }
    }

    map
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
                                    let app = iter.fold(String::new(), |mut acc:String, s: &&str| {
                                    acc=acc.split("/").last().unwrap().to_string();
                                    let tmp=s.split("/").last().unwrap();

                                    if acc.contains('%'){
                                        return s.to_string();
                                    }
                                    if s.contains('%'){
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
