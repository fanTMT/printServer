use axum::{
    Router,
    routing::{get, post},
};

use crate::{AppState, hander};

pub fn index_router() -> Router<AppState> {
    Router::new()
        .route("/getall", get(hander::getall)) // 获取全部列表
        .route("/print", post(hander::print)) // 打印单个
        .route("/print_all", post(hander::print_all))
        .route("/clear_queue", post(hander::clear_queue)) // 清空队列
        .route("/reset", post(hander::reset)) // 重置系统
}
