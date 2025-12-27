use std::sync::Arc;
use tracing::info;

// 从数据库获取所有打印队列（对应「获取队列」场景）
pub async fn get_all(
    db: &sqlx::Pool<sqlx::Sqlite>,
) -> anyhow::Result<Vec<super::models::PrintQueue>> {
    let queues = sqlx::query_as::<_, super::models::PrintQueue>(
        "SELECT * FROM print_queue ORDER BY updated_at DESC",
    )
    .fetch_all(db)
    .await?;
    Ok(queues)
}

// 从数据库获取一个打印队列项（对应「打印全部」场景）
pub async fn get_select(
    db: &sqlx::Pool<sqlx::Sqlite>,
    status_str: String,
) -> anyhow::Result<Vec<super::models::PrintQueue>> {
    let queues = sqlx::query_as::<_, super::models::PrintQueue>(
        "SELECT * FROM print_queue WHERE status = ? ORDER BY updated_at DESC",
    )
    .bind(status_str)
    .fetch_all(db)
    .await?;
    Ok(queues)
}

// 通过文件名获取数据库info
pub async fn get_one(
    db: &sqlx::Pool<sqlx::Sqlite>,
    original_name: String,
) -> anyhow::Result<super::models::PrintQueue> {
    let result = sqlx::query_as::<_, super::models::PrintQueue>(
        "SELECT * FROM print_queue WHERE original_name = ?",
    )
    .bind(original_name)
    .fetch_one(db)
    .await?;
    Ok(result)
}

// 向数据库添加队列项（对应「文件上传」场景）
pub async fn add_queue_item(
    item: super::models::PrintQueue,
    db: &sqlx::Pool<sqlx::Sqlite>,
) -> anyhow::Result<()> {
    let a = sqlx::query(
        "INSERT INTO print_queue (original_name, file_path,file_size,status,printer,page_size,orientation,created_at,updated_at) VALUES (?, ?,?,?,?,?,?,?,?)")
        .bind(item.original_name)
        .bind(item.file_path)
        .bind(item.file_size)
        .bind(item.status)
        .bind(item.printer)
        .bind(item.page_size)
        .bind(item.orientation)
        .bind(item.created_at)
        .bind(item.updated_at)
    .execute(db)
    .await?;
    info!(target: "axum","插入行的ID:{:#?}", a.last_insert_rowid());
    Ok(())
}

// 更新
pub async fn update_queue_item(
    status: String,
    original_name: String,
    db: &sqlx::Pool<sqlx::Sqlite>,
) -> anyhow::Result<bool> {
    let result = sqlx::query("UPDATE print_queue SET status = ? WHERE original_name = ?")
        .bind(status)
        .bind(original_name)
        .execute(db)
        .await?;
    // 判断是否成功更新了一行数据
    Ok(result.rows_affected() == 1)
}

// 清空队列
pub async fn clear_queue(pool: &sqlx::Pool<sqlx::Sqlite>) -> anyhow::Result<String> {
    let result = sqlx::query("UPDATE print_queue SET status = $1 WHERE status = $2")
        .bind("打印完成")
        .bind("打印失败")
        .execute(pool)
        .await?;
    // 成功了几行数据
    Ok(result.rows_affected().to_string())
}

// 删除一个
pub async fn delete_item(pool: Arc<sqlx::SqlitePool>, user_id: i64) -> anyhow::Result<bool> {
    let result = sqlx::query("DELETE FROM print_queue WHERE id = ?")
        .bind(user_id)
        .execute(pool.as_ref())
        .await?;

    // 判断是否成功删除了一行数据
    Ok(result.rows_affected() == 1)
}

// 全部删除
pub async fn delete_all(pool: &sqlx::Pool<sqlx::Sqlite>) -> anyhow::Result<String> {
    let result = sqlx::query("DELETE FROM print_queue").execute(pool).await?;
    // 判断是否成功删除了一行数据
    Ok(result.rows_affected().to_string())
}
