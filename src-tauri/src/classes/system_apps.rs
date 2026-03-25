use crate::scripts::list_apps::LIST_APPS_SCRIPT;
use futures_util::StreamExt;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// AppState structure for Tauri managed state

pub struct SystemAppsState {
    pub data: Arc<RwLock<HashMap<String, Vec<String>>>>,
}

impl SystemAppsState {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    pub async fn get_or_insert(&self, container_id: String) -> Vec<String> {
        // Try read lock first
        {
            let read_guard = self.data.read().unwrap();
            if let Some(value) = read_guard.get(&container_id) {
                return value.clone();
            }
        }

        // If not found, perform the async operation without holding any lock
        let value = execute_script_in_container(container_id.clone()).await;
        let apps = value.unwrap();

        // Now acquire write lock to insert the result
        let mut write_guard = self.data.write().unwrap();
        
        // Double-check after acquiring write lock (avoid race condition)
        if let Some(existing_value) = write_guard.get(&container_id) {
            return existing_value.clone();
        }

        write_guard.insert(container_id.to_string(), apps.clone());
        apps
    }
}
// Function to execute the script in a container using podman-api
pub async fn execute_script_in_container(container_id: String) -> Result<Vec<String>, ()> {
    // Get the Podman instance
    let podman = crate::classes::socket::PodmanSocket::get_instance()
        .await
        .socket
        .clone();
    let container = podman.containers().get(container_id);
    //TODO: add error checking and error handiling
    container
        .copy_file_into("/tmp/dbx_system_apps_script", LIST_APPS_SCRIPT.as_bytes())
        .await;
    let chmodexec = container
        .create_exec(
            &podman_api::opts::ExecCreateOpts::builder()
                // .command(["sh","-c","cat", ">" ,"/tmp/dbxSystemApp", "<<'EOF'", "&&", "chmod", "+x", "/tmp/dbxSystemApp",LIST_APPS_SCRIPT])
                .command(["sudo", "chmod", "+x", "/tmp/dbx_system_apps_script"])
                .attach_stdout(true)
                .attach_stdin(true)
                .attach_stderr(true)
                .build(),
        )
        .await
        .unwrap();
    let opts = Default::default();
    let mut chmodstream = chmodexec.start(&opts).await.unwrap().unwrap();

    while let Some(chunk) = chmodstream.next().await {
        println!(
            "{}",
            String::from_utf8(chunk.unwrap().to_ascii_lowercase()).unwrap()
        );
    }
    let scriptexec = container
        .create_exec(
            &podman_api::opts::ExecCreateOpts::builder()
                // .command(["sh","-c","cat", ">" ,"/tmp/dbxSystemApp", "<<'EOF'", "&&", "chmod", "+x", "/tmp/dbxSystemApp",LIST_APPS_SCRIPT])
                .command(["/tmp/dbx_system_apps_script"])
                .attach_stdout(true)
                .attach_stdin(true)
                .attach_stderr(true)
                .build(),
        )
        .await
        .unwrap();
    let opts = Default::default();
    let mut scriptstream = scriptexec.start(&opts).await.unwrap().unwrap();

    // while let Some(chunk) = scriptstream.next().await {
    //     println!(
    //         "{}",
    //         String::from_utf8(chunk.unwrap().to_ascii_lowercase()).unwrap()
    //     );
    // }
    let mut chunks :String=String::new();

while let Some(result) = scriptstream.next().await {
    let chunk = result.unwrap();
    let text = String::from_utf8(chunk.to_ascii_lowercase()).unwrap();
    chunks+=text.as_str();
}

    Ok(chunks.lines().map(|s| s.to_string()).collect())
}
