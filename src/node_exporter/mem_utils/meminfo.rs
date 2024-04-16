use std::fs::File;
use std::io::{BufRead, BufReader};
use serde::{Deserialize, Serialize};

// 定义一个名为 MemInfo 的结构体，用于存储 Linux 系统的内存信息
#[derive(Debug, Serialize, Deserialize)]
pub struct MemInfo {
    // 总内存（单位：KB）
    pub total: u64,
    // 可用内存（单位：KB）
    pub free: u64,
    // 缓冲区占用的内存（单位：KB）
    pub buffers: u64,
    // 缓存占用的内存（单位：KB）
    pub cached: u64,
}

// 为 MemInfo 结构体实现一个方法，从 /proc/meminfo 文件中获取内存信息
impl MemInfo {
    // 从 /proc/meminfo 文件中解析内存信息并返回 MemInfo 实例
    pub(crate) fn init() -> Result<MemInfo, Box<dyn std::error::Error>> {
        // 打开 /proc/meminfo 文件
        let file = File::open("/proc/meminfo")?;

        // 创建一个缓冲读取器以高效读取文件内容
        let reader = BufReader::new(file);

        // 初始化内存信息变量
        let mut total: u64 = 0;
        let mut free: u64 = 0;
        let mut buffers: u64 = 0;
        let mut cached: u64 = 0;

        // 遍历每一行
        for line in reader.lines() {
            // 解析每一行的内容
            let line_content = line?;
            let mut parts = line_content.split_whitespace();

            // 提取键（如 MemTotal、MemFree 等）
            let key = parts.next().expect("Invalid line in /proc/meminfo");

            // 提取并解析值（内存大小）
            let value = parts.next().and_then(|v| v.parse::<u64>().ok());

            // 根据键设置相应的内存信息
            match key {
                "MemTotal:" => total = value.unwrap_or_default(),
                "MemFree:" => free = value.unwrap_or_default(),
                "Buffers:" => buffers = value.unwrap_or_default(),
                "Cached:" => cached = value.unwrap_or_default(),
                // 忽略其他非所需内存指标
                _ => (),
            }
        }

        // 创建并返回 MemInfo 实例
        Ok(MemInfo {
            total,
            free,
            buffers,
            cached,
        })
    }
}
