
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Performance{
    pub avg_cpu:f64,
    pub avg_memory:f64, 
    pub perc_cpu:f64,
    pub perc_memory:f64,
}