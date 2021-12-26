package uk.hasali.zenith.screens.player

import android.app.Activity
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material.Surface
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import kotlinx.coroutines.flow.collect
import uk.hasali.zenith.media.MediaSession
import uk.hasali.zenith.navigation.hiltViewModel

@Composable
fun VideoPlayerScreen(model: VideoPlayerViewModel = hiltViewModel(), onNavigateUp: () -> Unit) {
    val session by model.session.collectAsState()

    val stopAndExit = {
        model.stop()
        onNavigateUp()
    }

    DisposableEffect(Unit) {
        onDispose {
            stopAndExit()
        }
    }

    Surface(
        color = Color.Black,
        modifier = Modifier.fillMaxSize(),
    ) {
        VideoPlayerScreen(
            session = session,
            onClosePressed = stopAndExit,
            onNavigateUp = onNavigateUp,
        )
    }
}

@Composable
fun VideoPlayerScreen(
    session: MediaSession?,
    onClosePressed: () -> Unit,
    onNavigateUp: () -> Unit,
) {
    val context = LocalContext.current

    SideEffect {
        if (session == null) {
            onNavigateUp()
        }
    }

    if (session != null) {
        val player by session.player.collectAsState()

        LaunchedEffect(session) {
            session.state.collect {
                if (it == uk.hasali.zenith.media.VideoPlayer.State.Ended) {
                    onNavigateUp()
                }
            }
        }

        if (context is Activity) {
            ExtendContentIntoDisplayCutout(context.window)
        }

        if (player.isLocal) {
            KeepScreenOn()
        }

        VideoPlayer(
            player = player,
            onClosePressed = onClosePressed,
        )
    }
}
