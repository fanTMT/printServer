use std::path::Path;
use std::process::Command;

fn main() {
    let is_release = cfg!(not(debug_assertions));
    let build_script_dir = Path::new(env!("CARGO_MANIFEST_DIR")); // 拿到Cargo.toml所在的目录（项目根目录）
    let ui_dir = build_script_dir.join("ui"); // 拼接成“根目录/ui”的路径

    // 仅在 Release 模式执行
    if !is_release {
        println!("cargo:warning= 非Release 模式，跳过前端构建");
        return;
    }

    if !ui_dir.exists() {
        eprintln!("cargo:warning= ❌ 错误：找不到ui目录！请确认前端项目在ui文件夹下");
        std::process::exit(1);
    }

    println!("cargo:warning= 📦 开始构建前端项目（ui目录)");

    let mut cmd = if cfg!(target_os = "windows") {
        // Windows系统用cmd.exe执行
        let mut cmd = Command::new("cmd");
        cmd.arg("/C").arg("npm run build");
        cmd
    } else {
        // Mac/Linux系统用sh执行
        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg("npm run build");
        cmd
    };

    // 设置命令执行的目录为ui目录
    cmd.current_dir(ui_dir);

    // 运行命令并检查结果
    let status = cmd
        .status()
        .expect("cargo:warning= ❌ 无法执行npm命令！请确保已安装Node.js和npm");

    if status.success() {
        println!("cargo:warning= ✅ 前端项目构建成功！");
    } else {
        eprintln!("cargo:warning= ❌ 前端项目构建失败！请进入ui目录手动执行npm run build查看错误");
        std::process::exit(1);
    }

    println!("cargo:rerun-if-changed=ui/src");
}
