package uk.hasali.zenith.media

import android.app.PendingIntent
import android.content.Context
import android.graphics.Bitmap
import android.net.Uri
import android.support.v4.media.session.MediaSessionCompat
import android.widget.Toast
import androidx.annotation.OptIn
import androidx.core.graphics.drawable.toBitmap
import androidx.media3.common.*
import androidx.media3.common.util.UnstableApi
import androidx.media3.datasource.DefaultHttpDataSource
import androidx.media3.exoplayer.ExoPlayer
import androidx.media3.exoplayer.source.DefaultMediaSourceFactory
import androidx.media3.exoplayer.source.MergingMediaSource
import androidx.media3.exoplayer.source.SingleSampleMediaSource
import androidx.media3.exoplayer.trackselection.DefaultTrackSelector
import androidx.media3.session.MediaSession
import androidx.media3.ui.PlayerNotificationManager
import coil.imageLoader
import coil.request.ImageRequest
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.MainScope
import kotlinx.coroutines.cancel
import kotlinx.coroutines.flow.*
import kotlinx.coroutines.launch

@OptIn(UnstableApi::class)
class LocalVideoPlayer(private val context: Context) : VideoPlayer {
    private val scope = MainScope()

    override val isLocal: Boolean
        get() = true

    private val trackSelector = DefaultTrackSelector(context)
    override val player = ExoPlayer.Builder(context)
        .setTrackSelector(trackSelector)
        .build()

    private val session = MediaSession.Builder(context, player)
        .build()

    private val notificationManager = PlayerNotificationManager.Builder(context, 1, "media")
        .setMediaDescriptionAdapter(object : PlayerNotificationManager.MediaDescriptionAdapter {
            override fun getCurrentContentTitle(player: Player): CharSequence {
                return player.currentMediaItem?.mediaMetadata?.title ?: "Unknown"
            }

            override fun createCurrentContentIntent(player: Player): PendingIntent? {
                return null
            }

            override fun getCurrentContentText(player: Player): CharSequence? {
                return player.currentMediaItem?.mediaMetadata?.subtitle
            }

            override fun getCurrentLargeIcon(
                player: Player,
                callback: PlayerNotificationManager.BitmapCallback
            ): Bitmap? {
                val artwork = artwork.value
                if (artwork == null) {
                    scope.launch {
                        callback.onBitmap(this@LocalVideoPlayer.artwork.filterNotNull().first())
                    }
                }
                return artwork
            }
        })
        .build()

    private var textRenderer: Int? = null

    private var _currentItem = MutableStateFlow<VideoItem?>(null)
    override val currentItem: StateFlow<VideoItem?>
        get() = _currentItem

    private var _subtitleTrack = MutableStateFlow<SubtitleTrack?>(null)
    override val subtitleTrack: StateFlow<SubtitleTrack?>
        get() = _subtitleTrack

    private val _state = MutableStateFlow(VideoPlayer.State.Active)
    override val state: StateFlow<VideoPlayer.State>
        get() = _state

    private var _isPlaying = MutableStateFlow(false)
    override val isPlaying: StateFlow<Boolean>
        get() = _isPlaying

    private var _playWhenReady = MutableStateFlow(false)
    override val playWhenReady: StateFlow<Boolean>
        get() = _playWhenReady

    override val position: Long
        get() = player.currentPosition

    private val listener = object : Player.Listener {
        override fun onIsPlayingChanged(isPlaying: Boolean) {
            _isPlaying.value = isPlaying
        }

        override fun onPlayWhenReadyChanged(playWhenReady: Boolean, reason: Int) {
            _playWhenReady.value = playWhenReady
        }

        override fun onPlaybackStateChanged(playbackState: Int) {
            if (playbackState == ExoPlayer.STATE_ENDED) {
                _state.value = VideoPlayer.State.Ended
            } else {
                _state.value = VideoPlayer.State.Active
            }
        }
    }

    @kotlin.OptIn(ExperimentalCoroutinesApi::class)
    private val artwork = currentItem
        .filterNotNull()
        .mapLatest {
            val result = context.imageLoader.execute(
                ImageRequest.Builder(context)
                    .data(it.poster)
                    .build()
            )

            result.drawable?.toBitmap()
        }
        .stateIn(scope, SharingStarted.Eagerly, null)

    init {
        player.addListener(listener)

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

        notificationManager.setPlayer(player)
        notificationManager.setMediaSessionToken(session.sessionCompatToken as MediaSessionCompat.Token)
    }

    override fun setItem(item: VideoItem, startAt: Long) {
        val metadata = MediaMetadata.Builder()
            .setTitle(item.title)
            .setSubtitle(item.subtitle)
            .build()

        val mediaItem = MediaItem.Builder()
            .setUri(item.url)
            .setMediaMetadata(metadata)
            .build()

        val dataSourceFactory = DefaultHttpDataSource.Factory()
        val sources = mutableListOf(
            DefaultMediaSourceFactory(context)
                .createMediaSource(mediaItem)
        )

        item.subtitles.filterIsInstance<SubtitleTrack.External>()
            .forEach {
                val uri = Uri.parse(it.url)

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

        player.setMediaSource(mergedSource, startAt)
        player.prepare()
        player.play()

        _currentItem.value = item
    }

    override fun setSubtitleTrack(subtitle: SubtitleTrack?) {
        val renderer = textRenderer ?: return

        if (subtitle == null) {
            trackSelector.parameters = trackSelector.buildUponParameters()
                .setRendererDisabled(renderer, true)
                .build()
        } else {
            val trackId = when (subtitle) {
                is SubtitleTrack.External -> "external:${subtitle.id}"
                is SubtitleTrack.Embedded -> (subtitle.index + 1).toString()
            }

            val mappedTrackInfo = trackSelector.currentMappedTrackInfo ?: return
            val trackGroups = mappedTrackInfo.getTrackGroups(renderer)

            var group: TrackGroup? = null
            var track: Int? = null
            outer@ for (i in 0 until trackGroups.length) {
                group = trackGroups[i]
                for (j in 0 until group.length) {
                    val format = group.getFormat(j)
                    if (format.id == trackId) {
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
                .addOverride(TrackSelectionOverride(group, track))
                .build()
        }

        _subtitleTrack.value = subtitle
    }

    override fun setPlayWhenReady(playWhenReady: Boolean) {
        player.playWhenReady = playWhenReady
    }

    override fun seekTo(position: Long) {
        player.seekTo(position)
    }

    override fun restart() {
        player.seekTo(0)
        player.play()
    }

    override fun dispose() {
        scope.cancel()
        notificationManager.setPlayer(null)
        session.release()
        player.removeListener(listener)
        player.release()
    }
}