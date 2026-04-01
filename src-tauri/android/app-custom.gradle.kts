// Custom Android Gradle configuration for tauri-gobang
// This file is referenced from gen/android/app/build.gradle.kts via:
//   apply(from = "../../../android/app-custom.gradle.kts")
//
// Purpose: Separates custom build logic from Tauri-generated templates,
// keeping project-specific configuration in git-tracked src-tauri/android/.

import com.android.build.api.dsl.ApplicationExtension
import com.android.build.gradle.internal.tasks.factory.dependsOn
import java.io.FileInputStream
import java.util.Properties
import kotlin.text.RegexOption

// === Signing Configuration ===
val keystorePropertiesFile = file("../../../android/keystore.properties")
val keystoreProperties = Properties()
if (keystorePropertiesFile.exists()) {
    keystoreProperties.load(FileInputStream(keystorePropertiesFile))
}

// === Configure Android Extension ===
configure<ApplicationExtension> {
    // === Signing Configuration ===
    signingConfigs {
        create("release") {
            val keyAlias = keystoreProperties["keyAlias"] as? String
                ?: error("keyAlias not found in keystore.properties")
            val keyPassword = keystoreProperties["password"] as? String
                ?: error("password not found in keystore.properties")
            val storeFile = keystoreProperties["storeFile"] as? String
                ?: error("storeFile not found in keystore.properties")
            val storePassword = keystoreProperties["password"] as? String
                ?: error("password not found in keystore.properties")

            this.keyAlias = keyAlias
            this.keyPassword = keyPassword
            this.storeFile = file(storeFile)
            this.storePassword = storePassword
        }
    }

    // === Packaging Options ===
    // Configure APK packaging to include native libraries in jniLibs
    packaging {
        jniLibs {
            useLegacyPackaging = true
            // Prevent Gradle from stripping rapfi binaries (they're already stripped)
            // This also improves build performance by skipping redundant work
            keepDebugSymbols.add("*/arm64-v8a/librapfi.so")
            keepDebugSymbols.add("*/x86_64/librapfi.so")
        }
    }
    
    // === Apply signing to release build ===
    buildTypes {
        getByName("release") {
            signingConfig = signingConfigs.getByName("release")
        }
    }
}

// === Rapfi Binary Copy Task ===
// Copy rapfi AI engine binaries from src-tauri/binaries/ to app/jniLibs/
// These are loaded at runtime by android_rapfi.rs
//
// Skip copying if target files already exist (for faster subsequent builds)
tasks.register("copyRapfiBinaries") {
    description = "Copy rapfi executables to jniLibs for Android"
    group = "build"

    doLast {
        val archMap = mapOf(
            "arm64-v8a" to "../../../binaries/rapfi-aarch64-linux-android",
            "x86_64" to "../../../binaries/rapfi-x86_64-linux-android"
        )

        archMap.forEach { (abi, source) ->
            val sourceFile = file(source)
            if (sourceFile.exists()) {
                val targetDir = file("src/main/jniLibs/$abi")
                val targetFile = file("$targetDir/librapfi.so")
                
                if (targetFile.exists()) {
                    println("⏭️  Skipped rapfi for $abi (already exists)")
                } else {
                    targetDir.mkdirs()
                    copy {
                        from(sourceFile)
                        into(targetDir)
                        rename { "librapfi.so" }
                    }
                    targetFile.setExecutable(true, false)
                    println("✅ Copied rapfi for $abi: ${sourceFile.name} -> $targetFile (.so)")
                }
            } else {
                println("⚠️  Warning: rapfi binary not found for $abi: $source")
            }
        }

        // Copy libc++_shared.so from NDK for each ABI
        val ndkDir = System.getenv("ANDROID_NDK_HOME")
            ?: throw GradleException("ANDROID_NDK_HOME not set")
        val ndkSysroot = "$ndkDir/toolchains/llvm/prebuilt/linux-x86_64/sysroot/usr/lib"

        val ndkArchMap = mapOf(
            "arm64-v8a" to "aarch64-linux-android",
            "x86_64" to "x86_64-linux-android"
        )

        ndkArchMap.forEach { (abi, ndkArch) ->
            val libFile = file("$ndkSysroot/$ndkArch/libc++_shared.so")
            if (libFile.exists()) {
                val targetDir = file("src/main/jniLibs/$abi")
                val targetFile = file("$targetDir/libc++_shared.so")
                
                if (targetFile.exists()) {
                    println("⏭️  Skipped libc++_shared.so for $abi (already exists)")
                } else {
                    targetDir.mkdirs()
                    copy {
                        from(libFile)
                        into(targetDir)
                    }
                    println("✅ Copied libc++_shared.so for $abi")
                }
            } else {
                println("⚠️  Warning: libc++_shared.so not found for $abi at $libFile")
            }
        }
    }
}

// === Network Security Config Task ===
// Copy network_security_config.xml to Android resources
// This config restricts cleartext HTTP to specific domains (server IP, localhost, emulator)
tasks.register("copyNetworkSecurityConfig") {
    description = "Copy network_security_config.xml to Android resources"
    group = "build"

    doLast {
        val sourceFile = file("../../../android/res/xml/network_security_config.xml")
        if (sourceFile.exists()) {
            val targetDir = file("src/main/res/xml")
            val targetFile = file("$targetDir/network_security_config.xml")
            targetDir.mkdirs()
            copy {
                from(sourceFile)
                into(targetDir)
            }
            println("✅ Copied network_security_config.xml")
        } else {
            println("⚠️  Warning: network_security_config.xml not found at $sourceFile")
        }
    }
}

// === Patch AndroidManifest Task ===
// Add android:networkSecurityConfig attribute to <application> tag
tasks.register("patchAndroidManifest") {
    description = "Patch AndroidManifest.xml to reference network_security_config"
    group = "build"

    dependsOn("copyNetworkSecurityConfig")

    doLast {
        val manifestFile = file("src/main/AndroidManifest.xml")
        if (!manifestFile.exists()) {
            println("⚠️  Warning: AndroidManifest.xml not found at $manifestFile")
            return@doLast
        }

        val content = manifestFile.readText()

        if (content.contains("android:networkSecurityConfig")) {
            println("⏭️  Skipped: AndroidManifest.xml already has networkSecurityConfig")
            return@doLast
        }

        val updated = content.replace(
            Regex("""(<application\b.*?)>""", RegexOption.DOT_MATCHES_ALL),
            """$1 android:networkSecurityConfig="@xml/network_security_config">"""
        )

        manifestFile.writeText(updated)
        println("✅ Patched AndroidManifest.xml with networkSecurityConfig")
    }
}

// === preBuild Dependency Hook ===
// Ensure rapfi binaries are copied before compilation starts
tasks.named("preBuild") {
    dependsOn("copyRapfiBinaries", "patchAndroidManifest")
}
