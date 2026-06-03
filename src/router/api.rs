use axum::{
    Router,
    routing::{get, post},
};

use crate::{AppState, hander};

/// 不验证token /api/auth/
pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(hander::api::auth::login))
        .route("/register", post(hander::api::auth::register))
        .route("/qrcode", get(hander::api::qrcode_get))
}

/// 验证token /api/
pub fn api_v1() -> Router<AppState> {
    Router::new()
        .route("/getall", get(hander::getall)) // 获取全部列表
        .route("/print", post(hander::print)) // 打印单个
        .route("/print_all", post(hander::print_all))
        .route("/clear_queue", post(hander::clear_queue)) // 清空队列
        .route("/reset", post(hander::reset)) // 重置系统
        .route("/setting", get(hander::api::get_setting)) // 获取设置
        .route("/setting", post(hander::api::set_setting)) // 保存设置
        .route("/printer", get(hander::api::get_printer)) // 打印机列表/状态
}
