package uk.hasali.zenith.screens.player

import androidx.compose.animation.*
import androidx.compose.animation.core.animateFloatAsState
import androidx.compose.foundation.background
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.gestures.forEachGesture
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Forward30
import androidx.compose.material.icons.filled.Replay10
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.input.pointer.PointerEventType
import androidx.compose.ui.input.pointer.PointerInputScope
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.*

@OptIn(ExperimentalAnimationApi::class, ExperimentalMaterialApi::class)
@Composable
fun Controls(
    title: String,
    position: Long,
    duration: Long,
    isPlaying: Boolean,
    subtitles: List<SubtitleTrack>,
    selectedSubtitle: SubtitleTrack?,
    onSeekStart: () -> Unit,
    onSeekEnd: (Long) -> Unit,
    onTogglePlaying: () -> Unit,
    onSelectSubtitle: (SubtitleTrack?) -> Unit,
    onLaunchExternal: () -> Unit,
    onBackPressed: () -> Unit,
) {
    val scope = rememberCoroutineScope()
    val sheetState = rememberModalBottomSheetState(ModalBottomSheetValue.Hidden)
    val visibility = remember { OverlayVisibility() }

    LaunchedEffect(isPlaying) {
        visibility.setAutoHideEnabled(isPlaying)
    }

    ModalBottomSheetLayout(
        sheetState = sheetState,
        scrimColor = MaterialTheme.colors.surface.copy(alpha = 0.32f),
        sheetContent = {
            SubtitlesMenu(
                subtitles = subtitles,
                current = selectedSubtitle,
                onSelectSubtitle = {
                    onSelectSubtitle(it)
                    scope.launch {
                        sheetState.hide()
                    }
                },
            )
        },
    ) {
        Controls(
            isVisible = visibility.isVisible || sheetState.isVisible,
            title = title,
            position = position,
            duration = duration,
            isPlaying = isPlaying,
            onSeekStart = onSeekStart,
            onSeekEnd = onSeekEnd,
            onTogglePlaying = onTogglePlaying,
            onShowSubtitlesMenu = { scope.launch { sheetState.show() } },
            onLaunchExternal = onLaunchExternal,
            onBackPressed = onBackPressed,
            modifier = Modifier.pointerInput(Unit) {
                forEachGesture {
                    awaitPointerEventScope {
                        // Wait for pointer press
                        awaitPointerEvent(PointerEventType.Press)

                        // User has started an interaction -> cancel pending hide
                        visibility.cancelHide()

                        // Wait for pointer release
                        val up = awaitPointerEvent(PointerEventType.Release)
                        val change = up.changes[0]
                        if (change.consumed.downChange) {
                            // User interacted with something -> hide after a delay
                            visibility.hideAfterDelay()
                        } else {
                            // User pressed blank area -> toggle controls
                            visibility.toggleVisibility()
                        }
                    }
                }
            },
        )
    }
}

@OptIn(ExperimentalAnimationApi::class)
@Composable
private fun Controls(
    isVisible: Boolean,
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
    modifier: Modifier = Modifier,
) {
    val opacity by animateFloatAsState(if (isVisible) 0.4f else 0f)

    CompositionLocalProvider(
        LocalContentColor provides Color.White,
    ) {
        Box(
            modifier = Modifier
                .fillMaxSize()
                .background(Color.Black.copy(alpha = opacity))
                .then(modifier),
        ) {
            AnimatedVisibility(
                visible = isVisible,
                enter = slideInVertically() + fadeIn(),
                exit = slideOutVertically() + fadeOut(),
                modifier = Modifier.align(Alignment.TopCenter),
            ) {
                AppBar(
                    title = title,
                    onBackPressed = onBackPressed,
                    onShowSubtitlesMenu = onShowSubtitlesMenu,
                    onLaunchExternal = onLaunchExternal,
                    modifier = Modifier.pointerInput(Unit) { consumeTapGestures() }
                )
            }

            AnimatedVisibility(
                visible = isVisible,
                enter = fadeIn(),
                exit = fadeOut(),
                modifier = Modifier.align(Alignment.Center)
            ) {
                Row(
                    horizontalArrangement = Arrangement.SpaceEvenly,
                    verticalAlignment = Alignment.CenterVertically,
                    modifier = Modifier
                        .widthIn(max = 400.dp)
                        .fillMaxWidth()
                        .pointerInput(Unit) { consumeTapGestures() },
                ) {
                    SeekButton(Icons.Default.Replay10) {
                        onSeekEnd(maxOf(0, position - 10))
                    }

                    PlayPauseButton(isPlaying = isPlaying, onClick = {
                        onTogglePlaying()
                    })

                    SeekButton(Icons.Default.Forward30) {
                        onSeekEnd(minOf(duration, position + 30))
                    }
                }
            }

            AnimatedVisibility(
                visible = isVisible,
                enter = slideInVertically(initialOffsetY = { it / 2 }) + fadeIn(),
                exit = slideOutVertically(targetOffsetY = { it / 2 }) + fadeOut(),
                modifier = Modifier.align(Alignment.BottomCenter),
            ) {
                SeekBar(
                    position = position,
                    duration = duration,
                    onSeekStart = { onSeekStart() },
                    onSeekEnd = { onSeekEnd(it) },
                    modifier = Modifier.pointerInput(Unit) { consumeTapGestures() },
                )
            }
        }
    }
}

private class OverlayVisibility {
    private var enabled = true

    private var _isVisible by mutableStateOf(true)
    val isVisible: Boolean
        get() = _isVisible

    private val scope = CoroutineScope(Dispatchers.Main)
    private var job: Job? = null

    fun toggleVisibility() {
        if (_isVisible) {
            cancelHide()
            _isVisible = false
        } else {
            _isVisible = true
            hideAfterDelay()
        }
    }

    fun hideAfterDelay() {
        if (enabled) {
            cancelHide()
            job = scope.launch {
                delay(5000)
                _isVisible = false
            }
        }
    }

    fun cancelHide() {
        job?.cancel()
    }

    fun setAutoHideEnabled(enabled: Boolean) {
        if (enabled) {
            this.enabled = true
            hideAfterDelay()
        } else {
            this.enabled = false
            cancelHide()
        }
    }
}

private suspend fun PointerInputScope.consumeTapGestures() {
    detectTapGestures { /* Consume gestures */ }
}
