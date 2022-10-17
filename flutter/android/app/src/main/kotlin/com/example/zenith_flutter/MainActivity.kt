package com.example.zenith_flutter

import android.app.PictureInPictureParams
import android.content.res.Configuration
import android.os.Build
import io.flutter.embedding.android.FlutterActivity
import io.flutter.embedding.engine.FlutterEngine
import io.flutter.plugin.common.MethodCall
import io.flutter.plugin.common.MethodChannel
import java.util.concurrent.Executors

class MainActivity : FlutterActivity() {

    private val executor = Executors.newCachedThreadPool()

    private lateinit var updaterChannel: MethodChannel
    private lateinit var pipChannel: MethodChannel

    private var isPipModeEnabled = false

    object Channels {
        const val Updater = "zenith.hasali.uk/updater"
        const val Pip = "zenith.hasali.uk/pip"
    }

    override fun configureFlutterEngine(flutterEngine: FlutterEngine) {
        super.configureFlutterEngine(flutterEngine)

        val messenger = flutterEngine.dartExecutor.binaryMessenger

        updaterChannel = MethodChannel(messenger, Channels.Updater).apply {
            setMethodCallHandler(this@MainActivity::handleUpdaterMethodCall)
        }

        pipChannel = MethodChannel(messenger, Channels.Pip).apply {
            setMethodCallHandler(this@MainActivity::handlePipMethodCall)
        }
    }

    private fun handleUpdaterMethodCall(call: MethodCall, result: MethodChannel.Result) {
        when (call.method) {
            "install" -> {
                try {
                    val artifactId: Int = call.argument("artifactId")
                        ?: return result.error("missing_param", "artifactId is required", null)
                    install(artifactId)
                    result.success(null)
                } catch (ex: Exception) {
                    result.error(ex.javaClass.canonicalName!!, ex.message, null)
                }
            }
            else -> result.notImplemented()
        }
    }

    private fun handlePipMethodCall(call: MethodCall, result: MethodChannel.Result) {
        when (call.method) {
            "setPipEnabled" -> {
                isPipModeEnabled = call.argument("enabled")
                    ?: return result.error("missing_param", "enabled is required", null)

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
        }
    }

    private fun install(artifactId: Int) {
        executor.execute {
            AppUpdater(this)
                .downloadAndInstall("https://nightly.link/hasali19/zenith/actions/artifacts/$artifactId.zip") {
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
        pipChannel.invokeMethod("notifyPipChanged", isInPictureInPictureMode)
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
