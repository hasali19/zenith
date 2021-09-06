package uk.hasali.zenith.screens

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.height
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.produceState
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.launch
import uk.hasali.zenith.Episode
import uk.hasali.zenith.Season
import uk.hasali.zenith.Show
import uk.hasali.zenith.VideoInfo
import uk.hasali.zenith.ui.*

@Composable
fun EpisodeDetailsScreen(id: Int, onPlay: (replay: Boolean) -> Unit, onNavigateUp: () -> Unit) {
    val scope = rememberCoroutineScope()
    val client = LocalZenithClient.current

    val episode by produceState<Episode?>(null, id) {
        value = client.getEpisode(id)
    }

    val info by produceState<VideoInfo?>(null, id) {
        value = client.getVideoInfo(id)
    }

    val season by produceState<Season?>(null, episode) {
        episode?.let { episode ->
            value = client.getSeason(episode.seasonId)
        }
    }

    val show by produceState<Show?>(null, episode) {
        episode?.let { episode ->
            value = client.getShow(episode.showId)
        }
    }

    EpisodeDetailsScreen(
        show = show,
        season = season,
        episode = episode,
        info = info,
        onPlay = onPlay,
        onTranscode = { scope.launch { client.startTranscode(id) } },
        onRefreshMetadata = { scope.launch { client.refreshMetadata(id) } },
        onNavigateUp = onNavigateUp,
    )
}

@Composable
private fun EpisodeDetailsScreen(
    show: Show?,
    season: Season?,
    episode: Episode?,
    info: VideoInfo?,
    onPlay: (Boolean) -> Unit,
    onTranscode: () -> Unit,
    onRefreshMetadata: () -> Unit,
    onNavigateUp: () -> Unit,
) {
    when {
        show == null || season == null || episode == null -> CenteredLoadingIndicator()
        else -> VideoItemDetailsScreen(
            backdrop = episode.thumbnail,
            poster = season.poster,
            overview = episode.overview,
            isWatched = episode.isWatched,
            headerContent = { HeaderContent(show = show, season = season, episode = episode) },
            info = info,
            onPlay = onPlay,
            onTranscode = onTranscode,
            onRefreshMetadata = onRefreshMetadata,
            onNavigateUp = onNavigateUp,
        )
    }
}

@Composable
private fun HeaderContent(show: Show, season: Season, episode: Episode) {
    val seasonNumber = twoDigitNumber(season.seasonNumber)
    val episodeNumber = twoDigitNumber(episode.episodeNumber)
    val duration = displayDuration(episode.duration)

    Column {
        Text(text = show.name, style = MaterialTheme.typography.caption)
        Text(text = episode.name, style = MaterialTheme.typography.h6)
        Spacer(modifier = Modifier.height(8.dp))
        Text(
            text = "S${seasonNumber}E${episodeNumber} - $duration",
            style = MaterialTheme.typography.caption,
        )
    }
}