package com.example.video_player_android

import android.app.Activity
import android.content.Context
import android.net.Uri
import android.os.Build
import android.view.Surface
import android.view.WindowManager
import androidx.annotation.NonNull
import androidx.media3.common.MediaItem
import androidx.media3.common.Player
import androidx.media3.common.VideoSize
import androidx.media3.exoplayer.ExoPlayer
import androidx.media3.exoplayer.trackselection.DefaultTrackSelector

import io.flutter.embedding.engine.plugins.FlutterPlugin
import io.flutter.embedding.engine.plugins.activity.ActivityAware
import io.flutter.embedding.engine.plugins.activity.ActivityPluginBinding
import io.flutter.plugin.common.EventChannel
import io.flutter.plugin.common.MethodChannel
import io.flutter.plugin.common.MethodChannel.Result
import io.flutter.view.TextureRegistry

/** VideoPlayerAndroidPlugin */
class VideoPlayerAndroidPlugin : FlutterPlugin, ActivityAware {
    private lateinit var applicationContext: Context
    private lateinit var textureRegistry: TextureRegistry
    private lateinit var methodChannel: MethodChannel
    private lateinit var eventChannel: EventChannel

    private var activity: Activity? = null
    private val players = mutableMapOf<Long, PlayerInstance>()

    override fun onAttachedToEngine(@NonNull flutterPluginBinding: FlutterPlugin.FlutterPluginBinding) {
        applicationContext = flutterPluginBinding.applicationContext
        textureRegistry = flutterPluginBinding.textureRegistry

        methodChannel =
            MethodChannel(flutterPluginBinding.binaryMessenger, "video_player_android").apply {
                setMethodCallHandler { call, result ->
                    val responder = Responder(result)
                    when (call.method) {
                        "create" -> responder.create()
                        "load" -> responder.load(
                            id = call.argument("id")!!,
                            url = call.argument("url")!!,
                            startPosition = call.argument("startPosition")!!,
                        )
                        "play" -> responder.play(id = call.argument("id")!!)
                        "pause" -> responder.pause(id = call.argument("id")!!)
                        "seekTo" -> responder.seekTo(
                            id = call.argument("id")!!,
                            position = call.argument("position")!!,
                        )
                        "dispose" -> responder.dispose(id = call.argument("id")!!)
                        "extendIntoCutout" -> responder.extendIntoCutout()
                        "unsetExtendIntoCutout" -> responder.unsetExtendIntoCutout()
                    }
                }
            }

        eventChannel = EventChannel(
            flutterPluginBinding.binaryMessenger,
            "video_player_android/events"
        ).apply {
            setStreamHandler(object : EventChannel.StreamHandler {
                private fun Any.toId(): Long {
                    return when (this) {
                        is Long -> this
                        is Int -> toLong()
                        else -> throw IllegalArgumentException("id must be an integer")
                    }
                }

                override fun onListen(arguments: Any?, events: EventChannel.EventSink?) {
                    requireNotNull(arguments)
                    requireNotNull(events)
                    players[arguments.toId()]!!.setEventCallback {
                        when (it) {
                            is PlayerInstance.Event.AspectRatioChanged -> events.success(
                                mapOf(
                                    "type" to "aspectRatioChanged",
                                    "value" to it.value,
                                )
                            )
                            is PlayerInstance.Event.DurationChanged -> events.success(
                                mapOf(
                                    "type" to "durationChanged",
                                    "value" to it.value,
                                    "position" to it.position,
                                )
                            )
                            is PlayerInstance.Event.PlayWhenReadyChanged -> events.success(
                                mapOf(
                                    "type" to "playWhenReadyChanged",
                                    "value" to it.value,
                                    "position" to it.position,
                                )
                            )
                            is PlayerInstance.Event.PlaybackStateChanged -> events.success(
                                mapOf(
                                    "type" to "playbackStateChanged",
                                    "value" to it.state.value,
                                    "position" to it.position,
                                )
                            )
                            is PlayerInstance.Event.IsPlayingChanged -> events.success(
                                mapOf(
                                    "type" to "isPlayingChanged",
                                    "value" to it.value,
                                    "position" to it.position,
                                )
                            )
                            is PlayerInstance.Event.PositionDiscontinuity -> events.success(
                                mapOf(
                                    "type" to "positionDiscontinuity",
                                    "position" to it.position,
                                )
                            )
                        }
                    }
                }

                override fun onCancel(arguments: Any?) {
                    requireNotNull(arguments)
                    players[arguments.toId()]!!.setEventCallback(null)
                }
            })
        }
    }

    override fun onDetachedFromEngine(@NonNull binding: FlutterPlugin.FlutterPluginBinding) {
        methodChannel.setMethodCallHandler(null)
        eventChannel.setStreamHandler(null)
        players.values.forEach { it.release() }
        players.clear()
    }

    override fun onAttachedToActivity(binding: ActivityPluginBinding) {
        activity = binding.activity
    }

    override fun onDetachedFromActivityForConfigChanges() {
        activity = null
    }

    override fun onReattachedToActivityForConfigChanges(binding: ActivityPluginBinding) {
        activity = binding.activity
    }

