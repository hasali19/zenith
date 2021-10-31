package uk.hasali.zenith.screens.player

import android.app.Activity
import android.os.Build
import android.view.Window
import android.view.WindowManager
import androidx.activity.compose.LocalOnBackPressedDispatcherOwner
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import com.google.android.gms.cast.framework.CastContext
import com.google.android.gms.cast.framework.CastSession
import kotlinx.coroutines.flow.collect
import kotlinx.coroutines.flow.collectLatest
import uk.hasali.zenith.navigation.hiltViewModel
import uk.hasali.zenith.ui.rememberFlowWithLifecycle

@Composable
fun VideoPlayerScreen(model: PlayerViewModel = hiltViewModel()) {
    val context = LocalContext.current

    val castContext = remember(context) { CastContext.getSharedInstance(context) }
    val castSessionManager = remember(castContext) { castContext.sessionManager }
    val castSession by remember { mutableStateOf(castSessionManager.currentCastSession) }

    val item by rememberFlowWithLifecycle(model.item)
        .collectAsState(null)

    Box(modifier = Modifier.fillMaxSize()) {
        VideoPlayerScreen(
            item = item,
            castSession = castSession,
            onUpdateProgress = {
                model.updateProgress(it / 1000)
            }
        )
    }
}

@Composable
private fun VideoPlayerScreen(
    item: VideoItem?,
    castSession: CastSession?,
    onUpdateProgress: (Long) -> Unit,
) {
    if (item != null) {
        VideoPlayer(
            item = item,
            castSession = castSession,
            onUpdateProgress = onUpdateProgress,
        )
    }
}

@Composable
private fun ExtendContentIntoDisplayCutout(window: Window) {
    if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.P) {
        DisposableEffect(Unit) {
            window.attributes = window.attributes.apply {
                layoutInDisplayCutoutMode =
                    WindowManager.LayoutParams.LAYOUT_IN_DISPLAY_CUTOUT_MODE_SHORT_EDGES
            }

            onDispose {
                window.attributes = window.attributes.apply {
                    layoutInDisplayCutoutMode =
                        WindowManager.LayoutParams.LAYOUT_IN_DISPLAY_CUTOUT_MODE_DEFAULT
                }
            }
        }
    }
}

@Composable
private fun VideoPlayer(
    item: VideoItem,
    castSession: CastSession?,
    onUpdateProgress: (Long) -> Unit
) {
    val context = LocalContext.current
    var player by remember { mutableStateOf<VideoPlayer?>(null) }

    DisposableEffect(castSession) {
        castSession.let { session ->
            val playerImpl = when (session) {
                null -> LocalVideoPlayer(context)
                else -> RemoteVideoPlayer(context, session)
            }

            player = playerImpl
            onDispose {
                playerImpl.dispose()
            }
        }
    }

    LaunchedEffect(player, item) {
        player?.let { player ->
            player.setItem(item)
            player.isPlaying.collectLatest { isPlaying ->
                if (isPlaying) {
                    player.pollPosition(5000)
                        .collect {
                            onUpdateProgress(it)
                        }
                }
            }
        }
    }

    if (context is Activity) {
        ExtendContentIntoDisplayCutout(context.window)
    }

    if (castSession == null) {
        KeepScreenOn()
    }

    val onBackPressedDispatcher = LocalOnBackPressedDispatcherOwner.current
        ?.onBackPressedDispatcher

    player?.let {
        VideoPlayer(
            player = it,
            autoHideControls = castSession == null,
            onClosePressed = {
                it.stop()
                onBackPressedDispatcher?.onBackPressed()
            },
        )
    }
}
