package dev.hasali.zenith.video_player

import android.app.Activity
import android.content.Context
import android.net.Uri
import android.widget.Toast
import androidx.annotation.OptIn
import androidx.media3.common.C
import androidx.media3.common.MediaItem
import androidx.media3.common.MediaItem.SubtitleConfiguration
import androidx.media3.common.MediaMetadata
import androidx.media3.common.PlaybackParameters
import androidx.media3.common.Player
import androidx.media3.common.TrackGroup
import androidx.media3.common.TrackSelectionOverride
import androidx.media3.common.Tracks
import androidx.media3.common.VideoSize
import androidx.media3.common.text.CueGroup
import androidx.media3.common.util.UnstableApi
import androidx.media3.datasource.ContentDataSource
import androidx.media3.datasource.DataSource
import androidx.media3.datasource.DataSourceBitmapLoader
import androidx.media3.datasource.DefaultHttpDataSource
import androidx.media3.exoplayer.ExoPlayer
import androidx.media3.exoplayer.source.DefaultMediaSourceFactory
import androidx.media3.exoplayer.trackselection.DefaultTrackSelector
import androidx.media3.session.CacheBitmapLoader
import androidx.media3.session.MediaSession
import androidx.media3.ui.PlayerNotificationManager
import com.google.common.util.concurrent.MoreExecutors
import io.flutter.embedding.engine.plugins.FlutterPlugin
import io.flutter.embedding.engine.plugins.activity.ActivityAware
import io.flutter.embedding.engine.plugins.activity.ActivityPluginBinding
import io.flutter.plugin.common.EventChannel
import io.flutter.plugin.common.MethodChannel
import io.flutter.plugin.common.MethodChannel.Result
import io.flutter.view.TextureRegistry
import io.flutter.view.TextureRegistry.SurfaceProducer
import java.util.concurrent.Executors

