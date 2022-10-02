package uk.hasali.zenith.screens.library.itemdetails

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.height
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import uk.hasali.zenith.api.Episode
import uk.hasali.zenith.ui.BottomSheetController
import uk.hasali.zenith.ui.formatDuration

@Composable
fun EpisodeDetailsScreen(
    episode: Episode,
    bottomSheetController: BottomSheetController,
    onSetWatched: (Boolean) -> Unit,
    onPlay: (Double?) -> Unit,
    onTranscode: () -> Unit,
    onRefreshMetadata: () -> Unit,
    onImportSubtitle: (String, ByteArray) -> Unit,
    onNavigateUp: () -> Unit,
) {
    VideoItemDetailsScreen(
        name = episode.name ?: "Episode ${episode.episodeNumber}",
        backdrop = episode.thumbnail ?: episode.backdrop,
        poster = episode.poster,
        overview = episode.overview,
        headerContent = { HeaderContent(episode = episode) },
        video = episode,
        bottomSheetController = bottomSheetController,
        onSetWatched = onSetWatched,
        onPlay = onPlay,
        onConvertVideo = onTranscode,
        onRefreshMetadata = onRefreshMetadata,
        onImportSubtitle = onImportSubtitle,
        onNavigateUp = onNavigateUp,
    )
}

@Composable
private fun HeaderContent(episode: Episode) {
    val duration = formatDuration(episode.videoInfo.duration)
    val name = episode.name ?: "Episode ${episode.episodeNumber}"

    Column {
        Text(text = episode.showName, style = MaterialTheme.typography.caption)
        Text(text = name, style = MaterialTheme.typography.h6)
        Spacer(modifier = Modifier.height(8.dp))
        Text(
            text = "${episode.seasonEpisodeString()} - $duration",
            style = MaterialTheme.typography.caption,
        )
    }
}
