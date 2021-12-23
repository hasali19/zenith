package uk.hasali.zenith.media

import android.content.Context
import android.widget.Toast
import com.google.android.gms.cast.*
import com.google.android.gms.cast.framework.CastSession
import com.google.android.gms.cast.framework.media.RemoteMediaClient
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.flow

class RemoteVideoPlayer(private val context: Context, session: CastSession) : VideoPlayer {
    private val mediaClient = requireNotNull(session.remoteMediaClient) {
        "Remote media client is null"
    }

    private val callback = object : RemoteMediaClient.Callback() {
        override fun onStatusUpdated() {
            _isPlaying.value = mediaClient.isPlaying

            if (mediaClient.playerState == MediaStatus.PLAYER_STATE_IDLE &&
                mediaClient.mediaStatus?.idleReason == MediaStatus.IDLE_REASON_FINISHED
            ) {
                _videoEndedCallback?.invoke()
            }
        }
    }

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

    private var _playWhenReady = MutableStateFlow(true)
    override val playWhenReady: StateFlow<Boolean>
        get() = _playWhenReady

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
                emit(mediaClient.approximateStreamPosition)
                delay(resolution.toLong())
            }
        }
    }

    override fun setItem(item: VideoItem) {
        // Embedded subtitles that haven't been extracted are not supported when casting
        val supportedSubtitles = item.subtitles.filter { it.url != null }
        val subtitleTracks = supportedSubtitles
            .map {
                MediaTrack.Builder(it.id.toLong(), MediaTrack.TYPE_TEXT)
                    .setName(it.title ?: it.language)
                    .setSubtype(MediaTrack.SUBTYPE_SUBTITLES)
                    .setContentId(it.url)
                    .setContentType("text/vtt")
                    .setLanguage(it.language)
                    .build()
            }

        val currentMediaInfo = mediaClient.mediaInfo
        if (currentMediaInfo == null || currentMediaInfo.contentId != item.url) {
            val metadata = MediaMetadata(item.type.toCastMediaType()).apply {
                putString(MediaMetadata.KEY_TITLE, item.title)
            }

            val mediaInfo = MediaInfo.Builder(item.url)
                .setStreamType(MediaInfo.STREAM_TYPE_BUFFERED)
                .setContentType("video/mp4")
                .setMetadata(metadata)
                .setMediaTracks(subtitleTracks)
                .build()

            val request = MediaLoadRequestData.Builder()
                .setMediaInfo(mediaInfo)
                .setAutoplay(true)
                .build()

            mediaClient.load(request)
        } else {
            _isPlaying.value = mediaClient.isPlaying
        }

        mediaClient.registerCallback(callback)

        _currentItem.value = item.copy(subtitles = supportedSubtitles)
        _subtitleTrack.value = null
    }

    override fun setSubtitleTrack(subtitle: SubtitleTrack?) {
        val tracks = if (subtitle == null) {
            longArrayOf()
        } else {
            longArrayOf(subtitle.id.toLong())
        }

        mediaClient.setActiveMediaTracks(tracks)
            .setResultCallback { result ->
                if (result.status.isSuccess) {
                    _subtitleTrack.value = subtitle
                } else {
                    Toast.makeText(context, result.status.statusMessage, Toast.LENGTH_SHORT)
                        .show()
                }
            }
    }

    override fun setPlayWhenReady(playWhenReady: Boolean) {
        if (playWhenReady) {
            mediaClient.play()
            _playWhenReady.value = true
        } else {
            mediaClient.pause()
            _playWhenReady.value = false
        }
    }

    override fun stop() {
        mediaClient.stop()
    }

    override fun seekTo(position: Long) {
        mediaClient.seek(
            MediaSeekOptions.Builder()
                .setPosition(position)
                .setResumeState(MediaSeekOptions.RESUME_STATE_UNCHANGED)
                .build()
        )
    }

    override fun dispose() {
        mediaClient.unregisterCallback(callback)
    }

    private fun VideoItemType.toCastMediaType() = when (this) {
        VideoItemType.Movie -> MediaMetadata.MEDIA_TYPE_MOVIE
        VideoItemType.TvShow -> MediaMetadata.MEDIA_TYPE_TV_SHOW
    }
}
