package uk.co.hasali.zenith.ui.videoplayer

import android.content.Context
import android.support.v4.media.session.MediaSessionCompat
import android.view.SurfaceView
import com.google.android.exoplayer2.*
import com.google.android.exoplayer2.ext.mediasession.MediaSessionConnector
import com.google.android.exoplayer2.source.ProgressiveMediaSource
import com.google.android.exoplayer2.upstream.DefaultHttpDataSourceFactory
import com.google.android.exoplayer2.upstream.DefaultLoadErrorHandlingPolicy
import com.google.android.exoplayer2.upstream.LoadErrorHandlingPolicy
import com.google.android.exoplayer2.video.VideoListener

enum class PlayState {
    PLAYING,
    PAUSED;

    val isPlaying
        get() = this == PLAYING

    fun toggle() = when (this) {
        PLAYING -> PAUSED
        PAUSED -> PLAYING
    }
}

interface VideoItem {
    fun getUrlForPosition(position: Float): String
}

class Player(context: Context, surface: SurfaceView, session: MediaSessionCompat) : VideoListener,
    com.google.android.exoplayer2.Player.EventListener {

    private val player: SimpleExoPlayer = SimpleExoPlayer.Builder(context).build()

    private var item: VideoItem? = null
    private var startPosition = 0f
    private var isBuffering = false

    val position
        get() = startPosition + (player.currentPosition.toFloat() / 1000)

    val bufferedPosition
        get() = startPosition + (player.bufferedPosition.toFloat() / 1000)

    val isEnded
        get() = player.playbackState == ExoPlayer.STATE_ENDED

    var state
        get() = when (player.playWhenReady) {
            true -> PlayState.PLAYING
            false -> PlayState.PAUSED
        }
        set(value) {
            player.playWhenReady = value.isPlaying
        }

    var onVideoSizeChanged: ((width: Int, height: Int, aspectRatio: Float) -> Unit)? = null
    var onVideoBufferingChanged: ((buffering: Boolean) -> Unit)? = null
    var onVideoPlaybackStateChanged: ((state: PlayState) -> Unit)? = null
    var onVideoEnded: (() -> Unit)? = null

    init {
        player.setVideoSurfaceView(surface)
        player.addVideoListener(this)
        player.addListener(this)

        MediaSessionConnector(session).apply {
            setPlayer(player)
            setControlDispatcher(object : DefaultControlDispatcher() {})
        }
    }

    fun setVideoItem(item: VideoItem) {
        this.item = item
        setMediaItemUrl(item.getUrlForPosition(startPosition))
    }

    fun play() {
        player.play()
    }

    fun pause() {
        player.pause()
    }

    fun release() {
        player.release()
    }

    fun seekTo(position: Float) {
        item?.let { item ->
            player.stop()
            setMediaItemUrl(item.getUrlForPosition(position))
            startPosition = position
        }
    }

    private fun setMediaItemUrl(url: String) {
        val factory = DefaultHttpDataSourceFactory()
        val mediaItem = MediaItem.Builder()
            .setUri(url)
            .build()

        val source = ProgressiveMediaSource.Factory(factory)
            .setLoadErrorHandlingPolicy(object : DefaultLoadErrorHandlingPolicy() {
                override fun getRetryDelayMsFor(loadErrorInfo: LoadErrorHandlingPolicy.LoadErrorInfo): Long {
                    return C.TIME_UNSET
                }
            })
            .createMediaSource(mediaItem)

        player.setMediaSource(source)
        player.prepare()
    }

    override fun onVideoSizeChanged(
        width: Int,
        height: Int,
        unappliedRotationDegrees: Int,
        pixelWidthHeightRatio: Float,
    ) {
        val aspectRatio =
            if (width == 0 || height == 0) 1f
            else (width * pixelWidthHeightRatio) / height

        onVideoSizeChanged?.invoke(width, height, aspectRatio)
    }

    override fun onPlayWhenReadyChanged(playWhenReady: Boolean, reason: Int) {
        state = when (playWhenReady) {
            true -> PlayState.PLAYING
            false -> PlayState.PAUSED
        }

        onVideoPlaybackStateChanged?.invoke(state)
    }

    override fun onPlaybackStateChanged(state: Int) {
        val isBuffering = state == ExoPlayer.STATE_BUFFERING
        if (this.isBuffering != isBuffering) {
            this.isBuffering = isBuffering
            onVideoBufferingChanged?.invoke(isBuffering)
        }

        if (state == ExoPlayer.STATE_ENDED) {
            onVideoEnded?.invoke()
        }
    }

    override fun onPlayerError(error: ExoPlaybackException) {
        // If we temporarily lose connection to the server, this should
        // enable re-requesting the video at the current position, to avoid restarting
        // from the beginning.
        // TODO: Check if this will cause infinite reconnect attempts
        if (error.type == ExoPlaybackException.TYPE_SOURCE) {
            seekTo(position)
        } else {
            super.onPlayerError(error)
        }
    }
}
