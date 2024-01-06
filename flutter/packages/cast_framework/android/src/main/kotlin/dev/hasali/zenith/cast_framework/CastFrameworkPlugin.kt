package dev.hasali.zenith.cast_framework

import androidx.mediarouter.media.MediaRouter
import com.google.android.gms.cast.MediaMetadata.KEY_EPISODE_NUMBER
import com.google.android.gms.cast.MediaMetadata.KEY_SEASON_NUMBER
import com.google.android.gms.cast.MediaMetadata.KEY_SERIES_TITLE
import com.google.android.gms.cast.MediaMetadata.KEY_TITLE
import com.google.android.gms.cast.MediaMetadata.MEDIA_TYPE_MOVIE
import com.google.android.gms.cast.MediaMetadata.MEDIA_TYPE_TV_SHOW
import com.google.android.gms.cast.MediaStatus.PLAYER_STATE_BUFFERING
import com.google.android.gms.cast.MediaStatus.PLAYER_STATE_IDLE
import com.google.android.gms.cast.MediaStatus.PLAYER_STATE_LOADING
import com.google.android.gms.cast.MediaStatus.PLAYER_STATE_PAUSED
import com.google.android.gms.cast.MediaStatus.PLAYER_STATE_PLAYING
import com.google.android.gms.cast.framework.CastContext
import com.google.android.gms.cast.framework.CastSession
import com.google.android.gms.cast.framework.media.RemoteMediaClient
import dev.hasali.zenith.cast_framework.pigeon.CastApi
import dev.hasali.zenith.cast_framework.pigeon.CastEventsApi
import dev.hasali.zenith.cast_framework.pigeon.MediaInfo
import dev.hasali.zenith.cast_framework.pigeon.MediaMetadata
import dev.hasali.zenith.cast_framework.pigeon.MediaMetadataImage
import dev.hasali.zenith.cast_framework.pigeon.MediaStatus
import dev.hasali.zenith.cast_framework.pigeon.MediaType
import dev.hasali.zenith.cast_framework.pigeon.PlayerState
import io.flutter.embedding.engine.plugins.FlutterPlugin

class CastFrameworkPlugin : FlutterPlugin {
    override fun onAttachedToEngine(flutterPluginBinding: FlutterPlugin.FlutterPluginBinding) {
        val mediaRouter = MediaRouter.getInstance(flutterPluginBinding.applicationContext)
        val castContext = CastContext.getSharedInstance(flutterPluginBinding.applicationContext)

        val mediaRouterEventsApi =
            CastEventsApi(flutterPluginBinding.binaryMessenger)

        CastApi.setUp(
            flutterPluginBinding.binaryMessenger,
            CastApiImpl(
                mediaRouterEventsApi,
                mediaRouter,
                castContext.mergedSelector!!,
                castContext
            )
        )

        val remoteClientCallback = object : RemoteMediaClient.Callback() {
            override fun onStatusUpdated() {
                val session = castContext.sessionManager.currentCastSession ?: return
                val client = session.remoteMediaClient ?: return
                val status = client.mediaStatus
                if (status == null) {
                    mediaRouterEventsApi.onStatusUpdated(null) {}
                    return
                }

                val playerState = when (status.playerState) {
                    PLAYER_STATE_IDLE -> PlayerState.IDLE
                    PLAYER_STATE_BUFFERING -> PlayerState.BUFFERING
                    PLAYER_STATE_LOADING -> PlayerState.LOADING
                    PLAYER_STATE_PAUSED -> PlayerState.PAUSED
                    PLAYER_STATE_PLAYING -> PlayerState.PLAYING
                    else -> PlayerState.UNKNOWN
                }

                fun com.google.android.gms.cast.MediaMetadata.getIntOrNull(key: String): Int? {
                    return if (containsKey(key)) {
                        getInt(key)
                    } else {
                        null
                    }
                }

                mediaRouterEventsApi.onStatusUpdated(
                    MediaStatus(
                        playerState = playerState,
                        mediaInfo = status.mediaInfo?.let { mediaInfo ->
                            MediaInfo(
                                streamDuration = mediaInfo.streamDuration,
                                metadata = mediaInfo.metadata?.let { metadata ->
                                    MediaMetadata(
                                        mediaType = when (metadata.mediaType) {
                                            MEDIA_TYPE_MOVIE -> MediaType.MOVIE
                                            MEDIA_TYPE_TV_SHOW -> MediaType.TVSHOW
                                            else -> MediaType.UNKNOWN
                                        },
                                        title = metadata.getString(KEY_TITLE),
                                        seriesTitle = metadata.getString(KEY_SERIES_TITLE),
                                        seasonNumber = metadata.getIntOrNull(KEY_SEASON_NUMBER)
                                            ?.toLong(),
                                        episodeNumber = metadata.getIntOrNull(KEY_EPISODE_NUMBER)
                                            ?.toLong(),
                                        poster = metadata.images[0]?.let {
                                            MediaMetadataImage(
                                                it.url.toString(),
                                                it.width.toLong(),
                                                it.height.toLong()
                                            )
                                        },
                                        backdrop = metadata.images[1]?.let {
                                            MediaMetadataImage(
                                                it.url.toString(),
                                                it.width.toLong(),
                                                it.height.toLong()
                                            )
                                        },
                                    )
                                },
                            )
                        },
                        streamPosition = status.streamPosition,
                        playbackRate = status.playbackRate,
                    )
                ) {}
            }

            override fun onMetadataUpdated() {
                onStatusUpdated()
            }
        }

        val sessionManagerListener = object : CastSessionManagerListener() {
            override fun onSessionEnding(session: CastSession) {
                session.remoteMediaClient!!.unregisterCallback(remoteClientCallback)
            }

            override fun onSessionStarted(session: CastSession, sessionId: String) {
                session.remoteMediaClient!!.registerCallback(remoteClientCallback)
            }

            override fun onSessionResumed(session: CastSession, wasSuspended: Boolean) {
                session.remoteMediaClient!!.registerCallback(remoteClientCallback)
            }
        }

        castContext.sessionManager.addSessionManagerListener(
            sessionManagerListener,
            CastSession::class.java
        )
    }

    override fun onDetachedFromEngine(binding: FlutterPlugin.FlutterPluginBinding) {
    }
}
