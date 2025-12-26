use axum::{Json, extract::State};
use bcrypt::DEFAULT_COST;
use serde::Deserialize;
use serde_json::{Value, json};
use tracing::info;

use crate::{
    AppState,
    db::{self, models::User},
    error::{self, AuthError},
};

#[derive(Debug, Deserialize)]
pub struct LoginReq {
    username: String,
    password: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginReq>,
) -> Result<Json<Value>, error::AuthError> {
    let user = db::user::user_login(&state.db_pool, req.username, req.password).await?;
    let token = state
        .jwt_config
        .generate_token(&user.username, &user.id.to_string())?;
    let res = json!({
        "data":{
            "user": user,
            "token":token,
        },
        "code":200
    });
    Ok(Json(res))
}

// 用户注册请求
#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<Value>, AuthError> {
    // 验证输入
    if payload.username.is_empty() || payload.password.is_empty() || payload.email.is_empty() {
        return Err(AuthError::MissingCredentials("输入类型有问题".to_string()));
    }

    // 哈希密码
    let password_hash = bcrypt::hash(&payload.password, DEFAULT_COST)
        .map_err(|_| AuthError::TokenCreation("哈希密码生成失败".to_string()))?;

    // TODO: 保存用户到数据库
    info!(
        "注册用户: {}, 密码哈希: {}",
        payload.username, password_hash
    );
    let mut user = User {
        username: payload.username.clone(),
        password: password_hash,
        email: payload.email,
        ..Default::default()
    };
    if payload.username == "admin" {
        user.roles = "admin".to_string();
        user.avatar = "管理员".to_string();
    }
    let user = db::user::reg_user(state.db_pool.as_ref(), user).await?;
    let token = state
        .jwt_config
        .generate_token(&user.username, &user.id.to_string())?;
    let res = json!({
        "data":{
            "user": user,
            "token":token,
        },
        "code":200
    });
    Ok(Json(res))
}
