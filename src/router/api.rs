use axum::{
    Router,
    routing::{get, post},
};

use crate::{AppState, hander};

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/login", post(hander::api::auth::login))
        .route("/qrcode", get(hander::api::qrcode_get))
}

pub fn api_router() -> Router<AppState> {
    Router::new()
        .route("/get_setting", get(hander::api::get_setting))
        .route("/set_setting", post(hander::api::set_setting))
        .route("/get_printer", get(hander::api::get_printer))
}
