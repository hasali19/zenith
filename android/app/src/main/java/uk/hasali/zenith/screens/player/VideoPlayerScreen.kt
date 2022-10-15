package uk.hasali.zenith.screens.player

import android.app.Activity
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
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

    VideoPlayerScreen(
        player = player,
        onSessionEnd = stopAndExit,
    )
}

@Composable
fun VideoPlayerScreen(
    player: VideoPlayer?,
    onSessionEnd: () -> Unit,
) {
    if (player != null) {
        val context = LocalContext.current
        if (context is Activity) {
            ExtendContentIntoDisplayCutout(context.window)
        }

        if (player.isLocal) {
            KeepScreenOn()
        }

        Surface(
            color = Color.Black,
            modifier = Modifier.fillMaxSize(),
        ) {
            VideoPlayer(
                player = player,
                onClosePressed = onSessionEnd,
            )
        }
    }
}
