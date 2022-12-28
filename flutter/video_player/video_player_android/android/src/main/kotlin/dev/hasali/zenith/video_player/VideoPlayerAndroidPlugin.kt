package dev.hasali.zenith.video_player

import android.app.Activity
import android.content.Context
import android.net.Uri
import android.view.Surface
import android.widget.Toast
import androidx.media3.common.*
import androidx.media3.common.text.CueGroup
import androidx.media3.datasource.DefaultHttpDataSource
import androidx.media3.exoplayer.ExoPlayer
import androidx.media3.exoplayer.source.DefaultMediaSourceFactory
import androidx.media3.exoplayer.source.MergingMediaSource
import androidx.media3.exoplayer.source.SingleSampleMediaSource
import androidx.media3.exoplayer.trackselection.DefaultTrackSelector
import androidx.media3.session.MediaSession
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

    override fun onAttachedToEngine(flutterPluginBinding: FlutterPlugin.FlutterPluginBinding) {
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
                            subtitles = call.argument<List<Map<String, Any>>>("subtitles")!!
                                .map {
                                    SubtitleTrack(
                                        id = it["id"] as String,
                                        src = it["src"] as String,
                                        title = it["title"] as String?,
                                        language = it["language"] as String?,
                                    )
                                },
                            startPosition = call.argument("startPosition")!!,
                        )
                        "play" -> responder.play(id = call.argument("id")!!)
                        "pause" -> responder.pause(id = call.argument("id")!!)
                        "seekTo" -> responder.seekTo(
                            id = call.argument("id")!!,
                            position = call.argument("position")!!,
                        )
                        "setTextTrack" -> responder.setTextTrack(
                            id = call.argument("id")!!,
                            trackId = call.argument("trackId"),
                        )
                        "dispose" -> responder.dispose(id = call.argument("id")!!)
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
                            is PlayerInstance.Event.Cues -> events.success(
                                mapOf(
                                    "type" to "cues",
                                    "text" to it.text,
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

    override fun onDetachedFromEngine(binding: FlutterPlugin.FlutterPluginBinding) {
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

    private inner class Responder(private val result: Result) {
        fun create() {
            val texture = textureRegistry.createSurfaceTexture()
            val player = PlayerInstance(applicationContext, texture)
            players[texture.id()] = player
            result.success(texture.id())
        }

        fun load(id: Long, url: String, subtitles: List<SubtitleTrack>, startPosition: Long) {
            players[id]!!.load(url, subtitles, startPosition)
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

        fun setTextTrack(id: Long, trackId: String?) {
            println("Setting text track")
            players[id]!!.setTextTrack(trackId)
            result.success(null)
        }

        fun dispose(id: Long) {
            players.remove(id)!!.release()
            result.success(null)
        }
    }
}

private typealias EventCallback = (event: PlayerInstance.Event) -> Unit

data class SubtitleTrack(val id: String, val src: String, val title: String?, val language: String?)

private class PlayerInstance(
    private val context: Context,
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
        data class Cues(val text: String?) : Event()
    }

    private val surface = Surface(texture.surfaceTexture())
    private val trackSelector = DefaultTrackSelector(context)
    private val player = ExoPlayer.Builder(context)
        .setTrackSelector(trackSelector)
        .build()

    private val session = MediaSession.Builder(context, player)
        .build()

    private var textRenderer: Int? = null
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

            override fun onCues(cueGroup: CueGroup) {
                if (cueGroup.cues.isNotEmpty()) {
                    onEvent?.invoke(Event.Cues(cueGroup.cues[0].text?.toString()))
                } else {
                    onEvent?.invoke(Event.Cues(null))
                }
            }
        })

        for (i in 0 until player.rendererCount) {
            if (player.getRendererType(i) == C.TRACK_TYPE_TEXT) {
                textRenderer = i
                break
            }
        }

        // Disable the text renderer initially
        textRenderer.let { textRenderer ->
            if (textRenderer == null) {
                Toast.makeText(context, "Missing text renderer", Toast.LENGTH_LONG)
                    .show()
            } else {
                trackSelector.parameters = trackSelector.buildUponParameters()
                    .setRendererDisabled(textRenderer, true)
                    .build()
            }
        }
    }

    fun setEventCallback(callback: EventCallback?) {
        onEvent = callback
    }

    fun load(url: String, subtitles: List<SubtitleTrack>, startPosition: Long) {
        val mediaItem = MediaItem.Builder()
            .setUri(url)
            .build()

        val dataSourceFactory = DefaultHttpDataSource.Factory()
        val sources = mutableListOf(
            DefaultMediaSourceFactory(context)
                .createMediaSource(mediaItem)
        )

        subtitles.forEach {
            val uri = Uri.parse(it.src)

            val subtitle = MediaItem.SubtitleConfiguration.Builder(uri)
                .setId("external:${it.id}")
                .setMimeType(MimeTypes.TEXT_VTT)
                .setLanguage(it.language)
                .setLabel(it.title)
                .build()

            val source = SingleSampleMediaSource.Factory(dataSourceFactory)
                .createMediaSource(subtitle, C.TIME_UNSET)

            sources.add(source)
        }

        val mergedSource = MergingMediaSource(*sources.toTypedArray())

        player.setMediaSource(mergedSource, startPosition)
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

    fun setTextTrack(trackId: String?) {
        val renderer = textRenderer ?: return

        if (trackId == null) {
            trackSelector.parameters = trackSelector.buildUponParameters()
                .setRendererDisabled(renderer, true)
                .build()
        } else {
            val mappedTrackInfo = trackSelector.currentMappedTrackInfo ?: return
            val trackGroups = mappedTrackInfo.getTrackGroups(renderer)
            val requestedTrackId = "external:${trackId}"

            var group: TrackGroup? = null
            var track: Int? = null
            outer@ for (i in 0 until trackGroups.length) {
                group = trackGroups[i]
                for (j in 0 until group.length) {
                    val format = group.getFormat(j)
                    if (format.id == requestedTrackId) {
                        track = j
                        break@outer
                    }
                }
            }

            if (group == null || track == null) {
                val toast = Toast.makeText(
                    context,
                    "Failed to find requested subtitle track",
                    Toast.LENGTH_SHORT
                )
                return toast.show()
            }

            trackSelector.parameters = trackSelector.buildUponParameters()
                .setRendererDisabled(renderer, false)
                .setOverrideForType(TrackSelectionOverride(group, track))
                .build()
        }
    }

    fun release() {
        session.release()
        player.release()
        surface.release()
        texture.release()
    }
}
