package dev.hasali.zenith

import android.app.PictureInPictureParams
import android.content.res.Configuration
import android.os.Build
import android.view.WindowManager
import androidx.core.view.ViewCompat
import androidx.core.view.WindowCompat
import androidx.core.view.WindowInsetsCompat
import androidx.core.view.WindowInsetsControllerCompat
import androidx.mediarouter.media.MediaRouter
import com.google.android.gms.cast.MediaStatus.PLAYER_STATE_BUFFERING
import com.google.android.gms.cast.MediaStatus.PLAYER_STATE_IDLE
import com.google.android.gms.cast.MediaStatus.PLAYER_STATE_LOADING
import com.google.android.gms.cast.MediaStatus.PLAYER_STATE_PAUSED
import com.google.android.gms.cast.MediaStatus.PLAYER_STATE_PLAYING
import com.google.android.gms.cast.framework.CastContext
import com.google.android.gms.cast.framework.CastSession
import com.google.android.gms.cast.framework.CastState
import com.google.android.gms.cast.framework.media.RemoteMediaClient
import dev.hasali.zenith.generated.remoteplayback.MediaInfo
import dev.hasali.zenith.generated.remoteplayback.MediaStatus
import dev.hasali.zenith.generated.remoteplayback.PlayerState
import dev.hasali.zenith.generated.remoteplayback.RemotePlaybackApi
import dev.hasali.zenith.generated.remoteplayback.RemotePlaybackEventsApi
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

        val mediaRouter = MediaRouter.getInstance(this)
        val castContext = CastContext.getSharedInstance(this)

        val mediaRouterEventsApi =
            RemotePlaybackEventsApi(flutterEngine.dartExecutor.binaryMessenger)

        RemotePlaybackApi.setUp(
            flutterEngine.dartExecutor.binaryMessenger,
            RemotePlaybackApiImpl(
                mediaRouterEventsApi,
                mediaRouter,
                castContext.mergedSelector!!,
                castContext
            )
        )

        val remoteClientCallback = object : RemoteMediaClient.Callback() {
            override fun onStatusUpdated() {
                val session = castContext.sessionManager.currentCastSession ?: return
                val client = session.remoteMediaClient ?: return
                val status = client.mediaStatus ?: return
                val mediaInfo = status.mediaInfo ?: return

                val playerState = when (status.playerState) {
                    PLAYER_STATE_IDLE -> PlayerState.IDLE
                    PLAYER_STATE_BUFFERING -> PlayerState.BUFFERING
                    PLAYER_STATE_LOADING -> PlayerState.LOADING
                    PLAYER_STATE_PAUSED -> PlayerState.PAUSED
                    PLAYER_STATE_PLAYING -> PlayerState.PLAYING
                    else -> PlayerState.UNKNOWN
                }

                mediaRouterEventsApi.onStatusUpdated(
                    MediaStatus(
                        playerState = playerState,
                        mediaInfo = MediaInfo(
                            streamDuration = mediaInfo.streamDuration,
                        ),
                        streamPosition = status.streamPosition,
                        playbackRate = status.playbackRate,
                    )
                ) {}
            }
        }

        val sessionManagerListener = object : CastSessionManagerListener() {
            override fun onSessionEnding(session: CastSession) {
                session.remoteMediaClient!!.unregisterCallback(remoteClientCallback)
            }

            override fun onSessionStarted(session: CastSession, sessionId: String) {
                session.remoteMediaClient!!.registerCallback(remoteClientCallback)
            }

            override fun onSessionResumed(session: CastSession, wasSuspended: Boolean) {
                session.remoteMediaClient!!.registerCallback(remoteClientCallback)
            }
        }

        castContext.sessionManager.addSessionManagerListener(
            sessionManagerListener,
            CastSession::class.java
        )

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
