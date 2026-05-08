use std::ops::Add;

use crate::structs::config::Config;
use crate::structs::new_container_request::NewContainerRequest;
use crate::structs::socket::PodmanSocket;
use podman_api::opts::ContainerCommitOpts;
use tokio::process::Command;

pub async fn create_distrobox_container(request: NewContainerRequest) -> Result<(), String> {
    let mut cmd = _assemble_command_from_create_request(request);
    let output = cmd
        .output()
        .await
        .map_err(|e| format!("Failed to execute distrobox create: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("distrobox create failed: {}", stderr));
    }

    Ok(())
}

pub async fn restore_distrobox_container(tar_path: String) -> Result<(), String> {
    // Load the image via podman socket
    let podman = PodmanSocket::get_instance().await;
    // podman_api's load expects a reader; we use the convenience method that takes a path.
    // The ImageLoadOpts struct allows us to specify the input file.
    let tar_file = std::fs::read(&tar_path).unwrap();
    let image = podman
        .socket
        .images()
        .load(&tar_file)
        .await
        .map_err(|e| format!("Failed to load image from {}: {}", &tar_path, e))?;
    let loaded_image_name = image.names.unwrap().first().unwrap().clone();

    // Build the distrobox create command, reusing existing logic but adding the image.
    let clean_name=extract_image_name(loaded_image_name.clone().as_str()).unwrap();
    let new_container_request: NewContainerRequest = NewContainerRequest {
        container_name: clean_name,
        home_directory: String::new(),
        base_image: loaded_image_name.clone(),
        unshare_devsys: false,
        unshare_groups: false,
        unshare_ipc: false,
        unshare_netns: false,
        unshare_process: false,
        unshare_all: false,
    };
    match create_distrobox_container(new_container_request).await {
        Ok(_) => println!("OK"),
        Err(e) => println!("{}", e),
    };

    Ok(())
}

pub async fn create_container_backup(container_id: String, config: Config) -> Result<(), String> {
    let backup_dir = config.backup_location_path.clone();
    let podman = PodmanSocket::get_instance().await;
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M").to_string();
    // let image_name = format!("backup-{}-{}", container_id, &timestamp);
    // let tar_filename = format!("{}.tar", &image_name);
    // let tar_path = std::path::Path::new(&backup_dir).join(&tar_filename);

    let container = podman.socket.containers().get(&container_id);
    let container_name = container.inspect().await.unwrap().name.unwrap();

    // Commit the container to an image
    match container
        .commit(
            &ContainerCommitOpts::builder()
                .pause(true)
                .repo("dbx-manual-backup")
                .tag(&timestamp)
                .build(),
        )
        .await
    {
        Ok(_) => {}
        Err(e) => println!("Failed to execute podman commit: {}", e),
    };

    // Save the image to a tar file at the backup path
    // the podman-api crate dosnt implemint save method so we'r stuck with cmd
    let save_output = Command::new("podman")
        .args([
            "save",
            "-o",
            &backup_dir
                .add(format!("{}-{}.tar", &container_name, &timestamp).as_str())
                .as_str(),
            format!("dbx-manual-backup/{}:{}", &container_name, &timestamp).as_str(),
        ])
        .output()
        .await
        .map_err(|e| format!("Failed to execute podman save: {}", e))?;

    if !save_output.status.success() {
        let stderr = String::from_utf8_lossy(&save_output.stderr);
        return Err(format!("podman save failed: {}", stderr));
    }

    Ok(())
}

pub fn delete_backup_tarfile(path: String) -> Result<(), String> {
    if path.is_empty() {
        return Err("Path cannot be empty".to_string());
    }

    std::fs::remove_file(&path).map_err(|e| format!("Failed to delete file '{}': {}", path, e))
}

fn _assemble_command_from_create_request(request: NewContainerRequest) -> Command {
    let mut cmd = Command::new("distrobox");
    cmd.arg("create");

    // Add container name
    cmd.arg("--name").arg(&request.container_name);

    // Add home directory if not empty
    if !request.home_directory.is_empty() {
        cmd.arg("--home").arg(&request.home_directory);
    }
    if !request.base_image.is_empty() {
        cmd.arg("--image").arg(&request.base_image);
    }

    // Add unshare flags based on boolean values
    if request.unshare_devsys {
        cmd.arg("--unshare-devsys");
    }
    if request.unshare_groups {
        cmd.arg("--unshare-groups");
    }
    if request.unshare_ipc {
        cmd.arg("--unshare-ipc");
    }
    if request.unshare_netns {
        cmd.arg("--unshare-netns");
    }
    if request.unshare_process {
        cmd.arg("--unshare-process");
    }
    if request.unshare_all {
        cmd.arg("--unshare-all");
    }
    return cmd;
}

fn extract_image_name(text: &str) -> Option<String> {
    let start = text.find('/')? + 1;
    let end = text.find(':')?;
    if end > start {
        Some(text[start..end].to_string().add("-restore"))
    } else {
        None
    }
}