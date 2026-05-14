import { invoke } from "@tauri-apps/api/core";

export async function get_exported_apps() {
  await invoke("get_exported_apps");
}
export async function get_system_apps(containerId: string){
  await invoke("get_system_apps", { containerId: containerId });
}
export async function get_export_an_app_list(containerId: string){
  await invoke("get_export_an_app_list", { containerId: containerId });
}
export async function delete_an_exported_app(containerId: string,apps:Set<string>){
  await invoke("delete_exported_an_app", { containerId: containerId,appNames:Array.from(apps) });
}
export async function export_an_app(containerId: string,desktopFile:string){
  await invoke("export_an_app", { containerId: containerId,appName:desktopFile });
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