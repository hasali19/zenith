package uk.hasali.zenith

import android.app.PendingIntent
import android.content.Context
import android.content.Intent
import android.content.pm.PackageInstaller
import android.os.Handler
import android.os.Looper
import android.view.Surface
import com.google.android.exoplayer2.MediaItem
import com.google.android.exoplayer2.SimpleExoPlayer
import com.google.android.exoplayer2.video.VideoListener
import io.flutter.embedding.android.FlutterActivity
import io.flutter.embedding.engine.FlutterEngine
import io.flutter.plugin.common.MethodChannel
import io.flutter.view.TextureRegistry
import io.ktor.client.*
import io.ktor.client.request.*
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import java.io.ByteArrayInputStream
import java.io.ByteArrayOutputStream
import java.io.IOException
import java.util.zip.ZipInputStream

private const val UPDATER_CHANNEL = "zenith.hasali.uk/updater"
private const val VIDEO_PLAYER_CHANNEL = "zenith.hasali.uk/video-player"

private const val PROGRESS_UPDATE_INTERVAL = 500L

class Player(context: Context, private val channel: MethodChannel, private val texture: TextureRegistry.SurfaceTextureEntry, private val url: String) {
    private val player: SimpleExoPlayer
    private var progressShouldLoop = true

    val textureId: Long
        get() = texture.id()

    init {
        val surface = Surface(texture.surfaceTexture())
        val player = SimpleExoPlayer.Builder(context)
                .build()

        player.addVideoListener(object : VideoListener {
            override fun onVideoSizeChanged(width: Int, height: Int, unappliedRotationDegrees: Int, pixelWidthHeightRatio: Float) {
                val aspectRatio = width * pixelWidthHeightRatio / height
                channel.invokeMethod("onAspectRatioChanged", aspectRatio)
            }
        })

        player.addListener(object : com.google.android.exoplayer2.Player.EventListener {
            override fun onPlayWhenReadyChanged(playWhenReady: Boolean, reason: Int) {
                channel.invokeMethod("onPlaybackStateChanged", playWhenReady)
            }
        })

        launchProgressReporter()

        player.setVideoSurface(surface)
        player.setMediaItem(MediaItem.fromUri(url))
        player.prepare()
        player.play()

        this.player = player
    }

    fun play() {
        player.play()
    }

    fun pause() {
        player.pause()
    }

    private fun launchProgressReporter() {
        val handler = Handler(Looper.getMainLooper())
        var reporter: (() -> Unit)? = null

        reporter = {
            reporter?.let { runnable ->
                if (player.playWhenReady) {
                    channel.invokeMethod("onProgressUpdate", player.currentPosition)
                }

                if (progressShouldLoop) {
                    handler.postDelayed(runnable, PROGRESS_UPDATE_INTERVAL)
                }
            }
        }

        handler.postDelayed(reporter, PROGRESS_UPDATE_INTERVAL)
    }

    fun release() {
        progressShouldLoop = false
        player.release()
        texture.release()
    }
}

class MainActivity : FlutterActivity() {

    private var player: Player? = null

    override fun configureFlutterEngine(flutterEngine: FlutterEngine) {
        super.configureFlutterEngine(flutterEngine)

        MethodChannel(flutterEngine.dartExecutor.binaryMessenger, UPDATER_CHANNEL).setMethodCallHandler { call, result ->
            when (call.method) {
                "installApk" -> installApk(call.argument("url")!!)
                else -> result.notImplemented()
            }
        }

        val channel = MethodChannel(flutterEngine.dartExecutor.binaryMessenger, VIDEO_PLAYER_CHANNEL)

        channel.setMethodCallHandler { call, result ->
            when (call.method) {
                "init" -> {
                    if (player != null) {
                        result.error("already_initialised", "Player has already been initialised", null)
                    } else {
                        val url = call.arguments<String>()
                        val texture = flutterEngine.renderer.createSurfaceTexture()
                        player = Player(this, channel, texture, url)
                        result.success(texture.id())
                    }
                }
                "destroy" -> {
                    val id = when (val id = call.arguments<Any>()) {
                        is Int -> id.toLong()
                        is Long -> id
                        else -> throw ClassCastException()
                    }

                    if (player?.textureId != id) {
                        result.error("invalid_texture", "Invalid texture id", null)
                    }

                    player?.release()
                    player = null

                    result.success(true)
                }
                "pause" -> {
                    val id = when (val id = call.arguments<Any>()) {
                        is Int -> id.toLong()
                        is Long -> id
                        else -> throw ClassCastException()
                    }

                    if (player?.textureId != id) {
                        result.error("invalid_texture", "Invalid texture id", null)
                    }

                    player?.pause()

                    result.success(true)
                }
                "play" -> {
                    val id = when (val id = call.arguments<Any>()) {
                        is Int -> id.toLong()
                        is Long -> id
                        else -> throw ClassCastException()
                    }

                    if (player?.textureId != id) {
                        result.error("invalid_texture", "Invalid texture id", null)
                    }

                    player?.play()

                    result.success(true)
                }
                else -> result.notImplemented()
            }
        }
    }

    private fun installApk(url: String) {
        CoroutineScope(Dispatchers.Main).launch {
            val client = HttpClient()
            val response: ByteArray = client.request(url)
            val zip = ZipInputStream(ByteArrayInputStream(response))
            val entry = zip.nextEntry
            val content = ByteArrayOutputStream()

            zip.copyTo(content)

            var session: PackageInstaller.Session? = null
            try {
                val installer = packageManager.packageInstaller
                val params = PackageInstaller.SessionParams(PackageInstaller.SessionParams.MODE_FULL_INSTALL)

                withContext(Dispatchers.IO) {
                    val sessionId = installer.createSession(params)

                    session = installer.openSession(sessionId)
                    session?.let { session ->
                        session.openWrite("package", 0, -1).use { output ->
                            ByteArrayInputStream(content.toByteArray()).copyTo(output)
                            session.fsync(output)
                        }
                    }
                }

                val intent = Intent(application, InstallReceiver::class.java)
                val pendingIntent = PendingIntent.getBroadcast(application, 3439, intent, PendingIntent.FLAG_UPDATE_CURRENT)
                val receiver = pendingIntent.intentSender

                session?.commit(receiver)
                session?.close()
            } catch (e: IOException) {
                throw RuntimeException("Couldn't install package", e)
            } catch (e: RuntimeException) {
                session?.abandon()
                throw e
            }
        }

    }
}
