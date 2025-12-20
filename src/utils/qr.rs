use image::{ColorType, ImageEncoder, Luma, codecs::png::PngEncoder};
use qrcode::QrCode;
// 生成二维码函数
pub fn generate_qr_code(content: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let size = 256;
    // 生成QR码
    let code = QrCode::new(content)?;
    // 渲染为图像
    let image = code.render::<Luma<u8>>().min_dimensions(size, size).build();
    let width = image.width();
    let height = image.height();
    let image_data = image.into_vec();
    // 编码为PNG
    let mut buffer = Vec::new();
    let encoder = PngEncoder::new(&mut buffer);
    encoder.write_image(&image_data, width, height, ColorType::L8.into())?;

    Ok(buffer)
}
