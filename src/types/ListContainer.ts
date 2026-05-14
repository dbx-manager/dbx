// this name is misleading , this name is the same that is used in the backend (podman-api) that represent a single contaienr
// and i keept it the same so no confusion is happening betwean the type and the name betwean the backend and the frontend
export interface ExportableApp {
  app_name: string;
  sub_apps: string[];
}

export interface Container {
  name: string;
  id: string;
  state: string;
  exported_apps: ExportableApp[];
  exportable_apps: ExportableApp[] | null;
  system_apps: string[];
  autobackup: boolean;
  autostart: boolean;
  performance: Performance;
}

export interface Performance {
  avg_cpu: number;
  avg_memory: number;
  perc_cpu: number;
  perc_memory: number;
}
