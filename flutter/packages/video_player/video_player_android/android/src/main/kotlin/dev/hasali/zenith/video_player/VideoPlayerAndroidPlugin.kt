package dev.hasali.zenith.video_player

import android.app.Activity
import android.content.Context
import android.net.Uri
import android.widget.Toast
import androidx.annotation.OptIn
import androidx.media3.common.C
import androidx.media3.common.MediaItem
import androidx.media3.common.MediaItem.SubtitleConfiguration
import androidx.media3.common.PlaybackParameters
import androidx.media3.common.Player
import androidx.media3.common.TrackGroup
import androidx.media3.common.TrackSelectionOverride
import androidx.media3.common.Tracks
import androidx.media3.common.VideoSize
import androidx.media3.common.text.CueGroup
import androidx.media3.common.util.UnstableApi
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
import io.flutter.view.TextureRegistry.SurfaceProducer

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
                        "create" -> responder.create(headers = call.argument<Map<String, String>>("headers"))
                        "load" -> responder.load(
                            id = call.argument("id")!!,
                            items = call.argument<List<Map<String, Any>>>("items")!!.map { item ->
                                VideoItem(
                                    url = item.get("url") as String,
                                    subtitles = (item["subtitles"] as List<*>)
                                        .map { it as Map<*, *> }
                                        .map {
                                            SubtitleTrack(
                                                id = it["id"] as String,
                                                src = it["src"] as String,
                                                mimeType = it["mimeType"] as String,
                                                title = it["title"] as String?,
                                                language = it["language"] as String?,
                                            )
                                        },
                                )
                            },
                            startIndex = call.argument("startIndex")!!,
                            startPosition = call.argument("startPosition")!!,
                        )

                        "play" -> responder.play(id = call.argument("id")!!)
                        "pause" -> responder.pause(id = call.argument("id")!!)
                        "seekTo" -> responder.seekTo(
                            id = call.argument("id")!!,
                            position = call.argument("position")!!,
                        )

                        "seekToNextItem" -> responder.seekToNextItem(id = call.argument("id")!!)
                        "seekToPreviousItem" -> responder.seekToPreviousItem(id = call.argument("id")!!)

                        "setPlaybackSpeed" -> responder.setPlaybackSpeed(
                            id = call.argument("id")!!,
                            speed = call.argument("speed")!!,
                        )

                        "setAudioTrack" -> responder.setAudioTrack(
                            id = call.argument("id")!!,
                            index = call.argument("index")!!,
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

                            is PlayerInstance.Event.PlaybackSpeed -> events.success(
                                mapOf(
                                    "type" to "playbackSpeed",
                                    "speed" to it.speed,
                                    "position" to it.position,
                                )
                            )

                            is PlayerInstance.Event.MediaItemTransition -> events.success(
                                mapOf(
                                    "type" to "mediaItemTransition",
                                    "index" to it.index,
                                    "position" to it.position,
                                )
                            )

                            is PlayerInstance.Event.TextTracksChanged -> events.success(
                                mapOf(
                                    "type" to "textTracksChanged",
                                    "tracks" to it.tracks.map { group ->
                                        val format = group.getTrackFormat(0)
                                        mapOf(
                                            "id" to format.id,
                                            "label" to format.label,
                                            "lang" to format.language,
                                        )
                                    },
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
        fun create(headers: Map<String, String>?) {
            val surfaceProducer = textureRegistry.createSurfaceProducer()
            val player = PlayerInstance(applicationContext, surfaceProducer, headers ?: emptyMap())
            players[surfaceProducer.id()] = player
            result.success(surfaceProducer.id())
        }

        fun load(id: Long, items: List<VideoItem>, startIndex: Int, startPosition: Long) {
            players[id]!!.load(items, startIndex, startPosition)
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

        fun seekToNextItem(id: Long) {
            players[id]!!.seekToNextItem()
            result.success(null)
        }

        fun seekToPreviousItem(id: Long) {
            players[id]!!.seekToPreviousItem()
            result.success(null)
        }

        fun setPlaybackSpeed(id: Long, speed: Double) {
            players[id]!!.setPlaybackSpeed(speed)
            result.success(null)
        }

        fun setAudioTrack(id: Long, index: Int) {
            players[id]!!.setAudioTrack(index)
            result.success(null)
        }

        fun setTextTrack(id: Long, trackId: String?) {
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

data class VideoItem(
    val url: String,
    val subtitles: List<SubtitleTrack>,
)

data class SubtitleTrack(
    val id: String,
    val src: String,
    val mimeType: String,
    val title: String?,
    val language: String?,
)

@OptIn(UnstableApi::class)
private class PlayerInstance(
    private val context: Context,
    private val surfaceProducer: SurfaceProducer,
    private val headers: Map<String, String>,
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
        data class PlaybackSpeed(val speed: Double, val position: Long) : Event()
        data class MediaItemTransition(val index: Int, val position: Long) : Event()
        data class TextTracksChanged(val tracks: List<Tracks.Group>) : Event()
    }

    private val trackSelector = DefaultTrackSelector(context)
    private val player = ExoPlayer.Builder(context)
        .setTrackSelector(trackSelector)
        .build()

    private val session = MediaSession.Builder(context, player)
        .build()

    private var audioRenderer: Int? = null
    private var textRenderer: Int? = null

    private var previousDuration = 0L
    private var onEvent: EventCallback? = null

    init {
        surfaceProducer.setCallback(object : SurfaceProducer.Callback {
            override fun onSurfaceCreated() {
                player.setVideoSurface(surfaceProducer.surface)
            }

            override fun onSurfaceDestroyed() {
                player.setVideoSurface(null)
            }
        })

        player.setVideoSurface(surfaceProducer.surface)
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

            override fun onPlaybackParametersChanged(playbackParameters: PlaybackParameters) {
                onEvent?.invoke(
                    Event.PlaybackSpeed(
                        playbackParameters.speed.toDouble(),
                        player.currentPosition
                    )
                )
            }

            override fun onMediaItemTransition(mediaItem: MediaItem?, reason: Int) {
                onEvent?.invoke(
                    Event.MediaItemTransition(
                        player.currentMediaItemIndex,
                        player.currentPosition
                    )
                )
            }

            override fun onTracksChanged(tracks: Tracks) {
                onEvent?.invoke(Event.TextTracksChanged(tracks.groups.filter { it.type == C.TRACK_TYPE_TEXT }))
            }
        })

        for (i in 0 until player.rendererCount) {
            if (player.getRendererType(i) == C.TRACK_TYPE_AUDIO) {
                audioRenderer = i
            } else if (player.getRendererType(i) == C.TRACK_TYPE_TEXT) {
                textRenderer = i
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

        trackSelector.parameters = trackSelector.buildUponParameters()
            .setPreferredAudioLanguage("en")
            .build()
    }

    fun setEventCallback(callback: EventCallback?) {
        onEvent = callback
    }

    fun load(items: List<VideoItem>, startIndex: Int, startPosition: Long) {
        val httpDataSourceFactory = DefaultHttpDataSource.Factory()
        val dataSourceFactory = {
            val dataSource = httpDataSourceFactory.createDataSource()
            headers.forEach { (k, v) -> dataSource.setRequestProperty(k, v) }
            dataSource
        };

        val mediaSources = items.map { item ->
            val mediaItem = MediaItem.Builder()
                .setUri(item.url)
                .setSubtitleConfigurations(item.subtitles.map { track ->
                    SubtitleConfiguration.Builder(Uri.parse(track.src))
                        .setId("external:${track.id}")
                        .setMimeType(track.mimeType)
                        .setLanguage(track.language)
                        .setLabel(track.title)
                        .build()
                })
                .build()

            DefaultMediaSourceFactory(context)
                .setDataSourceFactory(dataSourceFactory)
                .createMediaSource(mediaItem)
        }

        player.setMediaSources(mediaSources, startIndex, startPosition)
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

    fun seekToNextItem() {
        player.seekToNextMediaItem()
    }

    fun seekToPreviousItem() {
        player.seekToPreviousMediaItem()
    }

    fun setPlaybackSpeed(speed: Double) {
        player.setPlaybackSpeed(speed.toFloat())
    }

    fun setTrackById(renderer: Int, id: String?) {
        if (id == null) {
            trackSelector.parameters = trackSelector.buildUponParameters()
                .setRendererDisabled(renderer, true)
                .build()
        } else {
            val mappedTrackInfo = trackSelector.currentMappedTrackInfo ?: return
            val trackGroups = mappedTrackInfo.getTrackGroups(renderer)

            var group: TrackGroup? = null
            var track: Int? = null
            outer@ for (i in 0 until trackGroups.length) {
                group = trackGroups[i]
                for (j in 0 until group.length) {
                    val format = group.getFormat(j)
                    if (format.id == id) {
                        track = j
                        break@outer
                    }
                }
            }

            if (group == null || track == null) {
                val toast =
                    Toast.makeText(context, "Failed to set requested track", Toast.LENGTH_SHORT)
                return toast.show()
            }

            trackSelector.parameters = trackSelector.buildUponParameters()
                .setRendererDisabled(renderer, false)
                .setOverrideForType(TrackSelectionOverride(group, track))
                .build()
        }
    }

    fun setAudioTrack(index: Int) {
        setTrackById(audioRenderer ?: return, (index + 1).toString())
    }

    fun setTextTrack(trackId: String?) {
        setTrackById(textRenderer ?: return, trackId)
    }

    fun release() {
        session.release()
        player.release()
        surfaceProducer.release()
    }
}
