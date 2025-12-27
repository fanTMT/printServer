use axum::{
    extract::State,
    http::{HeaderMap, HeaderValue},
    response::IntoResponse,
};
use tracing::info;

use crate::{AppState, utils::generate_qr_code};

// 创建一个二维码 方便连接
pub async fn qrcode_get(State(state): State<AppState>) -> impl IntoResponse {
    let config = state.config.read().unwrap();
    // 构建移动端页面URL：http://IP:Port/mobile
    let mobile_url = if let Some(public_ip) = config.app.public_ip.clone() {
        info!(target: "axum","使用公网IP-> http://{}", public_ip);
        format!("http://{}/mobile", public_ip)
    } else {
        info!(target: "axum","使用内网IP-> http://{}:{}", config.app.ip, config.app.port);
        format!("http://{}:{}/#/", config.app.ip, config.app.port)
    };
    match generate_qr_code(&mobile_url) {
        Ok(buffer) => {
            let mut headers = HeaderMap::new();
            headers.insert(
                axum::http::header::CONTENT_TYPE,
                HeaderValue::from_static("image/png"),
            );
            headers.insert(
                axum::http::header::CONTENT_DISPOSITION,
                HeaderValue::from_static("inline; filename=\"mobile_qrcode.png\""),
            );

            Ok((headers, buffer))
        }
        Err(e) => {
            tracing::error!("生成QR code失败: {}", e);
            Err((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "生成QRcode失败".to_string(),
            ))
        }
    }
}
