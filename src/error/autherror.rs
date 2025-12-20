use axum::Json;
use axum::http::{HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum AuthError {
    /// 错误的凭证
    WrongCredentials,
    /// 缺少凭据
    MissingCredentials,
    /// 令牌创建错误
    TokenCreation,
    /// 无效令牌
    InvalidToken,
    /// 过期
    ExpiredToken,
}

use axum::http::header::WWW_AUTHENTICATE;

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message, ochallenge) = match self {
            // 重要：这里改为 "Basic" 认证方案才能触发浏览器弹窗
            AuthError::WrongCredentials => (
                StatusCode::UNAUTHORIZED,
                "错误的凭证",
                Some("Bearer error=\"需要登录\", charset=\"UTF-8\""),
            ),
            AuthError::MissingCredentials => (
                StatusCode::UNAUTHORIZED,
                "缺少凭据",
                Some("Bearer error=\"需要登录\", charset=\"UTF-8\""),
            ),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "令牌创建错误", None),
            AuthError::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                "无效令牌",
                Some("Bearer error=\"凭证无效，请重试\", charset=\"UTF-8\""),
            ),
            AuthError::ExpiredToken => (
                StatusCode::UNAUTHORIZED,
                "令牌已过期",
                Some(
                    "Bearer error=\"invalid_token\", error_description=\"The access token expired\"",
                ),
            ),
        };

        let mut response = Json(json!({
            "error": error_message,
            "code": format!("{:?}", self)
        }))
        .into_response();
        // 状态码的可变引用
        *response.status_mut() = status;
        if let Some(challenge) = ochallenge
            && let Ok(header_value) = HeaderValue::from_str(challenge)
        {
            response
                .headers_mut()
                .insert(WWW_AUTHENTICATE, header_value);
        }

        response
    }
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::WrongCredentials => write!(f, "错误的凭证"),
            AuthError::MissingCredentials => write!(f, "缺少凭据"),
            AuthError::TokenCreation => write!(f, "令牌创建错误"),
            AuthError::InvalidToken => write!(f, "无效令牌"),
            AuthError::ExpiredToken => write!(f, "过期令牌"),
        }
    }
}
