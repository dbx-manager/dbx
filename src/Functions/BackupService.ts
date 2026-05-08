import { invoke } from "@tauri-apps/api/core";

export async function postCreateBackup(containerId:String) {
    return await invoke("create_backup",{containerId:containerId});
}
export async function postRestoreBackup(tar_file_path:String) {
    return await invoke("restore_backup",{tarFilePath:tar_file_path});
}
export async function delete_backup(tar_file_path:String) {
    return await invoke("delete_backup",{tarFilePath:tar_file_path});
}