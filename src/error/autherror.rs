use axum::Json;
use axum::http::{HeaderValue, StatusCode};
use axum::response::IntoResponse;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    /// 错误的凭证
    #[error("错误的凭证: {0}")]
    WrongCredentials(String),
    /// 缺少凭据
    #[error("缺少凭据: {0}")]
    MissingCredentials(String),
    /// 令牌创建错误
    #[error("令牌创建错误: {0}")]
    TokenCreation(String),
    /// 无效令牌
    #[error("无效令牌")]
    InvalidToken,
    /// 过期
    #[error("过期令牌")]
    ExpiredToken,
    #[error("服务内部错误: {0}")]
    AnyError(#[from] anyhow::Error),
}

use axum::http::header::WWW_AUTHENTICATE;

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_msg, och) = match self {
            AuthError::WrongCredentials(e) => (
                StatusCode::UNAUTHORIZED,
                format!("错误的凭证,{}", e),
                Some("Bearer error=\"需要登录\", charset=\"UTF-8\""),
            ),
            AuthError::MissingCredentials(e) => (
                StatusCode::UNAUTHORIZED,
                format!("缺少凭证,{}", e),
                Some("Bearer error=\"需要登录\", charset=\"UTF-8\""),
            ),
            AuthError::TokenCreation(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("令牌创建错误,{}", e),
                None,
            ),
            AuthError::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                "无效令牌".to_string(),
                Some("Bearer error=\"凭证无效，请重试\", charset=\"UTF-8\""),
            ),
            AuthError::ExpiredToken => (
                StatusCode::UNAUTHORIZED,
                "令牌已过期".to_string(),
                Some(
                    "Bearer error=\"invalid_token\", error_description=\"The access token expired\"",
                ),
            ),
            AuthError::AnyError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("程序错误,{}", e),
                Some("Bearer error=\"凭证无效，请重试\", charset=\"UTF-8\""),
            ),
        };
        let body = Json(json!({
            "error": error_msg,
            "status": status.as_u16(),
        }));
        let mut response = (status, body).into_response();
        *response.status_mut() = status;
        if let Some(challenge) = och
            && let Ok(header_value) = HeaderValue::from_str(challenge)
        {
            response
                .headers_mut()
                .insert(WWW_AUTHENTICATE, header_value);
        }
        response
    }
}

// impl IntoResponse for AuthError {
//     fn into_response(self) -> Response {
//         let (status, error_message, ochallenge) = match self {
//             AuthError::WrongCredentials => (
//                 StatusCode::UNAUTHORIZED,
//                 "错误的凭证",
//                 Some("Bearer error=\"需要登录\", charset=\"UTF-8\""),
//             ),
//             AuthError::MissingCredentials => (
//                 StatusCode::UNAUTHORIZED,
//                 "缺少凭据",
//                 Some("Bearer error=\"需要登录\", charset=\"UTF-8\""),
//             ),
//             AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "令牌创建错误", None),
//             AuthError::InvalidToken => (
//                 StatusCode::UNAUTHORIZED,
//                 "无效令牌",
//                 Some("Bearer error=\"凭证无效，请重试\", charset=\"UTF-8\""),
//             ),
//             AuthError::ExpiredToken => (
//                 StatusCode::UNAUTHORIZED,
//                 "令牌已过期",
//                 Some(
//                     "Bearer error=\"invalid_token\", error_description=\"The access token expired\"",
//                 ),
//             ),
//             AuthError::AnyError(_e) => (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 "程序错误",
//                 Some("Bearer error=\"凭证无效，请重试\", charset=\"UTF-8\""),
//             ),
//         };

//         let mut response = Json(json!({
//             "error": error_message,
//             "code": format!("{:?}", self)
//         }))
//         .into_response();
//         // 状态码的可变引用
//         *response.status_mut() = status;
//         if let Some(challenge) = ochallenge
//             && let Ok(header_value) = HeaderValue::from_str(challenge)
//         {
//             response
//                 .headers_mut()
//                 .insert(WWW_AUTHENTICATE, header_value);
//         }

//         response
//     }
// }

// impl fmt::Display for AuthError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             AuthError::WrongCredentials => write!(f, "错误的凭证"),
//             AuthError::MissingCredentials => write!(f, "缺少凭据"),
//             AuthError::TokenCreation => write!(f, "令牌创建错误"),
//             AuthError::InvalidToken => write!(f, "无效令牌"),
//             AuthError::ExpiredToken => write!(f, "过期令牌"),
//         }
//     }
// }
