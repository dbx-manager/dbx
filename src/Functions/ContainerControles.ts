import { invoke } from "@tauri-apps/api/core";

export async function start_container(containerId:string) {
    await  invoke('start_container',{id:containerId});
}
export async function stop_container(containerId:string) {
    await  invoke('stop_container',{id:containerId});
}
export async function pause_container(containerId:string) {
    await  invoke('pause_container',{id:containerId});
}
export async function unpause_container(containerId:string) {
    await  invoke('unpause_container',{id:containerId});
}