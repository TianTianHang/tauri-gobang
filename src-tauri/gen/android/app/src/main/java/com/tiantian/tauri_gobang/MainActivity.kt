package com.tiantian.tauri_gobang

import android.os.Bundle
import androidx.activity.enableEdgeToEdge

class MainActivity : TauriActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)
  }

  fun getNativeLibraryDir(): String {
    return applicationInfo.nativeLibraryDir
  }

  fun getRapfiExecutablePath(): String {
    val libDir = applicationInfo.nativeLibraryDir
    return "$libDir/librapfi.so"
  }
}
