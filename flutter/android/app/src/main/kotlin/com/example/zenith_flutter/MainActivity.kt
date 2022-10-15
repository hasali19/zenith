package com.example.zenith_flutter

import io.flutter.embedding.android.FlutterActivity
import io.flutter.embedding.engine.FlutterEngine
import io.flutter.plugin.common.MethodCall
import io.flutter.plugin.common.MethodChannel
import java.util.concurrent.Executors

class MainActivity : FlutterActivity() {

    private val executor = Executors.newCachedThreadPool()

    private lateinit var updaterChannel: MethodChannel

    object Channels {
        const val Updater = "zenith.hasali.uk/updater"
    }

    override fun configureFlutterEngine(flutterEngine: FlutterEngine) {
        super.configureFlutterEngine(flutterEngine)

        val messenger = flutterEngine.dartExecutor.binaryMessenger

        updaterChannel = MethodChannel(messenger, Channels.Updater).apply {
            setMethodCallHandler(this@MainActivity::handleUpdaterMethodCall)
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
}
