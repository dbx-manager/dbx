import { invoke } from "@tauri-apps/api/core";
import { ListContainer } from "../Types/ListContainer";

export async function  list_containers() {
    let contianersJson;
    contianersJson = await invoke<ListContainer[]>("get_container_list");
    console.log(contianersJson[0])
    return contianersJson
}
