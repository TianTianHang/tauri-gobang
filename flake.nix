{
  description = "Tauri Gobang - Android开发环境";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
          config.allowUnfree = true;
          config.android_sdk.accept_license = true;
        };

        pkgsMusl = import nixpkgs {
          inherit system overlays;
          config.allowUnfree = true;
          config.android_sdk.accept_license = true;
          crossSystem = {
            config = "x86_64-unknown-linux-musl";
          };
        };

        androidComposition = pkgs.androidenv.composeAndroidPackages {
          platformVersions = [ "35" "36" ];
          includeNDK = true;
          ndkVersions = [ "26.1.10909125" ];
          buildToolsVersions = [ "34.0.0" "35.0.0" "36.0.0"];
          includeEmulator = true;
          includeSystemImages = true;
          systemImageTypes = [ "google_apis" ];
          abiVersions = [ "x86_64" ];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
          targets = [
            "aarch64-linux-android"
            "x86_64-linux-android"
            "x86_64-unknown-linux-musl"
          ];
        };

      in {
        packages.emulator = pkgs.androidenv.emulateApp {
          name = "tauri-gobang-emulator";
          platformVersion = "36";
          abiVersion = "x86_64";
          systemImageType = "google_apis";
          enableGPU = true;
        };

        packages.default = self.packages.${system}.emulator;

        devShells.musl = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            rustup
            pkg-config
            python3
            sqlite
            gcc
          ];

          shellHook = ''
            export PKG_CONFIG_PATH="${pkgs.sqlite.dev}/lib/pkgconfig:$PKG_CONFIG_PATH"
            export PKG_CONFIG_ALL_STATIC=1
            export PKG_CONFIG_ALL_DYNAMIC=0
            export MUSL_STATIC=1
            
            # Disable FORTIFY_SOURCE for static linking with musl
            export CFLAGS="-U_FORTIFY_SOURCE -D_FORTIFY_SOURCE=0"
            export CC="${pkgs.gcc}/bin/gcc"

            echo "🦄 Musl静态编译环境"
            echo "━━━━━━━━━━━━━━━━━━━━━━"
            echo "Rust: $(rustc --version)"
            echo "GCC: $(gcc --version | head -n1)"
            echo ""
            echo "编译server (静态链接):"
            echo "  cd server"
            echo "  cargo build --release --target x86_64-unknown-linux-musl"
            echo ""
          '';
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            nodejs_24
            pnpm
            rustToolchain
            rustup
            androidComposition.androidsdk
            androidComposition.platform-tools
            gradle
            jdk17
            pkg-config
            gtk3
            gdk-pixbuf
            webkitgtk_4_1
            libsoup_3
            python3
            musl
            musl.dev
          ];

          shellHook = ''
            export ANDROID_HOME=${androidComposition.androidsdk}/libexec/android-sdk
            export ANDROID_SDK_ROOT=$ANDROID_HOME
            export ANDROID_NDK_HOME=$ANDROID_HOME/ndk-bundle
            export GRADLE_OPTS="-Dorg.gradle.project.android.aapt2FromMavenOverride=$ANDROID_HOME/build-tools/36.0.0/aapt2"
            export PATH=$ANDROID_HOME/platform-tools:$ANDROID_HOME/emulator:$PATH

            # PKG_CONFIG_PATH for Tauri dependencies
            export PKG_CONFIG_PATH="${pkgs.gtk3.dev}/lib/pkgconfig:${pkgs.gdk-pixbuf.dev}/lib/pkgconfig:${pkgs.webkitgtk_4_1.dev}/lib/pkgconfig:${pkgs.libsoup_3.dev}/lib/pkgconfig:$PKG_CONFIG_PATH"

            # NDK .so 文件复制权限设置
            export ANDROID_NDK_LIBS_OUT=$PWD/target/android-lib
            mkdir -p $ANDROID_NDK_LIBS_OUT || true
            chmod -R u+w $ANDROID_NDK_LIBS_OUT 2>/dev/null || true

            echo "🤖 Tauri Gobang - Android开发环境"
            echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
            echo "Node.js: $(node --version)"
            echo "pnpm: $(pnpm --version)"
            echo "Rust: $(rustc --version)"
            echo "Java: $(java -version 2>&1 | head -n1)"
            echo "Gradle: $(gradle --version | grep Gradle | head -n1)"
            echo ""
            echo "Android SDK: $ANDROID_HOME"
            echo "Android NDK: $ANDROID_NDK_HOME"
            echo ""
            echo "📱 模拟器管理:"
            echo "   nix build .#emulator      # 构建模拟器启动脚本"
            echo "   ./result/bin/run-emulator # 启动 Nix 管理的模拟器"
            echo ""
            echo "   或手动使用:"
            echo "   emulator -list-avds       # 列出虚拟设备"
            echo "   emulator -avd <name>      # 启动模拟器"
            echo ""
            echo "✨ 环境已就绪！运行: pnpm tauri android dev"
            echo ""
            echo "📦 Server Build:"
            echo "   ./build.sh production   - 静态链接（生产）"
            echo "   ./build.sh development  - 动态链接（开发）"
            echo "   ./build.sh both         - 构建两个版本"
            echo ""
            echo "🦄 Musl环境:"
            echo "   nix develop .#musl      - 进入musl静态编译环境"
          '';
        };
      }
    );
}
