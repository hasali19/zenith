package uk.hasali.zenith.screens.player

import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.platform.LocalContext
import com.google.android.gms.cast.framework.CastContext
import uk.hasali.zenith.ui.CenteredLoadingIndicator
import uk.hasali.zenith.ui.rememberFlowWithLifecycle

@Composable
fun PlayerScreen(
    model: PlayerViewModel,
    onLaunchExternal: (String) -> Unit,
    onNavigateUp: () -> Unit,
) {
    val item by rememberFlowWithLifecycle(model.item)
        .collectAsState(null)

    PlayerScreen(
        item = item,
        onVideoProgress = { model.updateProgress(it) },
        onLaunchExternal = { item?.let { onLaunchExternal(it.url) } },
        onNavigateUp = onNavigateUp,
    )
}

@Composable
private fun PlayerScreen(
    item: VideoItem?,
    onVideoProgress: (Long) -> Unit,
    onLaunchExternal: () -> Unit,
    onNavigateUp: () -> Unit,
) {
    when (item) {
        null -> CenteredLoadingIndicator()
        else -> {
            val context = LocalContext.current
            // TODO: Move to viewmodel
            val castSession = remember {
                CastContext.getSharedInstance(context)
                    .sessionManager
                    .currentCastSession
            }

            if (castSession != null && castSession.isConnected) {
                RemotePlayer(
                    item = item,
                    session = castSession,
                    onNavigateUp = onNavigateUp,
                )
            } else {
                LocalPlayer(
                    item = item,
                    onVideoProgress = onVideoProgress,
                    onLaunchExternal = onLaunchExternal,
                    onNavigateUp = onNavigateUp,
                )
            }
        }
    }
}
