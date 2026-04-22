// this name is misleading , this name is the same that is used in the backend (podman-api) that represent a single contaienr
// and i keept it the same so no confusion is happening betwean the type and the name betwean the backend and the frontend
export interface Container {
  name: string;
  id: string;
  state: string;
  exported_apps: string[];
  system_apps: string[];
  autobackup: boolean;
  autostart: boolean;
}
