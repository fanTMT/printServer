use anyhow::{Result, bail};
use tracing::info;

use crate::db::models::User;

/// 获取所有用户列表
pub async fn get_user_all(db: &sqlx::Pool<sqlx::Sqlite>) -> Result<Vec<super::models::User>> {
    let queues =
        sqlx::query_as::<_, super::models::User>("SELECT * FROM users ORDER BY updated_at DESC")
            .fetch_all(db)
            .await?;
    Ok(queues)
}

/// 用户登录验证
pub async fn user_login(
    db: &sqlx::Pool<sqlx::Sqlite>,
    username: String,
    password: String,
) -> Result<User> {
    // SELECT 列名称 FROM 表名称 WHERE 列 运算符 值
    let queue = sqlx::query_as::<_, super::models::User>("SELECT * from users WHERE username=?")
        .bind(username)
        .fetch_one(db)
        .await?;
    // verify 会从 stored_hash 中提取盐和成本因子，用相同规则验证输入密码
    let is_match = bcrypt::verify(&password, &queue.password)?;
    if !is_match {
        bail!("密码不匹配");
    }
    Ok(queue)
}

/// 注册用户
pub async fn reg_user(db: &sqlx::Pool<sqlx::Sqlite>, user: super::models::User) -> Result<User> {
    let ok =
        sqlx::query("INSERT INTO users (username,password,email,rules,avatar) VALUES (?,?,?,?,?)")
            .bind(user.username)
            .bind(user.password)
            .bind(user.email)
            .bind(user.rules)
            .bind(user.avatar)
            .execute(db)
            .await?;
    info!(target: "axum","插入行的ID:{:#?}", ok.last_insert_rowid());
    let new_user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, username, password, email, rules, avatar
        FROM users
        WHERE id = ?
        "#,
    )
    .bind(ok.last_insert_rowid())
    .fetch_one(db)
    .await?;
    Ok(new_user)
}

/// 修改用户密码（以及其他信息）
pub async fn update_user(db: &sqlx::Pool<sqlx::Sqlite>, user: super::models::User) -> Result<bool> {
    let result = sqlx::query("UPDATE users SET avatar = ? WHERE username = ?")
        .bind(user.avatar)
        .bind(user.username)
        .execute(db)
        .await?;
    // 判断是否成功更新了一行数据
    Ok(result.rows_affected() == 1)
}
// /// 忘记密码
// pub async fn forgot_user(db: &sqlx::Pool<sqlx::Sqlite>, user: super::models::User) -> Result<()> {
//     Ok(())
// }
