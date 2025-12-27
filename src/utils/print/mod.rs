use crate::error::HttpError;
use crate::{AppState, db};
use anyhow::anyhow;
use serde::Serialize;
use sqlx::Pool;
use std::process::Command;

pub async fn print_document(
    original_name: String,
    file_path: String,
    state: AppState,
) -> Result<bool, HttpError> {
    print_file(file_path.as_str(), state.clone()).await?;
    set_info("打印完成".to_string(), original_name, &state.db_pool).await
}

async fn set_info(
    status: String,
    original_name: String,
    d: &Pool<sqlx::Sqlite>,
) -> Result<bool, HttpError> {
    db::update_queue_item(status, original_name, d)
        .await
        .map_err(HttpError::Internal)
}

#[cfg(target_os = "linux")]
async fn print_file(file_path: &str, state: AppState) -> Result<bool, HttpError> {
    let g = state.config.read().expect("Unable to read config");
    // 调试信息输出
    let mut cmd = Command::new("lp");
    // 设置打印机
    if !&g.printer.printer.is_empty() {
        cmd.arg("-d").arg(&g.printer.printer);
    }
    // 设置页面大小
    cmd.arg("-o").arg(format!("media={}", g.printer.page_size));

    // 设置打印范围
    if let Some(ref ranges) = g.printer.page_ranges {
        let range_str = ranges
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",");
        cmd.arg("-P").arg(range_str);
    }

    cmd.arg(file_path);
    tracing::debug!(target: "axum_web_app","打印文件 {}, 大小 {}，方向 {} 打印机 {} \n 完整命令: {:?}",
        file_path, g.printer.page_size, g.printer.orientation, g.printer.printer,cmd
    );
    cmd.output()?;
    Ok(true)
}

// windows调用打印命令
#[cfg(target_os = "windows")]
async fn print_file(file_path: &str, state: AppState) -> Result<()> {
    let g = state.config.read().expect("Unable to read config");
    let output = Command::new("print")
        .arg("-d")
        .arg(&g.printer.printer)
        .arg(file_path)
        .output()?;
    if output.status.success() {
        Ok(())
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        Err(anyhow!("windows print command failed: {}", error_msg))
    }
}

#[cfg(target_os = "windows")]
pub fn get_printers() -> Result<Vec<String>> {
    let output = Command::new("wmic")
        .args(["printer", "get", "name", "/format:csv"])
        .output()?;

    if !output.status.success() {
        return Err(anyhow!(
            "wmic command failed with status: {}",
            output.status
        ));
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let printers: Vec<String> = output_str
        .lines()
        .skip(1) // 跳过标题行
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }

            // CSV格式：Node,Name
            if let Some(name) = line.split(',').nth(1) {
                let name = name.trim();
                if !name.is_empty() {
                    return Some(name.to_string());
                }
            }
            None
        })
        .collect();

    if printers.is_empty() {
        return Err(anyhow!("No printers found"));
    }

    Ok(printers)
}

#[cfg(target_os = "macos")]
pub fn get_printers() -> Result<Vec<String>> {
    let output = Command::new("lpstat").arg("-p").output()?;

    if !output.status.success() {
        return Err(anyhow!(
            "lpstat command failed with status: {}",
            output.status
        ));
    }

    let output_str = String::from_utf8_lossy(&output.stdout);
    let printers: Vec<String> = output_str
        .lines()
        .filter_map(|line| {
            if line.starts_with("printer") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    Some(parts[1].to_string())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    if printers.is_empty() {
        return Err(anyhow!("No printers found"));
    }

    Ok(printers)
}

// 定义带状态的打印机结构体
#[derive(Debug, Clone, Serialize)] // Serialize用于Web返回JSON
pub struct Printer {
    /// 打印机名称（如 HL-2240_HL-2240_series）
    pub name: String,
    /// 打印机状态（如 "自从 2025年12月06日 星期六 21时26分21秒 开始接受请求"）
    pub status: String,
}

/// 获取打印机列表
#[cfg(target_os = "linux")]
pub fn get_printers() -> Result<Vec<Printer>, HttpError> {
    // 执行 lpstat -a 命令
    let output = Command::new("lpstat")
        .arg("-a")
        .output()
        .map_err(|e| HttpError::Internal(anyhow!("执行lpstat命令失败: {}", e)))?;

    // 检查命令执行状态
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(HttpError::Internal(anyhow!(
            "lpstat命令执行失败，状态: {}, 错误信息: {}",
            output.status,
            stderr
        )));
    }

    // 解析输出（UTF-8 容错处理）
    let output_str = String::from_utf8_lossy(&output.stdout);
    let printers: Vec<Printer> = output_str
        .lines()
        .filter(|line| !line.is_empty()) // 过滤空行
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                // 理论上不会走到这里（已过滤空行）
                return Err(HttpError::Internal(anyhow!("无效的打印机行: {}", line)));
            }

            // 提取打印机名（第0个字段）
            let name = parts[0].to_string();

            // 提取状态描述（从第2个字段到行尾，拼接成完整描述）
            let status_parts = if parts.len() >= 2 {
                &parts[2..] // 跳过 "自从" 前面的第1个字段？不，原输出是 "打印机名 自从 ..."，所以 parts[1] 是 "自从"
            } else {
                &parts[1..] // 兼容异常格式（如果只有两个字段）
            };
            let status = status_parts.join(" "); // 拼接成完整状态字符串

            Ok(Printer { name, status })
        })
        .collect::<Result<Vec<_>, _>>()?; // 收集所有结果，处理可能的解析错误

    // 检查是否找到打印机
    if printers.is_empty() {
        return Err(HttpError::NotFound("未找到任何可用打印机".to_string()));
    }

    Ok(printers)
}

// 测试函数
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_printers() {
        match get_printers() {
            Ok(printers) => {
                println!("找到 {} 台打印机：", printers.len());
                for (i, printer) in printers.iter().enumerate() {
                    println!("{}: 名称={}, 状态={}", i + 1, printer.name, printer.status);
                }
            }
            Err(e) => eprintln!("获取打印机失败: {}", e),
        }
    }
}
