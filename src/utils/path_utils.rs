use std::fs;
use std::io;
use std::path::Path;

// 确保路径仅包含一个普通组件（无目录分隔符）
pub fn path_is_valid(path: &str) -> bool {
    let path = Path::new(path);
    let mut components = path.components().peekable();
    // 确保路径仅包含一个普通组件（无目录分隔符）
    if let Some(first) = components.peek()
        && !matches!(first, std::path::Component::Normal(_))
    {
        return false;
    }
    components.count() == 1
}

/// 清空文件夹
pub fn delete_directory_contents_recursive(path: &Path) -> io::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // 递归删除子目录
                fs::remove_dir_all(&path)?;
            } else {
                // 删除文件
                fs::remove_file(&path)?;
            }
        }
    }
    Ok(())
}
