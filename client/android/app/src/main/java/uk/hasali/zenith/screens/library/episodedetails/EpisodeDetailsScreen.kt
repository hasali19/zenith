package uk.hasali.zenith.screens.library.episodedetails

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.height
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.lifecycle.Lifecycle
import uk.hasali.zenith.api.Episode
import uk.hasali.zenith.api.Season
import uk.hasali.zenith.api.Show
import uk.hasali.zenith.navigation.hiltViewModel
import uk.hasali.zenith.ui.*

@Composable
fun EpisodeDetailsScreen(
    model: EpisodeDetailsViewModel = hiltViewModel(),
    onPlay: (position: Double?) -> Unit,
    bottomSheetController: BottomSheetController,
    onNavigateUp: () -> Unit,
) {
    val state by rememberFlowWithLifecycle(model.state)
        .collectAsState(EpisodeDetailsViewState())

    LifecycleEffect(Lifecycle.State.RESUMED) {
        model.refresh()
    }

    EpisodeDetailsScreen(
        show = state.show,
        season = state.season,
        episode = state.episode,
        onSetWatched = model::setWatched,
        bottomSheetController = bottomSheetController,
        onPlay = onPlay,
        onTranscode = model::startTranscode,
        onRefreshMetadata = model::refreshMetadata,
        onImportSubtitle = model::importSubtitle,
        onNavigateUp = onNavigateUp,
    )
}

@Composable
private fun EpisodeDetailsScreen(
    show: Show?,
    season: Season?,
    episode: Episode?,
    onSetWatched: (Boolean) -> Unit,
    bottomSheetController: BottomSheetController,
    onPlay: (Double?) -> Unit,
    onTranscode: () -> Unit,
    onRefreshMetadata: () -> Unit,
    onImportSubtitle: (String, ByteArray) -> Unit,
    onNavigateUp: () -> Unit,
) {
    when {
        show == null || season == null || episode == null -> CenteredLoadingIndicator()
        else -> VideoItemDetailsScreen(
            name = episode.name ?: "Episode ${episode.episodeNumber}",
            backdrop = episode.thumbnail,
            poster = season.poster,
            overview = episode.overview,
            headerContent = { HeaderContent(show = show, episode = episode) },
            info = episode.videoInfo,
            userData = episode.userData,
            bottomSheetController = bottomSheetController,
            onSetWatched = onSetWatched,
            onPlay = onPlay,
            onConvertVideo = onTranscode,
            onRefreshMetadata = onRefreshMetadata,
            onImportSubtitle = onImportSubtitle,
            onNavigateUp = onNavigateUp,
        )
    }
}

@Composable
private fun HeaderContent(show: Show, episode: Episode) {
    val seasonNumber = twoDigitNumber(episode.seasonNumber)
    val episodeNumber = twoDigitNumber(episode.episodeNumber)
    val duration = displayDuration(episode.videoInfo.duration)
    val name = episode.name ?: "Episode ${episode.episodeNumber}"

    Column {
        Text(text = show.name, style = MaterialTheme.typography.caption)
        Text(text = name, style = MaterialTheme.typography.h6)
        Spacer(modifier = Modifier.height(8.dp))
        Text(
            text = "S${seasonNumber}E${episodeNumber} - $duration",
            style = MaterialTheme.typography.caption,
        )
    }
}
