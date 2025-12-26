use std::{fs, path::Path};

use anyhow::Context;
use clap::Parser;
use serde::{Deserialize, Serialize};

use crate::utils::get_local_ip;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref SUPPORTED_EXT: HashMap<&'static str, bool> = {
        let mut map = HashMap::new();
        map.insert(".pdf", true);
        map.insert(".doc", true);
        map.insert(".docx", true);
        map.insert(".jpg", true);
        map.insert(".jpeg", true);
        map.insert(".png", true);
        map.insert(".txt", true);
        map
    };
    pub static ref IMAGE_EXT: HashMap<&'static str, bool> = {
        let mut map = HashMap::new();
        map.insert(".jpg", true);
        map.insert(".jpeg", true);
        map.insert(".png", true);
        map
    };
}

/// 应用整体配置（支持序列化和反序列化）
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub app: AppConfig,
    pub database: DatabaseConfig,
    pub log: LogConfig,
    pub printer: Printer,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub ip: String,
    pub port: u32,
    pub public_ip: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Printer {
    // 开启自动打印
    pub enabled_auto_print: bool,
    // 默认打印机名称
    pub printer: String,
    // 默认打印大小 A4
    pub page_size: String,
    // 默认方向 // 方向 `3` 横向，`4` 纵向
    pub orientation: u64,
    // 打印的页码
    pub page_ranges: Option<Vec<u64>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub timeout: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LogConfig {
    pub level: String,
    pub file_path: String,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // 自动生成版本/帮助信息
pub struct Args {
    /// 端口（可选）
    #[arg(short = 'p', long = "port", default_value_t = 3000)]
    port: u32,
    /// 公网IP（可选）
    #[arg(short = 'i', long = "ip")]
    ip: Option<String>,
}

/// 将配置保存到文件（覆盖原文件）
pub fn save_config<P: AsRef<Path>>(config: &Config, path: P) -> anyhow::Result<()> {
    // 将 Config 结构体序列化为格式化的 TOML 字符串（带缩进和注释）
    let toml_str =
        toml::to_string_pretty(config).context("配置序列化失败（结构体可能包含不支持的类型）")?;
    // 写入文件（覆盖原有内容）
    fs::write(&path, toml_str)
        .with_context(|| format!("无法写入配置到文件: {}", path.as_ref().display()))?;
    Ok(())
}

/// 从文件路径读取并解析配置
pub fn load_config<P: AsRef<Path>>(path: P) -> anyhow::Result<Config> {
    // 读取文件内容
    let config_str = fs::read_to_string(&path)
        .with_context(|| format!("无法读取配置文件: {}", path.as_ref().display()))?;

    // 解析 TOML 为 Config 结构体
    let config = toml::from_str(&config_str)
        .with_context(|| format!("配置文件格式错误: {}", path.as_ref().display()))?;

    Ok(config)
}

/// 获取默认配置文件路径（例如：用户目录下的 .printerAxum/config.toml）
pub fn default_config_path() -> anyhow::Result<std::path::PathBuf> {
    // 使用 dirs 库获取用户配置目录（跨平台：Linux 是 ~/.config，Windows 是 AppData/Roaming 等）
    let config_dir = dirs::config_dir()
        .context("无法获取系统配置目录")?
        .join("printerAxum"); // 应用专属目录
    // 确保目录存在
    fs::create_dir_all(&config_dir).context("无法创建配置目录")?;

    // 配置文件路径
    Ok(config_dir.join("config.toml"))
}

/// 生成默认配置文件
pub fn generate_default_config<P: AsRef<Path>>(
    path: P,
    Args { port, ip }: Args,
) -> anyhow::Result<()> {
    let local_ipv4 = get_local_ip().expect("获取本地IP失败！！！");
    let logspath = dirs::config_dir()
        .context("无法获取系统配置目录")?
        .join("printerAxum/logs")
        .to_string_lossy()
        .to_string(); // 应用专属目录
    fs::create_dir_all(&logspath).context("无法创建logs目录")?;

    let default_config = Config {
        app: AppConfig {
            ip: local_ipv4.unwrap_or("localhost".to_string()),
            port,
            public_ip: ip,
        },
        database: DatabaseConfig {
            url: "sqlite:apps.db?mode=rwc".to_string(),
            max_connections: 20,
            timeout: 2000,
        },
        log: LogConfig {
            level: "info".to_string(),
            file_path: logspath,
        },
        printer: Printer {
            enabled_auto_print: true,
            printer: "".to_string(),
            page_size: "A4".to_string(),
            orientation: 4,
            page_ranges: None,
        },
    };
    // 序列化为 TOML 字符串
    let toml_str = toml::to_string_pretty(&default_config)?;
    // 写入文件
    fs::write(&path, toml_str)
        .with_context(|| format!("无法写入默认配置到: {}", path.as_ref().display()))?;
    Ok(())
}

// 初始化配置 从配置文件读取
pub fn init_config() -> anyhow::Result<Config> {
    let config_path = default_config_path()?;
    // 检查配置文件是否存在，不存在则生成默认配置
    if !config_path.exists() {
        println!("配置文件不存在，生成默认配置...");
        generate_default_config(&config_path, Args::parse())?;
    }
    let config = load_config(config_path)?;
    // println!("加载配置------- {:#?}", config);
    Ok(config)
}
