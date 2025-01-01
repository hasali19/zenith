package dev.hasali.zenith

import android.content.Context
import android.os.Handler
import android.os.Looper
import io.flutter.embedding.engine.FlutterEngine
import io.flutter.embedding.engine.dart.DartExecutor
import io.flutter.embedding.engine.loader.FlutterLoader
import io.flutter.plugin.common.MethodChannel
import io.flutter.view.FlutterCallbackInformation
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import kotlin.coroutines.resume
import kotlin.coroutines.resumeWithException
import kotlin.coroutines.suspendCoroutine

class BackgroundFlutterEngine private constructor(
    private val flutterEngine: FlutterEngine,
) {
    val downloaderChannel: MethodChannel = MethodChannel(
        flutterEngine.dartExecutor.binaryMessenger,
        "zenith.hasali.dev/downloader"
    )

    companion object {

        private val mutex = Mutex()
        private val flutterLoader = FlutterLoader()
        private var engine: BackgroundFlutterEngine? = null

        suspend fun getInstance(context: Context): BackgroundFlutterEngine {
            return engine.let { engine ->
                mutex.withLock {
                    engine ?: initEngine(context).also { BackgroundFlutterEngine.engine = it }
                }
            }
        }

        private suspend fun initEngine(context: Context): BackgroundFlutterEngine {
            val flutterEngine = FlutterEngine(context)

            if (!flutterLoader.initialized()) {
                flutterLoader.startInitialization(context)
            }

            suspendCoroutine { continuation ->
                flutterLoader.ensureInitializationCompleteAsync(
                    context,
                    null,
                    Handler(Looper.getMainLooper())
                ) {
                    continuation.resume(Unit)
                }
            }

            val callbackDispatcherHandle =
                SharedPreferencesHelper.getCallbackDispatcherHandle(context)

            val dartCallback = DartExecutor.DartCallback(
                context.assets,
                flutterLoader.findAppBundlePath(),
                FlutterCallbackInformation.lookupCallbackInformation(callbackDispatcherHandle)
            )

            val channel = MethodChannel(
                flutterEngine.dartExecutor.binaryMessenger,
                "zenith.hasali.dev/downloader"
            )

            suspendCoroutine { continuation ->
                channel.setMethodCallHandler { call, result ->
                    if (call.method == "ready") {
                        result.success(null)
                        continuation.resume(Unit)
                    }
                }
                flutterEngine.dartExecutor.executeDartCallback(dartCallback)
            }

            return BackgroundFlutterEngine(flutterEngine)
        }
    }

    fun release() {
        flutterEngine.destroy()
    }
}

suspend fun MethodChannel.invokeMethodWithResult(name: String, args: Any?): Any? =
    suspendCoroutine { continuation ->
        invokeMethod(name, args, object : MethodChannel.Result {
            override fun success(result: Any?) {
                continuation.resume(result)
            }

            override fun error(errorCode: String, errorMessage: String?, errorDetails: Any?) {
                continuation.resumeWithException(Exception("Dart error: $errorCode - $errorMessage"))
            }

            override fun notImplemented() {
                continuation.resumeWithException(Exception("Method not implemented"))
            }
        })
    }
