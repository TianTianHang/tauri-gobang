// Android rapfi 二进制路径解析模块
//
// 修复 SELinux 权限问题：
// - cache 目录的 app_data_file 类型不允许执行
// - nativeLibraryDir (app_lib_file 类型) 允许执行
// - 直接从 APK 的 jniLibs 中执行 rapfi
//
// useLegacyPackaging=false 时 .so 从 APK 直接 mmap，
// /proc/self/maps 中显示 base.apk 而非 .so 路径，
// 因此需要从 base.apk 路径推导 nativeLibraryDir。

use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

pub fn get_rapfi_path() -> Result<PathBuf, String> {
    eprintln!("🤖 [Android] Resolving librapfi.so path");

    let abi = current_abi()?;
    eprintln!("📦 [Android] ABI: {}", abi);

    let maps = std::fs::read_to_string("/proc/self/maps")
        .map_err(|e| format!("Cannot read /proc/self/maps: {}", e))?;

    // Debug: show lines containing our package name
    eprintln!("🗺️ [Android] Maps lines with 'tauri_gobang' or 'base.apk':");
    for line in maps.lines() {
        if line.contains("tauri_gobang") || line.contains("base.apk") {
            eprintln!("  {}", line);
        }
    }

    // Method 1: Find base.apk → derive nativeLibraryDir
    //   /data/app/~~HASH/com.tiantian.tauri_gobang-HASH/base.apk
    //   → /data/app/~~HASH/com.tiantian.tauri_gobang-HASH/lib/x86_64/librapfi.so
    let mut checked_apk = false;
    for line in maps.lines() {
        let path = match maps_line_path(line) {
            Some(p) => p,
            None => continue,
        };

        // Skip system libraries
        if path.starts_with("/system/") || path.starts_with("/apex/") {
            continue;
        }

        // Check for base.apk (only check once)
        if !checked_apk && path.ends_with("base.apk") && path.contains("/data/app/") {
            checked_apk = true;
            eprintln!("🔍 [Android] Found base.apk: {}", path);
            if let Some(install_dir) = PathBuf::from(path).parent() {
                let candidate = install_dir.join("lib").join(&abi).join("librapfi.so");
                eprintln!("🔍 [Android] Checking: {}", candidate.display());

                // Debug: list lib/ directory contents
                let lib_dir = install_dir.join("lib").join(&abi);
                eprintln!("📂 [Android] Listing {}:", lib_dir.display());
                if let Ok(entries) = std::fs::read_dir(&lib_dir) {
                    for entry in entries.flatten() {
                        eprintln!("  📄 {:?}", entry.file_name());
                    }
                } else {
                    eprintln!("  (directory does not exist)");
                }

                if candidate.exists() {
                    eprintln!("✅ [Android] Found at: {}", candidate.display());
                    return Ok(candidate);
                }
                eprintln!("⚠️ [Android] Not found at derived path");
            }
        }

        // Check for app .so (in /data/app/.../lib/)
        if path.contains("/data/app/") && path.ends_with(".so") {
            if let Some(parent) = PathBuf::from(path).parent() {
                let candidate = parent.join("librapfi.so");
                if candidate.exists() {
                    eprintln!("✅ [Android] Found at: {}", candidate.display());
                    return Ok(candidate);
                }
            }
        }
    }

    eprintln!("⚠️ [Android] No path found from /proc/self/maps");

    // Method 2: Legacy paths
    for path in legacy_paths(&abi) {
        eprintln!("🔍 [Android] Trying: {}", path.display());
        if path.exists() {
            eprintln!("✅ [Android] Found at: {}", path.display());
            return Ok(path);
        }
    }

    Err(format!(
        "❌ [Android] librapfi.so not found for ABI '{}'",
        abi
    ))
}

fn current_abi() -> Result<String, String> {
    if cfg!(target_arch = "aarch64") {
        Ok("arm64-v8a".into())
    } else if cfg!(target_arch = "x86_64") {
        Ok("x86_64".into())
    } else if cfg!(target_arch = "arm") {
        Ok("armeabi-v7a".into())
    } else if cfg!(target_arch = "x86") {
        Ok("x86".into())
    } else {
        Err(format!(
            "Unsupported architecture: {}",
            std::env::consts::ARCH
        ))
    }
}

fn maps_line_path(line: &str) -> Option<&str> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    for candidate in [parts.last(), parts.len().checked_sub(2).map(|i| &parts[i])] {
        if let Some(p) = candidate {
            if p.starts_with('/') {
                return Some(*p);
            }
        }
    }
    None
}

fn legacy_paths(abi: &str) -> Vec<PathBuf> {
    vec![
        PathBuf::from(format!(
            "/data/data/com.tiantian.tauri_gobang/lib/{}/librapfi.so",
            abi
        )),
        PathBuf::from(format!(
            "/data/user/0/com.tiantian.tauri_gobang/lib/{}/librapfi.so",
            abi
        )),
        PathBuf::from(format!(
            "/data/user_de/0/com.tiantian.tauri_gobang/lib/{}/librapfi.so",
            abi
        )),
    ]
}
