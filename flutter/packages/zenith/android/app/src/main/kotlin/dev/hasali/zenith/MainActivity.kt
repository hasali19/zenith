package dev.hasali.zenith

import android.app.Activity
import android.app.PictureInPictureParams
import android.content.ComponentName
import android.content.Intent
import android.content.res.Configuration
import android.os.Build
import android.os.Bundle
import android.provider.OpenableColumns
import android.view.WindowManager
import android.widget.Toast
import androidx.core.net.toUri
import androidx.core.view.ViewCompat
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat
import androidx.lifecycle.lifecycleScope
import androidx.work.Constraints
import androidx.work.Data
import androidx.work.NetworkType
import androidx.work.OneTimeWorkRequestBuilder
import androidx.work.WorkManager
import io.flutter.embedding.android.FlutterActivity
import io.flutter.embedding.engine.FlutterEngine
import io.flutter.plugin.common.MethodCall
import io.flutter.plugin.common.MethodChannel
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import java.util.UUID

class MainActivity : FlutterActivity() {

    private lateinit var updaterChannel: MethodChannel
    private lateinit var platformChannel: MethodChannel
    private lateinit var downloaderChannel: MethodChannel

    private var isPipModeEnabled = false

    private var nextRequestCode = 424242
    private val pendingResultCallbacks =
        mutableMapOf<Int, (resultCode: Int, data: Intent?) -> Unit>()

    object Channels {
        const val UPDATER = "zenith.hasali.dev/updater"
        const val PLATFORM = "zenith.hasali.dev/platform"
        const val DOWNLOADER = "zenith.hasali.dev/downloader"
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        NotificationChannels.createAll(this)
    }

    override fun configureFlutterEngine(flutterEngine: FlutterEngine) {
        super.configureFlutterEngine(flutterEngine)

        val messenger = flutterEngine.dartExecutor.binaryMessenger

        updaterChannel = MethodChannel(messenger, Channels.UPDATER).also {
            it.setMethodCallHandler(::handleUpdaterMethodCall)
        }

        platformChannel = MethodChannel(messenger, Channels.PLATFORM).also {
            it.setMethodCallHandler(::handlePlatformMethodCall)
        }

        downloaderChannel = MethodChannel(messenger, Channels.DOWNLOADER).also {
            it.setMethodCallHandler(::handleDownloaderMethodCall)
        }

        ViewCompat.setOnApplyWindowInsetsListener(window.decorView) { _, insets ->
            val stableSystemBars =
                insets.getInsetsIgnoringVisibility(WindowInsetsCompat.Type.systemBars())
            platformChannel.invokeMethod(
                "setStableSystemBarInsets", mapOf(
                    "top" to stableSystemBars.top,
                    "bottom" to stableSystemBars.bottom,
                    "left" to stableSystemBars.left,
                    "right" to stableSystemBars.right,
                )
            )
            insets
        }
    }

