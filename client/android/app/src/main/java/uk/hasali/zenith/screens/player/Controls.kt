package uk.hasali.zenith.screens.player

import androidx.activity.compose.BackHandler
import androidx.activity.compose.LocalOnBackPressedDispatcherOwner
import androidx.compose.animation.*
import androidx.compose.animation.core.animateFloatAsState
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.gestures.forEachGesture
import androidx.compose.foundation.layout.*
import androidx.compose.material.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.input.pointer.PointerEventType
import androidx.compose.ui.input.pointer.PointerInputScope
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.unit.dp
import com.google.accompanist.insets.LocalWindowInsets
import kotlinx.coroutines.*
import uk.hasali.zenith.media.SubtitleTrack
import uk.hasali.zenith.media.VideoPlayer

enum class MenuType {
    Subtitle,
    ScaleMode,
}

@OptIn(ExperimentalAnimationApi::class, ExperimentalMaterialApi::class)
@Composable
fun Controls(
    title: String,
    position: Long,
    duration: Long,
    state: VideoPlayer.State,
    isLoading: Boolean,
    isPlaying: Boolean,
    subtitles: List<SubtitleTrack>,
    selectedSubtitle: SubtitleTrack?,
    scaleMode: ScaleMode,
    onSeekStart: () -> Unit,
    onSeekEnd: (Long) -> Unit,
    onTogglePlaying: () -> Unit,
    onReplay: () -> Unit,
    onSelectSubtitle: (SubtitleTrack?) -> Unit,
    onSetScaleMode: (ScaleMode) -> Unit,
    onClosePressed: () -> Unit,
    visibility: OverlayVisibility = rememberControlsVisibility(),
) {
    var bottomMenu by remember { mutableStateOf<MenuType?>(null) }

    BackHandler(bottomMenu != null) {
        bottomMenu = null
    }

    LaunchedEffect(state, isLoading, isPlaying, bottomMenu) {
        visibility.setAutoHideEnabled(
            state == VideoPlayer.State.Active &&
                    !isLoading &&
                    isPlaying &&
                    bottomMenu == null
        )
    }

    if (!visibility.isVisible) {
        FullScreen()
    }

    val insets = LocalWindowInsets.current
    val insetsPadding = with(LocalDensity.current) {
        PaddingValues(
            top = maxOf(insets.systemBars.top, insets.displayCutout.top).toDp(),
            bottom = maxOf(insets.systemBars.bottom, insets.displayCutout.bottom).toDp(),
            start = maxOf(insets.systemBars.left, insets.displayCutout.left).toDp(),
            end = maxOf(insets.systemBars.right, insets.displayCutout.right).toDp(),
        )
    }

    Box(modifier = Modifier.fillMaxSize()) {
        Controls(
            isVisible = visibility.isVisible,
            title = title,
            position = position,
            duration = duration,
            state = state,
            isLoading = isLoading,
            isPlaying = isPlaying,
            onSeekStart = onSeekStart,
            onSeekEnd = onSeekEnd,
            onTogglePlaying = onTogglePlaying,
            onReplay = onReplay,
            onShowMenu = { bottomMenu = it },
            onClosePressed = onClosePressed,
            padding = insetsPadding,
            modifier = Modifier.pointerInput(visibility) {
                forEachGesture {
                    awaitPointerEventScope {
                        // Wait for pointer press
                        awaitPointerEvent(PointerEventType.Press)

                        // User has started an interaction -> cancel pending hide
                        visibility.cancelHide()

                        // Wait for pointer release
                        val up = awaitPointerEvent(PointerEventType.Release)
                        val change = up.changes[0]
                        if (change.isConsumed) {
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

        if (bottomMenu != null) {
            Box(
                modifier = Modifier
                    .fillMaxSize()
                    .pointerInput(Unit) {
                        detectTapGestures {
                            visibility.hideAfterDelay()
                            bottomMenu = null
                        }
                    },
            )
        }

        BottomSheetMenu(
            visible = bottomMenu == MenuType.Subtitle,
            padding = insetsPadding,
        ) {
            SubtitlesMenu(
                subtitles = subtitles,
                current = selectedSubtitle,
                onSelectSubtitle = {
                    onSelectSubtitle(it)
                    bottomMenu = null
                },
            )
        }

        BottomSheetMenu(
            visible = bottomMenu == MenuType.ScaleMode,
            padding = insetsPadding,
        ) {
            ScaleModeMenu(
                scaleMode = scaleMode,
                onSetScaleMode = onSetScaleMode,
            )
        }
    }
}

@OptIn(ExperimentalMaterialApi::class)
@Composable
private fun ScaleModeMenu(scaleMode: ScaleMode, onSetScaleMode: (ScaleMode) -> Unit) {
    Column {
        Text(
            text = "Scale mode",
            style = MaterialTheme.typography.subtitle2,
            modifier = Modifier.padding(16.dp),
        )
        Divider()
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .heightIn(max = 400.dp),
        ) {
            ScaleModeMenuItem(
                icon = Icons.Default.FitScreen,
                label = "Fit",
                selected = scaleMode == ScaleMode.Fit,
            ) {
                onSetScaleMode(ScaleMode.Fit)
            }

            ScaleModeMenuItem(
                icon = Icons.Default.CropFree,
                label = "Zoom",
                selected = scaleMode == ScaleMode.Zoom,
            ) {
                onSetScaleMode(ScaleMode.Zoom)
            }
        }
    }
}

@ExperimentalMaterialApi
@Composable
private fun ScaleModeMenuItem(
    icon: ImageVector,
    label: String,
    selected: Boolean,
    onClick: () -> Unit,
) {
    ListItem(
        icon = { Icon(icon, null) },
        trailing = { if (selected) Icon(Icons.Default.Check, null) },
        modifier = Modifier.clickable(onClick = onClick),
    ) {
        Text(label)
    }
}

@Composable
private fun BoxScope.BottomSheetMenu(
    visible: Boolean,
    padding: PaddingValues,
    content: @Composable () -> Unit
) {
    AnimatedVisibility(
        visible = visible,
        enter = slideInVertically { it },
        exit = slideOutVertically { it },
        modifier = Modifier
            .fillMaxWidth()
            .heightIn(max = 600.dp)
            .align(Alignment.BottomCenter)
            .padding(padding),
    ) {
        Surface(
            content = content,
            modifier = Modifier
                .widthIn(max = 480.dp)
                .align(Alignment.BottomCenter),
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
    state: VideoPlayer.State,
    isLoading: Boolean,
    isPlaying: Boolean,
    onSeekStart: () -> Unit,
    onSeekEnd: (Long) -> Unit,
    onTogglePlaying: () -> Unit,
    onReplay: () -> Unit,
    onShowMenu: (MenuType) -> Unit,
    onClosePressed: () -> Unit,
    padding: PaddingValues,
    modifier: Modifier = Modifier,
) {
    val opacity by animateFloatAsState(if (isVisible) 0.4f else 0f)
    val onBackPressedDispatcher = LocalOnBackPressedDispatcherOwner.current?.onBackPressedDispatcher

    CompositionLocalProvider(
        LocalContentColor provides Color.White,
    ) {
        Box(
            modifier = modifier
                .fillMaxSize()
                .background(Color.Black.copy(alpha = opacity))
                .padding(padding),
        ) {
            AnimatedVisibility(
                visible = isVisible,
                enter = slideInVertically() + fadeIn(),
                exit = slideOutVertically() + fadeOut(),
                modifier = Modifier.align(Alignment.TopCenter),
            ) {
                AppBar(
                    title = title,
                    onNavigateUp = { onBackPressedDispatcher?.onBackPressed() },
                    actions = {
                        IconButton(onClick = { onShowMenu(MenuType.ScaleMode) }) {
                            Icon(Icons.Default.AspectRatio, "Aspect ratio")
                        }

                        IconButton(onClick = { onShowMenu(MenuType.Subtitle) }) {
                            Icon(Icons.Default.ClosedCaption, "Captions")
                        }

                        IconButton(onClick = onClosePressed) {
                            Icon(Icons.Default.Close, "Close")
                        }
                    },
                    modifier = Modifier.pointerInput(Unit) {
                        consumeTapGestures()
                    },
                )
            }

            AnimatedVisibility(
                visible = isVisible,
                enter = fadeIn(),
                exit = fadeOut(),
                modifier = Modifier.align(Alignment.Center),
            ) {
                Box(contentAlignment = Alignment.Center, modifier = Modifier.fillMaxWidth()) {
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

                        Box(contentAlignment = Alignment.Center, modifier = Modifier.size(90.dp)) {
                            when (state) {
                                VideoPlayer.State.Active -> {
                                    PrimaryControlsButton(
                                        icon = if (isPlaying) Icons.Default.Pause else Icons.Default.PlayArrow,
                                        onClick = onTogglePlaying,
                                    )
                                }
                                VideoPlayer.State.Ended -> {
                                    PrimaryControlsButton(
                                        icon = Icons.Default.Replay,
                                        onClick = onReplay,
                                    )
                                }
                            }
                        }

                        SeekButton(Icons.Default.Forward30) {
                            onSeekEnd(minOf(duration, position + 30))
                        }
                    }

                    if (isLoading) {
                        CircularProgressIndicator(
                            color = LocalContentColor.current,
                            strokeWidth = 6.dp,
                            modifier = Modifier.size(90.dp),
                        )
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

@Composable
fun rememberControlsVisibility(permanent: Boolean = false) =
    remember(permanent) { OverlayVisibility(permanent) }

class OverlayVisibility(private val permanent: Boolean) {
    private var enabled = true

    private var _isVisible by mutableStateOf(true)
    val isVisible: Boolean
        get() = _isVisible || permanent

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
