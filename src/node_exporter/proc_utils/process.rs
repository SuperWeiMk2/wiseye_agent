use std::{fs, io};

use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProcessStatus {
    pid: u32,
    name: String,
    command: String,
    pr: u32,
    nice: u32,
    status_info: String
}

impl ProcessStatus {
    pub fn processes() -> Result<Vec<Self>, io::Error> {
        let mut processes = Vec::new();

        let pids = Self::init_pids()?;

        for pid in pids {
            let name = Self::get_process_name(&pid)?;
            let command = Self::get_process_cmdline(&pid)?;
            let status_info = Self::get_process_status(pid)?;

            processes.push(Self {
                pid,
                name,
                command,
                pr: 0,
                nice: 0,
                status_info,
            });
        }
        Ok(processes)
    }

    fn get_pids() -> Result<Vec<u32>, io::Error> {
        let mut process_pids = Vec::new();

        // 遍历/proc目录获取进程 pid
        for entry in fs::read_dir("/proc")? {
            let entry = entry?;
            let path = entry.path();

            // 检查是否为目录并且名字可以转化为整数
            if path.is_dir() && path.file_name()
                .and_then(|name| name.to_str())
                .map_or(false, |name| name.parse::<u32>().is_ok())
            {
                let pid = path.file_name().unwrap().to_str().unwrap().parse::<u32>()?;
                process_pids.push(pid);
            }
        }
        Ok(process_pids)
    }

    pub fn get_process_name(pid: &u32) {
    }

    fn get_process_cmdline(pid: &u32) -> Result<String, io::Error> {
        // 读取进程的cmdline信息
        let cmdline = PathBuf::from(format!("/proc/{}/cmdline", pid));
        if let Ok(cmdline_content) = fs::read_to_string(cmdline) {
            Ok(cmdline_content.trim().replace('\0', " "))
        } else {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to read cmdline for PID {}", pid),
            ));
        }
    }

    fn get_process_status(pid: u32) -> Result<String, io::Error> {
        // 读取进程的状态信息（status）
        let status_path = PathBuf::from(format!("/proc/{}/status", pid));
        // 读取文件内容并转换为字符串
        let status_content = fs::read_to_string(status_path)?;
        // 将每一行内容合并成一个单一的字符串，每行之间用换行符分隔
        let combined_status = status_content.lines().map(|line| line.to_string()).collect::<Vec<_>>().join("\n");

        Ok(combined_status)
    }

    fn reorganize_process_information(p: String) -> Result<ProcessStatus, io::Error> {
        let mut result = Self {
            pid: 0,
            name: String::new(),
            command: String::new(),
            pr: 0,
            nice: 0,
            status_info: String::new(),
        };

        for line in p.lines() {
            let mut parts = line.split_whitespace();
            let key = parts.next().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "行格式无效"))?;
            let value = parts.next();

            match key {
                "Pid:" => {
                    result.pid = value.ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "PID解析失败"))?
                        .parse::<u32>()
                        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("PID解析失败: {}", e)))?;
                },
                "Name:" => {
                    result.name = value.map(String::from).unwrap_or_default();
                },
                _ => {}
            }
        }

        Ok(result)
    }

    fn get_nice(){}

    fn get_virt(){}


}


