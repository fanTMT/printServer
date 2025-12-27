use crate::{
    AppState,
    config::SUPPORTED_EXT,
    db,
    utils::{self, print_document},
};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use axum_extra::extract::Multipart;
use futures_core::stream::Stream;
use serde::Serialize;
use std::path::Path;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};
use tracing::info;
use uuid::Uuid;

// 响应数据结构
#[derive(Serialize)]
struct UploadResponse {
    success: bool,
    message: String,
    file_name: Option<String>,
    file_path: Option<String>,
}

// 错误处理
#[derive(Debug)]
pub enum UploadError {
    NoFile,
    InvalidFileName,
    InvalidFileType,
    FileTooLarge(usize),
    IoError(String),
    MultipartError(String),
}

impl IntoResponse for UploadError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            UploadError::NoFile => (StatusCode::BAD_REQUEST, "未找到文件".to_string()),
            UploadError::InvalidFileName => (StatusCode::BAD_REQUEST, "无效的文件名".to_string()),
            UploadError::InvalidFileType => {
                (StatusCode::BAD_REQUEST, "不支持的文件类型".to_string())
            }
            UploadError::FileTooLarge(max_size) => (
                StatusCode::PAYLOAD_TOO_LARGE,
                format!("文件大小超过限制: {}MB", max_size / 1024 / 1024),
            ),
            UploadError::IoError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            UploadError::MultipartError(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        let response = UploadResponse {
            success: false,
            message,
            file_name: None,
            file_path: None,
        };

        (status, Json(response)).into_response()
    }
}

// 检查文件名是否安全
fn is_valid_filename(filename: &str) -> bool {
    if filename.is_empty() || filename.len() > 255 {
        return false;
    }

    let path = Path::new(filename);
    let components: Vec<_> = path.components().collect();

    if components.len() != 1 {
        return false;
    }

    !filename.contains(|c: char| {
        c == '/'
            || c == '\\'
            || c == ':'
            || c == '*'
            || c == '?'
            || c == '"'
            || c == '<'
            || c == '>'
            || c == '|'
    })
}

// 检查文件类型是否允许 - 修复扩展名匹配逻辑
fn is_allowed_file_type(filename: &str) -> bool {
    filename
        .rsplit_once('.') // 获取最后一个点后的部分（如 "file.pdf" -> ("file", "pdf")）
        .map(|(_, ext)| format!(".{}", ext.to_lowercase())) // 转换为 ".pdf" 格式
        .and_then(|ext| SUPPORTED_EXT.get(ext.as_str())) // 查找带点的小写扩展名
        .copied()
        .unwrap_or(false)
}

impl From<UploadError> for (StatusCode, String) {
    fn from(error: UploadError) -> Self {
        match error {
            UploadError::NoFile => (StatusCode::BAD_REQUEST, "未找到文件".to_string()),
            UploadError::InvalidFileName => (StatusCode::BAD_REQUEST, "无效的文件名".to_string()),
            UploadError::InvalidFileType => {
                (StatusCode::BAD_REQUEST, "不支持的文件类型".to_string())
            }
            UploadError::FileTooLarge(max_size) => (
                StatusCode::PAYLOAD_TOO_LARGE,
                format!("文件大小超过限制: {}MB", max_size / 1024 / 1024),
            ),
            UploadError::IoError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            UploadError::MultipartError(msg) => (StatusCode::BAD_REQUEST, msg),
        }
    }
}

// 文件上传处理器
pub async fn upload_handler(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, UploadError> {
    // 确保上传目录存在
    let upload_dir = "./uploads";
    // 创建文件夹
    fs::create_dir_all(upload_dir)
        .await
        .map_err(|e| UploadError::IoError(format!("创建上传目录失败: {}", e)))?;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| UploadError::MultipartError(format!("解析表单数据失败: {}", e)))?
    {
        // 如果是文件字段
        if let Some(original_filename) = field.file_name() {
            let original_filename = original_filename.to_string();
            // 验证文件名
            if !is_valid_filename(&original_filename) {
                return Err(UploadError::InvalidFileName);
            }
            // 验证文件类型
            if !is_allowed_file_type(&original_filename) {
                return Err(UploadError::InvalidFileType);
            }
            // 限制文件大小 (10MB)
            let max_size = 10 * 1024 * 1024;
            if field.size_hint().0 > max_size {
                return Err(UploadError::FileTooLarge(max_size));
            }
            // 读取文件数据
            let data = field
                .bytes()
                .await
                .map_err(|e| UploadError::MultipartError(format!("读取文件数据失败: {}", e)))?;
            // 检查实际文件大小
            if data.len() > max_size {
                return Err(UploadError::FileTooLarge(max_size));
            }

            // 生成安全的文件名
            let file_extension = original_filename.split('.').next_back().unwrap_or("bin");
            let safe_filename = format!("{}.{}", Uuid::new_v4(), file_extension); // 文件名
            let save_path = format!("{}/{}", upload_dir, safe_filename); // 带文件名的路径

            // 保存文件
            let mut file = File::create(&save_path)
                .await
                .map_err(|e| UploadError::IoError(format!("创建文件失败: {}", e)))?;

            file.write_all(&data)
                .await
                .map_err(|e| UploadError::IoError(format!("写入文件失败: {}", e)))?;

            // 返回成功响应 - 关键修改：直接返回 Json，不需要 (StatusCode, Json)
            let response = UploadResponse {
                success: true,
                message: "文件上传成功".to_string(),
                file_name: Some(original_filename.clone()),
                file_path: Some(save_path.clone()),
            };

            let item = db::models::PrintQueue {
                original_name: original_filename.clone(),
                file_path: save_path.clone(),
                file_size: utils::human_readable_size(data.len() as u64),
                printer: state.config.read().unwrap().printer.printer.clone(),
                page_size: state.config.read().unwrap().printer.page_size.clone(),
                orientation: state.config.read().unwrap().printer.orientation.to_string(),
                ..Default::default()
            };
            // 添加到数据库
            db::add_queue_item(item, &state.db_pool)
                .await
                .map_err(|e| UploadError::IoError(format!("添加打印队列失败: {}", e)))?;
            // .expect("添加打印队列失败");

            if state.config.read().unwrap().printer.enabled_auto_print {
                info!(target: "axum","自动打印已经打印中:{}",original_filename);
                print_document(original_filename, save_path, state)
                    .await
                    .expect("打印有问题");
            }
            return Ok(Json(response));
        }
    }

    Err(UploadError::NoFile)
}
