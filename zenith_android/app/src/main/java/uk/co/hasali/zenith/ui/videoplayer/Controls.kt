package uk.co.hasali.zenith.ui.videoplayer

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.material.CircularProgressIndicator
import androidx.compose.material.FloatingActionButton
import androidx.compose.material.Icon
import androidx.compose.material.IconButton
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.gesture.tapGestureFilter
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.res.vectorResource
import androidx.compose.ui.unit.dp
import uk.co.hasali.zenith.R

@Composable
fun ControlsOverlay(
    buffering: Boolean,
    position: Float,
    duration: Float,
    state: PlayState,
    onPlayPause: () -> Unit,
    onSeekStart: () -> Unit,
    onSeekTo: (Float) -> Unit,
) {
    var visible by remember { mutableStateOf(false) }

    Box(
        modifier = Modifier
            .fillMaxSize()
            .tapGestureFilter { visible = !visible }
    ) {
        if (buffering) {
            CircularProgressIndicator(modifier = Modifier.align(Alignment.Center))
        }

        if (visible) {
            Column(
                modifier = Modifier
                    .align(Alignment.BottomCenter)
                    .background(Color(0f, 0f, 0f, 0.5f))
                    .padding(8.dp)
                    .tapGestureFilter { /* Intercept tap */ }
            ) {
                SeekBar(
                    position = position,
                    max = duration,
                    onSeekStart = onSeekStart,
                    onSeekEnd = onSeekTo
                )

                ButtonRow(
                    position = position,
                    duration = duration,
                    state = state,
                    onPlayPause = onPlayPause,
                    onSeekTo = onSeekTo,
                )
            }
        }
    }
}

@Composable
fun ButtonRow(
    position: Float,
    duration: Float,
    state: PlayState,
    onPlayPause: () -> Unit,
    onSeekTo: (Float) -> Unit,
) {
    Row(
        horizontalArrangement = Arrangement.Center,
        modifier = Modifier.fillMaxWidth(),
    ) {
        SeekButton(
            imageVector = vectorResource(id = R.drawable.rewind_10),
            onClick = { onSeekTo(maxOf(0f, position - 10)) },
        )

        PlayPauseButton(
            state = state,
            onPlayPause = onPlayPause,
        )

        SeekButton(
            imageVector = vectorResource(id = R.drawable.fast_forward_30),
            onClick = { onSeekTo(minOf(duration, position + 30)) },
        )
    }
}

@Composable
fun PlayPauseButton(state: PlayState, onPlayPause: () -> Unit) {
    val drawable = when (state) {
        PlayState.PAUSED -> R.drawable.play
        PlayState.PLAYING -> R.drawable.pause
    }

    FloatingActionButton(
        modifier = Modifier.padding(8.dp),
        onClick = onPlayPause
    ) {
        Icon(
            imageVector = vectorResource(id = drawable),
            contentDescription = "Play/Pause"
        )
    }
}

@Composable
fun SeekButton(imageVector: ImageVector, onClick: () -> Unit, contentDescription: String? = null) {
    IconButton(
        modifier = Modifier.padding(8.dp),
        onClick = onClick,
    ) {
        Icon(
            imageVector = imageVector,
            contentDescription = contentDescription,
            tint = Color.White,
        )
    }
}
