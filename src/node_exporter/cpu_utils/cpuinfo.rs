use serde::{Deserialize, Serialize};


struct CPUStat {
    usage: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CPUInfo {
    // cpu 型号
    model_name: String,

    // 核心数量
    core_count: u32,
    // cpu 状态信息
    cpu_stat: Vec<CPUStat>,
}