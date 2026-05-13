use crate::structs::container::{ContainerList, ExportableApp};
use crate::structs::socket::PodmanSocket;
use futures_util::StreamExt;
use tauri::State;

pub static LIST_EXPORTABLE_APPS_SCRIPT: &str = r#"#!/bin/bash

# Find all .desktop files in standard locations
find /usr/share/applications /usr/local/share/applications  -name "*.desktop" 2>/dev/null | while read -r desktop_file; do
    filename=$(basename "$desktop_file" .desktop)
    
    # Get the main Name= entry
    main_name=$(grep -m1 '^Name=' "$desktop_file" 2>/dev/null | cut -d'=' -f2-)
    
    # Get Actions= entries (comma-separated list of action IDs)
    actions_line=$(grep -m1 '^Actions=' "$desktop_file" 2>/dev/null | cut -d'=' -f2-)
    
    # Start with the main name as the first sub-app
    sub_apps="$main_name"
    
    # If there are Actions, extract their Names as sub-apps
    if [ -n "$actions_line" ]; then
        IFS=';' read -ra actions <<< "$actions_line"
        for action in "${actions[@]}"; do
            action_name=$(grep -A2 "^\[Desktop Action $action\]" "$desktop_file" 2>/dev/null | grep '^Name=' | cut -d'=' -f2-)
            if [ -n "$action_name" ]; then
                sub_apps="$sub_apps;$action_name"
            fi
        done
    fi
    
    echo "$main_name:$sub_apps"
done
"#;

pub async fn parse_container_exportable_apps(
    container_id: String,
    state: State<'_, ContainerList>,
) -> Result<bool, String> {
    let container_list = state.containers.read().await;
    let container = container_list
        .values()
        .find(|c| c.id == Some(container_id.clone()))
        .ok_or_else(|| String::from("container not found"))?;
    let container_key = match &container.name {
        Some(name) => name.clone(),
        None => return Err(String::from("container has no name")),
    };
    drop(container_list);

    // Get the Podman instance
    let podman = PodmanSocket::get_instance().await.socket.clone();
    let podman_container = podman.containers().get(container_id);
    
    // Copy the script into the container   
    let _ = podman_container
        .copy_file_into("/tmp/dbx_list_exportable_apps", LIST_EXPORTABLE_APPS_SCRIPT.as_bytes())
        .await;

    // Make the script executable
    let chmod_exec = podman_container
        .create_exec(
            &podman_api::opts::ExecCreateOpts::builder()
                .command(["chmod", "+x", "/tmp/dbx_list_exportable_apps"])
                .attach_stdout(true)
                .attach_stderr(true)
                .build(),
        )
        .await
        .map_err(|e| format!("Failed to create chmod exec: {}", e))?;

    let opts = Default::default();
    let mut chmod_stream = chmod_exec
        .start(&opts)
        .await
        .map_err(|e| format!("Failed to start chmod exec: {}", e))?
        .ok_or_else(|| String::from("chmod exec returned no stream"))?;

    while let Some(chunk) = chmod_stream.next().await {
        let _ = chunk;
    }

    // Run the script
    let script_exec = podman_container
        .create_exec(
            &podman_api::opts::ExecCreateOpts::builder()
                .command(["/tmp/dbx_list_exportable_apps"])
                .attach_stdout(true)
                .attach_stderr(true)
                .build(),
        )
        .await
        .map_err(|e| format!("Failed to create script exec: {}", e))?;

    let opts = Default::default();
    let mut script_stream = script_exec
        .start(&opts)
        .await
        .map_err(|e| format!("Failed to start script exec: {}", e))?
        .ok_or_else(|| String::from("script exec returned no stream"))?;

    let mut output = String::new();
    while let Some(result) = script_stream.next().await {
        let chunk = result.map_err(|e| format!("Failed to read script output: {}", e))?;
        let text = String::from_utf8(chunk.to_vec())
            .map_err(|e| format!("Failed to parse script output as UTF-8: {}", e))?;
        output.push_str(&text);
    }

    // Parse the output into Vec<ExportableApp>
    let mut exportable_apps: Vec<ExportableApp> = Vec::new();

    for line in output.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Format: "appname:subapp1;subapp2;..."
        if let Some(colon_pos) = line.find(':') {
            let app_name = line[..colon_pos].to_string();
            let sub_apps_str = &line[colon_pos + 1..];

            let sub_apps: Vec<String> = sub_apps_str
                .split(';')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            if !app_name.is_empty() && !sub_apps.is_empty() {
                exportable_apps.push(ExportableApp {
                    app_name,
                    sub_apps,
                });
            }
        }
    }

    // Store in the container state
    let mut write_state = state.containers.write().await;
    if let Some(container) = write_state.get_mut(&container_key) {
        container.exportable_apps = Some(exportable_apps);
    }

    Ok(true)
}

pub async fn export_app_to_host(
    container_id: String,
    app_name: String,
    state: State<'_, ContainerList>,
) -> Result<bool, String> {
    use tokio::process::Command;

    // Find the container in the state
    let container_list = state.containers.read().await;
    let container = container_list
        .values()
        .find(|c| c.id == Some(container_id.clone()))
        .ok_or_else(|| String::from("container not found"))?;
    let container_name = match &container.name {
        Some(name) => name.clone(),
        None => return Err(String::from("container has no name")),
    };
    drop(container_list);

    // Run distrobox-enter --name <container> -- distrobox-export --app <app_name>
    let output = Command::new("distrobox-enter")
        .arg("--name")
        .arg(&container_name)
        .arg("--")
        .arg("distrobox-export")
        .arg("--app")
        .arg(&app_name)
        .output()
        .await
        .map_err(|e| format!("Failed to execute distrobox-enter: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("distrobox-export failed: {}", stderr));
    }

    // Update the container's exported_apps in the state
    let mut write_state = state.containers.write().await;
    if let Some(container) = write_state.get_mut(&container_name) {
        container
            .exported_apps
            .get_or_insert_with(Vec::new)
            .push(format!("{}:{}", app_name, app_name));
    }

    Ok(true)
}