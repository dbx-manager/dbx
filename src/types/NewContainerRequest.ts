interface NewContainerRequest {
  container_name: string;
  home_directory: string;
  unshare_devsys: boolean;
  unshare_groups: boolean;
  unshare_ipc: boolean;
  unshare_netns: boolean;
  unshare_process: boolean;
  unshare_all: boolean;
}
