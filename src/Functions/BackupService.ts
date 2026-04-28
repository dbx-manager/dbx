import { invoke } from "@tauri-apps/api/core";

/**
 * Triggers a backup of a container.
 * @param containerId The ID of the container to back up.
 * @param backupDir The directory where the backup tar file should be stored.
 * @returns The full path to the created backup file.
 */
export async function backupContainer(containerId: string, backupDir: string): Promise<string> {
    // The backend Tauri command is `backup_container`
    return await invoke<string>("backup_container", { containerId, backupDir });
}

/**
 * Deletes a backup file.
 * @param backupPath Full path to the backup file.
 */
export async function deleteBackup(backupPath: string): Promise<void> {
    await invoke<void>("delete_backup", { backupPath });
}

/**
 * Recreates a container from a backup tar.
 * @param backupPath Full path to the backup file.
 * @param newContainerName Desired name for the new container.
 * @returns The name of the newly created container.
 */
export async function recreateContainerFromBackup(backupPath: string, newContainerName: string): Promise<string> {
    return await invoke<string>("recreate_container_from_backup", { backupPath, newContainerName });
}
