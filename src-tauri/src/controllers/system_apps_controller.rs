use crate::scripts::list_apps::LIST_APPS_SCRIPT;
use crate::structs::container::ContainerList;
use crate::structs::socket::PodmanSocket;
use futures_util::StreamExt;
use tauri::State;

// Function to execute the script fetch system packages in a container using podman-api
pub async fn parse_container_system_packages(
    container_id: String,
    state: State<'_, ContainerList>,
) -> Result<bool, String> {
    let container_list = state.containers.read().await;
    let container = container_list
        .values()
        .find(|c| c.id == Some(container_id.clone()))
        .unwrap();
    let container_key = match &container.name {
        Some(name) => name.clone(),
        None => return Err(String::from("container has no name")),
    };
    drop(container_list);
    // Get the Podman instance
    let podman = PodmanSocket::get_instance().await.socket.clone();

    let container = podman.containers().get(container_id);
    //TODO: add error checking and error handiling
    let _=container
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
    let mut chunks: String = String::new();

    while let Some(result) = scriptstream.next().await {
        let chunk = result.unwrap();
        let text = String::from_utf8(chunk.to_ascii_lowercase()).unwrap();
        chunks += text.as_str();
    }
    let system_packeges: Vec<String> = chunks.lines().map(|s| s.to_string()).collect();
    let mut write_state = state.containers.write().await;

    if let Some(container) = write_state.get_mut(&container_key) {
        container.system_apps = Some(system_packeges.clone());
    }

    Ok(true)
    // Ok(chunks.lines().map(|s| s.to_string()).collect())
}
