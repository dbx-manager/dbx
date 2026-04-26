import { invoke } from "@tauri-apps/api/core";

export async function postCreateNewContainer(container_request:NewContainerRequest) {
    return await invoke("create_new_container",{request:container_request});
}