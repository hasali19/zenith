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
fun PlayerScreen(id: Int, type: MediaItemType, replay: Boolean, onNavigateUp: () -> Unit) {
    val scope = rememberCoroutineScope()
    val context = LocalContext.current
    val client = LocalZenithClient.current

    val info by produceState<VideoInfo?>(null, id) {
        value = client.getVideoInfo(id)
    }

    val onVideoProgress: (Long) -> Unit = { position ->
        scope.launch {
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

    val title: String?
    val backdrop: String?

    when (type) {
        MediaItemType.Movie -> {
            val movie by produceState<Movie?>(null, id) {
                value = client.getMovie(id)
            }

            title = movie?.title
            backdrop = movie?.backdrop
        }
        MediaItemType.TvShow -> {
            val episode by produceState<Episode?>(null, id) {
                value = client.getEpisode(id)
            }

            title = episode?.name
            backdrop = episode?.thumbnail
        }
    }

    PlayerScreen(
        id = id,
        title = title ?: "",
        type = type,
        backdrop = backdrop,
        replay = replay,
        info = info,
        onVideoProgress = onVideoProgress,
        onLaunchExternal = onLaunchExternal,
        onNavigateUp = onNavigateUp,
    )
}

@Composable
private fun PlayerScreen(
    id: Int,
    title: String,
    type: MediaItemType,
    backdrop: String?,
    replay: Boolean,
    info: VideoInfo?,
    onVideoProgress: (Long) -> Unit,
    onLaunchExternal: () -> Unit,
    onNavigateUp: () -> Unit,
) {
    when (info) {
        null -> CenteredLoadingIndicator()
        else -> {
            val context = LocalContext.current
            val castSession = remember {
                CastContext.getSharedInstance(context)
                    .sessionManager
                    .currentCastSession
            }

            if (castSession != null && castSession.isConnected) {
                RemotePlayer(
                    id = id,
                    title = title,
                    type = type,
                    backdrop = backdrop,
                    info = info,
                    session = castSession,
                    onNavigateUp = onNavigateUp,
                )
            } else {
                LocalPlayer(
                    url = LocalZenithClient.current.getVideoUrl(id),
                    title = title,
                    info = info,
                    replay = replay,
                    onVideoProgress = onVideoProgress,
                    onLaunchExternal = onLaunchExternal,
                    onNavigateUp = onNavigateUp,
                )
            }
        }
    }
}
