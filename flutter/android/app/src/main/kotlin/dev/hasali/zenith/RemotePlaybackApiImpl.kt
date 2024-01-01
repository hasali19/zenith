package dev.hasali.zenith

import android.net.Uri
import androidx.mediarouter.media.MediaRouteSelector
import androidx.mediarouter.media.MediaRouter
import com.google.android.gms.cast.MediaInfo
import com.google.android.gms.cast.MediaMetadata
import com.google.android.gms.cast.MediaSeekOptions.RESUME_STATE_PAUSE
import com.google.android.gms.cast.MediaSeekOptions.RESUME_STATE_PLAY
import com.google.android.gms.cast.MediaSeekOptions.RESUME_STATE_UNCHANGED
import com.google.android.gms.cast.framework.CastContext
import com.google.android.gms.common.images.WebImage
import dev.hasali.zenith.generated.remoteplayback.MediaLoadRequestData
import dev.hasali.zenith.generated.remoteplayback.MediaRoute
import dev.hasali.zenith.generated.remoteplayback.MediaSeekOptions
import dev.hasali.zenith.generated.remoteplayback.MediaType
import dev.hasali.zenith.generated.remoteplayback.RemotePlaybackApi
import dev.hasali.zenith.generated.remoteplayback.RemotePlaybackEventsApi
import dev.hasali.zenith.generated.remoteplayback.ResumeState
import dev.hasali.zenith.generated.remoteplayback.RoutesScanningMode

