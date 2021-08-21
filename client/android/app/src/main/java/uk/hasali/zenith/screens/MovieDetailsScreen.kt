package uk.hasali.zenith.screens

import androidx.compose.foundation.layout.Column
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Text
import androidx.compose.runtime.*
import kotlinx.datetime.Instant
import kotlinx.datetime.TimeZone
import kotlinx.datetime.toLocalDateTime
import uk.hasali.zenith.Movie
import uk.hasali.zenith.ui.CenteredLoadingIndicator
import uk.hasali.zenith.ui.LocalZenithClient
import uk.hasali.zenith.ui.VideoItemDetailsScreen

@Composable
fun MovieDetailsScreen(id: Int, onPlay: (replay: Boolean) -> Unit, onNavigateUp: () -> Unit) {
    val client = LocalZenithClient.current
    var movie by remember { mutableStateOf<Movie?>(null) }

    LaunchedEffect(id) {
        movie = client.getMovie(id)
    }

    movie.let {
        if (it == null) {
            CenteredLoadingIndicator()
        } else {
            MovieDetailsScreen(movie = it, onPlay = onPlay, onNavigateUp = onNavigateUp)
        }
    }
}

@Composable
private fun MovieDetailsScreen(movie: Movie, onPlay: (Boolean) -> Unit, onNavigateUp: () -> Unit) {
    VideoItemDetailsScreen(
        id = movie.id,
        backdrop = movie.backdrop,
        poster = movie.poster,
        overview = movie.overview,
        isWatched = false, /* TODO */
        headerContent = { HeaderContent(movie = movie) },
        onPlay = onPlay,
        onNavigateUp = onNavigateUp,
    )
}

@Composable
private fun HeaderContent(movie: Movie) {
    Column {
        val year = Instant.fromEpochSeconds(movie.releaseDate)
            .toLocalDateTime(TimeZone.UTC)
            .year

        Text(movie.title, style = MaterialTheme.typography.h6)
        Text(year.toString(), style = MaterialTheme.typography.caption)
    }
}