    override fun onDetachedFromActivity() {
        activity = null
    }

    private fun setWindowLayoutInDisplayCutoutMode(getter: () -> Int) {
        activity?.window?.let { window ->
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.P) {
                window.attributes = window.attributes.apply {
                    layoutInDisplayCutoutMode = getter()
                }
            }
        }
    }

    private inner class Responder(private val result: Result) {
        fun create() {
            val texture = textureRegistry.createSurfaceTexture()
            val player = PlayerInstance(applicationContext, texture)
            players[texture.id()] = player
            result.success(texture.id())
        }

        fun load(id: Long, url: String, startPosition: Long) {
            players[id]!!.load(url, startPosition)
            result.success(null)
        }

        fun play(id: Long) {
            players[id]!!.play()
            result.success(null)
        }

        fun pause(id: Long) {
            players[id]!!.pause()
            result.success(null)
        }

        fun seekTo(id: Long, position: Long) {
            players[id]!!.seekTo(position)
            result.success(null)
        }

        fun dispose(id: Long) {
            players.remove(id)!!.release()
            result.success(null)
        }

        fun extendIntoCutout() {
            setWindowLayoutInDisplayCutoutMode { WindowManager.LayoutParams.LAYOUT_IN_DISPLAY_CUTOUT_MODE_SHORT_EDGES }
            result.success(null)
        }

        fun unsetExtendIntoCutout() {
            setWindowLayoutInDisplayCutoutMode { WindowManager.LayoutParams.LAYOUT_IN_DISPLAY_CUTOUT_MODE_DEFAULT }
            result.success(null)
        }
    }
}

private typealias EventCallback = (event: PlayerInstance.Event) -> Unit

private class PlayerInstance(
    context: Context,
    private val texture: TextureRegistry.SurfaceTextureEntry
) {
    enum class PlaybackState(val value: Int) {
        Idle(0),
        Active(1),
        Ended(2),
    }

    sealed class Event {
        data class AspectRatioChanged(val value: Double) : Event()
        data class DurationChanged(val value: Long, val position: Long) : Event()
        data class PlaybackStateChanged(val state: PlaybackState, val position: Long) : Event()
        data class PlayWhenReadyChanged(val value: Boolean, val position: Long) : Event()
        data class IsPlayingChanged(val value: Boolean, val position: Long) : Event()
        data class PositionDiscontinuity(val position: Long) : Event()
    }

    private val surface = Surface(texture.surfaceTexture())
    private val trackSelector = DefaultTrackSelector(context)
    private val player = ExoPlayer.Builder(context)
        .setTrackSelector(trackSelector)
        .build()

    private var previousDuration = 0L
    private var onEvent: EventCallback? = null

    init {
        player.setVideoSurface(surface)
        player.addListener(object : Player.Listener {
            override fun onVideoSizeChanged(videoSize: VideoSize) {
                val aspectRatio = if (videoSize.width == 0 || videoSize.height == 0) {
                    0.0
                } else {
                    videoSize.width.toDouble() * videoSize.pixelWidthHeightRatio / videoSize.height.toDouble()
                }
                onEvent?.invoke(Event.AspectRatioChanged(aspectRatio))
            }

            override fun onPlayWhenReadyChanged(playWhenReady: Boolean, reason: Int) {
                onEvent?.invoke(Event.PlayWhenReadyChanged(playWhenReady, player.currentPosition))
            }

            override fun onPlaybackStateChanged(playbackState: Int) {
                if (player.duration != previousDuration) {
                    previousDuration = player.duration
                    onEvent?.invoke(Event.DurationChanged(player.duration, player.currentPosition))
                }
                val state = when (playbackState) {
                    ExoPlayer.STATE_IDLE -> PlaybackState.Idle
                    ExoPlayer.STATE_READY, ExoPlayer.STATE_BUFFERING -> PlaybackState.Active
                    ExoPlayer.STATE_ENDED -> PlaybackState.Ended
                    else -> throw IllegalArgumentException("Unknown playback state: $playbackState")
                }
                onEvent?.invoke(Event.PlaybackStateChanged(state, player.currentPosition))
            }

            override fun onIsPlayingChanged(isPlaying: Boolean) {
                onEvent?.invoke(Event.IsPlayingChanged(isPlaying, player.currentPosition))
            }

            override fun onPositionDiscontinuity(
                oldPosition: Player.PositionInfo,
                newPosition: Player.PositionInfo,
                reason: Int
            ) {
                onEvent?.invoke(Event.PositionDiscontinuity(newPosition.positionMs))
            }
        })
    }

    fun setEventCallback(callback: EventCallback?) {
        onEvent = callback
    }

    fun load(url: String, startPosition: Long) {
        player.setMediaItem(MediaItem.fromUri(Uri.parse(url)), startPosition)
        player.prepare()
        player.play()
    }

    fun play() {
        player.play()
    }

    fun pause() {
        player.pause()
    }

    fun seekTo(position: Long) {
        player.seekTo(position)
    }

    fun release() {
        player.release()
        surface.release()
        texture.release()
    }
}
