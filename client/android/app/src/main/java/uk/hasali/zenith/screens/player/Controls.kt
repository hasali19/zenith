package uk.hasali.zenith.screens.player

import androidx.compose.animation.*
import androidx.compose.animation.core.animateFloatAsState
import androidx.compose.foundation.background
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material.LocalContentColor
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Forward30
import androidx.compose.material.icons.filled.Replay10
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.input.pointer.pointerInput
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch

class ControlsController(private val scope: CoroutineScope) {
    private var _visible by mutableStateOf(false)
    val visible get() = _visible

    private var job: Job? = null
    private var isPlaying = true

    /**
     * Toggles the current visibility. If the controls are shown, a coroutine
     * is started to hide them after a delay.
     */
    fun toggle() {
        if (!_visible) {
            showAndHideDelayed()
        } else {
            _visible = false
        }
    }

    /**
     * Shows the controls immediately, and starts a coroutine to hide
     * them after a delay.
     */
    fun showAndHideDelayed() {
        _visible = true
        job?.cancel()
        job = scope.launch {
            delay(5000)
            _visible = false
        }
    }

    /**
     * Cancels the current delayed hide job (if any).
     */
    fun cancelHide() {
        job?.cancel()
        job = null
    }

    /**
     * Notifies controller of playback state changes.
     *
     * If the video has been paused, the current hide job is cancelled
     * and the controls are set to be visible indefinitely.
     *
     * If the video is resumed, controls will be hidden after a delay.
     */
    fun setPlaying(isPlaying: Boolean) {
        if (!isPlaying && this.isPlaying) {
            cancelHide()
            _visible = true
        } else if (isPlaying && !this.isPlaying) {
            showAndHideDelayed()
        }

        this.isPlaying = isPlaying
    }
}

@Composable
private fun rememberControlsController(): ControlsController {
    val scope = rememberCoroutineScope()
    val controls = remember { ControlsController(scope) }

    // Set controls as initially visible
    DisposableEffect(Unit) {
        controls.showAndHideDelayed()
        onDispose { }
    }

    return controls
}

@OptIn(ExperimentalAnimationApi::class)
@Composable
fun Controls(
    title: String,
    position: Long,
    duration: Long,
    isPlaying: Boolean,
    onSeekStart: () -> Unit,
    onSeekEnd: (Long) -> Unit,
    onTogglePlaying: () -> Unit,
    onShowSubtitlesMenu: () -> Unit,
    onLaunchExternal: () -> Unit,
    onBackPressed: () -> Unit,
) {
    val controls = rememberControlsController()
    val opacity by animateFloatAsState(if (controls.visible) 0.4f else 0f)

    SideEffect {
        controls.setPlaying(isPlaying)
    }

    CompositionLocalProvider(
        LocalContentColor provides Color.White,
    ) {
        Box(
            modifier = Modifier
                .fillMaxSize()
                .background(Color.Black.copy(alpha = opacity))
                .pointerInput(Unit) {
                    detectTapGestures(onTap = { controls.toggle() })
                },
        ) {
            AnimatedVisibility(
                visible = controls.visible,
                enter = slideInVertically() + fadeIn(),
                exit = slideOutVertically() + fadeOut(),
                modifier = Modifier.align(Alignment.TopCenter),
            ) {
                AppBar(
                    title = title,
                    onBackPressed = onBackPressed,
                    onShowSubtitlesMenu = onShowSubtitlesMenu,
                    onLaunchExternal = onLaunchExternal,
                )
            }

            AnimatedVisibility(
                visible = controls.visible,
                enter = fadeIn() + expandIn(expandFrom = Alignment.Center),
                exit = fadeOut() + shrinkOut(shrinkTowards = Alignment.Center),
                modifier = Modifier.align(Alignment.Center)
            ) {
                Row(
                    verticalAlignment = Alignment.CenterVertically,
                    modifier = Modifier.pointerInput(Unit) { detectTapGestures { /* Consume tap events */ } },
                ) {
                    SeekButton(Icons.Default.Replay10) {
                        controls.showAndHideDelayed()
                        onSeekEnd(maxOf(0, position - 10))
                    }

                    PlayPauseButton(isPlaying = isPlaying, onClick = {
                        controls.showAndHideDelayed()
                        onTogglePlaying()
                    })

                    SeekButton(Icons.Default.Forward30) {
                        controls.showAndHideDelayed()
                        onSeekEnd(minOf(duration, position + 30))
                    }
                }
            }

            AnimatedVisibility(
                visible = controls.visible,
                enter = slideInVertically(initialOffsetY = { it / 2 }) + fadeIn(),
                exit = slideOutVertically(targetOffsetY = { it / 2 }) + fadeOut(),
                modifier = Modifier.align(Alignment.BottomCenter),
            ) {
                SeekBar(
                    position = position,
                    duration = duration,
                    onSeekStart = {
                        controls.cancelHide()
                        onSeekStart()
                    },
                    onSeekEnd = {
                        controls.showAndHideDelayed()
                        onSeekEnd(it)
                    },
                )
            }
        }
    }
}
