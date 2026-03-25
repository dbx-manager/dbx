import { invoke } from "@tauri-apps/api/core";

export async function get_exported_apps(): Promise<ExportedApps[]> {
  return await invoke("get_exported_apps");
}
export async function get_system_apps(containerId: string): Promise<string[]> {
  return await invoke("get_system_apps", { containerId: containerId });
}
