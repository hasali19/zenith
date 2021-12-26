package uk.hasali.zenith.screens.player

import android.app.Activity
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material.Surface
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.SideEffect
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalContext
import uk.hasali.zenith.media.VideoPlayer
import uk.hasali.zenith.navigation.hiltViewModel

@Composable
fun VideoPlayerScreen(model: VideoPlayerViewModel = hiltViewModel(), onNavigateUp: () -> Unit) {
    val player = model.player

    val stopAndExit = {
        model.stop()
        onNavigateUp()
    }

    Surface(
        color = Color.Black,
        modifier = Modifier.fillMaxSize(),
    ) {
        VideoPlayerScreen(
            player = player,
            onClosePressed = stopAndExit,
            onNavigateUp = onNavigateUp,
        )
    }
}

@Composable
fun VideoPlayerScreen(
    player: VideoPlayer?,
    onClosePressed: () -> Unit,
    onNavigateUp: () -> Unit,
) {
    val context = LocalContext.current

    SideEffect {
        if (player == null) {
            onNavigateUp()
        }
    }

    if (player != null) {
        LaunchedEffect(player) {
            player.state.collect {
                if (it == VideoPlayer.State.Ended) {
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
