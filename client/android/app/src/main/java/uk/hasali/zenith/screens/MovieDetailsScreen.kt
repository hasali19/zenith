package uk.hasali.zenith.screens

import androidx.compose.foundation.layout.Column
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.produceState
import androidx.compose.runtime.rememberCoroutineScope
import kotlinx.coroutines.launch
import kotlinx.datetime.Instant
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import uk.hasali.zenith.Movie
import uk.hasali.zenith.ui.CenteredLoadingIndicator
import uk.hasali.zenith.ui.LocalZenithClient
import uk.hasali.zenith.ui.VideoItemDetailsScreen
import uk.hasali.zenith.ui.displayDuration

@Composable
fun MovieDetailsScreen(id: Int, onPlay: (replay: Boolean) -> Unit, onNavigateUp: () -> Unit) {
    val scope = rememberCoroutineScope()
    val client = LocalZenithClient.current

    val movie by produceState<Movie?>(null, id) {
        value = client.getItem(id) as Movie
    }

    MovieDetailsScreen(
        movie = movie,
        onPlay = onPlay,
        onTranscode = { scope.launch { client.startTranscode(id) } },
        onRefreshMetadata = { scope.launch { client.refreshMetadata(id) } },
        onNavigateUp = onNavigateUp,
    )
}

@Composable
private fun MovieDetailsScreen(
    movie: Movie?,
    onPlay: (Boolean) -> Unit,
    onTranscode: () -> Unit,
    onRefreshMetadata: () -> Unit,
    onNavigateUp: () -> Unit,
) {
    when (movie) {
        null -> CenteredLoadingIndicator()
        else -> VideoItemDetailsScreen(
            backdrop = movie.backdrop,
            poster = movie.poster,
            overview = movie.overview,
            isWatched = movie.userData.isWatched,
            headerContent = { HeaderContent(movie = movie) },
            info = movie.videoInfo,
            userData = movie.userData,
            onPlay = onPlay,
            onTranscode = onTranscode,
            onRefreshMetadata = onRefreshMetadata,
            onNavigateUp = onNavigateUp,
        )
    }
}

@Composable
private fun HeaderContent(movie: Movie) {
    Column {
        val duration = displayDuration(movie.videoInfo.duration)
        val year = movie.releaseDate?.let {
            Instant.fromEpochSeconds(it)
                .toLocalDateTime(TimeZone.UTC)
                .year
        }

        val secondary = if (year != null) {
            "$year - $duration"
        } else {
            duration
        }

        Text(movie.title, style = MaterialTheme.typography.h6)
        Text(secondary, style = MaterialTheme.typography.caption)
    }
}
