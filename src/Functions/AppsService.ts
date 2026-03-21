import { invoke } from "@tauri-apps/api/core";

export async function get_exported_apps(): Promise<Record<string, string[]>> {
  return await invoke("get_exported_apps");
}
