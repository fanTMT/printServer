use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use crate::config::Config;

pub fn init_logger(config: &Config) {
    if config.log.level == "debug" {
        init_dev();
    } else {
        init_pro(config);
    }
}

/// 开发环境日志配置 - 详细输出
fn init_dev() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "axum=debug,tower_http=debug,info".into());

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
    tracing::debug!(target: "axum", "debug 显示");
    tracing::trace!("trace 显示");
}

/// 生产环境日志配置 - 精简输出
fn init_pro(config: &Config) {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        // 生产环境默认只显示警告和错误
        "axum=info,tower_http=info,warn".into()
    });

    let fmt_output = fmt::layer()
        .json() // 生产环境使用 JSON 格式，便于日志收集系统处理
        .with_ansi(false) // 禁用颜色
        .with_target(true)
        .with_file(false) // 生产环境不显示文件名（安全考虑）
        .with_line_number(false); // 生产环境不显示行号

    println!("日志目录:{:?}", &config.log.file_path);
    let file_appender = RollingFileAppender::builder()
        .filename_prefix("printAxum")
        .filename_suffix("log")
        .max_log_files(7)
        .rotation(Rotation::DAILY)
        .build(&config.log.file_path)
        .expect("日志系统启动失败!!!");

    let fmt_file = fmt::layer()
        .json()
        .with_ansi(false)
        .with_writer(file_appender); // 绑定到文件写入器

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_output)
        .with(fmt_file)
        .init();

    tracing::info!(target: "axum", "🏭 生产环境日志已初始化 - JSON 格式");
}
