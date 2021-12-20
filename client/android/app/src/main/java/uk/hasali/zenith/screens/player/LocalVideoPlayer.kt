package uk.hasali.zenith.screens.player

import android.content.Context
import android.net.Uri
import android.widget.Toast
import androidx.annotation.OptIn
import androidx.media3.common.*
import androidx.media3.common.util.UnstableApi
import androidx.media3.datasource.DefaultHttpDataSource
import androidx.media3.exoplayer.ExoPlayer
import androidx.media3.exoplayer.source.DefaultMediaSourceFactory
import androidx.media3.exoplayer.source.MergingMediaSource
import androidx.media3.exoplayer.source.SingleSampleMediaSource
import androidx.media3.exoplayer.trackselection.DefaultTrackSelector
import androidx.media3.session.MediaSession
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.flow

@OptIn(UnstableApi::class)
class LocalVideoPlayer(private val context: Context) : VideoPlayer {
    override val usePlayerView: Boolean
        get() = true

    private val trackSelector = DefaultTrackSelector(context)
    override val player = ExoPlayer.Builder(context)
        .setTrackSelector(trackSelector)
        .build()

    private val session = MediaSession.Builder(context, player)
        .build()

    private var textRenderer: Int? = null

    private var _videoEndedCallback: (() -> Unit)? = null

    private var _currentItem = MutableStateFlow<VideoItem?>(null)
    override val currentItem: StateFlow<VideoItem?>
        get() = _currentItem

    private var _subtitleTrack = MutableStateFlow<SubtitleTrack?>(null)
    override val subtitleTrack: StateFlow<SubtitleTrack?>
        get() = _subtitleTrack

    private var _isPlaying = MutableStateFlow(false)
    override val isPlaying: StateFlow<Boolean>
        get() = _isPlaying

    private var _playWhenReady = MutableStateFlow(false)
    override val playWhenReady: StateFlow<Boolean>
        get() = _playWhenReady

    private val listener = object : Player.Listener {
        override fun onIsPlayingChanged(isPlaying: Boolean) {
            _isPlaying.value = isPlaying
        }

        override fun onPlayWhenReadyChanged(playWhenReady: Boolean, reason: Int) {
            _playWhenReady.value = playWhenReady
        }

        override fun onPlaybackStateChanged(playbackState: Int) {
            if (playbackState == ExoPlayer.STATE_ENDED) {
                _videoEndedCallback?.invoke()
            }
        }
    }

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
    }

    override fun setVideoEndedCallback(callback: () -> Unit) {
        _videoEndedCallback = callback
    }

    override fun removeVideoEndedCallback(callback: () -> Unit) {
        if (_videoEndedCallback == callback) {
            _videoEndedCallback = null
        }
    }

    override fun pollPosition(resolution: Int): Flow<Long> {
        return flow {
            while (true) {
                emit(player.currentPosition)
                delay(resolution.toLong())
            }
        }
    }

    override fun setItem(item: VideoItem) {
        val mediaItem = MediaItem.fromUri(item.url)

        val dataSourceFactory = DefaultHttpDataSource.Factory()
        val sources = mutableListOf(
            DefaultMediaSourceFactory(context)
                .createMediaSource(mediaItem)
        )

        item.subtitles.filterIsInstance<SubtitleTrack.External>()
            .forEach {
                val uri = Uri.parse(it.url)

                val subtitle = MediaItem.SubtitleConfiguration.Builder(uri)
                    .setMimeType(MimeTypes.TEXT_VTT)
                    .setLanguage(it.language)
                    .setLabel(it.title)
                    .build()

                val source = SingleSampleMediaSource.Factory(dataSourceFactory)
                    .setTrackId("external:${it.id}")
                    .createMediaSource(subtitle, C.TIME_UNSET)

                sources.add(source)
            }

        val mergedSource = MergingMediaSource(*sources.toTypedArray())

        player.setMediaSource(mergedSource, item.startPosition.toLong() * 1000)
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

            val overrides = TrackSelectionOverrides.Builder()
                .setOverrideForType(
                    TrackSelectionOverrides.TrackSelectionOverride(
                        group,
                        listOf(track)
                    )
                )
                .build()

            trackSelector.parameters = trackSelector.buildUponParameters()
                .setRendererDisabled(renderer, false)
                .setTrackSelectionOverrides(overrides)
                .build()
        }

        _subtitleTrack.value = subtitle
    }

    override fun setPlayWhenReady(playWhenReady: Boolean) {
        player.playWhenReady = playWhenReady
    }

    override fun stop() {
        player.stop()
    }

    override fun seekTo(position: Long) {
        player.seekTo(position)
    }

    override fun dispose() {
        session.release()
        player.removeListener(listener)
        player.release()
    }
}
