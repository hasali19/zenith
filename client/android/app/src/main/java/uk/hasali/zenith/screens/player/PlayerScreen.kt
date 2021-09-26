package uk.hasali.zenith.screens.player

import android.content.Intent
import android.net.Uri
import android.util.Log
import androidx.compose.runtime.*
import androidx.compose.ui.platform.LocalContext
import com.google.android.gms.cast.framework.CastContext
import io.ktor.client.features.*
import kotlinx.coroutines.launch
import uk.hasali.zenith.*
import uk.hasali.zenith.ui.CenteredLoadingIndicator
import uk.hasali.zenith.ui.LocalZenithClient

enum class MediaItemType {
    Movie,
    TvShow,
}

@Composable
fun PlayerScreen(id: Int, type: MediaItemType, startPosition: Double?, onNavigateUp: () -> Unit) {
    val scope = rememberCoroutineScope()
    val context = LocalContext.current
    val client = LocalZenithClient.current

    val item by produceState<MediaItem?>(null, id) {
        value = client.getItem(id)
    }

    val onVideoProgress: (Long) -> Unit = { position ->
        if (!BuildConfig.DEBUG) scope.launch {
            try {
                client.updateProgress(id, position)
            } catch (e: ServerResponseException) {
                Log.w(
                    "PlayerScreen",
                    "Failed to update progress on server: ${e.message}",
                )
            }
        }
    }

    val onLaunchExternal = {
        onNavigateUp()
        context.startActivity(Intent(Intent.ACTION_VIEW).apply {
            setDataAndType(Uri.parse(client.getVideoUrl(id)), "video/x-matroska")
        })
    }

    PlayerScreen(
        item = item,
        type = type,
        startPosition = startPosition,
        onVideoProgress = onVideoProgress,
        onLaunchExternal = onLaunchExternal,
        onNavigateUp = onNavigateUp,
    )
}

@Composable
private fun PlayerScreen(
    item: MediaItem?,
    type: MediaItemType,
    startPosition: Double?,
    onVideoProgress: (Long) -> Unit,
    onLaunchExternal: () -> Unit,
    onNavigateUp: () -> Unit,
) {
    when (item) {
        null -> CenteredLoadingIndicator()
        else -> {
            val title: String?
            val backdrop: String?
            val videoInfo: VideoInfo?
            val userData: VideoUserData?


            when (item) {
                is Movie -> {
                    title = item.title
                    backdrop = item.backdrop
                    videoInfo = item.videoInfo
                    userData = item.userData
                }

                is Episode -> {
                    title = item.name
                    backdrop = item.thumbnail
                    videoInfo = item.videoInfo
                    userData = item.userData
                }

                else -> return
            }

            val context = LocalContext.current
            val castSession = remember {
                CastContext.getSharedInstance(context)
                    .sessionManager
                    .currentCastSession
            }

            if (castSession != null && castSession.isConnected) {
                RemotePlayer(
                    id = item.id,
                    title = title ?: "",
                    type = type,
                    backdrop = backdrop,
                    info = videoInfo,
                    session = castSession,
                    onNavigateUp = onNavigateUp,
                )
            } else {
                LocalPlayer(
                    url = LocalZenithClient.current.getVideoUrl(item.id),
                    title = title ?: "",
                    info = videoInfo,
                    userData = userData,
                    startPosition = startPosition ?: 0.0,
                    onVideoProgress = onVideoProgress,
                    onLaunchExternal = onLaunchExternal,
                    onNavigateUp = onNavigateUp,
                )
            }
        }
    }
}
