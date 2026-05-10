use podman_api::Podman;

use crate::structs::performance::Performance;
use crate::structs::{container::ContainerList, socket::PodmanSocket};

#[tauri::command]
pub async fn match_performance(
    container_state: tauri::State<'_, ContainerList>,
) -> Result<(), String> {
    let podman_socket = PodmanSocket::get_instance().await;
    let mut containerstate = container_state.containers.write().await;
    for c in containerstate.values_mut() {
        if let Some(id) = &c.id {
            match performance_getter(&podman_socket.socket, id.clone()).await {
                Ok(k) => c.performance = Some(k),
                Err(_) => {}
            };
        }
    }
    Ok(())
}
async fn performance_getter(
    podman_socket: &Podman,
    container_id: String,
) -> Result<Performance,()> {
    let stats = podman_socket.containers().get(container_id).stats().await;
    match stats {
        Ok(k) => {
            let stats_array = k["Stats"].as_array().and_then(|arr| arr.first());
            match stats_array {
                Some(first_stats) => Ok(Performance {
                    avg_cpu: first_stats["AvgCPU"].as_f64().unwrap_or(0.0),
                    avg_memory: first_stats["MemUsage"].as_f64().unwrap_or(0.0),
                    perc_memory:first_stats["MemPerc"].as_f64().unwrap_or(0.0),
                    perc_cpu: first_stats["CPU"].as_f64().unwrap_or(0.0),
                }),
                None => Err(()),
            }
        }
        Err(_) => Err(()),
    }
}
