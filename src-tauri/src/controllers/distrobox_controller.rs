use crate::structs::new_container_request::NewContainerRequest;
use tokio::process::Command;

pub async fn create_distrobox_container(request: NewContainerRequest) -> Result<(), String> {
    let mut cmd = Command::new("distrobox");
    cmd.arg("create");

    // Add container name
    cmd.arg("--name").arg(&request.container_name);

    // Add home directory if not empty
    if !request.home_directory.is_empty() {
        cmd.arg("--home").arg(&request.home_directory);
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
