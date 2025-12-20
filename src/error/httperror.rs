use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HttpError {
    #[error("数据库错误: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Http服务错误: {0}")]
    HttpErr(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("服务内部错误: {0}")]
    Internal(#[from] anyhow::Error),

    #[error("IO错误: {0}")]
    Io(#[from] std::io::Error), // 包含 I/O 错误
}

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            HttpError::Database(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            ),
            HttpError::HttpErr(msg) => (StatusCode::BAD_REQUEST, msg),
            HttpError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            HttpError::Internal(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal error: {}", e),
            ),
            HttpError::Io(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("IO错误: {}", error),
            ),
        };

        let body = Json(json!({
            "error": message,
            "status": status.as_u16(),
        }));

        (status, body).into_response()
    }
}
