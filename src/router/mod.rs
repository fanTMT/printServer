pub mod api;
pub mod index;

use axum::{
    Router,
    extract::DefaultBodyLimit,
    middleware::from_fn_with_state,
    routing::{get, post},
};

use crate::{
    AppState,
    hander::{self, api::auth_middleware},
    handle_asset,
};

pub async fn create_router(state: AppState) -> anyhow::Result<Router> {
    let public_routes = Router::new()
        .route("/", get(hander::index))
        .route("/favicon.ico", get(hander::favicon))
        .nest("/api/auth", api::auth_router())
        .route("/assets/{*path}", get(handle_asset))
        .route("/api/upload", post(hander::upload_handler)); // 上传接口
    // --------------- 定义受保护路由组（需要中间件）---------------
    let protected_routes = Router::new()
        .merge(index::index_router())
        .nest("/api", api::api_router()) // /api/xxx 除了 /api/auth 之外的路由
        .layer(from_fn_with_state(
            state.clone(),
            auth_middleware::auth_middleware,
        ));

    // --------------- 合并路由 + 全局配置 ---------------
    let router = Router::new()
        .merge(public_routes) // 公开路由（无中间件）
        .merge(protected_routes) // 受保护路由（有中间件）
        .layer(DefaultBodyLimit::max(20 * 1024 * 1024)) // 全局上传限制 20MB
        .with_state(state);

    Ok(router)
}
