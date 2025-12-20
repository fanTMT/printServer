/// 将字节数转换为人类可读的字符串（如 1.5 KB, 2.3 MB）
pub fn human_readable_size(bytes: u64) -> String {
    const UNITS: [&str; 7] = ["B", "KB", "MB", "GB", "TB", "PB", "EB"];
    if bytes == 0 {
        return "0 B".to_string();
    }
    let exponent = (bytes as f64).log10() / 3.0; // 每 3 个数量级为一个单位（1024 近似为 1000 简化计算）
    let exponent = exponent.floor() as usize;
    let exponent = exponent.min(UNITS.len() - 1); // 防止超出单位数组范围
    let size = bytes as f64 / 1024f64.powi(exponent as i32); // 用 1024 计算（二进制单位）
    format!("{:.1} {}", size, UNITS[exponent])
}
