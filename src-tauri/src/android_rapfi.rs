// Android rapfi 二进制提取逻辑模块
// 此模块提供 Android 平台上从 assets 提取 rapfi 二进制文件的功能

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use tauri::path::BaseDirectory;

/// Android 平台上从 assets 提取 rapfi 二进制
pub fn extract_rapfi_binary<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
) -> Result<PathBuf, String> {
    use std::os::unix::fs::PermissionsExt;

    // 获取应用的 cache 目录
    let cache_dir = app
        .path()
        .app_cache_dir()
        .map_err(|e| format!("Failed to get cache dir: {}", e))?;

    // 确保 cache 目录存在
    fs::create_dir_all(&cache_dir).map_err(|e| format!("Failed to create cache dir: {}", e))?;

    let rapfi_path = cache_dir.join("rapfi");

    // 如果已经提取过，直接返回
    if rapfi_path.exists() {
        eprintln!(
            "✅ [Android] rapfi already extracted: {}",
            rapfi_path.display()
        );
        return Ok(rapfi_path);
    }

    // 确定当前架构
    let arch = if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else {
        return Err(format!(
            "Unsupported Android architecture: {}",
            std::env::consts::ARCH
        ));
    };

    // 资源文件名
    let asset_name = format!("binaries/rapfi-{}-linux-android", arch);

    eprintln!("📦 [Android] Extracting rapfi from assets: {}", asset_name);

    // 从 assets 复制
    let asset_path = app
        .path()
        .resolve(&asset_name, BaseDirectory::Resource)
        .map_err(|e| format!("Failed to resolve asset path: {}", e))?;

    if !asset_path.exists() {
        return Err(format!(
            "Asset not found: {}. Please ensure rapfi binary is included in resources.",
            asset_name
        ));
    }

    // 复制文件
    fs::copy(&asset_path, &rapfi_path).map_err(|e| format!("Failed to copy rapfi: {}", e))?;

    // 设置执行权限 (rwxr-xr-x)
    let mut perms = fs::metadata(&rapfi_path)
        .map_err(|e| format!("Failed to get metadata: {}", e))?
        .permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&rapfi_path, perms)
        .map_err(|e| format!("Failed to set permissions: {}", e))?;

    eprintln!(
        "✅ [Android] rapfi extracted successfully: {}",
        rapfi_path.display()
    );

    Ok(rapfi_path)
}
