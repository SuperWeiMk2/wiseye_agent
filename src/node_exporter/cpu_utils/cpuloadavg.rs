/// linux 下的 cpu 状态信息在 /proc/loadavg 文件下, 文件内容大致如下:
/// 0.00 0.00 0.00 1/164 3582
/// 第一个 0.00 代表过去1分钟的平均负载。
/// 第二个 0.00 代表过去5分钟的平均负载。
/// 第三个 0.00 代表过去15分钟的平均负载。
/// 1/164 是两个数字的组合,
/// 1：表示当前运行队列中的进程数（也就是正在运行或等待CPU的进程数）。
/// 164：表示系统总的进程数或者线程数（包括睡眠中的进程或线程），取决于使用的 Linux 发行版或内核版本。
/// 3582：表示自系统启动以来发生的上下文切换的次数。

/// 获取CPU的负载平均值，返回一个包含1分钟、5分钟和15分钟负载平均值的元组，
/// 或者在出错时返回一个错误字符串。

use std::fs::File;
use std::io::Read;

pub fn get_cpu_loadavg() -> Result<(f32, f32, f32), &'static str> {
    // 负载平均值文件的路径
    let loadavg_path = "/proc/loadavg";

    // 尝试打开文件
    let mut file = match File::open(loadavg_path) {
        Ok(file) => file, // 如果成功打开，则继续处理
        Err(_) => return Err("无法打开负载平均值文件"), // 如果打开失败，则返回错误
    };

    let mut buffer = String::new(); // 创建一个空字符串用于存储文件内容

    // 尝试读取文件内容到缓冲区
    match file.read_to_string(&mut buffer) {
        Ok(_) => {}, // 如果读取成功，则继续处理
        Err(_) => return Err("无法读取负载平均值文件"), // 如果读取失败，则返回错误
    };

    // 使用空白字符分割文件内容
    let mut parts = buffer.split_whitespace();

    // 尝试获取并解析1分钟的负载平均值
    let one_minute_load = match parts.next() {
        Some(load) => match parse_load(load) {
            Ok(value) => value, // 如果解析成功，则使用解析后的值
            Err(error) => return Err(error), // 如果解析失败，则返回错误
        },
        None => return Err("负载平均值文件格式无效"), // 如果没有更多的部分，则返回错误
    };

    // 尝试获取并解析5分钟的负载平均值
    let five_minute_load = match parts.next() {
        Some(load) => match parse_load(load) {
            Ok(value) => value, // 如果解析成功，则使用解析后的值
            Err(error) => return Err(error), // 如果解析失败，则返回错误
        },
        None => return Err("负载平均值文件格式无效"), // 如果没有更多的部分，则返回错误
    };

    // 尝试获取并解析15分钟的负载平均值
    let fifteen_minute_load = match parts.next() {
        Some(load) => match parse_load(load) {
            Ok(value) => value, // 如果解析成功，则使用解析后的值
            Err(error) => return Err(error), // 如果解析失败，则返回错误
        },
        None => return Err("负载平均值文件格式无效"), // 如果没有更多的部分，则返回错误
    };

    // 如果所有负载平均值都成功获取并解析，则返回它们
    Ok((one_minute_load, five_minute_load, fifteen_minute_load))
}

/// 解析负载值，将字符串转换为f32类型的浮点数。
/// 如果解析成功，则返回Ok包含解析后的值；如果解析失败，则返回Err包含错误消息。
fn parse_load(load: &str) -> Result<f32, &'static str> {
    load.parse::<f32>().map_err(|_| "无法解析负载平均值为浮点数")
}