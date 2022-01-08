package uk.hasali.zenith.media

import android.content.Context
import android.net.Uri
import android.widget.Toast
import com.google.android.gms.cast.*
import com.google.android.gms.cast.framework.CastSession
import com.google.android.gms.cast.framework.media.RemoteMediaClient
import com.google.android.gms.common.images.WebImage
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import org.json.JSONObject

class RemoteVideoPlayer(private val context: Context, session: CastSession) : VideoPlayer {
    private val mediaClient = requireNotNull(session.remoteMediaClient) {
        "Remote media client is null"
    }

    private val callback = object : RemoteMediaClient.Callback() {
        override fun onStatusUpdated() {
            _isPlaying.value = mediaClient.isPlaying

            if (mediaClient.playerState != MediaStatus.PLAYER_STATE_LOADING &&
                mediaClient.playerState != MediaStatus.PLAYER_STATE_IDLE &&
                mediaClient.playerState != MediaStatus.PLAYER_STATE_UNKNOWN
            ) {
                _state.value = VideoPlayer.State.Active
            }

            if (mediaClient.playerState == MediaStatus.PLAYER_STATE_IDLE &&
                mediaClient.mediaStatus?.idleReason == MediaStatus.IDLE_REASON_FINISHED
            ) {
                _state.value = VideoPlayer.State.Ended
            }
        }

        override fun onMetadataUpdated() {
            _currentItem.value = getCurrentItemIfExists()
        }
    }

    private var _currentItem = MutableStateFlow(getCurrentItemIfExists())
    override val currentItem: StateFlow<VideoItem?>
        get() = _currentItem

    private var _subtitleTrack = MutableStateFlow<SubtitleTrack?>(null)
    override val subtitleTrack: StateFlow<SubtitleTrack?>
        get() = _subtitleTrack

    private val _state = MutableStateFlow(VideoPlayer.State.Active)
    override val state: StateFlow<VideoPlayer.State>
        get() = _state

    private var _isPlaying = MutableStateFlow(mediaClient.isPlaying)
    override val isPlaying: StateFlow<Boolean>
        get() = _isPlaying

    private var _playWhenReady = MutableStateFlow(!mediaClient.isPaused)
    override val playWhenReady: StateFlow<Boolean>
        get() = _playWhenReady

    override val position: Long
        get() = mediaClient.approximateStreamPosition

    init {
        mediaClient.registerCallback(callback)
    }

    private fun getCurrentItemIfExists(): VideoItem? {
        val currentItem = mediaClient.currentItem ?: return null
        val type = when (currentItem.media!!.metadata!!.mediaType) {
            MediaMetadata.MEDIA_TYPE_MOVIE -> VideoItemType.Movie
            MediaMetadata.MEDIA_TYPE_TV_SHOW -> VideoItemType.TvShow
            else -> throw IllegalArgumentException("Invalid video item type")
        }

        val data = currentItem.media?.customData ?: return null

        return VideoItem(
            id = 0,
            type = type,
            url = currentItem.media!!.contentId,
            title = data.getString("title"),
            subtitle = data.optString("subtitle"),
            poster = data.optString("poster"),
            backdrop = data.optString("backdrop"),
            duration = currentItem.media!!.streamDuration.toDouble() / 1000,
            subtitles = currentItem.media!!.mediaTracks!!
                .filter { it.subtype == MediaTrack.SUBTYPE_SUBTITLES }
                .map(::mapMediaTrackToSubtitleTrack),
        )
    }

    private fun mapMediaTrackToSubtitleTrack(track: MediaTrack): SubtitleTrack {
        val data = requireNotNull(track.customData) {
            "Subtitle track is missing custom data"
        }

        return when (val type = data.getString("type")) {
            "external" -> SubtitleTrack.External(
                url = track.contentId,
                id = data.getInt("id"),
                title = data.optString("title"),
                language = data.optString("language"),
            )

            "embedded" -> SubtitleTrack.Embedded(
                index = data.getInt("index"),
                url = track.contentId,
                id = data.getInt("id"),
                title = data.optString("title"),
                language = data.optString("language"),
            )

            else -> throw IllegalArgumentException("Invalid subtitle track type: $type")
        }
    }

    override fun setItem(item: VideoItem, startAt: Long) {
        println("Setting cast item: ${item.subtitles}")

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
                    .setCustomData(JSONObject().apply {
                        put("id", it.id)
                        put("title", it.title)
                        put("language", it.language)

                        when (it) {
                            is SubtitleTrack.External -> put("type", "external")
                            is SubtitleTrack.Embedded -> {
                                put("type", "embedded")
                                put("index", it.index)
                            }
                        }
                    })
                    .build()
            }

        val metadata = MediaMetadata(item.type.toCastMediaType()).apply {
            putString(MediaMetadata.KEY_TITLE, item.title)

            if (item.subtitle != null) {
                putString(MediaMetadata.KEY_SUBTITLE, item.subtitle)
            }

            if (item.poster != null) {
                addImage(WebImage(Uri.parse(item.poster)))
            }

            if (item.backdrop != null) {
                addImage(WebImage(Uri.parse(item.backdrop)))
            }
        }

        val mediaInfo = MediaInfo.Builder(item.url)
            .setStreamType(MediaInfo.STREAM_TYPE_BUFFERED)
            .setContentType("video/mp4")
            .setMetadata(metadata)
            .setMediaTracks(subtitleTracks)
            .setCustomData(JSONObject().apply {
                put("title", item.title)
                put("subtitle", item.subtitle)
                put("poster", item.poster)
                put("backdrop", item.backdrop)
            })
            .build()

        val request = MediaLoadRequestData.Builder()
            .setMediaInfo(mediaInfo)
            .setAutoplay(true)
            .build()

        mediaClient.load(request)
            .setResultCallback {
                println(it.mediaError?.reason)
            }

        _currentItem.value = item.copy(subtitles = supportedSubtitles)
        _subtitleTrack.value = null
        _state.value = VideoPlayer.State.Active
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

    override fun seekTo(position: Long) {
        mediaClient.seek(
            MediaSeekOptions.Builder()
                .setPosition(position)
                .setResumeState(MediaSeekOptions.RESUME_STATE_UNCHANGED)
                .build()
        )
    }

    override fun dispose() {
        mediaClient.stop()
        mediaClient.unregisterCallback(callback)
    }

    private fun VideoItemType.toCastMediaType() = when (this) {
        VideoItemType.Movie -> MediaMetadata.MEDIA_TYPE_MOVIE
        VideoItemType.TvShow -> MediaMetadata.MEDIA_TYPE_TV_SHOW
    }
}
