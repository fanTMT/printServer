use serde::{Deserialize, Serialize};

use crate::error::AuthError;
// 认证响应
#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub token_type: String,
}

// JWT Claims
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // 用户ID
    pub exp: usize,  // 过期时间
    pub iat: usize,  // 签发时间
    pub username: String,
}

pub struct JwtConfig {
    pub secret: String,        // 签名密钥
    pub expiration_hours: i64, // 过期时间配置
}

impl JwtConfig {
    pub fn new(secret: String) -> Self {
        Self {
            secret,
            expiration_hours: 24, // 密钥一天
        }
    }
    /// 生成
    pub fn generate_token(&self, username: &str, user_id: &str) -> Result<String, AuthError> {
        let now = time::OffsetDateTime::now_utc();
        let expires_at = now + time::Duration::hours(self.expiration_hours);
        let claims = Claims {
            sub: user_id.to_string(),
            username: username.to_string(),
            exp: expires_at.unix_timestamp() as usize,
            iat: now.unix_timestamp() as usize,
        };
        jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|_| AuthError::TokenCreation("创建令牌错误".to_string()))
    }

    /// 验证
    pub fn validate_token(&self, token: &str) -> Result<Claims, AuthError> {
        let token_data = jsonwebtoken::decode::<Claims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(self.secret.as_bytes()),
            &jsonwebtoken::Validation::default(),
        )
        .map_err(|err| match err.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AuthError::ExpiredToken,
            _ => AuthError::InvalidToken,
        })?;
        Ok(token_data.claims)
    }
}

#[test]
fn test_why_token_not_expiring() {
    let config = JwtConfig {
        secret: "test-secret".to_string(),
        expiration_hours: 1,
    };

    // 1. 生成一个立即过期的 token
    let expired_token = {
        let now = time::OffsetDateTime::now_utc();
        let past_time = now - time::Duration::hours(2); // 2小时前

        let claims = Claims {
            sub: "1".to_string(),
            username: "admin".to_string(),
            exp: past_time.unix_timestamp() as usize, // 已经过期
            iat: now.unix_timestamp() as usize,
        };

        jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(config.secret.as_bytes()),
        )
        .unwrap()
    };

    println!("生成的过期 Token: {}", expired_token);

    // 2. 验证它
    let result = config.validate_token(&expired_token);

    match result {
        Err(AuthError::ExpiredToken) => {
            println!("✅ 正确检测到过期 token");
        }
        Ok(claims) => {
            let now = time::OffsetDateTime::now_utc().unix_timestamp() as usize;
            println!("❌ 问题：过期 token 被接受了！");
            println!("  Token exp: {}", claims.exp);
            println!("  当前时间: {}", now);
            println!("  时间差: {}", now as i64 - claims.exp as i64);
            panic!("过期 token 不应该被接受");
        }
        Err(other) => {
            println!("❌ 预期 ExpiredToken，得到: {:?}", other);
        }
    }
}
