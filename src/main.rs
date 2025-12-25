use std::sync::{Arc, RwLock};

use psr::{AppState, config, db, jwt, logger, router};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init_logger();
    let config = config::init_config()?;
    let pool = db::create_pool().await;
    info!("启动 Sqlite 数据库...");
    run(pool, config).await?;
    Ok(())
}

async fn run(pool: sqlx::Pool<sqlx::Sqlite>, config: config::Config) -> anyhow::Result<()> {
    let app_state = AppState {
        db_pool: Arc::new(pool),
        config: Arc::new(RwLock::new(config.clone())),
        jwt_config: Arc::new(jwt::JwtConfig::new("ykU/3UQweBqMw9Ldxxp20JfY8G1q0b3n8zNnixbDmUtg8tDrZMwLlj84ssWws6YCjQ8lwI96FtnzZ7jVeagE2A==".to_string())),
    };
    let app = router::create_router(app_state).await?;
    let ip = format!("{}:{}", &config.app.ip, &config.app.port);
    info!(target: "axum","web页面运行： http://{}", ip.to_string());
    let address = format!("[::]:{}", &config.app.port);
    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
