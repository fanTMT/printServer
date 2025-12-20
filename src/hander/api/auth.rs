use axum::{Json, extract::State};
use bcrypt::DEFAULT_COST;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::{
    AppState,
    error::{self, AuthError},
    jwt::{AuthResponse, RegisterRequest},
};

#[derive(Debug, Deserialize)]
pub struct LoginReq {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginRes {
    user: User,
    token: String,
}

#[derive(Debug, Serialize)]
pub struct User {
    id: usize,
    username: String,
    email: String,
    avatar: String,
    roles: Vec<String>,
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginReq>,
) -> Result<Json<Value>, error::AuthError> {
    if req.username == "admin" && req.password == "admin123456" {
        let token = state.jwt_config.generate_token(&req.username, "1")?;
        return Ok(Json(json!({"data":LoginRes {
            user: User {
                id: 1,
                username: "admin".to_string(),
                email: "li0shang@163.com".to_string(),
                avatar: "https://foruda.gitee.com/avatar/1756372673422545498/7660892_li0shang_1756372673.png".to_string(),
                roles: vec!["admin".to_string()],
            },
            token,
        }})));
    };
    Err(error::AuthError::WrongCredentials)
}

pub async fn register(
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AuthError> {
    // 验证输入
    if payload.username.is_empty() || payload.password.is_empty() || payload.email.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    // 哈希密码
    let password_hash =
        bcrypt::hash(&payload.password, DEFAULT_COST).map_err(|_| AuthError::TokenCreation)?;

    // TODO: 保存用户到数据库
    println!(
        "注册用户: {}, 密码哈希: {}",
        payload.username, password_hash
    );

    // 创建 JWT token（这里简化，实际应该登录）
    Err(AuthError::TokenCreation)
}
