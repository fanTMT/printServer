use clap::Parser;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use crate::config;

pub fn init_logger() {
    if config::Args::parse().debug {
        init_dev();
    } else {
        init_pro();
    }
}

/// 开发环境日志配置 - 详细输出
fn init_dev() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "axum_web_app=debug,tower_http=debug,info".into());

    let fmt_layer = fmt::layer()
        .pretty() // 美化输出
        .with_ansi(true) // 启用颜色
        .with_target(true) // 显示目标模块
        .with_file(true) // 显示文件名
        .with_line_number(true); // 显示行号

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .init();

    tracing::info!("🚀 开发环境日志已初始化 - 详细模式");
    tracing::debug!(target: "axum_web_app", "调试信息将显示");
    tracing::trace!("跟踪信息将显示");
}

/// 生产环境日志配置 - 精简输出
fn init_pro() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        // 生产环境默认只显示警告和错误
        "axum_web_app=info,tower_http=info,warn".into()
    });

    let fmt_layer = fmt::layer()
        .json() // 生产环境使用 JSON 格式，便于日志收集系统处理
        .with_ansi(false) // 禁用颜色
        .with_target(true)
        .with_file(false) // 生产环境不显示文件名（安全考虑）
        .with_line_number(false); // 生产环境不显示行号

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .init();

    tracing::info!(target: "axum_web_app", "🏭 生产环境日志已初始化 - JSON 格式");
}
