use axum::body::Body;
use axum::extract::{Request, State};
use axum::http::Response;
use axum::middleware::Next;

use crate::AppState;
use crate::error::AuthError;

pub async fn auth_middleware(
    state: State<AppState>,
    req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, AuthError> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .ok_or(AuthError::MissingCredentials("缺少令牌".to_string()))?
        .to_str()
        .map_err(|_| AuthError::InvalidToken)?; // Header
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AuthError::InvalidToken)?;
    state.jwt_config.validate_token(token)?;
    Ok(next.run(req).await)
}
