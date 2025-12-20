use axum::{
    Json,
    extract::State,
    http,
    response::{Html, IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tracing::info;

use crate::{AppState, Templates, db, error, utils};

pub async fn index() -> Html<String> {
    let index_html = Templates::get("index.html").expect("找不到模板文件");
    let html_content = std::str::from_utf8(index_html.data.as_ref())
        .expect("无效的 UTF-8")
        .to_string();
    Html(html_content)
}

pub async fn favicon() -> impl IntoResponse {
    let icon = Templates::get("favicon.ico").expect("找不到模板文件");
    Response::builder()
        .status(200)
        .header(http::header::CONTENT_TYPE, "image/x-icon")
        .body(axum::body::Body::from(icon.data))
        .unwrap()
}

pub async fn getall(State(state): State<AppState>) -> Result<Json<Value>, error::HttpError> {
    let a = db::get_all(&state.db_pool).await?;

    Ok(Json(json!({
        "code": 200,
        "success": true,
        "data":a,
    })))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrintValue {
    #[serde(rename = "fileName")]
    filename: String, // 前端传递的原始文件名
    #[serde(rename = "printer")]
    printer: String,
    #[serde(rename = "paperSize")]
    page_size: String, // 大小 A4（Rust 惯例使用蛇形命名法）
    #[serde(rename = "layout")]
    orientation: String, // 方向
}
/// 打印文件
pub async fn print(
    State(state): State<AppState>,
    Json(payload): Json<PrintValue>,
) -> Result<impl IntoResponse, error::HttpError> {
    let p = state.db_pool.as_ref();
    let file = db::get_one(p, payload.filename.clone()).await?;
    // info!(target: "axum_web_app","默认");
    utils::print_document(
        payload.filename.clone(),
        file.file_path.clone(),
        state.clone(),
    )
    .await?;
    let b = db::update_queue_item("打印完成".to_string(), payload.filename, p).await?;
    if b {
        info!(target: "axum_web_app" ,"打印了一个文件，{:#?}", file.original_name)
    }
    Ok((
        axum::http::StatusCode::OK,
        Json(json!({
            "status": "success",
            "message": "打印完成",
            "data": file
        })),
    ))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrintAllValue {
    #[serde(rename = "printer")]
    printer: String,
    #[serde(rename = "pageSize")]
    page_size: String, // 大小 A4（Rust 惯例使用蛇形命名法）
    #[serde(rename = "orientation")]
    orientation: String, // 方向
}
pub async fn print_all(
    State(state): State<AppState>,
    Json(payload): Json<PrintAllValue>,
) -> Result<impl IntoResponse, error::HttpError> {
    println!("打印全部 配置：{:#?}", payload);
    let db_pool = state.db_pool.as_ref();
    let files = db::get_select(db_pool, "等待".to_string()).await?;
    let num = files.len();
    for file in files {
        // 发送打印任务
        utils::print_document(
            file.original_name.clone(),
            file.file_path.clone(),
            state.clone(),
        )
        .await?;
        db::update_queue_item("打印完成".to_string(), file.original_name.clone(), db_pool).await?;
    }
    Ok((
        http::StatusCode::OK,
        Json(json!({
            "message": "所有打印任务已发送",
            "data": num.to_string()
        })),
    ))
}

/// 清空队列
pub async fn clear_queue(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, error::HttpError> {
    let num = db::clear_queue(state.db_pool.as_ref()).await?;
    utils::delete_directory_contents_recursive(std::path::Path::new("./uploads"))?;
    Ok((
        http::StatusCode::OK,
        Json(json!({
            "message": "打印队列已清空",
            "data": num
        })),
    ))
}

/// 重置系统
pub async fn reset(State(state): State<AppState>) -> Result<impl IntoResponse, error::HttpError> {
    let id = db::delete_all(state.db_pool.as_ref()).await?;
    Ok(Json(json!({
        "code": 200,
        "success": true,
        "data":id,
    })))
}
