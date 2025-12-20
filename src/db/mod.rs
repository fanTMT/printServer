pub mod models;
mod queue;

pub use queue::*;

use std::time::Duration;

pub async fn create_pool() -> sqlx::SqlitePool {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(20)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(1800))
        .connect("sqlite:apps.db?mode=rwc")
        .await
        .unwrap();

    sqlx::query(models::PrintQueue::create_table_sql())
        .execute(&pool)
        .await
        .expect("创建表失败！！！");

    pool
}
