import java.util.Properties

plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
    id("rust")
}

val tauriProperties = Properties().apply {
    val propFile = file("tauri.properties")
    if (propFile.exists()) {
        propFile.inputStream().use { load(it) }
    }
}

  android {
     compileSdk = 35
     namespace = "com.tiantian.tauri_gobang"
    defaultConfig {
        manifestPlaceholders["usesCleartextTraffic"] = "false"
        applicationId = "com.tiantian.tauri_gobang"
        minSdk = 24
        targetSdk = 35
        versionCode = tauriProperties.getProperty("tauri.android.versionCode", "1").toInt()
        versionName = tauriProperties.getProperty("tauri.android.versionName", "1.0")
    }
    buildTypes {
        getByName("debug") {
            manifestPlaceholders["usesCleartextTraffic"] = "true"
            isDebuggable = true
            isJniDebuggable = true
            isMinifyEnabled = false
            packaging {
                jniLibs.keepDebugSymbols.add("*/arm64-v8a/*.so")
            }
        }
        getByName("release") {
            isMinifyEnabled = true
            proguardFiles(
                *fileTree(".") { include("**/*.pro") }
                    .plus(getDefaultProguardFile("proguard-android-optimize.txt"))
                    .toList().toTypedArray()
            )
        }
    }
    kotlinOptions {
        jvmTarget = "1.8"
    }
    buildFeatures {
        buildConfig = true
    }
    // Disable universal APK and only build for arm64-v8a
    packagingOptions {
        jniLibs {
            useLegacyPackaging = true
        }
        doNotStrip("*/arm64-v8a/librapfi.so")
        doNotStrip("*/armeabi-v7a/librapfi.so")
        doNotStrip("*/x86_64/librapfi.so")
        doNotStrip("*/x86/librapfi.so")
    }
}

rust {
    rootDirRel = "../../../"
}

// Copy rapfi binaries to jniLibs for execution
tasks.register("copyRapfiBinaries") {
    description = "Copy rapfi executables to jniLibs for Android"

    doLast {
        val archMap = mapOf(
            "arm64-v8a" to "../../../binaries/rapfi-aarch64-linux-android",
            "armeabi-v7a" to "../../../binaries/rapfi-armv7-linux-androideabi",
            "x86_64" to "../../../binaries/rapfi-x86_64-linux-android",
            "x86" to "../../../binaries/rapfi-i686-linux-android"
        )

        archMap.forEach { (abi, source) ->
            val sourceFile = file(source)
            if (sourceFile.exists()) {
                val targetDir = file("src/main/jniLibs/$abi")
                targetDir.mkdirs()
                val targetFile = file("$targetDir/librapfi.so")
                copy {
                    from(sourceFile)
                    into(targetDir)
                    rename { "librapfi.so" }
                }
                targetFile.setExecutable(true, false)
                println("✅ Copied rapfi for $abi: ${sourceFile.name} -> $targetFile (.so)")
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
            "armeabi-v7a" to "arm-linux-androideabi",
            "x86_64" to "x86_64-linux-android",
            "x86" to "i686-linux-android"
        )

        ndkArchMap.forEach { (abi, ndkArch) ->
            val libFile = file("$ndkSysroot/$ndkArch/libc++_shared.so")
            if (libFile.exists()) {
                val targetDir = file("src/main/jniLibs/$abi")
                targetDir.mkdirs()
                copy {
                    from(libFile)
                    into(targetDir)
                }
                println("✅ Copied libc++_shared.so for $abi")
            } else {
                println("⚠️  Warning: libc++_shared.so not found for $abi at $libFile")
            }
        }
    }
}

// Ensure rapfi binaries are copied before preBuild
tasks.named("preBuild") {
    dependsOn("copyRapfiBinaries")
}

dependencies {
    implementation("androidx.webkit:webkit:1.14.0")
    implementation("androidx.appcompat:appcompat:1.7.1")
    implementation("androidx.activity:activity-ktx:1.10.1")
    implementation("com.google.android.material:material:1.12.0")
    testImplementation("junit:junit:4.13.2")
    androidTestImplementation("androidx.test.ext:junit:1.1.4")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.5.0")
}

apply(from = "tauri.build.gradle.kts")