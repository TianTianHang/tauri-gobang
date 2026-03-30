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
          ];

          shellHook = ''
            export ANDROID_HOME=${androidComposition.androidsdk}/libexec/android-sdk
            export ANDROID_SDK_ROOT=$ANDROID_HOME
            export ANDROID_NDK_HOME=$ANDROID_HOME/ndk-bundle
            export GRADLE_OPTS="-Dorg.gradle.project.android.aapt2FromMavenOverride=$ANDROID_HOME/build-tools/36.0.0/aapt2"
            export PATH=$ANDROID_HOME/platform-tools:$ANDROID_HOME/emulator:$PATH

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
          '';
        };
      }
    );
}
