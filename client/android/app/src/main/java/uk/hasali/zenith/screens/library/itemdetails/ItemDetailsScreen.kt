package uk.hasali.zenith.screens.library.itemdetails

import androidx.compose.animation.AnimatedVisibility
import androidx.compose.animation.ExperimentalAnimationApi
import androidx.compose.animation.fadeIn
import androidx.compose.animation.fadeOut
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.LazyListScope
import androidx.compose.foundation.lazy.rememberLazyListState
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Check
import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.platform.LocalDensity
import androidx.compose.ui.unit.Dp
import androidx.compose.ui.unit.dp
import androidx.lifecycle.Lifecycle
import coil.compose.rememberImagePainter
import uk.hasali.zenith.navigation.hiltViewModel
import uk.hasali.zenith.ui.*

@Composable
fun ItemDetailsScreen(
    model: ItemDetailsViewModel = hiltViewModel(),
    bottomSheetController: BottomSheetController,
    onNavigateToPlayer: () -> Unit,
    onNavigateToItem: (id: Int) -> Unit,
    onNavigateUp: () -> Unit,
) {
    val data by rememberFlowWithLifecycle(model.item)
        .collectAsState(null)

    DisposableEffect(Unit) {
        model.enableCastNotifier()
        onDispose {
            model.disableCastNotifier()
        }
    }

    LifecycleEffect(Lifecycle.State.RESUMED) {
        model.refresh()
    }

    val onPlay: (Double?) -> Unit = {
        model.play(it)
        onNavigateToPlayer()
    }

    data.let { data ->
        when (data) {
            null -> CenteredLoadingIndicator()
            is MovieDetails -> MovieDetailsScreen(
                movie = data.movie,
                bottomSheetController = bottomSheetController,
                onSetWatched = model::setWatched,
                onPlay = onPlay,
                onTranscode = model::startTranscode,
                onRefreshMetadata = model::refreshMetadata,
                onImportSubtitle = model::importSubtitle,
                onNavigateUp = onNavigateUp,
            )
            is ShowDetails -> ShowDetailsScreen(
                show = data.show,
                seasons = data.seasons,
                bottomSheetController = bottomSheetController,
                onRefreshMetadata = model::refreshMetadata,
                onNavigateToSeason = { onNavigateToItem(it.id) },
                onNavigateUp = onNavigateUp,
            )
            is SeasonDetails -> SeasonDetailsScreen(
                show = data.show,
                season = data.season,
                episodes = data.episodes,
                bottomSheetController = bottomSheetController,
                onRefreshMetadata = model::refreshMetadata,
                onNavigateToEpisode = { onNavigateToItem(it.id) },
                onNavigateUp = onNavigateUp,
            )
            is EpisodeDetails -> EpisodeDetailsScreen(
                show = data.show,
                season = data.season,
                episode = data.episode,
                bottomSheetController = bottomSheetController,
                onSetWatched = model::setWatched,
                onPlay = onPlay,
                onTranscode = model::startTranscode,
                onRefreshMetadata = model::refreshMetadata,
                onImportSubtitle = model::importSubtitle,
                onNavigateUp = onNavigateUp,
            )
        }
    }
}

@OptIn(ExperimentalAnimationApi::class)
@Composable
fun ItemDetailsScreen(
    backdrop: String?,
    poster: @Composable BoxScope.() -> Unit,
    headerContent: @Composable () -> Unit,
    appBarActions: @Composable RowScope.() -> Unit = {},
    actionsRow: (@Composable () -> Unit)? = null,
    overview: String? = null,
    isWatched: Boolean = false,
    onNavigateUp: () -> Unit,
    content: LazyListScope.(width: Dp) -> Unit = {},
) {
    val listState = rememberLazyListState()

    Surface(modifier = Modifier.fillMaxSize()) {
        BoxWithConstraints(modifier = Modifier.fillMaxSize()) {
            LazyColumn(state = listState, contentPadding = PaddingValues(bottom = 16.dp)) {
                item {
                    val backdropHeight = with(LocalDensity.current) {
                        (constraints.maxWidth * 9f / 16f).toDp()
                    }

                    Box {
                        Box(modifier = Modifier.aspectRatio(16f / 9f)) {
                            Image(
                                painter = rememberImagePainter(
                                    data = backdrop,
                                    builder = { crossfade(true) },
                                ),
                                contentDescription = "Backdrop",
                                contentScale = ContentScale.Crop,
                                modifier = Modifier.fillMaxWidth(),
                            )

                            AnimatedVisibility(
                                visible = isWatched,
                                enter = fadeIn(),
                                exit = fadeOut()
                            ) {
                                Box(
                                    modifier = Modifier
                                        .fillMaxSize()
                                        .background(Color.Black.copy(alpha = 0.4f))
                                ) {
                                    Icon(
                                        imageVector = Icons.Default.Check,
                                        contentDescription = "Watched",
                                        modifier = Modifier
                                            .size(32.dp)
                                            .align(Alignment.Center),
                                        tint = Color.White,
                                    )
                                }
                            }
                        }

                        Column(
                            modifier = Modifier.padding(
                                top = backdropHeight - 48.dp,
                                start = 16.dp,
                                end = 16.dp,
                            )
                        ) {
                            HeaderSection(poster = poster, content = headerContent)
                            Spacer(modifier = Modifier.height(16.dp))

                            if (actionsRow != null) {
                                actionsRow()
                                Spacer(modifier = Modifier.height(16.dp))
                            }

                            if (!overview.isNullOrBlank()) {
                                Text(text = overview, style = MaterialTheme.typography.body2)
                                Spacer(modifier = Modifier.height(16.dp))
                            }
                        }
                    }
                }

                content(this, maxWidth)
            }
        }
    }

    val alpha = if (listState.firstVisibleItemIndex > 0) 1f else {
        listState.firstVisibleItemScrollOffset.toFloat() / 300f
    }

    FadingAppBar(
        alpha = alpha,
        onBackPressed = onNavigateUp,
        actions = appBarActions,
    )
}

@Composable
private fun HeaderSection(
    poster: @Composable BoxScope.() -> Unit,
    content: @Composable () -> Unit,
) {
    Row {
        Box(modifier = Modifier.width(150.dp)) {
            poster()
        }
        Spacer(modifier = Modifier.width(16.dp))
        Box(modifier = Modifier.align(Alignment.CenterVertically)) {
            content()
        }
    }
}
