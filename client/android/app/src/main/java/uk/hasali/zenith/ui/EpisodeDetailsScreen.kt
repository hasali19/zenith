package uk.hasali.zenith.ui

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.height
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import uk.hasali.zenith.Episode
import uk.hasali.zenith.Season
import uk.hasali.zenith.Show

@Composable
fun EpisodeDetailsScreen(show: Show, season: Season, episode: Episode) {
    VideoItemDetailsScreen(
        id = episode.id,
        backdrop = episode.thumbnail,
        poster = season.poster,
        overview = episode.overview,
        isWatched = episode.isWatched,
        headerContent = { HeaderContent(show = show, season = season, episode = episode) },
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
