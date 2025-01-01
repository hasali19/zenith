package dev.hasali.zenith

import android.app.PictureInPictureParams
import android.content.ComponentName
import android.content.Intent
import android.content.res.Configuration
import android.net.Uri
import android.os.Build
import android.os.Bundle
import android.view.WindowManager
import androidx.core.view.ViewCompat
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat
import androidx.work.Constraints
import androidx.work.Data
import androidx.work.NetworkType
import androidx.work.OneTimeWorkRequestBuilder
import androidx.work.WorkManager
import io.flutter.embedding.android.FlutterActivity
import io.flutter.embedding.engine.FlutterEngine
import io.flutter.plugin.common.MethodCall
import io.flutter.plugin.common.MethodChannel
import java.util.UUID
import java.util.concurrent.Executors

class MainActivity : FlutterActivity() {

    private val executor = Executors.newCachedThreadPool()

    private lateinit var updaterChannel: MethodChannel
    private lateinit var platformChannel: MethodChannel
    private lateinit var downloaderChannel: MethodChannel

    private var isPipModeEnabled = false
    private var nextInstallId = 424242

    private val pendingInstalls = mutableMapOf<Int, (resultCode: Int) -> Unit>()

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

                val id = nextInstallId++
                pendingInstalls[id] = {
                    result.success(id)
                }
                startActivityForResult(intent, id)
            }

            else -> result.notImplemented()
        }
    }

    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        super.onActivityResult(requestCode, resultCode, data)
        val pendingInstallCallback = pendingInstalls.remove(requestCode)
        if (pendingInstallCallback != null) {
            pendingInstallCallback(resultCode)
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
                val uri = Uri.parse(call.argument("uri")!!)
                context.contentResolver.delete(uri, null, null)
                result.success(null)
            }

            else -> result.notImplemented()
        }
    }

    override fun onPictureInPictureModeChanged(
        isInPictureInPictureMode: Boolean,
        newConfig: Configuration?
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
}
