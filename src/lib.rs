pub mod config;
pub mod db;
pub mod error;
pub mod hander;
pub mod jwt;
pub mod logger;
pub mod router;
pub mod utils;

use std::sync::{Arc, RwLock};

use axum::{
    http::{HeaderMap, StatusCode, header},
    response::IntoResponse,
};
use rust_embed::RustEmbed;
use tracing::debug;

#[derive(Clone)]
pub struct AppState {
    // 数据库连接池（Arc 确保线程安全共享）
    pub db_pool: Arc<sqlx::Pool<sqlx::Sqlite>>,
    // 全局配置（RwLock 允许多线程读写）
    pub config: Arc<RwLock<config::Config>>,
    pub jwt_config: Arc<jwt::JwtConfig>,
}

#[derive(RustEmbed)]
#[folder = "ui/dist"]
struct Templates;

#[derive(RustEmbed)]
#[folder = "ui/dist/assets/"]
struct Asset;

// 处理嵌入的文件并返回适当的响应
async fn handle_asset(
    axum::extract::Path(filename): axum::extract::Path<String>,
) -> impl IntoResponse {
    // 打印调试信息，帮助确认路径问题
    // info!("请求的资源路径: {}", filename);
    let possible_paths = [&filename, &format!("static/{}", filename)];
    // 尝试所有可能的路径
    for &path in &possible_paths {
        // println!("尝试查找资源: {}", path);
        debug!("尝试查找资源: {}", path);
        if let Some(content) = Asset::get(path) {
            // 确定MIME类型
            let mime_type = mime_guess::from_path(path).first_or_octet_stream();
            // 构建响应头
            let mut headers = HeaderMap::new();
            headers.insert(
                header::CONTENT_TYPE,
                mime_type.as_ref().parse().unwrap_or_else(|_| {
                    // 处理MIME类型解析失败的情况
                    mime_guess::mime::APPLICATION_OCTET_STREAM
                        .as_ref()
                        .parse()
                        .unwrap()
                }),
            );
            // 添加缓存头（可选，根据需要调整）
            headers.insert(
                header::CACHE_CONTROL,
                "public, max-age=86400".parse().unwrap(),
            );
            return (StatusCode::OK, headers, content.data).into_response();
        }
    }
    // 如果所有路径都找不到资源
    // println!("资源未找到: {:?}", possible_paths);
    debug!("资源未找到: {:?}", possible_paths);
    (
        StatusCode::NOT_FOUND,
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        "请求的资源不存在",
    )
        .into_response()
}
