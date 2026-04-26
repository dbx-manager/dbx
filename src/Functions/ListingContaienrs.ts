import { invoke } from "@tauri-apps/api/core";
import { Container } from "../types/ListContainer";

export async function  list_containers() {
    return  await invoke<Container[]>("get_container_list");   
}
