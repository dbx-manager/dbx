import { invoke } from "@tauri-apps/api/core";

export async function get_exported_apps() {
  await invoke("get_exported_apps");
}
export async function get_system_apps(containerId: string){
  await invoke("get_system_apps", { containerId: containerId });
}

// Backup Controls
export async function create_backup(containerId: string) {
  return await invoke("create_backup", { containerId });
}
export async function delete_backup(filePath: string) {
  return await invoke("delete_backup", { filePath });
}
export async function list_backups_for_container(containerId: string) {
  return await invoke("list_backups_for_container", { containerId });
}
export async function get_latest_backup(containerId: string) {
  return await invoke("get_latest_backup", { containerId });
}
export async function restore_container_from_backup(containerId: string, backupFile: string) {
  return await invoke("restore_container_from_backup", { containerId, backupFile });
}