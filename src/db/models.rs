use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use time::OffsetDateTime;

#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
pub struct PrintQueue {
    pub id: i64, // SQLx 通常使用 i64 而不是 uint
    pub original_name: String,
    pub file_path: String,
    pub file_size: String,
    pub status: String,
    pub printer: String,
    pub page_size: String,
    pub orientation: String,
    // SQLx 没有内置的自动时间戳字段，需要手动处理
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
}

impl PrintQueue {
    pub const TABLE_NAME: &'static str = "print_queue";

    pub fn create_table_sql() -> &'static str {
        r#"
        CREATE TABLE IF NOT EXISTS print_queue (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            original_name TEXT NOT NULL,
            file_path TEXT NOT NULL UNIQUE,
            file_size TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT '等待',
            printer TEXT NOT NULL,
            page_size TEXT NOT NULL DEFAULT 'A4',
            orientation TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )"#
    }

    pub fn create_indexes_sql() -> Vec<&'static str> {
        vec![
            "CREATE INDEX IF NOT EXISTS idx_print_queue_status ON print_queue(status)",
            "CREATE INDEX IF NOT EXISTS idx_print_queue_printer ON print_queue(printer)",
        ]
    }
}

impl Default for PrintQueue {
    fn default() -> Self {
        Self {
            id: 0,
            original_name: String::new(),
            file_path: String::new(),
            file_size: String::new(),
            status: "等待".to_string(),
            printer: String::new(),
            page_size: "A4".to_string(),
            orientation: String::new(),
            created_at: Some(get_now().unwrap()),
            updated_at: Some(get_now().unwrap()),
        }
    }
}

// // 获取当前时间返回格式化好的文本 目前没有使用
// fn gettime() -> anyhow::Result<String> {
//     let local_time = time::OffsetDateTime::now_local()?;
//     let format =
//         time::format_description::parse("[year]年[month]月[day]日 [hour]:[minute]:[second]")?;
//     let formatted_local = local_time.format(&format)?;
//     Ok(formatted_local)
// }

fn get_now() -> anyhow::Result<OffsetDateTime> {
    Ok(time::OffsetDateTime::now_local()?)
}

#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    // 用户名
    pub username: String,
    // 邮箱
    pub email: String,
    // 密码
    pub password: String,
    // 用户组
    pub rules: String,
    // 昵称
    pub avatar: String,
}

impl User {
    pub const TABLE_NAME: &'static str = "users";

    pub fn create_table_sql() -> &'static str {
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            rules TEXT NOT NULL,
            avatar TEXT,
            email TEXT
        )"#
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: Default::default(),
            username: Default::default(),
            email: Default::default(),
            password: Default::default(),
            rules: "user".to_string(),
            avatar: Default::default(),
        }
    }
}
