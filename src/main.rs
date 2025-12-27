use std::sync::{Arc, RwLock};

use psr::{AppState, config, db, jwt, logger, router};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化配置
    let config = config::init_config()?;
    // 初始化 log
    logger::init_logger(&config);
    // 初始化数据库
    let pool = db::create_pool(&config).await;
    info!("启动 Sqlite 数据库...");
    // 启动服务
    run(pool, config).await?;
    Ok(())
}

async fn run(pool: sqlx::Pool<sqlx::Sqlite>, config: config::Config) -> anyhow::Result<()> {
    let app_state = AppState {
        // 数据库
        db_pool: Arc::new(pool),
        // 配置
        config: Arc::new(RwLock::new(config.clone())),
        // jsonwebtoken
        jwt_config: Arc::new(jwt::JwtConfig::new("ykU/3UQweBqMw9Ldxxp20JfY8G1q0b3n8zNnixbDmUtg8tDrZMwLlj84ssWws6YCjQ8lwI96FtnzZ7jVeagE2A==".to_string())),
    };
    // 创建路由
    let app = router::create_router(app_state).await?;
    // 格式化输出
    let ip = format!("{}:{}", &config.app.ip, &config.app.port);
    info!(target: "axum","web页面运行： http://{}", ip.to_string());
    // 全地址监听
    let address = format!("[::]:{}", &config.app.port);
    let listener = tokio::net::TcpListener::bind(address).await?;
    // 启动axum服务
    axum::serve(listener, app).await?;
    Ok(())
}