class RemotePlaybackApiImpl(
    private val eventsApi: RemotePlaybackEventsApi,
    private val mediaRouter: MediaRouter,
    private val mediaRouteSelector: MediaRouteSelector,
    private val castContext: CastContext,
) : RemotePlaybackApi {

    private var activeListeners = 0
    private var passiveListeners = 0
    private var standardListeners = 0

    private var currentCallback: MediaRouterCallback? = null

    override fun registerRoutesListener(mode: RoutesScanningMode) {
        when (mode) {
            RoutesScanningMode.NONE -> standardListeners += 1
            RoutesScanningMode.PASSIVE -> passiveListeners += 1
            RoutesScanningMode.ACTIVE -> activeListeners += 1
        }
        updateCallback()
        currentCallback?.updateRoutes()
    }

    override fun unregisterRoutesListener(mode: RoutesScanningMode) {
        when (mode) {
            RoutesScanningMode.NONE -> standardListeners -= 1
            RoutesScanningMode.PASSIVE -> passiveListeners -= 1
            RoutesScanningMode.ACTIVE -> activeListeners -= 1
        }
        updateCallback()
    }

    override fun load(loadRequestData: MediaLoadRequestData) {
        val session = castContext.sessionManager.currentCastSession ?: return
        val client = session.remoteMediaClient ?: return
        client.load(com.google.android.gms.cast.MediaLoadRequestData.Builder()
            .apply {
                loadRequestData.mediaInfo?.let { mediaInfo ->
                    setMediaInfo(MediaInfo.Builder(mediaInfo.url)
                        .apply {
                            mediaInfo.metadata?.let { metadata ->
                                val mediaType = when (metadata.mediaType) {
                                    MediaType.MOVIE -> MediaMetadata.MEDIA_TYPE_MOVIE
                                    MediaType.TVSHOW -> MediaMetadata.MEDIA_TYPE_TV_SHOW
                                    else -> MediaMetadata.MEDIA_TYPE_GENERIC
                                }

                                setMetadata(MediaMetadata(mediaType).apply {
                                    metadata.title?.let {
                                        putString(MediaMetadata.KEY_TITLE, it)
                                    }

                                    metadata.seriesTitle?.let {
                                        putString(MediaMetadata.KEY_SERIES_TITLE, it)
                                    }

                                    metadata.seasonNumber?.let {
                                        putInt(MediaMetadata.KEY_SEASON_NUMBER, it.toInt())
                                    }

                                    metadata.episodeNumber?.let {
                                        putInt(MediaMetadata.KEY_EPISODE_NUMBER, it.toInt())
                                    }

                                    for (image in listOf(
                                        metadata.poster,
                                        metadata.backdrop
                                    )) {
                                        image?.let {
                                            addImage(
                                                WebImage(
                                                    Uri.parse(it.url),
                                                    it.width.toInt(),
                                                    it.height.toInt()
                                                )
                                            )
                                        }
                                    }
                                })
                            }
                        }
                        .build())
                }
            }
            .build())
    }

    override fun play() {
        val session = castContext.sessionManager.currentCastSession ?: return
        val client = session.remoteMediaClient ?: return
        client.play()
    }

    override fun pause() {
        val session = castContext.sessionManager.currentCastSession ?: return
        val client = session.remoteMediaClient ?: return
        client.pause()
    }

    override fun seek(options: MediaSeekOptions) {
        val session = castContext.sessionManager.currentCastSession ?: return
        val client = session.remoteMediaClient ?: return
        client.seek(
            com.google.android.gms.cast.MediaSeekOptions.Builder()
                .setPosition(options.position)
                .setResumeState(
                    when (options.resumeState) {
                        ResumeState.PAUSE -> RESUME_STATE_PAUSE
                        ResumeState.PLAY -> RESUME_STATE_PLAY
                        ResumeState.UNCHANGED -> RESUME_STATE_UNCHANGED
                    }
                )
                .build()
        )
    }

    override fun setPlaybackRate(playbackRate: Double) {
        val session = castContext.sessionManager.currentCastSession ?: return
        val client = session.remoteMediaClient ?: return
        client.setPlaybackRate(playbackRate)
    }

    override fun stop() {
        val session = castContext.sessionManager.currentCastSession ?: return
        val client = session.remoteMediaClient ?: return
        client.stop()
    }

    private fun updateCallback() {
        val targetMode = if (activeListeners > 0) {
            RoutesScanningMode.ACTIVE
        } else if (passiveListeners > 0) {
            RoutesScanningMode.PASSIVE
        } else if (standardListeners > 0) {
            RoutesScanningMode.NONE
        } else {
            null
        }

        val flags = when (targetMode) {
            null, RoutesScanningMode.NONE -> 0
            RoutesScanningMode.PASSIVE -> MediaRouter.CALLBACK_FLAG_REQUEST_DISCOVERY
            RoutesScanningMode.ACTIVE -> MediaRouter.CALLBACK_FLAG_PERFORM_ACTIVE_SCAN
        }

        currentCallback.let { callback ->
            when {
                targetMode == null -> {
                    callback?.let { mediaRouter.removeCallback(it) }
                    currentCallback = null
                }

                callback == null -> {
                    currentCallback = MediaRouterCallback(targetMode).also {
                        it.updateRoutes()
                        mediaRouter.addCallback(mediaRouteSelector, it, flags)
                    }
                }

                callback.mode.raw != targetMode.raw -> {
                    mediaRouter.removeCallback(callback)
                    currentCallback = MediaRouterCallback(targetMode).also {
                        it.updateRoutes()
                        mediaRouter.addCallback(mediaRouteSelector, it, flags)
                    }
                }
            }
        }
    }

    override fun selectRoute(id: String?) {
        if (id == null) {
            mediaRouter.unselect(MediaRouter.UNSELECT_REASON_DISCONNECTED)
        } else {
            val route = mediaRouter.routes.find { it.id == id }
            if (route != null) {
                mediaRouter.selectRoute(route)
            }
        }
    }

    private fun getMediaRoutes(): List<MediaRoute> {
        return mediaRouter.routes
            .filter { it.isEnabled && it.matchesSelector(mediaRouteSelector) }
            .map {
                MediaRoute(
                    id = it.id,
                    name = it.name,
                    description = it.description,
                    isSelected = it.isSelected,
                )
            }
    }

    private inner class MediaRouterCallback(val mode: RoutesScanningMode) : MediaRouter.Callback() {
        override fun onRouteAdded(router: MediaRouter, route: MediaRouter.RouteInfo) {
            updateRoutes()
        }

        override fun onRouteChanged(router: MediaRouter, route: MediaRouter.RouteInfo) {
            updateRoutes()
        }

        override fun onRouteRemoved(router: MediaRouter, route: MediaRouter.RouteInfo) {
            updateRoutes()
        }

        fun updateRoutes() {
            eventsApi.onRoutesChanged(getMediaRoutes()) {}
        }
    }
}
