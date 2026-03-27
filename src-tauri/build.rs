use std::fs;
use std::path::Path;

fn main() {
    tauri_build::build();

    println!("cargo:rerun-if-changed=binaries");

    let binaries_dir = Path::new("binaries");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    // Check for platform-specific Rapfi binary (Tauri sidecar naming: name-{target-triple})
    let target = std::env::var("TARGET").unwrap();
    let rapfi_binary = format!("rapfi-{}", target);

    let src_path = binaries_dir.join(&rapfi_binary);

    // Also check for simple names as fallback
    let fallback_path = if cfg!(target_os = "windows") {
        binaries_dir.join("rapfi.exe")
    } else {
        binaries_dir.join("rapfi")
    };

    let actual_path = if src_path.exists() {
        src_path
    } else if fallback_path.exists() {
        fallback_path
    } else {
        binaries_dir.join(&rapfi_binary) // For error message
    };

    if actual_path.exists() {
        let binary_name = actual_path.file_name().unwrap().to_str().unwrap();
        let dst_path = out_dir.join(binary_name);
        fs::copy(&actual_path, &dst_path).expect("Failed to copy Rapfi engine");
        println!(
            "cargo:warning=✓ Rapfi engine bundled: {}",
            dst_path.display()
        );

        // Also copy config and model files next to the binary for runtime access
        // Copy to both the build output directory AND the target debug/release directory
        // out_dir is target/debug/build/<hash>/out, we need to go up to target/debug
        let profile_dir = out_dir
            .ancestors()
            .nth(3) // Go up from build/<hash>/out to target/debug or target/release
            .unwrap_or_else(|| out_dir.ancestors().nth(2).unwrap_or(out_dir));

        for runtime_binaries_dir in &[out_dir.join("binaries"), profile_dir.join("binaries")] {
            fs::create_dir_all(runtime_binaries_dir).expect("Failed to create binaries directory");

            // Copy config file
            let config_src = binaries_dir.join("config.toml");
            if config_src.exists() {
                let config_dst = runtime_binaries_dir.join("config.toml");
                fs::copy(&config_src, &config_dst).expect("Failed to copy config file");
                println!(
                    "cargo:warning=✓ Config file copied to {}",
                    runtime_binaries_dir.display()
                );
            }

            // Copy model files
            for model in &[
                "mix9svqfreestyle_bsmix.bin.lz4",
                "mix9svqstandard_bs15.bin.lz4",
                "mix9svqrenju_bs15_black.bin.lz4",
                "mix9svqrenju_bs15_white.bin.lz4",
            ] {
                let model_src = binaries_dir.join(model);
                if model_src.exists() {
                    let model_dst = runtime_binaries_dir.join(model);
                    fs::copy(&model_src, &model_dst).expect(&format!("Failed to copy {}", model));
                }
            }
            println!(
                "cargo:warning=✓ Model files copied to {}",
                runtime_binaries_dir.display()
            );
        }
    } else {
        println!(
            "cargo:warning=⚠ Rapfi engine not found at {}",
            actual_path.display()
        );
        println!("cargo:warning=  To enable the external AI, download Rapfi from:");
        println!("cargo:warning=  https://github.com/dhbloo/rapfi/releases");
        println!(
            "cargo:warning=  Then place the binary in: {}",
            binaries_dir.display()
        );
    }
}