class VideoPlayerPlugin : FlutterPlugin, ActivityAware {
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
            MethodChannel(flutterPluginBinding.binaryMessenger, "video_player").apply {
                setMethodCallHandler { call, result ->
                    val responder = Responder(result)
                    when (call.method) {
                        "create" -> responder.create(headers = call.argument<Map<String, String>>("headers"))
                        "load" -> responder.load(
                            id = call.argument("id")!!,
                            items = call.argument<List<Map<String, Any>>>("items")!!.map { item ->
                                VideoItem(
                                    source = when {
                                        item.containsKey("url") -> VideoSource.Network(item["url"] as String)
                                        item.containsKey("path") -> VideoSource.LocalFile(item["path"] as String)
                                        else -> throw IllegalArgumentException("url or path is required")
                                    },
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
                                    title = item["title"] as String?,
                                    subtitle = item["subtitle"] as String?,
                                    seriesTitle = item["seriesTitle"] as String?,
                                    seasonNumber = item["seasonNumber"] as Int?,
                                    episodeNumber = item["episodeNumber"] as Int?,
                                    posterUrl = item["posterUrl"] as String?,
                                    backdropUrl = item["backdropUrl"] as String?,
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
            "video_player/events"
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
                            is PlayerInstance.Event.VideoSizeChanged -> events.success(
                                mapOf(
                                    "type" to "videoSizeChanged",
                                    "width" to it.width,
                                    "height" to it.height,
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

                            is PlayerInstance.Event.TracksChanged -> {
                                val tracks = mutableListOf<Map<String, Any?>>()
                                var activeTextTrack: String? = null

                                for (group in it.tracks) {
                                    val format = group.getTrackFormat(0)
                                    if (group.type == C.TRACK_TYPE_TEXT && group.isSelected) {
                                        activeTextTrack = format.id
                                    }

                                    tracks.add(
                                        mapOf(
                                            "id" to format.id,
                                            "type" to when (group.type) {
                                                C.TRACK_TYPE_VIDEO -> 1
                                                C.TRACK_TYPE_AUDIO -> 2
                                                C.TRACK_TYPE_TEXT -> 3
                                                else -> 4
                                            },
                                            "label" to format.label,
                                            "lang" to format.language,
                                            "codec" to format.codecs,
                                        )
                                    )
                                }

                                events.success(
                                    mapOf(
                                        "type" to "tracksChanged",
                                        "tracks" to tracks,
                                        "activeTextTrack" to activeTextTrack,
                                    )
                                )
                            }
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

sealed interface VideoSource {
    data class Network(val url: String) : VideoSource
    data class LocalFile(val path: String) : VideoSource
}

data class VideoItem(
    val source: VideoSource,
    val subtitles: List<SubtitleTrack>,
    val title: String?,
    val subtitle: String?,
    val seriesTitle: String?,
    val seasonNumber: Int?,
    val episodeNumber: Int?,
    val posterUrl: String?,
    val backdropUrl: String?,
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
        data class VideoSizeChanged(val width: Int, val height: Int) : Event()
        data class DurationChanged(val value: Long, val position: Long) : Event()
        data class PlaybackStateChanged(val state: PlaybackState, val position: Long) : Event()
        data class PlayWhenReadyChanged(val value: Boolean, val position: Long) : Event()
        data class IsPlayingChanged(val value: Boolean, val position: Long) : Event()
        data class PositionDiscontinuity(val position: Long) : Event()
        data class Cues(val text: String?) : Event()
        data class PlaybackSpeed(val speed: Double, val position: Long) : Event()
        data class MediaItemTransition(val index: Int, val position: Long) : Event()
        data class TracksChanged(val tracks: List<Tracks.Group>) : Event()
    }

    private val trackSelector = DefaultTrackSelector(context)
    private val player = ExoPlayer.Builder(context)
        .setTrackSelector(trackSelector)
        .build()

    private val httpDataSourceFactory =
        DefaultHttpDataSource.Factory().apply { setDefaultRequestProperties(headers) }
    private val contentDataSourceFactory = DataSource.Factory { ContentDataSource(context) }

    private val bitmapLoaderExecutorService = Executors.newSingleThreadExecutor()
    private val session = MediaSession.Builder(context, player)
        .setBitmapLoader(
            CacheBitmapLoader(
                DataSourceBitmapLoader(
                    MoreExecutors.listeningDecorator(bitmapLoaderExecutorService),
                    httpDataSourceFactory
                )
            )
        )
        .build()

    private val notificationManager = PlayerNotificationManager.Builder(context, 424242, "media")
        .build()

    private var audioRenderer: Int? = null
    private var textRenderer: Int? = null

    private var previousDuration = 0L
    private var onEvent: EventCallback? = null

    init {
        surfaceProducer.setCallback(object : SurfaceProducer.Callback {
            override fun onSurfaceAvailable() {
                player.setVideoSurface(surfaceProducer.surface)
            }

            override fun onSurfaceDestroyed() {
                player.setVideoSurface(null)
            }
        })

        player.setVideoSurface(surfaceProducer.surface)
        player.addListener(object : Player.Listener {
            override fun onVideoSizeChanged(videoSize: VideoSize) {
                onEvent?.invoke(Event.VideoSizeChanged(videoSize.width, videoSize.height))
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
                onEvent?.invoke(Event.TracksChanged(tracks.groups))
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

        notificationManager.setPlayer(player)
        notificationManager.setMediaSessionToken(session.platformToken)
    }

    fun setEventCallback(callback: EventCallback?) {
        onEvent = callback
    }

    fun load(items: List<VideoItem>, startIndex: Int, startPosition: Long) {
        val mediaSources = items.map { item ->
            val uri = when (item.source) {
                is VideoSource.Network -> item.source.url
                is VideoSource.LocalFile -> item.source.path
            }

            val backdropUrl = when (item.backdropUrl) {
                null -> null
                else -> Uri.parse(item.backdropUrl)
            }

            val metadata = MediaMetadata.Builder()
                .setTitle(item.title)
                .setSubtitle(item.subtitle)
                .setArtworkUri(backdropUrl)
                .build()

            val subtitles = item.subtitles.map { track ->
                SubtitleConfiguration.Builder(Uri.parse(track.src))
                    .setId("external:${track.id}")
                    .setMimeType(track.mimeType)
                    .setLanguage(track.language)
                    .setLabel(track.title)
                    .build()
            }

            val mediaItem = MediaItem.Builder()
                .setUri(uri)
                .setSubtitleConfigurations(subtitles)
                .setMediaMetadata(metadata)
                .build()

            val dataSourceFactory = when (item.source) {
                is VideoSource.Network -> httpDataSourceFactory
                is VideoSource.LocalFile -> contentDataSourceFactory
            }

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
        bitmapLoaderExecutorService.shutdown()
        notificationManager.setPlayer(null)
        session.release()
        player.release()
        surfaceProducer.release()
    }
}
