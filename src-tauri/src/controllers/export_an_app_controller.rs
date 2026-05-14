use crate::structs::container::{ContainerList, ExportableApp};
use crate::structs::socket::PodmanSocket;
use futures_util::StreamExt;
use tauri::State;

pub static LIST_EXPORTABLE_APPS_SCRIPT: &str = r#"#!/bin/bash

# Find all .desktop files in standard locations
find /usr/share/applications /usr/local/share/applications -name "*.desktop" 2>/dev/null | while read -r desktop_file; do
    filename=$(basename "$desktop_file" .desktop)

    # Get the main Name= entry
    main_name=$(grep -m1 '^Name=' "$desktop_file" 2>/dev/null | cut -d'=' -f2-)

    # Get the Exec= line to extract the executable basename
    exec_line=$(grep -m1 '^Exec=' "$desktop_file" 2>/dev/null | cut -d'=' -f2-)
    # Extract just the command basename (first word, strip path)
    executable=$(echo "$exec_line" | awk '{print $1}' | xargs basename 2>/dev/null)
    # If no executable could be extracted, fall back to the desktop filename
    if [ -z "$executable" ]; then
        executable="$filename"
    fi

    # Get Actions= entries (comma-separated list of action IDs)
    actions_line=$(grep -m1 '^Actions=' "$desktop_file" 2>/dev/null | cut -d'=' -f2-)

    # Collect all names: start with the main name
    names="$main_name"

    # If there are Actions, extract their Names
    if [ -n "$actions_line" ]; then
        IFS=';' read -ra actions <<< "$actions_line"
        for action in "${actions[@]}"; do
            action_name=$(grep -A2 "^\[Desktop Action $action\]" "$desktop_file" 2>/dev/null | grep '^Name=' | cut -d'=' -f2-)
            if [ -n "$action_name" ]; then
                names="$names;$action_name"
            fi
        done
    fi

    # Output format: executable:name1;name2;...
    echo "$executable:$names"
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
    // Format: "executable:name1;name2;..."
    // Group by executable so duplicate executables merge their names
    let mut exportable_apps: Vec<ExportableApp> = Vec::new();

    for line in output.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some(colon_pos) = line.find(':') {
            let executable = line[..colon_pos].to_string();
            let names_str = &line[colon_pos + 1..];

            let names: Vec<String> = names_str
                .split(';')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            if !executable.is_empty() && !names.is_empty() {
                // Check if this executable already exists in our list
                if let Some(existing) = exportable_apps
                    .iter_mut()
                    .find(|app: &&mut ExportableApp| app.app_name == executable)
                {
                    // Merge unique names that aren't already in sub_apps
                    for name in names {
                        if !existing.sub_apps.contains(&name) {
                            existing.sub_apps.push(name);
                        }
                    }
                } else {
                    exportable_apps.push(ExportableApp {
                        app_name: executable,
                        sub_apps: names,
                    });
                }
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

pub async fn delete_exported_apps(
    container_id: String,
    app_names: Vec<String>,
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

    // Run distrobox-enter --name <container> -- distrobox-export --app <app_name> --delete
    // for each app name, failing fast on the first error
    for app_name in &app_names {
        let output = Command::new("distrobox-enter")
            .arg("--name")
            .arg(&container_name)
            .arg("--")
            .arg("distrobox-export")
            .arg("--app")
            .arg(app_name)
            .arg("--delete")
            .output()
            .await
            .map_err(|e| format!("Failed to execute distrobox-enter: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to delete exported app '{}': {}", app_name, stderr));
        }
    }

    // Remove the deleted apps from the container's exported_apps in state
    let mut write_state = state.containers.write().await;
    if let Some(container) = write_state.get_mut(&container_name) {
        if let Some(ref mut exported_apps) = container.exported_apps {
            exported_apps.retain(|app| !app_names.contains(&app.app_name));
        }
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

    // Update the container's exported_apps in the state.
    // The exported desktop file will be re-parsed by parse_distrobox_apps on next refresh.
    // For now, insert a placeholder using the app_name as both key and sub_app.
    // On next refresh, parse_distrobox_apps will regroup by the actual executable.
    let mut write_state = state.containers.write().await;
    if let Some(container) = write_state.get_mut(&container_name) {
        container
            .exported_apps
            .get_or_insert_with(Vec::new)
            .push(ExportableApp {
                app_name: app_name.clone(),
                sub_apps: vec![app_name],
            });
    }

    Ok(true)
}