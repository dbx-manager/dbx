#[derive(Clone, Debug,serde::Serialize, serde::Deserialize)]
pub struct NewContainerRequest {
  pub container_name: String,
  pub home_directory: String,
  pub unshare_devsys: bool,
  pub unshare_groups: bool,
  pub unshare_ipc: bool,
  pub unshare_netns: bool,
  pub unshare_process: bool,
  pub unshare_all: bool,
}