    private fun handleUpdaterMethodCall(call: MethodCall, result: MethodChannel.Result) {
        when (call.method) {
            "installWithLuna" -> {
                val intent = Intent().apply {
                    component =
                        ComponentName("dev.hasali.luna", "dev.hasali.luna.InAppUpdateActivity")
                    putExtra("packageName", packageName)
                }

                startActivityForResult(intent) { _, _ ->
                    result.success(Unit)
                }
            }

            else -> result.notImplemented()
        }
    }

    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        super.onActivityResult(requestCode, resultCode, data)
        val callback = pendingResultCallbacks.remove(requestCode)
        if (callback != null) {
            callback(resultCode, data)
        }
    }

    private fun handlePlatformMethodCall(call: MethodCall, result: MethodChannel.Result) {
        when (call.method) {
            "getSupportedAbis" -> {
                result.success(Build.SUPPORTED_ABIS.toList())
            }

            "setPipEnabled" -> {
                val isPipModeEnabled = call.arguments as Boolean
                if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
                    setPictureInPictureParams(
                        PictureInPictureParams.Builder()
                            // TODO: Set video aspect ratio
                            // .setAspectRatio(...)
                            .apply {
                                if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
                                    setAutoEnterEnabled(isPipModeEnabled)
                                }
                            }
                            .build()
                    )
                } else {
                    this.isPipModeEnabled = true
                }
                result.success(null)
            }

            "setExtendIntoCutout" -> {
                val extendIntoCutout = call.arguments as Boolean
                if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.P) {
                    window.attributes = window.attributes.apply {
                        layoutInDisplayCutoutMode = if (extendIntoCutout) {
                            WindowManager.LayoutParams.LAYOUT_IN_DISPLAY_CUTOUT_MODE_SHORT_EDGES
                        } else {
                            WindowManager.LayoutParams.LAYOUT_IN_DISPLAY_CUTOUT_MODE_DEFAULT
                        }
                    }
                }
                result.success(null)
            }

            "setSystemBarsVisible" -> {
                val visible = call.arguments as Boolean
                val controller = WindowCompat.getInsetsController(window, window.decorView)
                if (visible) {
                    controller.show(WindowInsetsCompat.Type.statusBars() or WindowInsetsCompat.Type.navigationBars())
                } else {
                    controller.hide(WindowInsetsCompat.Type.statusBars() or WindowInsetsCompat.Type.navigationBars())
                    controller.systemBarsBehavior =
                        WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE
                }
                result.success(null)
            }

            "showFilePicker" -> {
                val intent = Intent(Intent.ACTION_GET_CONTENT)
                    .addCategory(Intent.CATEGORY_OPENABLE)
                    .setType("*/*")

                startActivityForResult(intent) { resultCode, data ->
                    val uri = data?.takeIf { resultCode == Activity.RESULT_OK }?.data
                    if (uri == null) {
                        result.success(null)
                    } else {
                        lifecycleScope.launch {
                            withContext(Dispatchers.IO) {
                                val name = contentResolver.query(uri, null, null, null, null)
                                    .use { cursor ->
                                        if (cursor != null && cursor.moveToFirst()) {
                                            val index =
                                                cursor.getColumnIndexOrThrow(OpenableColumns.DISPLAY_NAME)
                                            cursor.getString(index)
                                        } else {
                                            null
                                        }
                                    }

                                val bytes =
                                    contentResolver.openInputStream(uri)?.use { it.readBytes() }

                                result.success(
                                    mapOf(
                                        "path" to uri.toString(),
                                        "name" to name,
                                        "bytes" to bytes,
                                    )
                                )
                            }
                        }
                    }
                }
            }

            else -> result.notImplemented()
        }
    }

    private fun handleDownloaderMethodCall(call: MethodCall, result: MethodChannel.Result) {
        when (call.method) {
            "init" -> {
                val callbackDispatcherHandle = call.argument<Long>("callbackDispatcherHandle")
                if (callbackDispatcherHandle == null) {
                    result.error("missing_argument", "callbackDispatcherHandle is required", null)
                    return
                }

                SharedPreferencesHelper
                    .setCallbackDispatcherHandle(this, callbackDispatcherHandle)

                result.success(null)
            }

            "enqueue" -> {
                val id = UUID.fromString(call.argument<String>("id")!!)
                val uri = call.argument<String>("uri")!!
                val cookies = call.argument<String>("cookies")
                val filename = call.argument<String>("filename")!!

                val constraints = Constraints.Builder()
                    .setRequiredNetworkType(NetworkType.UNMETERED)
                    .build()

                val inputData = Data.Builder()
                    .putString("uri", uri)
                    .putString("cookies", cookies)
                    .putString("filename", filename)
                    .build()

                val request = OneTimeWorkRequestBuilder<DownloadWorker>()
                    .setId(id)
                    .setConstraints(constraints)
                    .setInputData(inputData)
                    .build()

                WorkManager.getInstance(this)
                    .enqueue(request)

                DownloadWorker.showStartingNotification(this, id, filename)
            }

            "cancel" -> {
                val id = UUID.fromString(call.argument("id"))
                WorkManager.getInstance(this).cancelWorkById(id)
                result.success(null)
            }

            "deleteFile" -> {
                val uri = call.argument<String>("uri")!!.toUri()
                lifecycleScope.launch {
                    withContext(Dispatchers.IO) {
                        try {
                            contentResolver.delete(uri, null, null)
                            result.success(null)
                        } catch (e: Exception) {
                            runOnUiThread {
                                Toast.makeText(context, "Failed to delete file", Toast.LENGTH_SHORT)
                                    .show()
                            }
                            result.error("error", e.message, null)
                        }
                    }
                }
            }

            else -> result.notImplemented()
        }
    }

    override fun onPictureInPictureModeChanged(
        isInPictureInPictureMode: Boolean,
        newConfig: Configuration
    ) {
        super.onPictureInPictureModeChanged(isInPictureInPictureMode, newConfig)
        platformChannel.invokeMethod("setIsInPipMode", isInPictureInPictureMode)
    }

    override fun onUserLeaveHint() {
        super.onUserLeaveHint()
        if (isPipModeEnabled) {
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.N) {
                @Suppress("DEPRECATION")
                enterPictureInPictureMode()
            }
        }
    }

    private fun startActivityForResult(
        intent: Intent,
        callback: (resultCode: Int, data: Intent?) -> Unit
    ) {
        val requestCode = nextRequestCode++
        pendingResultCallbacks[requestCode] = callback
        startActivityForResult(intent, requestCode)
    }
}
