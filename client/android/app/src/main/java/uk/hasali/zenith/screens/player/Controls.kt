package uk.hasali.zenith.screens.player

import androidx.compose.animation.*
import androidx.compose.animation.core.animateFloatAsState
import androidx.compose.foundation.background
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Forward30
import androidx.compose.material.icons.filled.Replay10
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.input.pointer.pointerInput
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import uk.hasali.zenith.SubtitleStreamInfo

@OptIn(ExperimentalAnimationApi::class, ExperimentalMaterialApi::class)
@Composable
fun Controls(
    title: String,
    position: Long,
    duration: Long,
    isPlaying: Boolean,
    subtitles: List<SubtitleStreamInfo>,
    selectedSubtitle: SubtitleStreamInfo?,
    onSeekStart: () -> Unit,
    onSeekEnd: (Long) -> Unit,
    onTogglePlaying: () -> Unit,
    onSelectSubtitle: (SubtitleStreamInfo?) -> Unit,
    onLaunchExternal: () -> Unit,
    onBackPressed: () -> Unit,
) {
    val scope = rememberCoroutineScope()
    val sheetState = rememberModalBottomSheetState(ModalBottomSheetValue.Hidden)

    // interactionCount is incremented on any user interaction, triggering the effect
    // below to reset the delayed hide
    var interactionCount by remember { mutableStateOf(0) }
    var isInteracting by remember { mutableStateOf(false) }
    var isVisible by remember { mutableStateOf(true) }

    LaunchedEffect(isPlaying, isInteracting, interactionCount) {
        if (!isPlaying) {
            isVisible = true
        } else if (!isInteracting) {
            delay(5000)
            isVisible = false
        }
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
            isVisible = isVisible || sheetState.isVisible,
            title = title,
            position = position,
            duration = duration,
            isPlaying = isPlaying,
            onSeekStart = {
                interactionCount++
                isInteracting = true
                onSeekStart()
            },
            onSeekEnd = {
                interactionCount++
                isInteracting = false
                onSeekEnd(it)
            },
            onTogglePlaying = {
                interactionCount++
                onTogglePlaying()
            },
            onShowSubtitlesMenu = {
                interactionCount++
                scope.launch { sheetState.show() }
            },
            onLaunchExternal = onLaunchExternal,
            onBackPressed = onBackPressed,
            onToggleVisibility = {
                interactionCount++
                isVisible = !isVisible
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
    onToggleVisibility: () -> Unit,
) {
    val opacity by animateFloatAsState(if (isVisible) 0.4f else 0f)

    CompositionLocalProvider(
        LocalContentColor provides Color.White,
    ) {
        Box(
            modifier = Modifier
                .fillMaxSize()
                .background(Color.Black.copy(alpha = opacity))
                .pointerInput(Unit) {
                    detectTapGestures(onTap = { onToggleVisibility() })
                },
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
                )
            }

            AnimatedVisibility(
                visible = isVisible,
                enter = fadeIn() + expandIn(expandFrom = Alignment.Center),
                exit = fadeOut() + shrinkOut(shrinkTowards = Alignment.Center),
                modifier = Modifier.align(Alignment.Center)
            ) {
                Row(
                    verticalAlignment = Alignment.CenterVertically,
                    modifier = Modifier.pointerInput(Unit) { detectTapGestures { /* Consume tap events */ } },
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
                )
            }
        }
    }
}
