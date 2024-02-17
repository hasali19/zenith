package dev.hasali.zenith

import android.app.PictureInPictureParams
import android.content.ComponentName
import android.content.Intent
import android.content.res.Configuration
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
        const val Updater = "zenith.hasali.dev/updater"
        const val Platform = "zenith.hasali.dev/platform"
        const val Downloader = "zenith.hasali.dev/downloader"
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        NotificationChannels.createAll(this)
    }

    override fun configureFlutterEngine(flutterEngine: FlutterEngine) {
        super.configureFlutterEngine(flutterEngine)

        val messenger = flutterEngine.dartExecutor.binaryMessenger

        updaterChannel = MethodChannel(messenger, Channels.Updater).also {
            it.setMethodCallHandler(::handleUpdaterMethodCall)
        }

        platformChannel = MethodChannel(messenger, Channels.Platform).also {
            it.setMethodCallHandler(::handlePlatformMethodCall)
        }

        downloaderChannel = MethodChannel(messenger, Channels.Downloader).also {
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
            "useLunaUpdater" -> {
                val installerPackageName = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.R) {
                    val installSource =
                        context.packageManager.getInstallSourceInfo(packageName)
                    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.UPSIDE_DOWN_CAKE) {
                        installSource.updateOwnerPackageName
                    } else {
                        installSource.installingPackageName
                    }
                } else {
                    @Suppress("DEPRECATION")
                    context.packageManager.getInstallerPackageName(packageName)
                }

                result.success(installerPackageName == "dev.hasali.luna")
            }

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

            "install" -> {
                try {
                    val url: String = call.argument("url")
                        ?: return result.error("missing_param", "url is required", null)
                    install(url)
                    result.success(null)
                } catch (ex: Exception) {
                    result.error(ex.javaClass.canonicalName!!, ex.message, null)
                }
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
            "enqueue" -> {
                val uri = call.argument<String>("uri")!!
                val cookies = call.argument<String>("cookies")
                val filename = call.argument<String>("filename")!!

                val id = UUID.randomUUID()

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

            else -> result.notImplemented()
        }
    }

    private fun install(url: String) {
        executor.execute {
            AppUpdater(this)
                .downloadAndInstall(url) {
                    runOnUiThread {
                        updaterChannel.invokeMethod("install/onProgress", it)
                    }
                }
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
