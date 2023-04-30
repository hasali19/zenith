package dev.hasali.zenith

import android.app.PictureInPictureParams
import android.content.res.Configuration
import android.os.Build
import android.view.WindowManager
import androidx.core.view.ViewCompat
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat
import io.flutter.embedding.android.FlutterActivity
import io.flutter.embedding.engine.FlutterEngine
import io.flutter.plugin.common.MethodCall
import io.flutter.plugin.common.MethodChannel
import java.util.concurrent.Executors

class MainActivity : FlutterActivity() {

    private val executor = Executors.newCachedThreadPool()

    private lateinit var updaterChannel: MethodChannel
    private lateinit var platformChannel: MethodChannel

    private var isPipModeEnabled = false

    object Channels {
        const val Updater = "zenith.hasali.dev/updater"
        const val Platform = "zenith.hasali.dev/platform"
    }

    override fun configureFlutterEngine(flutterEngine: FlutterEngine) {
        super.configureFlutterEngine(flutterEngine)

        val messenger = flutterEngine.dartExecutor.binaryMessenger

        updaterChannel = MethodChannel(messenger, Channels.Updater).apply {
            setMethodCallHandler(this@MainActivity::handleUpdaterMethodCall)
        }

        platformChannel = MethodChannel(messenger, Channels.Platform).apply {
            setMethodCallHandler(this@MainActivity::handlePlatformMethodCall)
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
                val controller = WindowCompat.getInsetsController(window, window.decorView)!!
                if (visible) {
                    controller.show(WindowInsetsCompat.Type.statusBars() or WindowInsetsCompat.Type.navigationBars())
                } else {
                    controller.hide(WindowInsetsCompat.Type.statusBars() or WindowInsetsCompat.Type.navigationBars())
                    controller.systemBarsBehavior =
                        WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE
                }
                result.success(null)
            }
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
