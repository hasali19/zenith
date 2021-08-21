package uk.hasali.zenith.screens

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.height
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.produceState
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import uk.hasali.zenith.Episode
import uk.hasali.zenith.Season
import uk.hasali.zenith.Show
import uk.hasali.zenith.ui.*

@Composable
fun EpisodeDetailsScreen(id: Int, onPlay: (replay: Boolean) -> Unit, onNavigateUp: () -> Unit) {
    val client = LocalZenithClient.current

    val episode by produceState<Episode?>(null) {
        value = client.getEpisode(id)
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

    if (show == null || season == null || episode == null) {
        CenteredLoadingIndicator()
    } else {
        EpisodeDetailsScreen(
            show = show!!,
            season = season!!,
            episode = episode!!,
            onPlay = onPlay,
            onNavigateUp = onNavigateUp,
        )
    }
}

@Composable
private fun EpisodeDetailsScreen(
    show: Show,
    season: Season,
    episode: Episode,
    onPlay: (Boolean) -> Unit,
    onNavigateUp: () -> Unit,
) {
    VideoItemDetailsScreen(
        id = episode.id,
        backdrop = episode.thumbnail,
        poster = season.poster,
        overview = episode.overview,
        isWatched = episode.isWatched,
        headerContent = { HeaderContent(show = show, season = season, episode = episode) },
        onPlay = onPlay,
        onNavigateUp = onNavigateUp,
    )
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
