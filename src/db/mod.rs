pub mod models;

mod queue;
pub use queue::*;
pub mod user;

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
        .expect("创建print_queue表失败！！！");

    sqlx::query(models::User::create_table_sql())
        .execute(&pool)
        .await
        .expect("创建user表失败!!!");

    pool
}
