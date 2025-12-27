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
pub fn api_router() -> Router<AppState> {
    Router::new()
        .route("/setting", get(hander::api::get_setting))
        .route("/setting", post(hander::api::set_setting))
        .route("/printer", get(hander::api::get_printer))
}
