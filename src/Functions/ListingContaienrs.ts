import { invoke } from "@tauri-apps/api/core";
import { Container } from "../types/ListContainer";

export async function  list_containers() {
    let contianersJson;
    contianersJson = await invoke<Container[]>("get_container_list");
    // console.log(contianersJson["Ubuntu"])
    return contianersJson
}
